//! RQ2 bank exhaustion, shared eligibility, atomicity and public projection.
use super::*;
use crate::game_model::CardLocation;
use crate::{DevelopmentKind, GameConfig};

fn seat(id: u8) -> PlayerId {
    PlayerId::try_from(id).unwrap()
}
fn card(id: u8) -> CardId {
    CardId::try_from(id).unwrap()
}
fn resources(counts: [u8; 5]) -> Resources {
    Resources::try_from(counts).unwrap()
}
fn invention(id: u8, counts: [u8; 5]) -> Command {
    Command::PlayInvention {
        card: card(id),
        resources: resources(counts),
    }
}
fn fixture(bank: [u8; 5], phase: Phase) -> Engine {
    let (mut engine, _) =
        Engine::create(GameConfig::fixed(&[0, 1, 2, 3]).unwrap(), [5; 32]).unwrap();
    let position = &mut engine.state.position;
    position.active_player = seat(0);
    position.turn = 2;
    position.phase = phase;
    for (draw, id, owner, acquired_turn) in
        [(0, 18, 0, 1), (1, 19, 0, 2), (2, 16, 0, 1), (3, 8, 1, 1)]
    {
        let index = position
            .deck
            .iter()
            .position(|&value| value == card(id))
            .unwrap();
        position.deck.swap(draw, index);
        position.card_locations[id as usize] = CardLocation::Held {
            owner: seat(owner),
            acquired_turn,
        };
    }
    position.draw_cursor = 4;
    position.bank = resources(bank);
    position.hands = [
        Resources::default(),
        resources(bank.map(|count| 19 - count)),
        Resources::default(),
        Resources::default(),
    ];
    engine
}
fn act(engine: &mut Engine, actor: u8, command: Command) -> Receipt {
    engine
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(actor),
            expected_revision: engine.state.revision,
            command,
        })
        .unwrap()
}
fn reject(engine: &mut Engine, actor: u8, command: Command, error: RuleError) {
    let position = engine.state.position.clone();
    let random = engine.state.random.clone();
    let revision = engine.state.revision;
    assert_eq!(
        engine.validate(seat(actor), &command).unwrap_err().code,
        error
    );
    assert_eq!(
        engine
            .execute(CommandEnvelope {
                api_version: 1,
                actor: seat(actor),
                expected_revision: revision,
                command
            })
            .unwrap_err()
            .code,
        error
    );
    assert_eq!(engine.state.position, position);
    assert_eq!(engine.state.random, random);
    assert_eq!(engine.state.revision, revision);
}
fn choices(engine: &Engine, actor: u8) -> Vec<CardId> {
    engine
        .legal_actions(seat(actor))
        .unwrap()
        .choices
        .into_iter()
        .find_map(|choice| match choice {
            LegalAction::PlayInvention { cards } => Some(cards),
            _ => None,
        })
        .unwrap_or_default()
}

#[test]
fn base_rules_br029_exact_two_same_or_mixed_and_sole_card_resolve_atomically() {
    let cases = [
        ([3, 2, 1, 0, 0], [2, 0, 0, 0, 0], [1, 2, 1, 0, 0]),
        ([3, 2, 1, 0, 0], [1, 1, 0, 0, 0], [2, 1, 1, 0, 0]),
        ([1, 1, 0, 0, 0], [1, 1, 0, 0, 0], [0, 0, 0, 0, 0]),
        ([0, 0, 2, 0, 0], [0, 0, 2, 0, 0], [0, 0, 0, 0, 0]),
        ([0, 0, 0, 0, 1], [0, 0, 0, 0, 1], [0, 0, 0, 0, 0]),
    ];
    for (bank, selection, expected_bank) in cases {
        for phase in [Phase::PreRoll, Phase::Action] {
            let mut engine = fixture(bank, phase);
            let random = engine.state.random.clone();
            assert_eq!(choices(&engine, 0), vec![card(18)]);
            assert!(engine.validate(seat(0), &invention(18, selection)).is_ok());
            let receipt = act(&mut engine, 0, invention(18, selection));
            assert_eq!((receipt.previous_revision, receipt.revision), (0, 1));
            assert_eq!(engine.state.position.phase, phase);
            assert_eq!(engine.state.random, random);
            assert_eq!(engine.state.position.bank.counts(), &expected_bank);
            assert_eq!(engine.state.position.hands[0].counts(), &selection);
            assert_eq!(
                engine.state.position.hands[1].counts(),
                &bank.map(|count| 19 - count)
            );
            assert_eq!(
                engine.state.position.card_locations[18],
                CardLocation::Spent { owner: seat(0) }
            );
            assert_eq!(
                receipt.events,
                vec![Event::InventionPlayed {
                    seat: seat(0),
                    card: card(18),
                    resources: resources(selection)
                }]
            );
            for resource in 0..5 {
                assert_eq!(
                    u16::from(engine.state.position.bank.counts()[resource])
                        + engine
                            .state
                            .position
                            .hands
                            .iter()
                            .map(|hand| u16::from(hand.counts()[resource]))
                            .sum::<u16>(),
                    19
                );
            }
            for viewer in [
                Viewer::Spectator,
                Viewer::Seat(seat(0)),
                Viewer::Seat(seat(1)),
                Viewer::Seat(seat(2)),
                Viewer::Seat(seat(3)),
            ] {
                assert_eq!(
                    engine
                        .view(viewer)
                        .unwrap()
                        .players
                        .iter()
                        .map(|player| (player.seat, player.spent_progress.clone()))
                        .collect::<Vec<_>>(),
                    vec![
                        (seat(0), vec![DevelopmentKind::Invention]),
                        (seat(1), vec![]),
                        (seat(2), vec![]),
                        (seat(3), vec![])
                    ]
                );
                assert_eq!(
                    engine.project_receipt(&receipt, viewer).unwrap(),
                    ProjectedReceipt {
                        revision: 1,
                        events: vec![ProjectedEvent::InventionPlayed {
                            seat: seat(0),
                            card: card(18),
                            resources: resources(selection)
                        }]
                    }
                );
            }
            engine.state.position.phase = Phase::Action;
            act(&mut engine, 0, Command::EndTurn);
            for viewer in [
                Viewer::Spectator,
                Viewer::Seat(seat(0)),
                Viewer::Seat(seat(1)),
                Viewer::Seat(seat(2)),
                Viewer::Seat(seat(3)),
            ] {
                assert_eq!(
                    engine.project_receipt(&receipt, viewer).unwrap(),
                    ProjectedReceipt {
                        revision: 1,
                        events: vec![ProjectedEvent::InventionPlayed {
                            seat: seat(0),
                            card: card(18),
                            resources: resources(selection)
                        }],
                    }
                );
            }
        }
    }
}

#[test]
fn base_rules_br029_invalid_bundle_or_empty_bank_preserves_card_allowance_and_state() {
    let mut engine = fixture([1, 1, 0, 0, 0], Phase::PreRoll);
    for counts in [[0, 0, 0, 0, 0], [1, 0, 0, 0, 0], [1, 1, 1, 0, 0]] {
        reject(
            &mut engine,
            0,
            invention(18, counts),
            RuleError::InvalidInput,
        );
    }
    reject(
        &mut engine,
        0,
        invention(18, [2, 0, 0, 0, 0]),
        RuleError::BankShortage,
    );
    assert_eq!(choices(&engine, 0), vec![card(18)]);
    let mut sole = fixture([0, 0, 0, 0, 1], Phase::Action);
    reject(
        &mut sole,
        0,
        invention(18, [0, 0, 0, 1, 0]),
        RuleError::BankShortage,
    );
    reject(
        &mut sole,
        0,
        invention(18, [0, 0, 0, 0, 2]),
        RuleError::InvalidInput,
    );
    let mut empty = fixture([0; 5], Phase::PreRoll);
    reject(
        &mut empty,
        0,
        invention(18, [0; 5]),
        RuleError::BankShortage,
    );
    assert!(choices(&empty, 0).is_empty());
}

#[test]
fn base_rules_br026_invention_shares_per_card_and_cross_kind_allowance_validation() {
    let mut engine = fixture([2, 2, 0, 0, 0], Phase::PreRoll);
    for id in [18, 19, 16, 8, 15] {
        assert_eq!(
            engine
                .validate(seat(0), &invention(id, [1, 1, 0, 0, 0]))
                .is_ok(),
            choices(&engine, 0).contains(&card(id))
        );
    }
    for id in [19, 16, 8, 15] {
        reject(
            &mut engine,
            0,
            invention(id, [1, 1, 0, 0, 0]),
            RuleError::CardNotPlayable,
        );
    }
    reject(
        &mut engine,
        1,
        invention(18, [1, 1, 0, 0, 0]),
        RuleError::WrongActor,
    );
    let before = engine.state.position.clone();
    let random = engine.state.random.clone();
    let revision = engine.state.revision;
    assert_eq!(
        engine
            .execute(CommandEnvelope {
                api_version: 1,
                actor: seat(0),
                expected_revision: revision + 1,
                command: invention(18, [1, 1, 0, 0, 0])
            })
            .unwrap_err()
            .code,
        RuleError::StaleRevision
    );
    assert_eq!(engine.state.position, before);
    assert_eq!(engine.state.random, random);
    assert_eq!(engine.state.revision, revision);
    act(
        &mut engine,
        0,
        Command::PlayMonopoly {
            card: card(16),
            resource: Resource::Wool,
        },
    );
    engine.state.position.phase = Phase::Action;
    reject(
        &mut engine,
        0,
        invention(18, [1, 1, 0, 0, 0]),
        RuleError::CardNotPlayable,
    );
    assert!(choices(&engine, 0).is_empty());
    act(&mut engine, 0, Command::EndTurn);
    for actor in [1, 2, 3] {
        engine.state.position.phase = Phase::Action;
        act(&mut engine, actor, Command::EndTurn);
    }
    assert_eq!(choices(&engine, 0), vec![card(18), card(19)]);
    act(&mut engine, 0, invention(18, [1, 1, 0, 0, 0]));
    reject(
        &mut engine,
        0,
        invention(19, [1, 1, 0, 0, 0]),
        RuleError::CardNotPlayable,
    );
}

#[test]
fn base_rules_br029_pending_and_revision_exhaustion_reject_without_mutation() {
    let mut engine = fixture([2, 1, 0, 0, 0], Phase::PreRoll);
    for phase in [
        Phase::Discard {
            obligations: [1, 0, 0, 0],
        },
        Phase::RobberMove {
            origin: crate::RobberOrigin::Seven,
            continuation: crate::ResumePhase::Action,
        },
        Phase::SetupSettlement {
            actor: seat(0),
            round: 1,
        },
    ] {
        engine.state.position.phase = phase;
        reject(
            &mut engine,
            0,
            invention(18, [2, 0, 0, 0, 0]),
            RuleError::WrongPhase,
        );
        assert!(choices(&engine, 0).is_empty());
    }
    engine.state.position.phase = Phase::Action;
    engine.state.revision = u64::MAX;
    reject(
        &mut engine,
        0,
        invention(18, [2, 0, 0, 0, 0]),
        RuleError::CounterExhausted,
    );
}
