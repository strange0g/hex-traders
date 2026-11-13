//! Public purchase histories: CN3081 fixed hex claims and a separate pinned-rand oracle.
//! Seed20498, all32bytes little-endian first4bytes/restzero, no runtime /tmp dependency.
use super::seat;
use hex_traders_engine::{
    CardId, Command, CommandEnvelope, DevelopmentCardView, DevelopmentKind, Engine, Event,
    GameConfig, LegalAction, Phase, ProjectedEvent, Receipt, Resources, RuleError, Viewer,
};
fn card(id: u8) -> CardId {
    CardId::try_from(id).unwrap()
}
fn act(engine: &mut Engine, actor: u8, command: Command) -> Receipt {
    let expected_revision = engine.view(Viewer::Spectator).unwrap().revision;
    assert!(engine.validate(seat(actor), &command).is_ok());
    let receipt = engine
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(actor),
            expected_revision,
            command,
        })
        .unwrap();
    assert_eq!(receipt.previous_revision, expected_revision);
    assert_eq!(receipt.revision, expected_revision + 1);
    receipt
}
fn purchase_choices(engine: &Engine, seats: &[u8], affordable: bool) {
    for &actor in seats {
        let valid = actor == 2 && affordable;
        assert_eq!(
            engine
                .validate(seat(actor), &Command::BuyDevelopmentCard)
                .is_ok(),
            valid
        );
        assert_eq!(
            engine
                .legal_actions(seat(actor))
                .unwrap()
                .choices
                .contains(&LegalAction::BuyDevelopmentCard),
            valid
        );
    }
}
fn project(engine: &Engine, receipt: &Receipt, seats: &[u8], id: u8, remaining: u8) {
    let viewers =
        std::iter::once(Viewer::Spectator).chain(seats.iter().map(|&id| Viewer::Seat(seat(id))));
    for viewer in viewers {
        let projected = engine.project_receipt(receipt, viewer).unwrap();
        assert_eq!(projected.revision, receipt.revision);
        assert_eq!(
            projected.events,
            vec![
                ProjectedEvent::ConstructionPaid {
                    seat: seat(2),
                    resources: Resources::try_from([0, 0, 1, 1, 1]).unwrap()
                },
                ProjectedEvent::DevelopmentPurchased {
                    seat: seat(2),
                    remaining,
                    card: if viewer == Viewer::Seat(seat(2)) {
                        Some(card(id))
                    } else {
                        None
                    }
                },
            ]
        );
    }
}
fn assert_hands(engine: &Engine, seats: &[u8], expected: &[[u8; 5]]) {
    for (&actor, counts) in seats.iter().zip(expected) {
        assert_eq!(
            engine
                .view(Viewer::Seat(seat(actor)))
                .unwrap()
                .own_player
                .unwrap()
                .resources
                .counts(),
            counts
        );
    }
    let bank = engine.view(Viewer::Spectator).unwrap().bank;
    for resource in 0..5 {
        assert_eq!(
            u16::from(bank.counts()[resource])
                + expected
                    .iter()
                    .map(|hand| u16::from(hand[resource]))
                    .sum::<u16>(),
            19
        );
    }
}
fn run(
    seats: &[u8],
    rolls: &[(u8, [u8; 2])],
    first_turn: u64,
    first_hand: [u8; 5],
    first_bank: [u8; 5],
    final_hands: &[[u8; 5]],
    final_bank: [u8; 5],
) {
    let mut seed = [0; 32];
    seed[..4].copy_from_slice(&20498_u32.to_le_bytes());
    let (mut engine, _) = Engine::create(GameConfig::fixed(seats).unwrap(), seed).unwrap();
    assert_eq!(
        engine.view(Viewer::Spectator).unwrap().starting_player,
        seat(2)
    );
    let mut receipts = Vec::new();
    for (index, &(actor, dice)) in rolls.iter().enumerate() {
        let turn = index as u64 + 1;
        let rolled = act(&mut engine, actor, Command::RollDice);
        assert_eq!(
            rolled.events[0],
            Event::DiceRolled {
                seat: seat(actor),
                dice
            }
        );
        let view = engine.view(Viewer::Spectator).unwrap();
        assert_eq!(view.turn, turn);
        assert_eq!(view.phase, Phase::Action);
        if turn == first_turn {
            assert_eq!(actor, 2);
            assert_eq!(
                engine
                    .view(Viewer::Seat(seat(2)))
                    .unwrap()
                    .own_player
                    .unwrap()
                    .resources
                    .counts(),
                &first_hand
            );
            assert_eq!(view.bank.counts(), &first_bank);
            for (purchase, id) in [24, 5].into_iter().enumerate() {
                purchase_choices(&engine, seats, true);
                let receipt = act(&mut engine, 2, Command::BuyDevelopmentCard);
                assert_eq!(
                    receipt.events[1],
                    Event::DevelopmentPurchased {
                        seat: seat(2),
                        card: card(id),
                        remaining: 24 - purchase as u8
                    }
                );
                project(&engine, &receipt, seats, id, 24 - purchase as u8);
                let own = engine
                    .view(Viewer::Seat(seat(2)))
                    .unwrap()
                    .own_player
                    .unwrap();
                assert_eq!(own.true_score, 3); // Only VP24 scores; Knight5 scores nothing.
                assert!(
                    own.development_cards
                        .iter()
                        .all(|card| card.acquired_turn == first_turn
                            && !card.acquired_on_earlier_turn)
                );
                assert_eq!(
                    own.resources.counts(),
                    &[
                        first_hand[0],
                        first_hand[1],
                        1 - purchase as u8,
                        1 - purchase as u8,
                        1 - purchase as u8
                    ]
                );
                assert_eq!(
                    engine.view(Viewer::Spectator).unwrap().bank.counts(),
                    &[
                        first_bank[0],
                        first_bank[1],
                        first_bank[2] + 1 + purchase as u8,
                        first_bank[3] + 1 + purchase as u8,
                        first_bank[4] + 1 + purchase as u8
                    ]
                );
                receipts.push(receipt);
            }
            purchase_choices(&engine, seats, false);
        }
        if index + 1 == rolls.len() {
            assert_eq!(actor, 2);
            purchase_choices(&engine, seats, true);
            receipts.push(act(&mut engine, 2, Command::BuyDevelopmentCard));
            purchase_choices(&engine, seats, false);
        } else {
            act(&mut engine, actor, Command::EndTurn);
        }
    }
    assert_hands(&engine, seats, final_hands);
    assert_eq!(
        engine.view(Viewer::Spectator).unwrap().bank.counts(),
        &final_bank
    );
    let own = engine
        .view(Viewer::Seat(seat(2)))
        .unwrap()
        .own_player
        .unwrap();
    assert_eq!(own.true_score, 3);
    assert_eq!(
        own.development_cards,
        vec![
            DevelopmentCardView {
                id: card(4),
                kind: DevelopmentKind::Knight,
                acquired_turn: rolls.len() as u64,
                acquired_on_earlier_turn: false
            },
            DevelopmentCardView {
                id: card(5),
                kind: DevelopmentKind::Knight,
                acquired_turn: first_turn,
                acquired_on_earlier_turn: true
            },
            DevelopmentCardView {
                id: card(24),
                kind: DevelopmentKind::VictoryPoint,
                acquired_turn: first_turn,
                acquired_on_earlier_turn: true
            },
        ]
    );
    for viewer in
        std::iter::once(Viewer::Spectator).chain(seats.iter().map(|&id| Viewer::Seat(seat(id))))
    {
        let view = engine.view(viewer).unwrap();
        assert_eq!(view.development_deck_remaining, 22);
        assert_eq!(view.revealed_victory_cards, [None; 5]);
        for player in view.players {
            assert_eq!(player.public_score, 2);
            assert_eq!(
                player.development_card_count,
                if player.seat == seat(2) { 3 } else { 0 }
            );
        }
        match viewer {
            Viewer::Spectator => assert!(view.own_player.is_none()),
            Viewer::Seat(actor) => {
                let own = view.own_player.unwrap();
                assert_eq!(own.seat, actor);
                if actor == seat(2) {
                    assert_eq!(own.true_score, 3);
                    assert_eq!(own.development_cards.len(), 3);
                } else {
                    assert!(own.development_cards.is_empty());
                    assert_eq!(own.true_score, 2);
                }
            }
        }
    }
    for (receipt, id, remaining) in [
        (&receipts[0], 24, 24),
        (&receipts[1], 5, 23),
        (&receipts[2], 4, 22),
    ] {
        project(&engine, receipt, seats, id, remaining);
    }
    let before = format!("{:?}", engine.view(Viewer::Spectator).unwrap());
    let first = &receipts[0];
    assert_eq!(
        engine
            .execute(CommandEnvelope {
                api_version: 1,
                actor: seat(2),
                expected_revision: first.previous_revision,
                command: Command::BuyDevelopmentCard
            })
            .unwrap_err()
            .code,
        RuleError::StaleRevision
    );
    assert_eq!(
        format!("{:?}", engine.view(Viewer::Spectator).unwrap()),
        before
    );
    act(&mut engine, 2, Command::EndTurn);
    let next_actor = if seats.len() == 3 { 0 } else { 3 };
    let next = act(&mut engine, next_actor, Command::RollDice);
    assert_eq!(
        next.events[0],
        Event::DiceRolled {
            seat: seat(next_actor),
            dice: [2, 2]
        }
    );
    // Reprojection remains the original purchase after another roll and resource change.
    project(&engine, &receipts[0], seats, 24, 24);
}
#[test]
fn development_purchase_three_seat_public_payment_vp_and_individual_acquisition_turns() {
    run(
        &[0, 1, 2],
        &[
            (2, [2, 4]),
            (0, [6, 3]),
            (1, [5, 5]),
            (2, [4, 5]),
            (0, [2, 3]),
            (1, [1, 2]),
            (2, [4, 2]),
            (0, [2, 3]),
            (1, [1, 2]),
            (2, [3, 6]),
        ],
        7,
        [3, 0, 2, 2, 2],
        [11, 18, 15, 14, 14],
        &[[4, 0, 2, 3, 1], [3, 1, 1, 1, 2], [3, 0, 0, 0, 0]],
        [9, 18, 16, 15, 16],
    );
}
#[test]
fn development_purchase_four_seat_public_payment_vp_and_individual_acquisition_turns() {
    run(
        &[0, 1, 2, 3],
        &[
            (2, [6, 3]),
            (3, [5, 5]),
            (0, [4, 5]),
            (1, [2, 3]),
            (2, [1, 2]),
            (3, [4, 2]),
            (0, [2, 3]),
            (1, [1, 2]),
            (2, [3, 6]),
        ],
        5,
        [1, 0, 2, 2, 2],
        [9, 18, 14, 12, 16],
        &[
            [4, 0, 2, 3, 1],
            [3, 1, 1, 1, 1],
            [2, 0, 0, 0, 0],
            [5, 0, 1, 3, 0],
        ],
        [5, 18, 15, 12, 17],
    );
}
