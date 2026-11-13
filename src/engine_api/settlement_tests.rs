//! Isolated transition fixtures use independently traced graph edges, not full histories.
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
    let position = &mut engine.state.position;
    position.active_player = seat(0);
    position.phase = Phase::Action;
    position.buildings = [None; 54];
    position.buildings[12] = Some(Building {
        owner: seat(0),
        kind: BuildingKind::Settlement,
    });
    position.roads = [None; 72];
    // V12-E12-V8-E7-V4; V12 is two steps from the target.
    for edge in [7, 12] {
        position.roads[edge] = Some(seat(0));
    }
    position.road_lengths = [2, 0, 0, 0];
    position.longest_route_holder = None;
    position.hands = [Resources::default(); 4];
    position.hands[0] = Resources::try_from([2, 2, 2, 2, 0]).unwrap();
    position.bank = Resources::try_from([17, 17, 17, 17, 19]).unwrap();
    engine
}
fn execute(engine: &mut Engine, id: u8) -> Receipt {
    engine
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(0),
            expected_revision: engine.state.revision,
            command: Command::BuildSettlement { vertex: vertex(id) },
        })
        .unwrap()
}
fn reject(engine: &mut Engine, actor: PlayerId, id: u8, error: RuleError) {
    let before = engine.state.position.clone();
    let random = engine.state.random.clone();
    let revision = engine.state.revision;
    let command = Command::BuildSettlement { vertex: vertex(id) };
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
fn settlement_payment_candidates_and_old_receipt_projection_are_atomic() {
    let mut engine = fixture();
    crate::base_rules::reset_route_searches();
    let actions = engine.legal_actions(seat(0)).unwrap();
    assert!(actions.choices.contains(&LegalAction::BuildSettlement {
        vertices: vec![vertex(4)]
    }));
    for id in 0..54 {
        assert_eq!(
            engine
                .validate(seat(0), &Command::BuildSettlement { vertex: vertex(id) })
                .is_ok(),
            id == 4
        );
    }
    let random = engine.state.random.clone();
    let receipt = execute(&mut engine, 4);
    let cost = Resources::try_from([1, 1, 1, 1, 0]).unwrap();
    assert_eq!(
        receipt.events,
        vec![
            Event::ConstructionPaid {
                seat: seat(0),
                resources: cost
            },
            Event::SettlementPlaced {
                seat: seat(0),
                vertex: vertex(4)
            }
        ]
    );
    assert_eq!(engine.state.position.hands[0].counts(), &[1, 1, 1, 1, 0]);
    assert_eq!(engine.state.position.bank.counts(), &[18, 18, 18, 18, 19]);
    assert_eq!(engine.state.position.road_lengths, [2, 0, 0, 0]);
    assert_eq!(engine.state.random, random);
    assert_eq!(engine.state.revision, 1);
    assert_eq!(crate::base_rules::route_searches(), 0);
    let projected = vec![
        ProjectedEvent::ConstructionPaid {
            seat: seat(0),
            resources: cost,
        },
        ProjectedEvent::SettlementPlaced {
            seat: seat(0),
            vertex: vertex(4),
        },
    ];
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
        assert_eq!(
            engine.project_receipt(&receipt, viewer).unwrap(),
            ProjectedReceipt {
                revision: 1,
                events: projected.clone()
            }
        );
        let view = engine.view(viewer).unwrap();
        assert_eq!(view.players[0].settlements_remaining, 3);
        assert_eq!(view.players[0].public_score, 2);
    }
    assert_eq!(crate::base_rules::route_searches(), 0);
}

#[test]
fn base_rules_br018_settlement_rejections_preserve_complete_state() {
    let mut engine = fixture();
    reject(&mut engine, seat(1), 4, RuleError::WrongActor);
    reject(&mut engine, seat(0), 12, RuleError::Occupied);
    reject(&mut engine, seat(0), 8, RuleError::DistanceRule);
    reject(&mut engine, seat(0), 53, RuleError::Disconnected);
    engine.state.position.phase = Phase::PreRoll;
    reject(&mut engine, seat(0), 4, RuleError::WrongPhase);
    engine.state.position.phase = Phase::Action;
    for owner in [seat(0), seat(1)] {
        engine.state.position.buildings[0] = Some(Building {
            owner,
            kind: BuildingKind::Settlement,
        });
        reject(&mut engine, seat(0), 4, RuleError::DistanceRule);
    }
    engine.state.position.buildings[0] = None;
    for resource in 0..4 {
        let mut hand = [1, 1, 1, 1, 0];
        hand[resource] = 0;
        engine.state.position.hands[0] = Resources::try_from(hand).unwrap();
        engine.state.position.bank = Resources::try_from(hand.map(|n| 19 - n)).unwrap();
        reject(&mut engine, seat(0), 4, RuleError::InsufficientResources);
    }
    engine = fixture();
    // Pairwise distance-valid five own settlements; V4 remains empty and nonadjacent.
    for id in [9, 2, 31, 40] {
        engine.state.position.buildings[id] = Some(Building {
            owner: seat(0),
            kind: BuildingKind::Settlement,
        });
    }
    reject(&mut engine, seat(0), 4, RuleError::NoPieces);
    engine = fixture();
    let before = engine.state.position.clone();
    let random = engine.state.random.clone();
    assert_eq!(
        engine
            .execute(CommandEnvelope {
                api_version: 1,
                actor: seat(0),
                expected_revision: 1,
                command: Command::BuildSettlement { vertex: vertex(4) }
            })
            .unwrap_err()
            .code,
        RuleError::StaleRevision
    );
    assert_eq!(engine.state.position, before);
    assert_eq!(engine.state.random, random);
    assert_eq!(engine.state.revision, 0);
    engine.state.revision = u64::MAX;
    reject(&mut engine, seat(0), 4, RuleError::CounterExhausted);
}

#[test]
fn base_rules_br035_settlement_refreshes_only_through_opponent_and_returns_award() {
    let mut engine = fixture();
    // V7-E6-V3-E0-V0-E1-V4-E2-V1-E3-V5-E4-V2, exactly six edges.
    for edge in [6, 0, 1, 2, 3, 4] {
        engine.state.position.roads[edge] = Some(seat(1));
    }
    engine.state.position.buildings[2] = Some(Building {
        owner: seat(1),
        kind: BuildingKind::Settlement,
    });
    engine.state.position.road_lengths = [2, 6, 0, 0];
    engine.state.position.longest_route_holder = Some(seat(1));
    assert_eq!(
        crate::base_rules::longest_route_length(
            engine.board.graph,
            &engine.state.position.roads,
            &engine.state.position.buildings,
            seat(1)
        ),
        6
    );
    crate::base_rules::reset_route_searches();
    let receipt = execute(&mut engine, 4);
    assert_eq!(engine.state.position.road_lengths, [2, 3, 0, 0]);
    assert_eq!(engine.state.position.longest_route_holder, None);
    assert_eq!(
        receipt.events.last(),
        Some(&Event::LongestRouteChanged {
            previous: Some(seat(1)),
            holder: None
        })
    );
    assert_eq!(crate::base_rules::route_searches(), 1);
    assert_eq!(
        engine.view(Viewer::Spectator).unwrap().players[1].public_score,
        1
    );
    let roads = engine.state.position.roads;
    assert_eq!(roads[1], Some(seat(1)));
    assert_eq!(roads[2], Some(seat(1)));
    reject(&mut engine, seat(0), 4, RuleError::Occupied);
    assert_eq!(crate::base_rules::route_searches(), 1);
}

#[test]
fn settlement_multiple_opponents_touching_do_not_create_an_impossible_through_route() {
    let mut engine = fixture();
    engine.state.position.roads[1] = Some(seat(1));
    engine.state.position.roads[2] = Some(seat(2));
    for (id, owner) in [(0, 1), (3, 2)] {
        engine.state.position.roads[id] = Some(seat(owner));
    }
    for (id, owner) in [(3, 1), (5, 2)] {
        engine.state.position.buildings[id] = Some(Building {
            owner: seat(owner),
            kind: BuildingKind::Settlement,
        });
    }
    engine.state.position.road_lengths = [2, 2, 2, 0];
    crate::base_rules::reset_route_searches();
    execute(&mut engine, 4);
    assert_eq!(engine.state.position.road_lengths, [2, 2, 2, 0]);
    assert_eq!(crate::base_rules::route_searches(), 0);
}

#[test]
fn settlement_score_increase_finishes_active_owner_before_more_actions() {
    let mut engine = fixture();
    engine.state.position.buildings = [None; 54];
    for (id, kind) in [
        (9, BuildingKind::City),
        (17, BuildingKind::City),
        (31, BuildingKind::City),
        (40, BuildingKind::Settlement),
    ] {
        engine.state.position.buildings[id] = Some(Building {
            owner: seat(0),
            kind,
        });
    }
    engine.state.position.roads = [None; 72];
    // V4-V8-V13-V18-V23-V17: five-edge trail connected to own V17 city.
    for edge in [7, 13, 20, 27, 26] {
        engine.state.position.roads[edge] = Some(seat(0));
    }
    engine.state.position.road_lengths = [5, 0, 0, 0];
    engine.state.position.longest_route_holder = Some(seat(0));
    assert_eq!(
        crate::base_rules::current_score(&engine.state.position, seat(0)),
        9
    );
    let receipt = execute(&mut engine, 4);
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
    assert!(engine.legal_actions(seat(0)).unwrap().choices.is_empty());
    reject(&mut engine, seat(0), 53, RuleError::GameFinished);
}

// Independent subset-DP expected results were verified during T13.
// These are bounded engineered positions, not accepted setup-to-win histories.
fn award_fixture(off_turn: bool) -> Engine {
    let mut engine = fixture();
    let position = &mut engine.state.position;
    position.roads = [None; 72];
    let roads: [&[usize]; 3] = if off_turn {
        [
            &[11, 19, 70],
            &[0, 6, 10, 18, 23, 33, 39, 49, 54],
            &[5, 9, 17, 22, 32, 38, 48, 53, 61],
        ]
    } else {
        [
            &[11, 19, 26, 35, 43, 51, 58, 64],
            &[0, 6, 10, 18, 23, 33, 39, 49, 54],
            &[5, 46],
        ]
    };
    for (owner, ids) in roads.into_iter().enumerate() {
        for &id in ids {
            position.roads[id] = Some(seat(owner as u8));
        }
    }
    position.buildings = [None; 54];
    let buildings: &[(u8, usize, BuildingKind)] = if off_turn {
        &[
            (0, 17, BuildingKind::Settlement),
            (0, 49, BuildingKind::Settlement),
            (1, 0, BuildingKind::Settlement),
            (1, 21, BuildingKind::Settlement),
            (2, 6, BuildingKind::City),
            (2, 15, BuildingKind::City),
            (2, 26, BuildingKind::City),
            (2, 37, BuildingKind::City),
        ]
    } else {
        &[
            (0, 17, BuildingKind::City),
            (0, 29, BuildingKind::City),
            (0, 40, BuildingKind::City),
            (0, 49, BuildingKind::Settlement),
            (1, 0, BuildingKind::Settlement),
            (1, 21, BuildingKind::Settlement),
            (2, 6, BuildingKind::Settlement),
            (2, 31, BuildingKind::Settlement),
        ]
    };
    for &(owner, id, kind) in buildings {
        position.buildings[id] = Some(Building {
            owner: seat(owner),
            kind,
        });
    }
    position.road_lengths = if off_turn { [2, 9, 9, 0] } else { [8, 9, 1, 0] };
    position.longest_route_holder = Some(seat(1));
    position.hands = [Resources::default(); 4];
    position.hands[0] = Resources::try_from([1, 1, 1, 1, 0]).unwrap();
    position.bank = Resources::try_from([18, 18, 18, 18, 19]).unwrap();
    for id in 0..54 {
        if position.buildings[id].is_some() {
            assert!(
                engine
                    .board
                    .graph
                    .vertex_neighbors(vertex(id as u8))
                    .iter()
                    .all(|neighbor| position.buildings[neighbor.index()].is_none())
            );
        }
    }
    for owner in 0..3 {
        assert_eq!(
            crate::base_rules::longest_route_length(
                engine.board.graph,
                &position.roads,
                &position.buildings,
                seat(owner)
            ),
            position.road_lengths[owner as usize]
        );
    }
    engine
}
#[test]
fn settlement_award_transfer_and_actor_score_finish_in_the_same_revision() {
    let mut engine = award_fixture(false);
    assert_eq!(
        engine
            .view(Viewer::Spectator)
            .unwrap()
            .players
            .iter()
            .map(|p| p.public_score)
            .collect::<Vec<_>>(),
        [7, 4, 2]
    );
    crate::base_rules::reset_route_searches();
    let receipt = execute(&mut engine, 7);
    assert_eq!(engine.state.position.road_lengths, [8, 7, 1, 0]);
    assert_eq!(engine.state.position.longest_route_holder, Some(seat(0)));
    assert_eq!(
        &receipt.events[2..],
        &[
            Event::LongestRouteChanged {
                previous: Some(seat(1)),
                holder: Some(seat(0))
            },
            Event::GameFinished {
                winner: seat(0),
                score: 10
            }
        ]
    );
    assert_eq!(engine.state.position.bank.counts(), &[19; 5]);
    assert_eq!(engine.state.position.hands[0], Resources::default());
    assert_eq!(engine.state.revision, 1);
    assert_eq!(crate::base_rules::route_searches(), 1);
    assert_eq!(
        engine
            .view(Viewer::Spectator)
            .unwrap()
            .players
            .iter()
            .map(|p| p.public_score)
            .collect::<Vec<_>>(),
        [10, 2, 2]
    );
    reject(&mut engine, seat(0), 53, RuleError::GameFinished);
}
#[test]
fn settlement_offturn_award_gives_ten_but_waits_until_the_owners_turn() {
    let mut engine = award_fixture(true);
    assert_eq!(
        engine
            .view(Viewer::Spectator)
            .unwrap()
            .players
            .iter()
            .map(|p| p.public_score)
            .collect::<Vec<_>>(),
        [2, 4, 8]
    );
    crate::base_rules::reset_route_searches();
    let receipt = execute(&mut engine, 7);
    assert_eq!(engine.state.position.road_lengths, [2, 7, 9, 0]);
    assert_eq!(engine.state.position.longest_route_holder, Some(seat(2)));
    assert_eq!(
        receipt.events.last(),
        Some(&Event::LongestRouteChanged {
            previous: Some(seat(1)),
            holder: Some(seat(2))
        })
    );
    assert_eq!(engine.state.position.phase, Phase::Action);
    assert_eq!(
        engine
            .view(Viewer::Spectator)
            .unwrap()
            .players
            .iter()
            .map(|p| p.public_score)
            .collect::<Vec<_>>(),
        [3, 2, 10]
    );
    assert_eq!(crate::base_rules::route_searches(), 1);
    engine
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(0),
            expected_revision: 1,
            command: Command::EndTurn,
        })
        .unwrap();
    assert_eq!(engine.state.position.phase, Phase::PreRoll);
    engine
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(1),
            expected_revision: 2,
            command: Command::RollDice,
        })
        .unwrap();
    if matches!(engine.state.position.phase, Phase::RobberMove { .. }) {
        engine
            .execute(CommandEnvelope {
                api_version: 1,
                actor: seat(1),
                expected_revision: engine.state.revision,
                command: Command::MoveRobber {
                    hex: HexId::try_from(18).unwrap(),
                },
            })
            .unwrap();
    }
    assert_eq!(engine.state.position.phase, Phase::Action);
    let revision = engine.state.revision;
    let receipt = engine
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(1),
            expected_revision: revision,
            command: Command::EndTurn,
        })
        .unwrap();
    assert_eq!(
        receipt.events.last(),
        Some(&Event::GameFinished {
            winner: seat(2),
            score: 10
        })
    );
    assert_eq!(
        engine.state.position.phase,
        Phase::Finished {
            winner: seat(2),
            score: 10
        }
    );
    assert!(engine.legal_actions(seat(2)).unwrap().choices.is_empty());
}

#[test]
fn base_rules_br037_offturn_ten_can_be_beaten_by_active_purchase_before_owner_turn() {
    let mut engine = award_fixture(true);
    let p = &mut engine.state.position;
    for (draw, id) in [0u8, 1, 2, 20, 21, 22, 23, 24].into_iter().enumerate() {
        let index = p
            .deck
            .iter()
            .position(|c| c.index() == usize::from(id))
            .unwrap();
        p.deck.swap(draw, index);
        if draw < 7 {
            p.card_locations[usize::from(id)] = if id < 3 {
                crate::game_model::CardLocation::PlayedKnight { owner: seat(0) }
            } else {
                crate::game_model::CardLocation::Held {
                    owner: seat(0),
                    acquired_turn: 1,
                }
            };
        }
    }
    p.draw_cursor = 7;
    p.largest_army_holder = Some(seat(0));
    p.hands[0] = Resources::try_from([1, 1, 2, 2, 1]).unwrap();
    p.bank = Resources::try_from([18, 18, 17, 17, 18]).unwrap();
    assert_eq!(crate::base_rules::current_score(p, seat(0)), 8);
    let rng = engine.state.random.clone();
    let r = execute(&mut engine, 7);
    assert_eq!(r.final_scores, None);
    assert_eq!(engine.state.position.phase, Phase::Action);
    assert_eq!(
        crate::base_rules::current_score(&engine.state.position, seat(0)),
        9
    );
    assert_eq!(
        crate::base_rules::current_score(&engine.state.position, seat(2)),
        10
    );
    let r = engine
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(0),
            expected_revision: 1,
            command: Command::BuyDevelopmentCard,
        })
        .unwrap();
    assert_eq!(r.final_scores, Some([10, 2, 10, 0]));
    assert_eq!(
        engine.state.position.phase,
        Phase::Finished {
            winner: seat(0),
            score: 10
        }
    );
    assert_eq!(
        r.events.last(),
        Some(&Event::GameFinished {
            winner: seat(0),
            score: 10
        })
    );
    assert_eq!(engine.state.random, rng);
    assert_eq!(engine.state.position.active_player, seat(0));
    assert_eq!(
        engine.view(Viewer::Spectator).unwrap().players[2].public_score,
        10
    );
}
