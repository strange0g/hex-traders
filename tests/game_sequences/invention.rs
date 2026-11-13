//! Pinned-rand and source-layout oracle histories for public Invention play.
use super::seat;
use hex_traders_engine::{
    CardId, Command, CommandEnvelope, DevelopmentKind, Engine, Event, GameConfig, LegalAction,
    Phase, ProjectedEvent, ProjectedReceipt, Receipt, Resources, RuleError, Viewer,
};
fn resources(counts: [u8; 5]) -> Resources {
    Resources::try_from(counts).unwrap()
}
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
    for (id, counts) in expected.iter().enumerate() {
        assert_eq!(
            engine
                .view(Viewer::Seat(seat(id as u8)))
                .unwrap()
                .own_player
                .unwrap()
                .resources
                .counts(),
            counts
        );
    }
}
fn choices(engine: &Engine, actor: u8, selection: Resources, available: bool) {
    let card = CardId::try_from(18).unwrap();
    assert_eq!(
        engine
            .validate(
                seat(actor),
                &Command::PlayInvention {
                    card,
                    resources: selection
                }
            )
            .is_ok(),
        available
    );
    assert_eq!(
        engine
            .legal_actions(seat(actor))
            .unwrap()
            .choices
            .contains(&LegalAction::PlayInvention { cards: vec![card] }),
        available
    );
}
struct Choice<'a> {
    selection: [u8; 5],
    before_roll: &'a [[u8; 5]],
    bank_before_roll: [u8; 5],
    after_roll: &'a [[u8; 5]],
    bank_after_roll: [u8; 5],
}
struct Fixture<'a> {
    seats: &'a [u8],
    rolls: &'a [(u8, [u8; 2])],
    buyer: u8,
    before: &'a [[u8; 5]],
    bank_before: [u8; 5],
    next_dice: [u8; 2],
    following_dice: [u8; 2],
    options: [Choice<'a>; 2],
}
#[test]
fn invention_public_purchase_play_and_original_receipts_in_both_phases() {
    let fixtures = [
        Fixture {
            seats: &[0, 1, 2],
            rolls: &[(0, [2, 4]), (1, [1, 4]), (2, [1, 4]), (0, [2, 1])],
            buyer: 1,
            before: &[[1, 0, 2, 2, 1], [0, 1, 0, 0, 0], [2, 0, 2, 3, 0]],
            bank_before: [16, 18, 15, 14, 18],
            next_dice: [2, 6],
            following_dice: [5, 6],
            options: [
                Choice {
                    selection: [0, 0, 2, 0, 0],
                    before_roll: &[[1, 0, 2, 2, 1], [0, 1, 2, 0, 0], [2, 0, 2, 3, 0]],
                    bank_before_roll: [16, 18, 13, 14, 18],
                    after_roll: &[[1, 1, 2, 2, 1], [0, 1, 2, 1, 0], [2, 0, 2, 3, 0]],
                    bank_after_roll: [16, 17, 13, 13, 18],
                },
                Choice {
                    selection: [1, 0, 0, 0, 1],
                    before_roll: &[[1, 0, 2, 2, 1], [1, 1, 0, 0, 1], [2, 0, 2, 3, 0]],
                    bank_before_roll: [15, 18, 15, 14, 17],
                    after_roll: &[[1, 1, 2, 2, 1], [1, 1, 0, 1, 1], [2, 0, 2, 3, 0]],
                    bank_after_roll: [15, 17, 15, 13, 17],
                },
            ],
        },
        Fixture {
            seats: &[0, 1, 2, 3],
            rolls: &[(0, [1, 4]), (1, [1, 4]), (2, [2, 1]), (3, [2, 6])],
            buyer: 0,
            before: &[
                [1, 1, 1, 1, 0],
                [0, 1, 1, 2, 0],
                [1, 0, 2, 3, 0],
                [2, 1, 0, 3, 0],
            ],
            bank_before: [15, 16, 15, 10, 19],
            next_dice: [5, 6],
            following_dice: [5, 1],
            options: [
                Choice {
                    selection: [0, 0, 2, 0, 0],
                    before_roll: &[
                        [1, 1, 3, 1, 0],
                        [0, 1, 1, 2, 0],
                        [1, 0, 2, 3, 0],
                        [2, 1, 0, 3, 0],
                    ],
                    bank_before_roll: [15, 16, 13, 10, 19],
                    after_roll: &[
                        [1, 2, 3, 1, 0],
                        [0, 1, 2, 2, 0],
                        [1, 1, 2, 3, 0],
                        [2, 1, 0, 3, 0],
                    ],
                    bank_after_roll: [15, 14, 12, 10, 19],
                },
                Choice {
                    selection: [1, 0, 0, 0, 1],
                    before_roll: &[
                        [2, 1, 1, 1, 1],
                        [0, 1, 1, 2, 0],
                        [1, 0, 2, 3, 0],
                        [2, 1, 0, 3, 0],
                    ],
                    bank_before_roll: [14, 16, 15, 10, 18],
                    after_roll: &[
                        [2, 2, 1, 1, 1],
                        [0, 1, 2, 2, 0],
                        [1, 1, 2, 3, 0],
                        [2, 1, 0, 3, 0],
                    ],
                    bank_after_roll: [14, 14, 14, 10, 18],
                },
            ],
        },
    ];
    for fixture in fixtures {
        for option in fixture.options {
            for pre_roll in [true, false] {
                let mut seed = [0; 32];
                seed[0] = 8;
                let (mut engine, _) =
                    Engine::create(GameConfig::fixed(fixture.seats).unwrap(), seed).unwrap();
                let card = CardId::try_from(18).unwrap();
                let selection = resources(option.selection);
                let mut purchase = None;
                for &(actor, dice) in fixture.rolls {
                    assert_eq!(
                        act(&mut engine, actor, Command::RollDice).events[0],
                        Event::DiceRolled {
                            seat: seat(actor),
                            dice
                        }
                    );
                    if actor == fixture.buyer && purchase.is_none() {
                        let receipt = act(&mut engine, actor, Command::BuyDevelopmentCard);
                        assert!(receipt.events.contains(&Event::DevelopmentPurchased {
                            seat: seat(actor),
                            card,
                            remaining: 24
                        }));
                        choices(&engine, actor, selection, false);
                        purchase = Some(receipt);
                    }
                    act(&mut engine, actor, Command::EndTurn);
                }
                let original_purchase = purchase.unwrap();
                let before = engine.view(Viewer::Spectator).unwrap();
                assert_eq!(
                    (before.active_player, before.phase),
                    (seat(fixture.buyer), Phase::PreRoll)
                );
                assert_eq!(before.bank.counts(), &fixture.bank_before);
                hands(&engine, fixture.before);
                choices(&engine, fixture.buyer, selection, true);
                for &other in fixture.seats.iter().filter(|&&id| id != fixture.buyer) {
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
                        act(&mut engine, fixture.buyer, Command::RollDice).events[0],
                        Event::DiceRolled {
                            seat: seat(fixture.buyer),
                            dice: fixture.next_dice
                        }
                    );
                }
                let played = act(
                    &mut engine,
                    fixture.buyer,
                    Command::PlayInvention {
                        card,
                        resources: selection,
                    },
                );
                assert_eq!(
                    played.events,
                    vec![Event::InventionPlayed {
                        seat: seat(fixture.buyer),
                        card,
                        resources: selection
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
                choices(&engine, fixture.buyer, selection, false);
                assert_eq!(
                    engine
                        .validate(
                            seat(fixture.buyer),
                            &Command::PlayInvention {
                                card,
                                resources: selection
                            }
                        )
                        .unwrap_err()
                        .code,
                    RuleError::CardNotPlayable
                );
                if pre_roll {
                    hands(&engine, option.before_roll);
                    assert_eq!(
                        engine.view(Viewer::Spectator).unwrap().bank.counts(),
                        &option.bank_before_roll
                    );
                    assert_eq!(
                        act(&mut engine, fixture.buyer, Command::RollDice).events[0],
                        Event::DiceRolled {
                            seat: seat(fixture.buyer),
                            dice: fixture.next_dice
                        }
                    );
                }
                hands(&engine, option.after_roll);
                assert_eq!(
                    engine.view(Viewer::Spectator).unwrap().bank.counts(),
                    &option.bank_after_roll
                );
                let next = fixture.seats[(fixture
                    .seats
                    .iter()
                    .position(|&id| id == fixture.buyer)
                    .unwrap()
                    + 1)
                    % fixture.seats.len()];
                act(&mut engine, fixture.buyer, Command::EndTurn);
                for viewer in std::iter::once(Viewer::Spectator)
                    .chain(fixture.seats.iter().map(|&id| Viewer::Seat(seat(id))))
                {
                    let view = engine.view(viewer).unwrap();
                    for player in &view.players {
                        assert_eq!(
                            player.spent_progress,
                            if player.seat == seat(fixture.buyer) {
                                vec![DevelopmentKind::Invention]
                            } else {
                                vec![]
                            }
                        );
                    }
                    assert_eq!(
                        engine.project_receipt(&played, viewer).unwrap(),
                        ProjectedReceipt {
                            revision: played.revision,
                            events: vec![ProjectedEvent::InventionPlayed {
                                seat: seat(fixture.buyer),
                                card,
                                resources: selection
                            }]
                        }
                    );
                    assert_eq!(
                        engine.project_receipt(&original_purchase, viewer).unwrap(),
                        ProjectedReceipt {
                            revision: original_purchase.revision,
                            events: vec![
                                ProjectedEvent::ConstructionPaid {
                                    seat: seat(fixture.buyer),
                                    resources: resources([0, 0, 1, 1, 1])
                                },
                                ProjectedEvent::DevelopmentPurchased {
                                    seat: seat(fixture.buyer),
                                    remaining: 24,
                                    card: if viewer == Viewer::Seat(seat(fixture.buyer)) {
                                        Some(card)
                                    } else {
                                        None
                                    }
                                }
                            ]
                        }
                    );
                }
                assert_eq!(
                    act(&mut engine, next, Command::RollDice).events[0],
                    Event::DiceRolled {
                        seat: seat(next),
                        dice: fixture.following_dice
                    }
                );
            }
        }
    }
}
