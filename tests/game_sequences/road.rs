//! Public paid-road proof from the captured fixed board and standalone RNG oracle.
use super::{edge, seat};
use hex_traders_engine::{
    Command, CommandEnvelope, Engine, Event, GameConfig, LegalAction, Phase, ProjectedEvent,
    Resources, RuleError, Viewer,
};

#[test]
fn road_public_payments_candidates_and_fifth_edge_award_are_atomic() {
    // Independent rand/chacha oracle, seed15508 encoded as u32 little-endian.
    // Captured source: Blue V31 earns wood on 8/11; V40 brick on 9.
    // Three eights + two nines fund three roads before any seven.
    let mut seed = [0; 32];
    seed[..4].copy_from_slice(&15508u32.to_le_bytes());
    let (mut engine, _) = Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), seed).unwrap();
    assert_eq!(
        engine.view(Viewer::Spectator).unwrap().starting_player,
        seat(2)
    );
    for (turn, (id, dice)) in [
        (2, [6, 2]),
        (0, [3, 5]),
        (1, [6, 3]),
        (2, [4, 5]),
        (0, [6, 2]),
    ]
    .into_iter()
    .enumerate()
    {
        engine
            .execute(CommandEnvelope {
                api_version: 1,
                actor: seat(id),
                expected_revision: (turn * 2) as u64,
                command: Command::RollDice,
            })
            .unwrap();
        assert_eq!(
            engine.view(Viewer::Spectator).unwrap().last_dice,
            Some(dice)
        );
        if turn < 4 {
            engine
                .execute(CommandEnvelope {
                    api_version: 1,
                    actor: seat(id),
                    expected_revision: (turn * 2 + 1) as u64,
                    command: Command::EndTurn,
                })
                .unwrap();
        }
    }
    let viewers = [
        Viewer::Spectator,
        Viewer::Seat(seat(0)),
        Viewer::Seat(seat(1)),
        Viewer::Seat(seat(2)),
    ];
    let public = engine.view(Viewer::Spectator).unwrap();
    assert_eq!(
        (public.revision, public.turn, public.phase),
        (9, 5, Phase::Action)
    );
    assert_eq!(public.bank.counts(), &[13, 15, 17, 13, 16]);
    assert_eq!(public.longest_route_holder, None);
    let initial_roads = public.board.roads;
    let initial_buildings = public.board.buildings;
    for (id, hand) in [[3, 3, 0, 1, 1], [2, 1, 1, 4, 0], [1, 0, 1, 1, 2]]
        .into_iter()
        .enumerate()
    {
        assert_eq!(
            engine
                .view(Viewer::Seat(seat(id as u8)))
                .unwrap()
                .own_player
                .unwrap()
                .resources
                .counts(),
            &hand
        );
    }
    // Independent endpoint enumeration from the frozen 72-edge fixture.
    let candidate_ids = [
        vec![37, 45, 47, 51, 52, 56, 58, 63],
        vec![36, 37, 44, 47, 51, 52, 56, 58, 63],
        vec![36, 37, 43, 47, 51, 52, 56, 58, 63],
    ];
    let mut roads = initial_roads;
    for (index, id) in [45, 44, 51].into_iter().enumerate() {
        let candidates: Vec<_> = candidate_ids[index].iter().map(|&id| edge(id)).collect();
        assert_eq!(
            engine.legal_actions(seat(0)).unwrap().choices,
            vec![
                LegalAction::ProposeTrade {
                    recipients: vec![seat(1), seat(2)]
                },
                LegalAction::EndTurn,
                LegalAction::BuildRoad { edges: candidates }
            ]
        );
        for id in 0..72 {
            assert_eq!(
                engine
                    .validate(seat(0), &Command::BuildRoad { edge: edge(id) })
                    .is_ok(),
                candidate_ids[index].contains(&id)
            );
        }
        let before: Vec<_> = viewers
            .iter()
            .map(|&viewer| format!("{:?}", engine.view(viewer).unwrap()))
            .collect();
        assert_eq!(
            engine
                .execute(CommandEnvelope {
                    api_version: 1,
                    actor: seat(0),
                    expected_revision: 9 + index as u64,
                    command: Command::BuildRoad { edge: edge(46) },
                })
                .unwrap_err()
                .code,
            RuleError::Occupied
        );
        assert_eq!(
            viewers
                .iter()
                .map(|&viewer| format!("{:?}", engine.view(viewer).unwrap()))
                .collect::<Vec<_>>(),
            before
        );
        let receipt = engine
            .execute(CommandEnvelope {
                api_version: 1,
                actor: seat(0),
                expected_revision: 9 + index as u64,
                command: Command::BuildRoad { edge: edge(id) },
            })
            .unwrap();
        let cost = Resources::try_from([1, 1, 0, 0, 0]).unwrap();
        let mut events = vec![
            Event::ConstructionPaid {
                seat: seat(0),
                resources: cost,
            },
            Event::RoadPlaced {
                seat: seat(0),
                edge: edge(id),
            },
        ];
        let mut projected = vec![
            ProjectedEvent::ConstructionPaid {
                seat: seat(0),
                resources: cost,
            },
            ProjectedEvent::RoadPlaced {
                seat: seat(0),
                edge: edge(id),
            },
        ];
        if index == 2 {
            events.push(Event::LongestRouteChanged {
                previous: None,
                holder: Some(seat(0)),
            });
            projected.push(ProjectedEvent::LongestRouteChanged {
                previous: None,
                holder: Some(seat(0)),
            });
        }
        assert_eq!(receipt.events, events);
        assert_eq!(receipt.revision, 10 + index as u64);
        roads[id as usize] = Some(seat(0));
        for viewer in viewers {
            assert_eq!(
                engine.project_receipt(&receipt, viewer).unwrap().events,
                projected
            );
            let view = engine.view(viewer).unwrap();
            assert_eq!(view.board.roads, roads);
            assert_eq!(view.board.buildings, initial_buildings);
            assert_eq!(
                (view.phase, view.revision),
                (Phase::Action, 10 + index as u64)
            );
            assert_eq!(
                view.bank.counts(),
                &[14 + index as u8, 16 + index as u8, 17, 13, 16]
            );
            assert_eq!(
                view.longest_route_holder,
                if index == 2 { Some(seat(0)) } else { None }
            );
            let blue = &view.players[0];
            assert_eq!(
                (
                    blue.roads_remaining,
                    blue.longest_route_length,
                    blue.public_score
                ),
                [(12, 2, 2), (11, 3, 2), (10, 5, 4)][index]
            );
        }
        let own = engine
            .view(Viewer::Seat(seat(0)))
            .unwrap()
            .own_player
            .unwrap();
        assert_eq!(
            own.resources.counts(),
            &[2 - index as u8, 2 - index as u8, 0, 1, 1]
        );
        assert_eq!(own.true_score, if index == 2 { 4 } else { 2 });
    }
    // Frozen witness: V31-E46-V36-E45-V30-E44-V35-E51-V40-E57-V44.
    // All five owned edges form a trail, so five is also the exact upper bound.
    assert_eq!(
        engine.legal_actions(seat(0)).unwrap().choices,
        vec![
            LegalAction::ProposeTrade {
                recipients: vec![seat(1), seat(2)]
            },
            LegalAction::EndTurn
        ]
    );
    engine
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(0),
            expected_revision: 12,
            command: Command::EndTurn,
        })
        .unwrap();
    engine
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(1),
            expected_revision: 13,
            command: Command::RollDice,
        })
        .unwrap();
    let public = engine.view(Viewer::Spectator).unwrap();
    assert_eq!(public.last_dice, Some([1, 4]));
    assert_eq!(public.longest_route_holder, Some(seat(0)));
    assert_eq!(public.players[0].longest_route_length, 5);
    assert_eq!(public.board.roads, roads);
    assert_eq!(public.bank.counts(), &[16, 18, 16, 12, 16]);
}
