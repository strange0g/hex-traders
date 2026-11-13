//! Bank/port fixtures use the captured fixed coastal endpoint literals.
use super::*;
use crate::{Building, BuildingKind, GameConfig};
const KINDS: [Resource; 5] = [
    Resource::Brick,
    Resource::Wood,
    Resource::Wool,
    Resource::Wheat,
    Resource::Ore,
];
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
    position.roads = [None; 72];
    position.road_lengths = [0; 4];
    position.longest_route_holder = None;
    position.buildings = [None; 54];
    position.hands = [Resources::default(); 4];
    position.hands[0] = Resources::try_from([4; 5]).unwrap();
    position.bank = Resources::try_from([15; 5]).unwrap();
    engine
}
fn building(engine: &mut Engine, id: usize, owner: u8, kind: BuildingKind) {
    engine.state.position.buildings[id] = Some(Building {
        owner: seat(owner),
        kind,
    });
}
fn execute(engine: &mut Engine, give: Resource, receive: Resource) -> Receipt {
    engine
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(0),
            expected_revision: engine.state.revision,
            command: Command::BankTrade {
                give_resource: give,
                receive_resource: receive,
            },
        })
        .unwrap()
}
fn reject(
    engine: &mut Engine,
    actor: PlayerId,
    give: Resource,
    receive: Resource,
    error: RuleError,
) {
    let before = engine.state.position.clone();
    let random = engine.state.random.clone();
    let revision = engine.state.revision;
    let command = Command::BankTrade {
        give_resource: give,
        receive_resource: receive,
    };
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
fn base_rules_br023_bank_trade_all_ports_both_endpoints_building_kinds_and_best_rate() {
    // Generic E0/E17/E38/E66; wool E3, brick E61, wood E69, wheat E49, ore E18.
    for (vertices, give, rate) in [
        ([0, 3], Resource::Brick, 3),
        ([10, 15], Resource::Wood, 3),
        ([26, 32], Resource::Wool, 3),
        ([47, 51], Resource::Wheat, 3),
        ([1, 5], Resource::Wool, 2),
        ([42, 46], Resource::Brick, 2),
        ([49, 52], Resource::Wood, 2),
        ([33, 38], Resource::Wheat, 2),
        ([11, 16], Resource::Ore, 2),
    ] {
        for id in vertices {
            for kind in [BuildingKind::Settlement, BuildingKind::City] {
                let mut engine = fixture();
                building(&mut engine, id, 0, kind);
                let receive = if give == Resource::Ore {
                    Resource::Brick
                } else {
                    Resource::Ore
                };
                let receipt = execute(&mut engine, give, receive);
                assert_eq!(
                    receipt.events,
                    vec![Event::BankTraded {
                        seat: seat(0),
                        give_resource: give,
                        receive_resource: receive,
                        rate
                    }]
                );
                assert_eq!(
                    engine.state.position.hands[0].counts()[give.index()],
                    4 - rate
                );
                assert_eq!(engine.state.position.hands[0].counts()[receive.index()], 5);
                assert_eq!(engine.state.position.bank.counts()[give.index()], 15 + rate);
                assert_eq!(engine.state.position.bank.counts()[receive.index()], 14);
            }
        }
    }
    let mut engine = fixture();
    building(&mut engine, 0, 0, BuildingKind::Settlement);
    building(&mut engine, 42, 0, BuildingKind::Settlement);
    assert_eq!(
        execute(&mut engine, Resource::Brick, Resource::Ore).events[0],
        Event::BankTraded {
            seat: seat(0),
            give_resource: Resource::Brick,
            receive_resource: Resource::Ore,
            rate: 2
        }
    );
    assert_eq!(
        execute(&mut engine, Resource::Wood, Resource::Ore).events[0],
        Event::BankTraded {
            seat: seat(0),
            give_resource: Resource::Wood,
            receive_resource: Resource::Ore,
            rate: 3
        }
    );
    let mut engine = fixture();
    building(&mut engine, 0, 1, BuildingKind::City);
    building(&mut engine, 42, 2, BuildingKind::Settlement);
    assert_eq!(
        execute(&mut engine, Resource::Brick, Resource::Ore).events[0],
        Event::BankTraded {
            seat: seat(0),
            give_resource: Resource::Brick,
            receive_resource: Resource::Ore,
            rate: 4
        }
    );
    let mut engine = fixture();
    building(&mut engine, 42, 0, BuildingKind::Settlement);
    assert_eq!(
        execute(&mut engine, Resource::Wood, Resource::Ore).events[0],
        Event::BankTraded {
            seat: seat(0),
            give_resource: Resource::Wood,
            receive_resource: Resource::Ore,
            rate: 4
        }
    );
}
#[test]
fn bank_trade_all_pairs_queries_and_execute_agree_without_rng_or_route_searches() {
    let mut engine = fixture();
    let before = engine.state.position.clone();
    let random = engine.state.random.clone();
    crate::base_rules::reset_route_searches();
    let expected = KINDS
        .into_iter()
        .flat_map(|give| {
            KINDS
                .into_iter()
                .filter(move |&receive| receive != give)
                .map(move |receive| (give, receive))
        })
        .collect::<Vec<_>>();
    assert!(
        engine
            .legal_actions(seat(0))
            .unwrap()
            .choices
            .contains(&LegalAction::BankTrade {
                pairs: expected.clone()
            })
    );
    assert_eq!(crate::base_rules::route_searches(), 0);
    for give in KINDS {
        for receive in KINDS {
            assert_eq!(
                engine
                    .validate(
                        seat(0),
                        &Command::BankTrade {
                            give_resource: give,
                            receive_resource: receive
                        }
                    )
                    .is_ok(),
                give != receive
            );
            if give != receive {
                let mut fresh = fixture();
                crate::base_rules::reset_route_searches();
                execute(&mut fresh, give, receive);
                assert_eq!(crate::base_rules::route_searches(), 0);
            }
        }
    }
    assert_eq!(engine.state.position, before);
    assert_eq!(engine.state.random, random);
    assert_eq!(engine.state.revision, 0);
    assert_eq!(crate::base_rules::route_searches(), 0);
    let receipt = execute(&mut engine, Resource::Brick, Resource::Ore);
    assert_eq!(engine.state.position.hands[0].counts(), &[0, 4, 4, 4, 5]);
    assert_eq!(engine.state.position.bank.counts(), &[19, 15, 15, 15, 14]);
    assert_eq!(engine.state.revision, 1);
    assert_eq!(engine.state.random, random);
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
                events: vec![ProjectedEvent::BankTraded {
                    seat: seat(0),
                    give_resource: Resource::Brick,
                    receive_resource: Resource::Ore,
                    rate: 4
                }]
            }
        );
    }
    assert_eq!(crate::base_rules::route_searches(), 0);
}
#[test]
fn bank_trade_rejections_are_atomic_and_queries_ignore_other_private_hands() {
    let mut engine = fixture();
    reject(
        &mut engine,
        seat(1),
        Resource::Brick,
        Resource::Ore,
        RuleError::WrongActor,
    );
    reject(
        &mut engine,
        seat(0),
        Resource::Brick,
        Resource::Brick,
        RuleError::InvalidTrade,
    );
    for phase in [
        Phase::PreRoll,
        Phase::Discard {
            obligations: [1, 0, 0, 0],
        },
    ] {
        engine.state.position.phase = phase;
        reject(
            &mut engine,
            seat(0),
            Resource::Brick,
            Resource::Ore,
            RuleError::WrongPhase,
        );
    }
    engine.state.position.phase = Phase::Action;
    engine.state.position.hands[0] = Resources::try_from([3, 0, 0, 0, 0]).unwrap();
    engine.state.position.bank = Resources::try_from([16, 19, 19, 19, 19]).unwrap();
    reject(
        &mut engine,
        seat(0),
        Resource::Brick,
        Resource::Ore,
        RuleError::InsufficientResources,
    );
    assert!(
        !engine
            .legal_actions(seat(0))
            .unwrap()
            .choices
            .iter()
            .any(|choice| matches!(choice, LegalAction::BankTrade { .. }))
    );
    engine.state.position.hands[0] = Resources::try_from([4, 0, 0, 0, 19]).unwrap();
    engine.state.position.bank = Resources::try_from([15, 19, 19, 19, 0]).unwrap();
    reject(
        &mut engine,
        seat(0),
        Resource::Brick,
        Resource::Ore,
        RuleError::BankShortage,
    );
    engine = fixture();
    let before = engine.state.position.clone();
    let random = engine.state.random.clone();
    assert_eq!(
        engine
            .execute(CommandEnvelope {
                api_version: 1,
                actor: seat(0),
                expected_revision: 1,
                command: Command::BankTrade {
                    give_resource: Resource::Brick,
                    receive_resource: Resource::Ore
                }
            })
            .unwrap_err()
            .code,
        RuleError::StaleRevision
    );
    assert_eq!(engine.state.position, before);
    assert_eq!(engine.state.random, random);
    assert_eq!(engine.state.revision, 0);
    engine.state.revision = u64::MAX;
    reject(
        &mut engine,
        seat(0),
        Resource::Brick,
        Resource::Ore,
        RuleError::CounterExhausted,
    );
    engine = fixture();
    let choices = engine.legal_actions(seat(0)).unwrap();
    engine.state.position.hands[1] = Resources::try_from([1, 2, 3, 4, 5]).unwrap();
    engine.state.position.hands[2] = Resources::try_from([5, 4, 3, 2, 1]).unwrap();
    engine.state.position.bank = Resources::try_from([9; 5]).unwrap();
    assert_eq!(engine.legal_actions(seat(0)).unwrap(), choices);
    assert_eq!(
        engine.legal_actions(seat(1)).unwrap().choices,
        vec![LegalAction::ProposeTrade {
            recipients: vec![seat(0)]
        }]
    );
    assert_eq!(RuleError::InvalidTrade.to_string(), "invalid trade");
    assert_eq!(RuleError::BankShortage.to_string(), "bank shortage");
}
#[test]
fn base_rules_br024_bank_trade_new_port_and_owned_city_rate_apply_immediately() {
    let mut engine = fixture();
    engine.state.position.hands[0] = Resources::try_from([3, 1, 1, 1, 0]).unwrap();
    engine.state.position.bank = Resources::try_from([16, 18, 18, 18, 19]).unwrap();
    building(&mut engine, 31, 0, BuildingKind::Settlement);
    for edge in [47, 53] {
        engine.state.position.roads[edge] = Some(seat(0));
    }
    engine.state.position.road_lengths = [2, 0, 0, 0];
    reject(
        &mut engine,
        seat(0),
        Resource::Brick,
        Resource::Ore,
        RuleError::InsufficientResources,
    );
    crate::base_rules::reset_route_searches();
    engine
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(0),
            expected_revision: 0,
            command: Command::BuildSettlement { vertex: vertex(42) },
        })
        .unwrap();
    assert_eq!(
        execute(&mut engine, Resource::Brick, Resource::Ore).events,
        vec![Event::BankTraded {
            seat: seat(0),
            give_resource: Resource::Brick,
            receive_resource: Resource::Ore,
            rate: 2
        }]
    );
    assert_eq!(engine.state.position.hands[0].counts(), &[0, 0, 0, 0, 1]);
    assert_eq!(engine.state.position.bank.counts(), &[19, 19, 19, 19, 18]);
    assert_eq!(crate::base_rules::route_searches(), 0);
    let mut engine = fixture();
    building(&mut engine, 42, 0, BuildingKind::Settlement);
    engine.state.position.hands[0] = Resources::try_from([4, 0, 0, 2, 3]).unwrap();
    engine.state.position.bank = Resources::try_from([15, 19, 19, 17, 16]).unwrap();
    engine
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(0),
            expected_revision: 0,
            command: Command::UpgradeCity { vertex: vertex(42) },
        })
        .unwrap();
    assert_eq!(
        execute(&mut engine, Resource::Brick, Resource::Ore).events[0],
        Event::BankTraded {
            seat: seat(0),
            give_resource: Resource::Brick,
            receive_resource: Resource::Ore,
            rate: 2
        }
    );
    assert_eq!(engine.state.position.hands[0].counts(), &[2, 0, 0, 0, 1]);
    assert_eq!(engine.state.position.bank.counts(), &[17, 19, 19, 19, 18]);
}

#[test]
fn base_rules_br015_robber_allows_adjacent_construction_port_trade_and_resource_use() {
    let mut e = fixture();
    let hex = crate::HexId::try_from(15).unwrap();
    e.state.position.robber = hex;
    building(&mut e, 31, 0, BuildingKind::Settlement);
    for id in [47, 53] {
        e.state.position.roads[id] = Some(seat(0));
    }
    e.state.position.road_lengths = [2, 0, 0, 0];
    e.state.position.hands[0] = Resources::try_from([4, 2, 1, 1, 0]).unwrap();
    e.state.position.hands[1] = Resources::try_from([0, 0, 1, 0, 0]).unwrap();
    e.state.position.bank = Resources::try_from([15, 17, 17, 18, 19]).unwrap();
    // H15 [31,37,42,46,41,36], brick port E61 [42,46] are independent source facts.
    assert_eq!(
        e.board.graph.hex_vertices(hex).map(VertexId::index),
        [31, 37, 42, 46, 41, 36]
    );
    let random = e.state.random.clone();
    for command in [
        Command::BuildSettlement { vertex: vertex(42) },
        Command::BuildRoad {
            edge: EdgeId::try_from(61).unwrap(),
        },
        Command::BankTrade {
            give_resource: Resource::Brick,
            receive_resource: Resource::Ore,
        },
        Command::ProposeTrade {
            recipient: seat(1),
            give: Resources::try_from([0, 0, 0, 0, 1]).unwrap(),
            receive: Resources::try_from([0, 0, 1, 0, 0]).unwrap(),
        },
    ] {
        assert!(e.validate(seat(0), &command).is_ok());
        e.execute(CommandEnvelope {
            api_version: 1,
            actor: seat(0),
            expected_revision: e.state.revision,
            command,
        })
        .unwrap();
        assert_eq!(e.state.position.robber, hex);
    }
    let offer = e.state.position.offer.unwrap();
    e.execute(CommandEnvelope {
        api_version: 1,
        actor: seat(1),
        expected_revision: e.state.revision,
        command: Command::AcceptTrade { offer_id: offer.id },
    })
    .unwrap();
    assert_eq!(
        e.state.position.buildings[42],
        Some(Building {
            owner: seat(0),
            kind: BuildingKind::Settlement
        })
    );
    assert_eq!(e.state.position.roads[61], Some(seat(0)));
    assert_eq!(e.state.position.hands[0].counts(), &[0, 0, 1, 0, 0]);
    assert_eq!(e.state.position.hands[1].counts(), &[0, 0, 0, 0, 1]);
    assert_eq!(e.state.position.bank.counts(), &[19, 19, 18, 19, 18]);
    assert_eq!(e.state.position.robber, hex);
    assert_eq!(e.state.random, random);
}
