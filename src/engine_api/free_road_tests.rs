//! Bounded Road Building edge cases; public seeded histories are separate.
use super::*;
use crate::game_model::CardLocation;
use crate::{Building, BuildingKind, DevelopmentKind, GameConfig, ResumePhase};
fn seat(id: u8) -> PlayerId {
    PlayerId::try_from(id).unwrap()
}
fn card(id: u8) -> CardId {
    CardId::try_from(id).unwrap()
}
fn edge(id: u8) -> EdgeId {
    EdgeId::try_from(id).unwrap()
}
fn fixture() -> Engine {
    let (mut e, _) = Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), [0; 32]).unwrap();
    let p = &mut e.state.position;
    p.turn = 2;
    p.active_player = seat(0);
    p.phase = Phase::PreRoll;
    for (draw, id, owner, turn) in [(0, 14, 0, 1), (1, 15, 0, 2), (2, 18, 0, 1), (3, 20, 1, 1)] {
        let index = p.deck.iter().position(|&c| c == card(id)).unwrap();
        p.deck.swap(draw, index);
        p.card_locations[id as usize] = CardLocation::Held {
            owner: seat(owner),
            acquired_turn: turn,
        };
    }
    p.draw_cursor = 4;
    e
}
fn play() -> Command {
    Command::PlayRoadBuilding { card: card(14) }
}
fn place(id: u8) -> Command {
    Command::PlaceFreeRoad { edge: edge(id) }
}
fn check(e: &Engine) {
    let p = &e.state.position;
    for resource in 0..5 {
        assert_eq!(
            u16::from(p.bank.counts()[resource])
                + p.hands
                    .iter()
                    .map(|h| u16::from(h.counts()[resource]))
                    .sum::<u16>(),
            19
        );
    }
    for (index, id) in p.deck.iter().enumerate() {
        assert_eq!(
            p.card_locations[id.index()] == CardLocation::Undrawn,
            index >= usize::from(p.draw_cursor)
        );
    }
    for &s in p.config.seat_order() {
        assert!(p.roads.iter().filter(|&&o| o == Some(s)).count() <= 15);
        assert_eq!(
            p.road_lengths[s.index()],
            crate::base_rules::longest_route_length(e.board.graph, &p.roads, &p.buildings, s)
        );
    }
    for (id, b) in p.buildings.iter().enumerate() {
        if b.is_some() {
            assert!(
                e.board
                    .graph
                    .vertex_neighbors(VertexId::try_from(id as u8).unwrap())
                    .iter()
                    .all(|v| p.buildings[v.index()].is_none())
            );
        }
    }
}
fn act(e: &mut Engine, command: Command) -> Receipt {
    assert!(e.validate(seat(0), &command).is_ok());
    let revision = e.state.revision;
    let r = e
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(0),
            expected_revision: revision,
            command,
        })
        .unwrap();
    assert_eq!((r.previous_revision, r.revision), (revision, revision + 1));
    check(e);
    r
}
fn reject(e: &mut Engine, actor: u8, command: Command, error: RuleError) {
    let p = e.state.position.clone();
    let rng = e.state.random.clone();
    let rev = e.state.revision;
    assert_eq!(e.validate(seat(actor), &command).unwrap_err().code, error);
    assert_eq!(
        e.execute(CommandEnvelope {
            api_version: 1,
            actor: seat(actor),
            expected_revision: rev,
            command
        })
        .unwrap_err()
        .code,
        error
    );
    assert_eq!(e.state.position, p);
    assert_eq!(e.state.random, rng);
    assert_eq!(e.state.revision, rev);
}
fn simple_board(e: &mut Engine, roads: &[u8], opponent_roads: &[u8], opponent_vertices: &[u8]) {
    let p = &mut e.state.position;
    p.roads = [None; 72];
    p.buildings = [None; 54];
    p.buildings[0] = Some(Building {
        owner: seat(0),
        kind: BuildingKind::Settlement,
    });
    for &v in opponent_vertices {
        p.buildings[v as usize] = Some(Building {
            owner: seat(1),
            kind: BuildingKind::Settlement,
        });
    }
    for &id in roads {
        p.roads[id as usize] = Some(seat(0));
    }
    for &id in opponent_roads {
        p.roads[id as usize] = Some(seat(1));
    }
    p.road_lengths = std::array::from_fn(|id| {
        crate::base_rules::longest_route_length(
            e.board.graph,
            &p.roads,
            &p.buildings,
            seat(id as u8),
        )
    });
    p.longest_route_holder = crate::base_rules::reconcile_longest_route(&p.road_lengths, None);
}
#[test]
fn free_roads_both_continuations_sequential_candidates_cost_and_allowance() {
    for (phase, resume) in [
        (Phase::PreRoll, ResumePhase::PreRoll),
        (Phase::Action, ResumePhase::Action),
    ] {
        let mut e = fixture();
        e.state.position.phase = phase;
        let hands = e.state.position.hands;
        let bank = e.state.position.bank;
        let rng = e.state.random.clone();
        assert!(e.legal_actions(seat(0)).unwrap().choices.contains(
            &LegalAction::PlayRoadBuilding {
                cards: vec![card(14)]
            }
        ));
        reject(&mut e, 0, place(45), RuleError::WrongPhase);
        let r = act(&mut e, play());
        let pending = Phase::FreeRoads {
            remaining: 2,
            continuation: resume,
        };
        assert_eq!(
            r.events,
            vec![
                Event::RoadBuildingPlayed {
                    seat: seat(0),
                    card: card(14)
                },
                Event::PhaseChanged { phase: pending }
            ]
        );
        assert_eq!(e.state.position.phase, pending);
        assert_eq!(
            e.state.position.card_locations[14],
            CardLocation::Spent { owner: seat(0) }
        );
        for viewer in [
            Viewer::Spectator,
            Viewer::Seat(seat(0)),
            Viewer::Seat(seat(1)),
            Viewer::Seat(seat(2)),
        ] {
            assert_eq!(
                e.project_receipt(&r, viewer).unwrap().events,
                vec![
                    ProjectedEvent::RoadBuildingPlayed {
                        seat: seat(0),
                        card: card(14)
                    },
                    ProjectedEvent::PhaseChanged { phase: pending }
                ]
            );
            let v = e.view(viewer).unwrap();
            assert_eq!(v.phase, pending);
            assert_eq!(
                v.players[0].spent_progress,
                vec![DevelopmentKind::RoadBuilding]
            );
            assert_eq!(v.players[0].played_knights, 0);
        }
        reject(&mut e, 0, place(44), RuleError::Disconnected);
        reject(&mut e, 0, place(46), RuleError::Occupied);
        reject(&mut e, 1, place(45), RuleError::WrongActor);
        for command in [
            Command::RollDice,
            Command::EndTurn,
            Command::BuyDevelopmentCard,
            Command::BuildRoad { edge: edge(45) },
            Command::PlayInvention {
                card: card(18),
                resources: Resources::try_from([2, 0, 0, 0, 0]).unwrap(),
            },
            play(),
        ] {
            reject(&mut e, 0, command, RuleError::WrongPhase);
        }
        act(&mut e, place(45));
        assert_eq!(
            e.state.position.phase,
            Phase::FreeRoads {
                remaining: 1,
                continuation: resume
            }
        );
        let choices = e.legal_actions(seat(0)).unwrap().choices;
        let [LegalAction::PlaceFreeRoad { edges }] = choices.as_slice() else {
            panic!("only mandatory free placement")
        };
        assert!(edges.contains(&edge(44)));
        for id in 0..72 {
            assert_eq!(
                edges.contains(&edge(id)),
                e.validate(seat(0), &place(id)).is_ok()
            );
        }
        act(&mut e, place(44));
        assert_eq!(e.state.position.phase, phase);
        assert_eq!(e.state.position.road_lengths, [3, 1, 1, 0]);
        assert_eq!(e.state.position.hands, hands);
        assert_eq!(e.state.position.bank, bank);
        assert_eq!(e.state.random, rng);
        reject(
            &mut e,
            0,
            Command::PlayInvention {
                card: card(18),
                resources: Resources::try_from([2, 0, 0, 0, 0]).unwrap(),
            },
            RuleError::CardNotPlayable,
        );
        reject(&mut e, 0, play(), RuleError::CardNotPlayable);
    }
}
#[test]
fn base_rules_br030_activation_no_stock_or_sites_and_one_site_without_lookahead() {
    let mut e = fixture();
    simple_board(&mut e, &[0, 6, 10], &[2, 7, 11, 19], &[1, 11, 17]);
    check(&e);
    assert_eq!(
        e.legal_actions(seat(0))
            .unwrap()
            .choices
            .iter()
            .filter(|a| matches!(a, LegalAction::PlayRoadBuilding { .. }))
            .count(),
        1
    );
    act(&mut e, play());
    assert_eq!(
        e.legal_actions(seat(0)).unwrap().choices,
        vec![LegalAction::PlaceFreeRoad {
            edges: vec![edge(1)]
        }]
    );
    let rng = e.state.random.clone();
    act(&mut e, place(1));
    assert_eq!(e.state.position.phase, Phase::PreRoll);
    assert_eq!(e.state.random, rng);
    let mut e = fixture();
    simple_board(&mut e, &[0, 1, 6, 10], &[2, 7, 11, 19], &[1, 11, 17]);
    reject(&mut e, 0, play(), RuleError::Disconnected);
    assert!(
        !e.legal_actions(seat(0))
            .unwrap()
            .choices
            .iter()
            .any(|a| matches!(a, LegalAction::PlayRoadBuilding { .. }))
    );
    let mut e = fixture();
    simple_board(
        &mut e,
        &[0, 1, 2, 3, 4, 5, 9, 17, 22, 32, 38, 48, 53, 61, 65],
        &[],
        &[],
    );
    reject(&mut e, 0, play(), RuleError::NoPieces);
}
#[test]
fn free_roads_last_piece_ends_automatically() {
    let mut e = fixture();
    simple_board(
        &mut e,
        &[1, 2, 3, 4, 5, 9, 17, 22, 32, 38, 48, 53, 61, 65],
        &[],
        &[],
    );
    let hands = e.state.position.hands;
    act(&mut e, play());
    act(&mut e, place(0));
    assert_eq!(e.state.position.phase, Phase::PreRoll);
    assert_eq!(
        e.view(Viewer::Spectator).unwrap().players[0].roads_remaining,
        0
    );
    assert_eq!(e.state.position.hands, hands);
}
#[test]
fn free_roads_timing_foreign_kind_phase_stale_and_counter_rejections() {
    let mut e = fixture();
    for id in [0, 15, 18, 20] {
        reject(
            &mut e,
            0,
            Command::PlayRoadBuilding { card: card(id) },
            RuleError::CardNotPlayable,
        );
    }
    reject(&mut e, 1, play(), RuleError::WrongActor);
    for phase in [
        Phase::SetupSettlement {
            actor: seat(0),
            round: 1,
        },
        Phase::Discard {
            obligations: [1, 0, 0, 0],
        },
        Phase::RobberMove {
            origin: crate::RobberOrigin::Knight,
            continuation: ResumePhase::PreRoll,
        },
    ] {
        e.state.position.phase = phase;
        reject(&mut e, 0, play(), RuleError::WrongPhase);
    }
    e.state.position.phase = Phase::PreRoll;
    e.state.revision = u64::MAX;
    reject(&mut e, 0, play(), RuleError::CounterExhausted);
    e.state.revision = 1;
    let p = e.state.position.clone();
    let rng = e.state.random.clone();
    assert_eq!(
        e.execute(CommandEnvelope {
            api_version: 1,
            actor: seat(0),
            expected_revision: 0,
            command: play()
        })
        .unwrap_err()
        .code,
        RuleError::StaleRevision
    );
    assert_eq!(e.state.position, p);
    assert_eq!(e.state.random, rng);
    e.state.revision = 0;
    act(&mut e, play());
    e.state.revision = u64::MAX;
    reject(&mut e, 0, place(45), RuleError::CounterExhausted);
}
#[test]
fn base_rules_br038_first_free_road_victory_stops_before_second_or_roll() {
    let mut e = fixture();
    let p = &mut e.state.position;
    p.buildings = [None; 54];
    p.roads = [None; 72];
    for (v, kind) in [
        (0, BuildingKind::City),
        (9, BuildingKind::City),
        (17, BuildingKind::City),
        (31, BuildingKind::Settlement),
        (40, BuildingKind::Settlement),
    ] {
        p.buildings[v] = Some(Building {
            owner: seat(0),
            kind,
        });
    }
    for id in [0, 6, 10, 18, 14, 25, 46, 57] {
        p.roads[id] = Some(seat(0));
    }
    p.road_lengths = [4, 0, 0, 0];
    check(&e);
    let rng = e.state.random.clone();
    let hands = e.state.position.hands;
    act(&mut e, play());
    let r = act(&mut e, place(23));
    assert_eq!(
        r.events,
        vec![
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
        e.state.position.phase,
        Phase::Finished {
            winner: seat(0),
            score: 10
        }
    );
    assert_eq!(r.final_scores, Some([10, 1, 0, 0]));
    assert_eq!(e.state.position.hands, hands);
    assert_eq!(e.state.random, rng);
    for command in [place(22), Command::RollDice, Command::EndTurn, play()] {
        reject(&mut e, 0, command, RuleError::GameFinished);
    }
    assert!(e.legal_actions(seat(0)).unwrap().choices.is_empty());
}

#[test]
fn base_rules_br030_free_roads_can_use_two_distinct_owned_networks() {
    let mut e = fixture();
    simple_board(&mut e, &[0, 46], &[], &[]);
    e.state.position.buildings[31] = Some(Building {
        owner: seat(0),
        kind: BuildingKind::Settlement,
    });
    // Independent endpoints: E0 [0,3], E1 [0,4]; E46 [31,36], E45 [30,36].
    assert_eq!(
        e.board.graph.edge_vertices(edge(1)).map(VertexId::index),
        [0, 4]
    );
    assert_eq!(
        e.board.graph.edge_vertices(edge(45)).map(VertexId::index),
        [30, 36]
    );
    let hands = e.state.position.hands;
    let bank = e.state.position.bank;
    let random = e.state.random.clone();
    act(&mut e, play());
    assert!(e.validate(seat(0), &place(1)).is_ok());
    assert!(e.validate(seat(0), &place(45)).is_ok());
    act(&mut e, place(1));
    assert!(e.validate(seat(0), &place(45)).is_ok());
    act(&mut e, place(45));
    for id in [0, 1, 45, 46] {
        assert_eq!(e.state.position.roads[id], Some(seat(0)));
    }
    assert_eq!(
        e.state
            .position
            .roads
            .iter()
            .filter(|o| o.is_some())
            .count(),
        4
    );
    assert_eq!(e.state.position.road_lengths, [2, 0, 0, 0]);
    assert_eq!(e.state.position.phase, Phase::PreRoll);
    assert_eq!(e.state.position.hands, hands);
    assert_eq!(e.state.position.bank, bank);
    assert_eq!(e.state.random, random);
}
