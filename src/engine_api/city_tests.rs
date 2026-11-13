//! Checked city transitions and independently traced stock/score/route fixtures.
use super::*;
use crate::{Building, BuildingKind, GameConfig};
fn seat(id: u8) -> PlayerId {
    PlayerId::try_from(id).unwrap()
}
fn vertex(id: u8) -> VertexId {
    VertexId::try_from(id).unwrap()
}
fn fixture() -> Engine {
    let (mut engine, _) = Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), [0; 32]).unwrap();
    engine.state.position.active_player = seat(0);
    engine.state.position.phase = Phase::Action;
    engine.state.position.hands = [Resources::default(); 4];
    engine.state.position.hands[0] = Resources::try_from([0, 0, 0, 2, 3]).unwrap();
    engine.state.position.bank = Resources::try_from([19, 19, 19, 17, 16]).unwrap();
    engine
}
fn execute(engine: &mut Engine, id: u8) -> Receipt {
    engine
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(0),
            expected_revision: engine.state.revision,
            command: Command::UpgradeCity { vertex: vertex(id) },
        })
        .unwrap()
}
fn reject(engine: &mut Engine, actor: PlayerId, id: u8, error: RuleError) {
    let before = engine.state.position.clone();
    let random = engine.state.random.clone();
    let revision = engine.state.revision;
    let command = Command::UpgradeCity { vertex: vertex(id) };
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
fn city_exact_payment_candidate_replacement_and_old_public_receipt() {
    let mut engine = fixture();
    crate::base_rules::reset_route_searches();
    assert!(
        engine
            .legal_actions(seat(0))
            .unwrap()
            .choices
            .contains(&LegalAction::UpgradeCity {
                vertices: vec![vertex(31), vertex(40)]
            })
    );
    for id in 0..54 {
        assert_eq!(
            engine
                .validate(seat(0), &Command::UpgradeCity { vertex: vertex(id) })
                .is_ok(),
            [31, 40].contains(&id)
        );
    }
    let roads = engine.state.position.roads;
    let cache = engine.state.position.road_lengths;
    let random = engine.state.random.clone();
    let receipt = execute(&mut engine, 40);
    let cost = Resources::try_from([0, 0, 0, 2, 3]).unwrap();
    assert_eq!(
        receipt.events,
        vec![
            Event::ConstructionPaid {
                seat: seat(0),
                resources: cost
            },
            Event::CityUpgraded {
                seat: seat(0),
                vertex: vertex(40)
            }
        ]
    );
    assert_eq!(
        engine.state.position.buildings[40],
        Some(Building {
            owner: seat(0),
            kind: BuildingKind::City
        })
    );
    assert_eq!(engine.state.position.hands[0], Resources::default());
    assert_eq!(engine.state.position.bank.counts(), &[19; 5]);
    assert_eq!(engine.state.position.roads, roads);
    assert_eq!(engine.state.position.road_lengths, cache);
    assert_eq!(engine.state.random, random);
    assert_eq!(crate::base_rules::route_searches(), 0);
    engine
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(0),
            expected_revision: 1,
            command: Command::EndTurn,
        })
        .unwrap();
    for viewer in [
        Viewer::Spectator,
        Viewer::Seat(seat(0)),
        Viewer::Seat(seat(1)),
        Viewer::Seat(seat(2)),
    ] {
        let view = engine.view(viewer).unwrap();
        assert_eq!(
            (
                view.players[0].settlements_remaining,
                view.players[0].cities_remaining,
                view.players[0].public_score
            ),
            (4, 3, 3)
        );
        assert_eq!(
            engine.project_receipt(&receipt, viewer).unwrap(),
            ProjectedReceipt {
                revision: 1,
                events: vec![
                    ProjectedEvent::ConstructionPaid {
                        seat: seat(0),
                        resources: cost
                    },
                    ProjectedEvent::CityUpgraded {
                        seat: seat(0),
                        vertex: vertex(40)
                    }
                ]
            }
        );
    }
    assert_eq!(crate::base_rules::route_searches(), 0);
}
#[test]
fn base_rules_br019_city_rejects_nonowned_nonsettlement_cost_and_piece_failures_atomically() {
    let mut engine = fixture();
    reject(&mut engine, seat(1), 40, RuleError::WrongActor);
    reject(&mut engine, seat(0), 9, RuleError::NotOwned);
    reject(&mut engine, seat(0), 0, RuleError::NotOwned);
    engine.state.position.buildings[40].as_mut().unwrap().kind = BuildingKind::City;
    reject(&mut engine, seat(0), 40, RuleError::NotOwned);
    engine.state.position.buildings[40].as_mut().unwrap().kind = BuildingKind::Settlement;
    engine.state.position.phase = Phase::PreRoll;
    reject(&mut engine, seat(0), 40, RuleError::WrongPhase);
    engine.state.position.phase = Phase::Action;
    for hand in [[0, 0, 0, 1, 3], [0, 0, 0, 2, 2]] {
        engine.state.position.hands[0] = Resources::try_from(hand).unwrap();
        engine.state.position.bank = Resources::try_from(hand.map(|n| 19 - n)).unwrap();
        reject(&mut engine, seat(0), 40, RuleError::InsufficientResources);
    }
    engine = fixture();
    engine.state.position.buildings = [None; 54];
    for id in [2, 17, 31, 40] {
        engine.state.position.buildings[id] = Some(Building {
            owner: seat(0),
            kind: BuildingKind::City,
        });
    }
    engine.state.position.buildings[9] = Some(Building {
        owner: seat(0),
        kind: BuildingKind::Settlement,
    });
    reject(&mut engine, seat(0), 9, RuleError::NoPieces);
    engine = fixture();
    let before = engine.state.position.clone();
    let random = engine.state.random.clone();
    assert_eq!(
        engine
            .execute(CommandEnvelope {
                api_version: 1,
                actor: seat(0),
                expected_revision: 1,
                command: Command::UpgradeCity { vertex: vertex(40) }
            })
            .unwrap_err()
            .code,
        RuleError::StaleRevision
    );
    assert_eq!(engine.state.position, before);
    assert_eq!(engine.state.random, random);
    assert_eq!(engine.state.revision, 0);
    engine.state.revision = u64::MAX;
    reject(&mut engine, seat(0), 40, RuleError::CounterExhausted);
    assert_eq!(RuleError::NotOwned.to_string(), "not owned");
}
#[test]
fn city_returns_the_fifth_settlement_piece_for_a_later_paid_placement() {
    let mut engine = fixture();
    engine.state.position.buildings = [None; 54];
    for id in [2, 9, 17, 31, 40] {
        engine.state.position.buildings[id] = Some(Building {
            owner: seat(0),
            kind: BuildingKind::Settlement,
        });
    }
    engine.state.position.roads = [None; 72];
    for id in [7, 13, 14] {
        engine.state.position.roads[id] = Some(seat(0));
    }
    engine.state.position.road_lengths = [3, 0, 0, 0];
    engine.state.position.longest_route_holder = None;
    engine.state.position.hands[0] = Resources::try_from([1, 1, 1, 3, 3]).unwrap();
    engine.state.position.bank = Resources::try_from([18, 18, 18, 16, 16]).unwrap();
    assert_eq!(
        engine.view(Viewer::Spectator).unwrap().players[0].settlements_remaining,
        0
    );
    assert_eq!(
        engine
            .validate(seat(0), &Command::BuildSettlement { vertex: vertex(4) })
            .unwrap_err()
            .code,
        RuleError::NoPieces
    );
    execute(&mut engine, 9);
    assert_eq!(
        engine.view(Viewer::Spectator).unwrap().players[0].settlements_remaining,
        1
    );
    engine
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(0),
            expected_revision: 1,
            command: Command::BuildSettlement { vertex: vertex(4) },
        })
        .unwrap();
    let view = engine.view(Viewer::Spectator).unwrap();
    assert_eq!(
        (
            view.players[0].settlements_remaining,
            view.players[0].cities_remaining,
            view.players[0].public_score
        ),
        (0, 3, 7)
    );
    assert_eq!(view.bank.counts(), &[19; 5]);
    assert_eq!(engine.state.position.hands[0], Resources::default());
}
#[test]
fn city_winning_upgrade_preserves_route_award_and_freezes_game() {
    let mut engine = fixture();
    engine.state.position.buildings = [None; 54];
    for (id, kind) in [
        (31, BuildingKind::City),
        (40, BuildingKind::City),
        (42, BuildingKind::City),
        (48, BuildingKind::Settlement),
    ] {
        engine.state.position.buildings[id] = Some(Building {
            owner: seat(0),
            kind,
        });
    }
    // V42-V37-V31-V36-V30-V35-V40-V44-V48, eight distinct owned edges.
    engine.state.position.roads = [None; 72];
    for id in [53, 47, 46, 45, 44, 51, 57, 63] {
        engine.state.position.roads[id] = Some(seat(0));
    }
    engine.state.position.road_lengths = [8, 0, 0, 0];
    engine.state.position.longest_route_holder = Some(seat(0));
    for id in [31, 40, 42, 48] {
        assert!(
            engine
                .board
                .graph
                .vertex_neighbors(vertex(id))
                .iter()
                .all(|neighbor| engine.state.position.buildings[neighbor.index()].is_none())
        );
    }
    assert_eq!(
        crate::base_rules::longest_route_length(
            engine.board.graph,
            &engine.state.position.roads,
            &engine.state.position.buildings,
            seat(0)
        ),
        8
    );
    assert_eq!(
        engine.view(Viewer::Spectator).unwrap().players[0].public_score,
        9
    );
    crate::base_rules::reset_route_searches();
    let receipt = execute(&mut engine, 48);
    assert_eq!(
        receipt.events.last(),
        Some(&Event::GameFinished {
            winner: seat(0),
            score: 10
        })
    );
    assert_eq!(
        engine.state.position.phase,
        Phase::Finished {
            winner: seat(0),
            score: 10
        }
    );
    assert_eq!(engine.state.position.road_lengths, [8, 0, 0, 0]);
    assert_eq!(engine.state.position.longest_route_holder, Some(seat(0)));
    assert_eq!(crate::base_rules::route_searches(), 0);
    assert!(engine.legal_actions(seat(0)).unwrap().choices.is_empty());
    reject(&mut engine, seat(0), 40, RuleError::GameFinished);
}
