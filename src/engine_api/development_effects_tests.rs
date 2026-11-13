//! Card-effect acceptance and rejection against bounded private positions.
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

fn fixture() -> Engine {
    let (mut engine, _) =
        Engine::create(GameConfig::fixed(&[0, 1, 2, 3]).unwrap(), [21; 32]).unwrap();
    let position = &mut engine.state.position;
    position.active_player = seat(0);
    position.turn = 2;
    position.phase = Phase::PreRoll;
    for (draw, id, owner, acquired_turn) in
        [(0, 16, 0, 1), (1, 17, 0, 2), (2, 18, 0, 1), (3, 8, 1, 1)]
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
    position.hands = [
        resources([1, 2, 0, 0, 1]),
        resources([3, 0, 2, 0, 1]),
        resources([2, 1, 1, 0, 0]),
        resources([0, 0, 3, 0, 0]),
    ];
    position.bank = resources([13, 16, 13, 19, 17]);
    engine
}
fn monopoly(id: u8, resource: Resource) -> Command {
    Command::PlayMonopoly {
        card: card(id),
        resource,
    }
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
            LegalAction::PlayMonopoly { cards } => Some(cards),
            _ => None,
        })
        .unwrap_or_default()
}

#[test]
fn base_rules_br026_owned_earlier_card_and_one_turn_allowance_are_shared_with_queries() {
    let mut engine = fixture();
    assert_eq!(choices(&engine, 0), vec![card(16)]);
    for id in [16, 17, 18, 8, 15] {
        assert_eq!(
            engine
                .validate(seat(0), &monopoly(id, Resource::Brick))
                .is_ok(),
            choices(&engine, 0).contains(&card(id))
        );
    }
    for id in [17, 18, 8, 15] {
        reject(
            &mut engine,
            0,
            monopoly(id, Resource::Brick),
            RuleError::CardNotPlayable,
        );
    }
    reject(
        &mut engine,
        1,
        monopoly(16, Resource::Brick),
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
                command: monopoly(16, Resource::Brick)
            })
            .unwrap_err()
            .code,
        RuleError::StaleRevision
    );
    assert_eq!(engine.state.position, before);
    assert_eq!(engine.state.random, random);
    assert_eq!(engine.state.revision, revision);
    let receipt = act(&mut engine, 0, monopoly(16, Resource::Brick));
    assert_eq!(receipt.revision, revision + 1);
    assert_eq!(engine.state.position.phase, Phase::PreRoll);
    assert!(choices(&engine, 0).is_empty());
    reject(
        &mut engine,
        0,
        monopoly(16, Resource::Brick),
        RuleError::CardNotPlayable,
    );
    engine.state.position.phase = Phase::Action;
    reject(
        &mut engine,
        0,
        monopoly(17, Resource::Brick),
        RuleError::CardNotPlayable,
    );
    act(&mut engine, 0, Command::EndTurn);
    assert_eq!(engine.state.position.active_player, seat(1));
    for actor in [1, 2, 3] {
        engine.state.position.phase = Phase::Action;
        act(&mut engine, actor, Command::EndTurn);
    }
    assert_eq!(engine.state.position.active_player, seat(0));
    assert_eq!(choices(&engine, 0), vec![card(17)]);
    engine.state.position.phase = Phase::Action;
    act(&mut engine, 0, monopoly(17, Resource::Wheat));
    assert_eq!(engine.state.position.phase, Phase::Action);
}

#[test]
fn base_rules_br028_monopoly_transfers_all_opponents_and_projects_original_public_facts() {
    let mut engine = fixture();
    let bank = engine.state.position.bank;
    let random = engine.state.random.clone();
    let receipt = act(&mut engine, 0, monopoly(16, Resource::Brick));
    assert_eq!(
        engine.state.position.hands.map(|hand| *hand.counts()),
        [
            [6, 2, 0, 0, 1],
            [0, 0, 2, 0, 1],
            [0, 1, 1, 0, 0],
            [0, 0, 3, 0, 0]
        ]
    );
    assert_eq!(engine.state.position.bank, bank);
    assert_eq!(engine.state.random, random);
    assert_eq!(
        engine.state.position.card_locations[16],
        CardLocation::Spent { owner: seat(0) }
    );
    assert_eq!(
        receipt.events,
        vec![Event::MonopolyPlayed {
            seat: seat(0),
            card: card(16),
            resource: Resource::Brick,
            transfers: [0, 3, 2, 0]
        }]
    );
    for viewer in [
        Viewer::Spectator,
        Viewer::Seat(seat(0)),
        Viewer::Seat(seat(1)),
        Viewer::Seat(seat(2)),
        Viewer::Seat(seat(3)),
    ] {
        let view = engine.view(viewer).unwrap();
        assert_eq!(
            view.players
                .iter()
                .map(|player| (player.seat, player.spent_progress.clone()))
                .collect::<Vec<_>>(),
            vec![
                (seat(0), vec![DevelopmentKind::Monopoly]),
                (seat(1), vec![]),
                (seat(2), vec![]),
                (seat(3), vec![])
            ]
        );
        assert_eq!(
            engine.project_receipt(&receipt, viewer).unwrap().events,
            vec![ProjectedEvent::MonopolyPlayed {
                seat: seat(0),
                card: card(16),
                resource: Resource::Brick,
                transfers: [0, 3, 2, 0]
            }]
        );
    }
    engine.state.position.phase = Phase::Action;
    act(&mut engine, 0, Command::EndTurn);
    assert_eq!(
        engine
            .project_receipt(&receipt, Viewer::Spectator)
            .unwrap()
            .events,
        vec![ProjectedEvent::MonopolyPlayed {
            seat: seat(0),
            card: card(16),
            resource: Resource::Brick,
            transfers: [0, 3, 2, 0]
        }]
    );
}

#[test]
fn base_rules_br026_pending_phases_and_exhausted_revision_reject_atomically() {
    let mut engine = fixture();
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
            monopoly(16, Resource::Brick),
            RuleError::WrongPhase,
        );
        assert!(choices(&engine, 0).is_empty());
    }
    engine.state.position.phase = Phase::Action;
    engine.state.revision = u64::MAX;
    reject(
        &mut engine,
        0,
        monopoly(16, Resource::Brick),
        RuleError::CounterExhausted,
    );
}

#[test]
fn base_rules_br028_each_resource_including_zero_result_conserves_the_supply() {
    for (kind, transfers, expected_hands) in [
        (
            Resource::Brick,
            [0, 3, 2, 0],
            [
                [6, 2, 0, 0, 1],
                [0, 0, 2, 0, 1],
                [0, 1, 1, 0, 0],
                [0, 0, 3, 0, 0],
            ],
        ),
        (
            Resource::Wood,
            [0, 0, 1, 0],
            [
                [1, 3, 0, 0, 1],
                [3, 0, 2, 0, 1],
                [2, 0, 1, 0, 0],
                [0, 0, 3, 0, 0],
            ],
        ),
        (
            Resource::Wool,
            [0, 2, 1, 3],
            [
                [1, 2, 6, 0, 1],
                [3, 0, 0, 0, 1],
                [2, 1, 0, 0, 0],
                [0, 0, 0, 0, 0],
            ],
        ),
        (
            Resource::Wheat,
            [0, 0, 0, 0],
            [
                [1, 2, 0, 0, 1],
                [3, 0, 2, 0, 1],
                [2, 1, 1, 0, 0],
                [0, 0, 3, 0, 0],
            ],
        ),
        (
            Resource::Ore,
            [0, 1, 0, 0],
            [
                [1, 2, 0, 0, 2],
                [3, 0, 2, 0, 0],
                [2, 1, 1, 0, 0],
                [0, 0, 3, 0, 0],
            ],
        ),
    ] {
        let mut engine = fixture();
        engine.state.position.phase = Phase::Action;
        let random = engine.state.random.clone();
        let bank = engine.state.position.bank;
        let receipt = act(&mut engine, 0, monopoly(16, kind));
        assert_eq!(
            receipt.events,
            vec![Event::MonopolyPlayed {
                seat: seat(0),
                card: card(16),
                resource: kind,
                transfers
            }]
        );
        assert_eq!(
            engine.state.position.hands.map(|hand| *hand.counts()),
            expected_hands
        );
        assert_eq!(engine.state.position.bank, bank);
        assert_eq!(engine.state.random, random);
        assert_eq!(engine.state.position.phase, Phase::Action);
        for resource in 0..5 {
            let total: u16 = engine
                .state
                .position
                .hands
                .iter()
                .map(|hand| u16::from(hand.counts()[resource]))
                .sum::<u16>()
                + u16::from(bank.counts()[resource]);
            assert_eq!(total, 19);
        }
    }
}

#[test]
fn base_rules_br026_preroll_play_blocks_another_older_card_during_action() {
    let mut engine = fixture();
    engine.state.position.card_locations[17] = CardLocation::Held {
        owner: seat(0),
        acquired_turn: 1,
    };
    assert_eq!(choices(&engine, 0), vec![card(16), card(17)]);
    act(&mut engine, 0, monopoly(16, Resource::Brick));
    engine.state.position.phase = Phase::Action;
    assert!(choices(&engine, 0).is_empty());
    reject(
        &mut engine,
        0,
        monopoly(17, Resource::Wool),
        RuleError::CardNotPlayable,
    );
}
