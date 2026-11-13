//! Synthetic hands and RNG positions; literals from the independent pinned sampler.
use super::*;
use crate::{Building, BuildingKind, GameConfig, HexId, Resource, ResumePhase, RobberOrigin};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha12Rng;

fn seat(id: u8) -> PlayerId {
    PlayerId::try_from(id).unwrap()
}
fn hex(id: u8) -> HexId {
    HexId::try_from(id).unwrap()
}
fn fixture(buildings: &[(usize, u8)], hand: [u8; 5]) -> Engine {
    let (mut engine, _) =
        Engine::create(GameConfig::fixed(&[0, 1, 2, 3]).unwrap(), [0; 32]).unwrap();
    engine.state.position.active_player = seat(0);
    engine.state.position.phase = Phase::RobberMove {
        origin: RobberOrigin::Seven,
        continuation: ResumePhase::Action,
    };
    engine.state.position.roads = [None; 72];
    engine.state.position.road_lengths = [0; 4];
    engine.state.position.buildings = [None; 54];
    for &(vertex, owner) in buildings {
        engine.state.position.buildings[vertex] = Some(Building {
            owner: seat(owner),
            kind: BuildingKind::Settlement,
        });
    }
    engine.state.position.hands = [Resources::default(); 4];
    engine.state.position.hands[1] = Resources::try_from(hand).unwrap();
    engine.state.position.bank = Resources::try_from([19; 5])
        .unwrap()
        .checked_sub(&engine.state.position.hands[1])
        .unwrap();
    engine.state.random = ChaCha12Rng::from_seed([0; 32]);
    engine
}
fn representative_development_locations(engine: &mut Engine) {
    use crate::game_model::CardLocation;
    engine.state.position.turn = 2;
    for (draw, id, location) in [
        (
            0,
            0,
            CardLocation::Held {
                owner: seat(1),
                acquired_turn: 1,
            },
        ),
        (1, 1, CardLocation::PlayedKnight { owner: seat(0) }),
        (2, 14, CardLocation::Spent { owner: seat(0) }),
        (
            3,
            20,
            CardLocation::Held {
                owner: seat(1),
                acquired_turn: 1,
            },
        ),
    ] {
        let card = crate::CardId::try_from(id).unwrap();
        let index = engine
            .state
            .position
            .deck
            .iter()
            .position(|&c| c == card)
            .unwrap();
        engine.state.position.deck.swap(draw, index);
        engine.state.position.card_locations[id as usize] = location;
    }
    engine.state.position.draw_cursor = 4;
}
fn execute(engine: &mut Engine, command: Command) -> Receipt {
    engine
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(0),
            expected_revision: engine.state.revision,
            command,
        })
        .unwrap()
}
fn unchanged_rejection(engine: &mut Engine, actor: PlayerId, command: Command, error: RuleError) {
    let before = engine.state.position.clone();
    let revision = engine.state.revision;
    let random = engine.state.random.clone();
    assert_eq!(engine.validate(actor, &command).unwrap_err().code, error);
    assert_eq!(
        engine
            .execute(CommandEnvelope {
                api_version: 1,
                actor,
                expected_revision: revision,
                command
            })
            .unwrap_err()
            .code,
        error
    );
    assert_eq!(engine.state.position, before);
    assert_eq!(engine.state.revision, revision);
    assert_eq!(engine.state.random, random);
}

#[test]
fn robber_weighted_theft_matches_literal_sampler_goldens() {
    // /tmp oracle was independent of this engine. Never read it at test runtime.
    let cases = [
        (27u32, 0u128, Resource::Brick, 6975729785065778220u64),
        (4, 0, Resource::Wood, 13134249596676133781),
        (0, 0, Resource::Wood, 1355055178205189383),
        (6, 0, Resource::Wood, 3765984729640471752),
        (9, 0, Resource::Wheat, 14497479467543819096),
        (19, 0, Resource::Ore, 3208253144526753463),
        (3, 0, Resource::Ore, 14674572298155976291),
        (7, 37, Resource::Brick, 11765454936741049779),
        (2, 37, Resource::Wood, 16366331500603535403),
        (3, 37, Resource::Ore, 7125495773872005146),
    ];
    for (seed_word, start, resource, next) in cases {
        let mut engine = fixture(&[(1, 1)], [1, 3, 0, 1, 2]);
        representative_development_locations(&mut engine);
        let mut seed = [0; 32];
        seed[..4].copy_from_slice(&seed_word.to_le_bytes());
        engine.state.random = ChaCha12Rng::from_seed(seed);
        engine.state.random.set_word_pos(start);
        let before = engine.state.position.clone();
        for viewer in [
            Viewer::Spectator,
            Viewer::Seat(seat(0)),
            Viewer::Seat(seat(1)),
        ] {
            engine.view(viewer).unwrap();
        }
        engine.legal_actions(seat(0)).unwrap();
        assert_eq!(engine.state.random.get_word_pos(), start);
        let receipt = execute(&mut engine, Command::MoveRobber { hex: hex(1) });
        assert_eq!(
            receipt.events,
            vec![
                Event::RobberMoved {
                    seat: seat(0),
                    hex: hex(1)
                },
                Event::RobberTheft {
                    thief: seat(0),
                    victim: seat(1),
                    resource: Some(resource)
                },
                Event::PhaseChanged {
                    phase: Phase::Action
                }
            ]
        );
        let mut stolen = [0; 5];
        stolen[resource.index()] = 1;
        let mut remaining = [1, 3, 0, 1, 2];
        remaining[resource.index()] -= 1;
        assert_eq!(engine.state.position.hands[0].counts(), &stolen);
        assert_eq!(engine.state.position.hands[1].counts(), &remaining);
        assert_eq!(engine.state.position.bank, before.bank);
        assert_eq!(engine.state.position.card_locations, before.card_locations);
        assert_eq!(engine.state.position.deck, before.deck);
        assert_eq!(engine.state.position.roads, before.roads);
        assert_eq!(engine.state.position.buildings, before.buildings);
        assert_eq!(engine.state.position.phase, Phase::Action);
        assert_eq!(engine.state.revision, 1);
        assert_eq!(engine.state.random.get_word_pos(), start + 1);
        assert_eq!(engine.state.random.clone().next_u64(), next);
    }
}

#[test]
fn robber_zero_empty_and_distinct_multiple_victims_resume_without_draws() {
    for buildings in [&[][..], &[(1, 1)][..]] {
        let mut engine = fixture(buildings, [0; 5]);
        let random = engine.state.random.clone();
        let receipt = execute(&mut engine, Command::MoveRobber { hex: hex(1) });
        assert_eq!(engine.state.position.phase, Phase::Action);
        assert_eq!(engine.state.random, random);
        assert_eq!(
            receipt.events.len(),
            if buildings.is_empty() { 2 } else { 3 }
        );
        if !buildings.is_empty() {
            assert_eq!(
                receipt.events[1],
                Event::RobberTheft {
                    thief: seat(0),
                    victim: seat(1),
                    resource: None
                }
            );
        }
    }
    // H1's independent vertices V1/V9/V8 are mutually distance-valid.
    let mut engine = fixture(&[(1, 1), (9, 1), (8, 2)], [1, 3, 0, 1, 2]);
    let random = engine.state.random.clone();
    execute(&mut engine, Command::MoveRobber { hex: hex(1) });
    let phase = Phase::RobberVictim {
        origin: RobberOrigin::Seven,
        continuation: ResumePhase::Action,
        victims: [None, Some(seat(1)), Some(seat(2)), None],
    };
    assert_eq!(engine.state.position.phase, phase);
    assert_eq!(engine.state.random, random);
    assert_eq!(
        engine.legal_actions(seat(0)).unwrap().choices,
        vec![LegalAction::ChooseRobberVictim {
            victims: vec![seat(1), seat(2)]
        }]
    );
    assert!(engine.legal_actions(seat(1)).unwrap().choices.is_empty());
    let receipt = execute(&mut engine, Command::ChooseRobberVictim { victim: seat(2) });
    assert_eq!(
        receipt.events,
        vec![
            Event::RobberTheft {
                thief: seat(0),
                victim: seat(2),
                resource: None
            },
            Event::PhaseChanged {
                phase: Phase::Action
            }
        ]
    );
    assert_eq!(engine.state.position.phase, Phase::Action);
    assert_eq!(engine.state.random, random);
    assert_eq!(engine.state.position.hands[1].counts(), &[1, 3, 0, 1, 2]);
    // Desert is legal after relocation, with no building or resource change.
    engine.state.position.phase = Phase::RobberMove {
        origin: RobberOrigin::Seven,
        continuation: ResumePhase::Action,
    };
    execute(&mut engine, Command::MoveRobber { hex: hex(0) });
    assert_eq!(engine.state.position.robber, hex(0));
    assert_eq!(engine.state.random, random);
}

#[test]
fn robber_rejections_preserve_complete_position_rng_and_revision() {
    let mut engine = fixture(&[(1, 1), (8, 2)], [1, 3, 0, 1, 2]);
    unchanged_rejection(
        &mut engine,
        seat(1),
        Command::MoveRobber { hex: hex(1) },
        RuleError::WrongActor,
    );
    unchanged_rejection(
        &mut engine,
        seat(0),
        Command::MoveRobber { hex: hex(0) },
        RuleError::InvalidInput,
    );
    unchanged_rejection(
        &mut engine,
        seat(0),
        Command::ChooseRobberVictim { victim: seat(1) },
        RuleError::WrongPhase,
    );
    execute(&mut engine, Command::MoveRobber { hex: hex(1) });
    unchanged_rejection(
        &mut engine,
        seat(0),
        Command::MoveRobber { hex: hex(2) },
        RuleError::WrongPhase,
    );
    unchanged_rejection(
        &mut engine,
        seat(1),
        Command::ChooseRobberVictim { victim: seat(2) },
        RuleError::WrongActor,
    );
    for victim in [seat(0), seat(3)] {
        unchanged_rejection(
            &mut engine,
            seat(0),
            Command::ChooseRobberVictim { victim },
            RuleError::InvalidVictim,
        );
    }
    engine.state.revision = u64::MAX;
    unchanged_rejection(
        &mut engine,
        seat(0),
        Command::ChooseRobberVictim { victim: seat(1) },
        RuleError::CounterExhausted,
    );
    assert_eq!(RuleError::InvalidInput.to_string(), "invalid input");
    assert_eq!(RuleError::InvalidVictim.to_string(), "invalid victim");
}

#[test]
fn privacy_theft_reveals_type_only_to_parties_and_uses_original_receipt() {
    let mut engine = fixture(&[(1, 1), (8, 2)], [1, 3, 0, 1, 2]);
    let movement = execute(&mut engine, Command::MoveRobber { hex: hex(1) });
    let receipt = execute(&mut engine, Command::ChooseRobberVictim { victim: seat(1) });
    assert_eq!(
        receipt.events[0],
        Event::RobberTheft {
            thief: seat(0),
            victim: seat(1),
            resource: Some(Resource::Wood)
        }
    );
    execute(&mut engine, Command::EndTurn);
    for viewer in [
        Viewer::Spectator,
        Viewer::Seat(seat(0)),
        Viewer::Seat(seat(1)),
        Viewer::Seat(seat(2)),
        Viewer::Seat(seat(3)),
    ] {
        let projected = engine.project_receipt(&receipt, viewer).unwrap();
        let resource = if matches!(viewer, Viewer::Seat(owner) if owner == seat(0) || owner == seat(1))
        {
            Some(Resource::Wood)
        } else {
            None
        };
        assert_eq!(projected.revision, 2);
        assert_eq!(
            projected.events,
            vec![
                ProjectedEvent::RobberTheft {
                    thief: seat(0),
                    victim: seat(1),
                    count: 1,
                    resource
                },
                ProjectedEvent::PhaseChanged {
                    phase: Phase::Action
                }
            ]
        );
        let public_move = engine.project_receipt(&movement, viewer).unwrap();
        assert_eq!(public_move.revision, 1);
        assert_eq!(
            public_move.events[0],
            ProjectedEvent::RobberMoved {
                seat: seat(0),
                hex: hex(1)
            }
        );
    }
    let mut empty = fixture(&[(1, 1)], [0; 5]);
    let receipt = execute(&mut empty, Command::MoveRobber { hex: hex(1) });
    for viewer in [
        Viewer::Spectator,
        Viewer::Seat(seat(0)),
        Viewer::Seat(seat(1)),
    ] {
        assert_eq!(
            empty.project_receipt(&receipt, viewer).unwrap().events[1],
            ProjectedEvent::RobberTheft {
                thief: seat(0),
                victim: seat(1),
                count: 0,
                resource: None
            }
        );
    }
}

#[test]
fn base_rules_br014_chosen_nonempty_theft_preserves_every_development_location() {
    let mut engine = fixture(&[(1, 1), (8, 2)], [1, 3, 0, 1, 2]);
    representative_development_locations(&mut engine);
    let locations = engine.state.position.card_locations;
    let deck = engine.state.position.deck;
    execute(&mut engine, Command::MoveRobber { hex: hex(1) });
    assert_eq!(engine.state.position.card_locations, locations);
    let receipt = execute(&mut engine, Command::ChooseRobberVictim { victim: seat(1) });
    assert_eq!(
        receipt.events[0],
        Event::RobberTheft {
            thief: seat(0),
            victim: seat(1),
            resource: Some(Resource::Wood),
        }
    );
    assert_eq!(engine.state.position.hands[0].counts(), &[0, 1, 0, 0, 0]);
    assert_eq!(engine.state.position.hands[1].counts(), &[1, 2, 0, 1, 2]);
    assert_eq!(engine.state.position.card_locations, locations);
    assert_eq!(engine.state.position.deck, deck);
    assert_eq!(engine.state.position.draw_cursor, 4);
}
