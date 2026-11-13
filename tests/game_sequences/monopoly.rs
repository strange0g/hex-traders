//! Fixed-layout production and dice from the independent pinned-rand oracle.
use super::seat;
use hex_traders_engine::{
    CardId, Command, CommandEnvelope, DevelopmentKind, Engine, Event, GameConfig, LegalAction,
    Phase, ProjectedEvent, ProjectedReceipt, Receipt, Resource, Resources, RuleError, Viewer,
};

fn act(engine: &mut Engine, actor: u8, command: Command) -> Receipt {
    let revision = engine.view(Viewer::Spectator).unwrap().revision;
    assert!(engine.validate(seat(actor), &command).is_ok());
    let receipt = engine
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(actor),
            expected_revision: revision,
            command,
        })
        .unwrap();
    assert_eq!(
        (receipt.previous_revision, receipt.revision),
        (revision, revision + 1)
    );
    receipt
}
fn hands(engine: &Engine, expected: &[[u8; 5]]) {
    for (id, values) in expected.iter().enumerate() {
        assert_eq!(
            engine
                .view(Viewer::Seat(seat(id as u8)))
                .unwrap()
                .own_player
                .unwrap()
                .resources
                .counts(),
            values
        );
    }
}
fn monopoly_choice(engine: &Engine, actor: u8, expected: bool) {
    let card = CardId::try_from(16).unwrap();
    let command = Command::PlayMonopoly {
        card,
        resource: Resource::Brick,
    };
    assert_eq!(engine.validate(seat(actor), &command).is_ok(), expected);
    assert_eq!(
        engine
            .legal_actions(seat(actor))
            .unwrap()
            .choices
            .contains(&LegalAction::PlayMonopoly { cards: vec![card] }),
        expected
    );
}

#[test]
fn monopoly_public_purchase_play_and_original_receipts_in_both_phases() {
    struct Fixture<'a> {
        first: u8,
        seats: &'a [u8],
        rolls: &'a [(u8, [u8; 2])],
        buyer: u8,
        before: &'a [[u8; 5]],
        bank_before: [u8; 5],
        transfers: [u8; 4],
        after: &'a [[u8; 5]],
        next_dice: [u8; 2],
        final_hands: &'a [[u8; 5]],
        bank_after: [u8; 5],
        following_dice: [u8; 2],
    }
    let cases = [
        Fixture {
            first: 5,
            seats: &[0, 1, 2],
            rolls: &[(0, [1, 5]), (1, [5, 4]), (2, [3, 2]), (0, [6, 5])],
            buyer: 1,
            before: &[[2, 1, 1, 1, 1], [1, 1, 1, 0, 0], [2, 1, 1, 2, 1]],
            bank_before: [14, 16, 16, 16, 17],
            transfers: [2, 0, 2, 0],
            after: &[[0, 1, 1, 1, 1], [5, 1, 1, 0, 0], [0, 1, 1, 2, 1]],
            next_dice: [3, 2],
            final_hands: &[[0, 1, 2, 1, 1], [5, 1, 1, 0, 0], [0, 1, 1, 3, 1]],
            bank_after: [14, 16, 15, 15, 17],
            following_dice: [2, 5],
        },
        Fixture {
            first: 89,
            seats: &[0, 1, 2, 3],
            rolls: &[
                (1, [6, 2]),
                (2, [6, 6]),
                (3, [1, 4]),
                (0, [4, 6]),
                (1, [4, 6]),
                (2, [5, 4]),
                (3, [4, 4]),
            ],
            buyer: 0,
            before: &[
                [2, 2, 0, 0, 0],
                [1, 1, 1, 3, 0],
                [1, 0, 1, 2, 1],
                [4, 2, 2, 2, 0],
            ],
            bank_before: [11, 14, 15, 12, 18],
            transfers: [0, 1, 1, 4],
            after: &[
                [8, 2, 0, 0, 0],
                [0, 1, 1, 3, 0],
                [0, 0, 1, 2, 1],
                [0, 2, 2, 2, 0],
            ],
            next_dice: [5, 3],
            final_hands: &[
                [8, 3, 0, 0, 0],
                [0, 1, 1, 4, 0],
                [0, 0, 1, 2, 1],
                [0, 3, 2, 2, 0],
            ],
            bank_after: [11, 12, 15, 11, 18],
            following_dice: [6, 3],
        },
    ];
    for Fixture {
        first,
        seats,
        rolls,
        buyer,
        before,
        bank_before,
        transfers,
        after,
        next_dice,
        final_hands,
        bank_after,
        following_dice,
    } in cases
    {
        for pre_roll in [true, false] {
            let mut seed = [0; 32];
            seed[0] = first;
            let (mut engine, _) = Engine::create(GameConfig::fixed(seats).unwrap(), seed).unwrap();
            let card = CardId::try_from(16).unwrap();
            let mut purchase = None;
            for &(actor, dice) in rolls {
                assert_eq!(
                    act(&mut engine, actor, Command::RollDice).events[0],
                    Event::DiceRolled {
                        seat: seat(actor),
                        dice
                    }
                );
                assert_eq!(engine.view(Viewer::Spectator).unwrap().phase, Phase::Action);
                if actor == buyer && purchase.is_none() {
                    let receipt = act(&mut engine, buyer, Command::BuyDevelopmentCard);
                    assert!(receipt.events.contains(&Event::DevelopmentPurchased {
                        seat: seat(buyer),
                        card,
                        remaining: 24
                    }));
                    monopoly_choice(&engine, buyer, false);
                    purchase = Some(receipt);
                }
                act(&mut engine, actor, Command::EndTurn);
            }
            let original_purchase = purchase.unwrap();
            let original = engine.view(Viewer::Spectator).unwrap();
            assert_eq!(original.phase, Phase::PreRoll);
            assert_eq!(original.active_player, seat(buyer));
            assert_eq!(original.bank.counts(), &bank_before);
            hands(&engine, before);
            monopoly_choice(&engine, buyer, true);
            for &other in seats.iter().filter(|&&id| id != buyer) {
                assert!(
                    engine
                        .legal_actions(seat(other))
                        .unwrap()
                        .choices
                        .is_empty()
                );
            }
            if !pre_roll {
                assert_eq!(
                    act(&mut engine, buyer, Command::RollDice).events[0],
                    Event::DiceRolled {
                        seat: seat(buyer),
                        dice: next_dice
                    }
                );
            }
            let played = act(
                &mut engine,
                buyer,
                Command::PlayMonopoly {
                    card,
                    resource: Resource::Brick,
                },
            );
            assert_eq!(
                played.events,
                vec![Event::MonopolyPlayed {
                    seat: seat(buyer),
                    card,
                    resource: Resource::Brick,
                    transfers
                }]
            );
            assert_eq!(
                engine.view(Viewer::Spectator).unwrap().phase,
                if pre_roll {
                    Phase::PreRoll
                } else {
                    Phase::Action
                }
            );
            monopoly_choice(&engine, buyer, false);
            assert_eq!(
                engine
                    .validate(
                        seat(buyer),
                        &Command::PlayMonopoly {
                            card,
                            resource: Resource::Brick
                        }
                    )
                    .unwrap_err()
                    .code,
                RuleError::CardNotPlayable
            );
            if pre_roll {
                hands(&engine, after);
                assert_eq!(
                    engine.view(Viewer::Spectator).unwrap().bank.counts(),
                    &bank_before
                );
                assert_eq!(
                    act(&mut engine, buyer, Command::RollDice).events[0],
                    Event::DiceRolled {
                        seat: seat(buyer),
                        dice: next_dice
                    }
                );
            }
            hands(&engine, final_hands);
            assert_eq!(
                engine.view(Viewer::Spectator).unwrap().bank.counts(),
                &bank_after
            );
            let next = seats[(seats.iter().position(|&id| id == buyer).unwrap() + 1) % seats.len()];
            act(&mut engine, buyer, Command::EndTurn);
            for viewer in std::iter::once(Viewer::Spectator)
                .chain(seats.iter().map(|&id| Viewer::Seat(seat(id))))
            {
                let view = engine.view(viewer).unwrap();
                assert_eq!(
                    view.players
                        .iter()
                        .find(|player| player.seat == seat(buyer))
                        .unwrap()
                        .spent_progress,
                    vec![DevelopmentKind::Monopoly]
                );
                assert_eq!(
                    engine.project_receipt(&played, viewer).unwrap(),
                    ProjectedReceipt {
                        revision: played.revision,
                        events: vec![ProjectedEvent::MonopolyPlayed {
                            seat: seat(buyer),
                            card,
                            resource: Resource::Brick,
                            transfers,
                        }],
                    }
                );
                assert_eq!(
                    engine.project_receipt(&original_purchase, viewer).unwrap(),
                    ProjectedReceipt {
                        revision: original_purchase.revision,
                        events: vec![
                            ProjectedEvent::ConstructionPaid {
                                seat: seat(buyer),
                                resources: Resources::try_from([0, 0, 1, 1, 1]).unwrap()
                            },
                            ProjectedEvent::DevelopmentPurchased {
                                seat: seat(buyer),
                                remaining: 24,
                                card: if viewer == Viewer::Seat(seat(buyer)) {
                                    Some(card)
                                } else {
                                    None
                                },
                            },
                        ],
                    }
                );
            }
            assert_eq!(
                act(&mut engine, next, Command::RollDice).events[0],
                Event::DiceRolled {
                    seat: seat(next),
                    dice: following_dice
                }
            );
        }
    }
}
