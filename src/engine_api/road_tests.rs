//! Paid transition checks; graph paths are independently recorded fixture IDs.
use super::*;
use crate::{Building, BuildingKind, GameConfig};
use rand::Rng;
fn seat(id: u8) -> PlayerId {
    PlayerId::try_from(id).unwrap()
}
fn edge(id: u8) -> EdgeId {
    EdgeId::try_from(id).unwrap()
}
fn fixture() -> Engine {
    let (mut engine, _) = Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), [0; 32]).unwrap();
    engine.state.position.active_player = seat(0);
    engine.state.position.phase = Phase::Action;
    engine.state.position.roads = [None; 72];
    engine.state.position.road_lengths = [0; 4];
    engine.state.position.longest_route_holder = None;
    engine.state.position.buildings = [None; 54];
    engine.state.position.buildings[0] = Some(Building {
        owner: seat(0),
        kind: BuildingKind::Settlement,
    });
    engine.state.position.hands = [Resources::default(); 4];
    engine.state.position.hands[0] = Resources::try_from([2, 2, 0, 0, 0]).unwrap();
    engine.state.position.bank = Resources::try_from([17, 17, 19, 19, 19]).unwrap();
    engine
}
fn execute(engine: &mut Engine, id: u8) -> Receipt {
    engine
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(0),
            expected_revision: engine.state.revision,
            command: Command::BuildRoad { edge: edge(id) },
        })
        .unwrap()
}
fn reject(engine: &mut Engine, actor: PlayerId, id: u8, error: RuleError) {
    let before = engine.state.position.clone();
    let random = engine.state.random.clone();
    let revision = engine.state.revision;
    let command = Command::BuildRoad { edge: edge(id) };
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
    assert_eq!(engine.state.random, random);
    assert_eq!(engine.state.revision, revision);
}

#[test]
fn road_candidates_share_validation_and_atomic_paid_event_order() {
    let mut engine = fixture();
    crate::base_rules::reset_route_searches();
    assert_eq!(
        engine.legal_actions(seat(0)).unwrap().choices,
        vec![
            LegalAction::ProposeTrade {
                recipients: vec![seat(1), seat(2)]
            },
            LegalAction::EndTurn,
            LegalAction::BuildRoad {
                edges: vec![edge(0), edge(1)]
            }
        ]
    );
    for id in 0..72 {
        assert_eq!(
            engine
                .validate(seat(0), &Command::BuildRoad { edge: edge(id) })
                .is_ok(),
            id < 2
        );
    }
    assert_eq!(crate::base_rules::route_searches(), 0);
    let random = engine.state.random.clone();
    let next = random.clone().next_u64();
    let receipt = execute(&mut engine, 0);
    assert_eq!(
        receipt.events,
        vec![
            Event::ConstructionPaid {
                seat: seat(0),
                resources: Resources::try_from([1, 1, 0, 0, 0]).unwrap()
            },
            Event::RoadPlaced {
                seat: seat(0),
                edge: edge(0)
            }
        ]
    );
    assert_eq!(engine.state.position.hands[0].counts(), &[1, 1, 0, 0, 0]);
    assert_eq!(engine.state.position.bank.counts(), &[18, 18, 19, 19, 19]);
    assert_eq!(engine.state.position.road_lengths, [1, 0, 0, 0]);
    assert_eq!(engine.state.position.longest_route_holder, None);
    assert_eq!(engine.state.revision, 1);
    assert_eq!(engine.state.random, random);
    assert_eq!(engine.state.random.clone().next_u64(), next);
    assert_eq!(crate::base_rules::route_searches(), 1);
    for viewer in [
        Viewer::Spectator,
        Viewer::Seat(seat(0)),
        Viewer::Seat(seat(1)),
        Viewer::Seat(seat(2)),
    ] {
        assert_eq!(
            engine.project_receipt(&receipt, viewer).unwrap().events,
            vec![
                ProjectedEvent::ConstructionPaid {
                    seat: seat(0),
                    resources: Resources::try_from([1, 1, 0, 0, 0]).unwrap()
                },
                ProjectedEvent::RoadPlaced {
                    seat: seat(0),
                    edge: edge(0)
                }
            ]
        );
        let view = engine.view(viewer).unwrap();
        assert_eq!(view.players[0].roads_remaining, 14);
        assert_eq!(view.players[0].public_score, 1);
    }
    engine.legal_actions(seat(0)).unwrap();
    assert_eq!(crate::base_rules::route_searches(), 1);
}

#[test]
fn road_rejections_preserve_all_resources_pieces_revision_rng_and_route_cache() {
    let mut engine = fixture();
    reject(&mut engine, seat(1), 0, RuleError::WrongActor);
    reject(&mut engine, seat(0), 6, RuleError::Disconnected);
    engine.state.position.phase = Phase::PreRoll;
    reject(&mut engine, seat(0), 0, RuleError::WrongPhase);
    engine.state.position.phase = Phase::Action;
    engine.state.position.hands[0] = Resources::try_from([0, 2, 0, 0, 0]).unwrap();
    engine.state.position.bank = Resources::try_from([19, 17, 19, 19, 19]).unwrap();
    reject(&mut engine, seat(0), 0, RuleError::InsufficientResources);
    assert_eq!(
        engine.legal_actions(seat(0)).unwrap().choices,
        vec![
            LegalAction::ProposeTrade {
                recipients: vec![seat(1), seat(2)]
            },
            LegalAction::EndTurn
        ]
    );
    engine.state.position.hands[0] = Resources::try_from([2, 2, 0, 0, 0]).unwrap();
    engine.state.position.bank = Resources::try_from([17, 17, 19, 19, 19]).unwrap();
    engine.state.position.roads[0] = Some(seat(1));
    reject(&mut engine, seat(0), 0, RuleError::Occupied);
    engine.state.position.roads = [None; 72];
    for id in [0, 1, 2, 3, 4, 5, 9, 17, 22, 32, 38, 48, 53, 61, 65] {
        engine.state.position.roads[id] = Some(seat(0));
    }
    engine.state.position.road_lengths = [15, 0, 0, 0];
    engine.state.position.longest_route_holder = Some(seat(0));
    reject(&mut engine, seat(0), 6, RuleError::NoPieces);
    engine.state.position.roads = [None; 72];
    engine.state.position.road_lengths = [0; 4];
    engine.state.position.longest_route_holder = None;
    engine.state.revision = u64::MAX;
    reject(&mut engine, seat(0), 0, RuleError::CounterExhausted);
    assert_eq!(
        RuleError::InsufficientResources.to_string(),
        "insufficient resources"
    );
    assert_eq!(RuleError::NoPieces.to_string(), "no pieces");
}

#[test]
fn road_one_unblocked_connected_endpoint_remains_usable() {
    let mut engine = fixture();
    engine.state.position.buildings = [None; 54];
    for (vertex, owner) in [(3, 0), (11, 1)] {
        engine.state.position.buildings[vertex] = Some(Building {
            owner: seat(owner),
            kind: BuildingKind::Settlement,
        });
    }
    for id in [6, 10] {
        engine.state.position.roads[id] = Some(seat(0));
    }
    engine.state.position.road_lengths = [2, 0, 0, 0];
    reject(&mut engine, seat(0), 18, RuleError::Disconnected);
    engine.state.position.roads[23] = Some(seat(0));
    engine.state.position.buildings[21] = Some(Building {
        owner: seat(0),
        kind: BuildingKind::Settlement,
    });
    execute(&mut engine, 18);
    assert_eq!(engine.state.position.road_lengths, [2, 0, 0, 0]);
    assert_eq!(engine.state.position.roads[18], Some(seat(0)));
}

#[test]
fn road_fifth_edge_award_runs_shared_victory_hook_before_another_decision() {
    let mut engine = fixture();
    engine.state.position.buildings = [None; 54];
    // Source fixed vertices are pairwise distance-valid; 3 cities + 2 settlements = 8.
    for (vertex, kind) in [
        (0, BuildingKind::City),
        (9, BuildingKind::City),
        (17, BuildingKind::City),
        (31, BuildingKind::Settlement),
        (40, BuildingKind::Settlement),
    ] {
        engine.state.position.buildings[vertex] = Some(Building {
            owner: seat(0),
            kind,
        });
    }
    for id in [0, 6, 10, 18, 14, 25, 46, 57] {
        engine.state.position.roads[id] = Some(seat(0));
    }
    engine.state.position.road_lengths = [4, 0, 0, 0];
    engine.state.position.hands[0] = Resources::try_from([1, 1, 0, 0, 0]).unwrap();
    engine.state.position.bank = Resources::try_from([18, 18, 19, 19, 19]).unwrap();
    assert_eq!(
        crate::base_rules::current_score(&engine.state.position, seat(0)),
        8
    );
    assert_eq!(
        crate::base_rules::longest_route_length(
            engine.board.graph,
            &engine.state.position.roads,
            &engine.state.position.buildings,
            seat(0)
        ),
        4
    );
    for id in [0, 9, 17, 31, 40] {
        assert!(
            engine
                .board
                .graph
                .vertex_neighbors(VertexId::try_from(id).unwrap())
                .iter()
                .all(|neighbor| engine.state.position.buildings[neighbor.index()].is_none())
        );
    }
    let random = engine.state.random.clone();
    let receipt = execute(&mut engine, 23);
    assert_eq!(
        receipt.events,
        vec![
            Event::ConstructionPaid {
                seat: seat(0),
                resources: Resources::try_from([1, 1, 0, 0, 0]).unwrap()
            },
            Event::RoadPlaced {
                seat: seat(0),
                edge: edge(23)
            },
            Event::LongestRouteChanged {
                previous: None,
                holder: Some(seat(0))
            },
            Event::GameFinished {
                winner: seat(0),
                score: 10
            }
        ]
    );
    assert_eq!(
        engine.state.position.phase,
        Phase::Finished {
            winner: seat(0),
            score: 10
        }
    );
    assert_eq!(engine.state.position.hands[0], Resources::default());
    assert_eq!(engine.state.position.bank.counts(), &[19; 5]);
    assert_eq!(engine.state.position.road_lengths, [5, 0, 0, 0]);
    assert_eq!(engine.state.random, random);
    assert_eq!(engine.state.revision, 1);
    let view = engine.view(Viewer::Seat(seat(0))).unwrap();
    assert_eq!(view.players[0].public_score, 10);
    assert_eq!(view.own_player.unwrap().true_score, 10);
    assert_eq!(view.players[0].roads_remaining, 6);
    assert!(engine.legal_actions(seat(0)).unwrap().choices.is_empty());
    reject(&mut engine, seat(0), 6, RuleError::GameFinished);
}
