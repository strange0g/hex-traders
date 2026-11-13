//! Source fixed-board production and independent pinned RNG/endpoint literals.
use super::{edge, seat, vertex};
use hex_traders_engine::{
    Command, CommandEnvelope, Engine, Event, GameConfig, LegalAction, Phase, ProjectedEvent,
    Receipt, Resources, RuleError, Viewer,
};
fn game(seed_value: u32) -> Engine {
    let mut seed = [0; 32];
    seed[..4].copy_from_slice(&seed_value.to_le_bytes());
    Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), seed)
        .unwrap()
        .0
}
fn act(engine: &mut Engine, actor: u8, command: Command) -> Receipt {
    let revision = engine.view(Viewer::Spectator).unwrap().revision;
    engine
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(actor),
            expected_revision: revision,
            command,
        })
        .unwrap()
}
fn candidates(engine: &Engine, actor: u8, expected: &[u8]) {
    let actions = engine.legal_actions(seat(actor)).unwrap();
    let listed = actions
        .choices
        .iter()
        .find_map(|action| match action {
            LegalAction::BuildSettlement { vertices } => Some(vertices.clone()),
            _ => None,
        })
        .unwrap_or_default();
    assert_eq!(
        listed,
        expected.iter().map(|&id| vertex(id)).collect::<Vec<_>>()
    );
    for id in 0..54 {
        assert_eq!(
            engine
                .validate(
                    seat(actor),
                    &Command::BuildSettlement { vertex: vertex(id) }
                )
                .is_ok(),
            expected.contains(&id)
        );
    }
}
#[test]
fn settlement_public_coastal_port_payment_candidates_and_old_receipt() {
    // Independent pinned rand/chacha seed29573; source six rolls fund two roads + settlement.
    let mut engine = game(29573);
    for (index, (actor, dice)) in [
        (1, [5, 4]),
        (2, [5, 6]),
        (0, [4, 4]),
        (1, [6, 3]),
        (2, [4, 1]),
        (0, [4, 4]),
    ]
    .into_iter()
    .enumerate()
    {
        act(&mut engine, actor, Command::RollDice);
        assert_eq!(
            engine.view(Viewer::Spectator).unwrap().last_dice,
            Some(dice)
        );
        if index < 5 {
            act(&mut engine, actor, Command::EndTurn);
        }
    }
    assert_eq!(
        engine.view(Viewer::Spectator).unwrap().bank.counts(),
        &[13, 14, 15, 13, 16]
    );
    assert_eq!(
        engine
            .view(Viewer::Seat(seat(0)))
            .unwrap()
            .own_player
            .unwrap()
            .resources
            .counts(),
        &[3, 3, 1, 1, 1]
    );
    candidates(&engine, 0, &[]);
    act(&mut engine, 0, Command::BuildRoad { edge: edge(47) });
    candidates(&engine, 0, &[]);
    act(&mut engine, 0, Command::BuildRoad { edge: edge(53) });
    candidates(&engine, 0, &[42]);
    // V31-E47-V37-E53-V42; port E61 has endpoints V42/V46.
    let receipt = act(
        &mut engine,
        0,
        Command::BuildSettlement { vertex: vertex(42) },
    );
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
                vertex: vertex(42)
            }
        ]
    );
    let public = engine.view(Viewer::Spectator).unwrap();
    assert_eq!(public.board.buildings[42].unwrap().owner, seat(0));
    assert_eq!(public.players[0].settlements_remaining, 2);
    assert_eq!(public.players[0].public_score, 3);
    assert_eq!(public.bank.counts(), &[16, 17, 16, 14, 16]);
    assert_eq!(
        engine
            .view(Viewer::Seat(seat(0)))
            .unwrap()
            .own_player
            .unwrap()
            .resources
            .counts(),
        &[0, 0, 0, 0, 1]
    );
    let before = format!("{:?}", public);
    let revision = public.revision;
    assert_eq!(
        engine
            .execute(CommandEnvelope {
                api_version: 1,
                actor: seat(0),
                expected_revision: revision,
                command: Command::BuildSettlement { vertex: vertex(42) }
            })
            .unwrap_err()
            .code,
        RuleError::Occupied
    );
    assert_eq!(
        format!("{:?}", engine.view(Viewer::Spectator).unwrap()),
        before
    );
    act(&mut engine, 0, Command::EndTurn);
    act(&mut engine, 1, Command::RollDice);
    assert_eq!(
        engine.view(Viewer::Spectator).unwrap().last_dice,
        Some([3, 6])
    );
    assert_eq!(
        engine.view(Viewer::Spectator).unwrap().bank.counts(),
        &[14, 17, 16, 14, 15]
    );
    for viewer in [
        Viewer::Spectator,
        Viewer::Seat(seat(0)),
        Viewer::Seat(seat(1)),
        Viewer::Seat(seat(2)),
    ] {
        let projected = engine.project_receipt(&receipt, viewer).unwrap();
        assert_eq!(projected.revision, revision);
        assert_eq!(
            projected.events,
            vec![
                ProjectedEvent::ConstructionPaid {
                    seat: seat(0),
                    resources: cost
                },
                ProjectedEvent::SettlementPlaced {
                    seat: seat(0),
                    vertex: vertex(42)
                }
            ]
        );
    }
}
#[test]
fn settlement_public_blocker_returns_opponents_five_edge_award_atomically() {
    // Independent seed36673; no seven in the first eight rolls. Source claims fund both actors.
    let mut engine = game(36673);
    for (index, (actor, dice)) in [
        (1, [1, 3]),
        (2, [5, 6]),
        (0, [5, 6]),
        (1, [5, 3]),
        (2, [5, 4]),
        (0, [6, 3]),
        (1, [1, 5]),
        (2, [3, 3]),
    ]
    .into_iter()
    .enumerate()
    {
        act(&mut engine, actor, Command::RollDice);
        assert_eq!(
            engine.view(Viewer::Spectator).unwrap().last_dice,
            Some(dice)
        );
        if index == 5 {
            for id in [45, 44, 51] {
                act(&mut engine, 0, Command::BuildRoad { edge: edge(id) });
            }
            assert_eq!(
                engine.view(Viewer::Spectator).unwrap().players[0].longest_route_length,
                5
            );
            assert_eq!(
                engine.view(Viewer::Spectator).unwrap().longest_route_holder,
                Some(seat(0))
            );
        }
        if index < 7 {
            act(&mut engine, actor, Command::EndTurn);
        }
    }
    assert_eq!(
        engine.view(Viewer::Spectator).unwrap().bank.counts(),
        &[14, 14, 15, 15, 13]
    );
    assert_eq!(
        engine
            .view(Viewer::Seat(seat(2)))
            .unwrap()
            .own_player
            .unwrap()
            .resources
            .counts(),
        &[3, 3, 1, 1, 2]
    );
    act(&mut engine, 2, Command::BuildRoad { edge: edge(29) });
    act(&mut engine, 2, Command::BuildRoad { edge: edge(36) });
    candidates(&engine, 2, &[30]);
    // V19-E29-V24-E36-V30, meeting Blue E45/E44; no adjacent preset building.
    let roads = engine.view(Viewer::Spectator).unwrap().board.roads;
    let receipt = act(
        &mut engine,
        2,
        Command::BuildSettlement { vertex: vertex(30) },
    );
    let cost = Resources::try_from([1, 1, 1, 1, 0]).unwrap();
    assert_eq!(
        receipt.events,
        vec![
            Event::ConstructionPaid {
                seat: seat(2),
                resources: cost
            },
            Event::SettlementPlaced {
                seat: seat(2),
                vertex: vertex(30)
            },
            Event::LongestRouteChanged {
                previous: Some(seat(0)),
                holder: None
            }
        ]
    );
    let view = engine.view(Viewer::Spectator).unwrap();
    assert_eq!(view.board.roads, roads);
    assert_eq!(
        view.players
            .iter()
            .map(|p| p.longest_route_length)
            .collect::<Vec<_>>(),
        [3, 1, 3]
    );
    assert_eq!(
        view.players
            .iter()
            .map(|p| p.public_score)
            .collect::<Vec<_>>(),
        [2, 2, 3]
    );
    assert_eq!(view.bank.counts(), &[17, 17, 16, 16, 13]);
    assert_eq!(view.phase, Phase::Action);
    assert_eq!(view.longest_route_holder, None);
    assert_eq!(
        engine
            .view(Viewer::Seat(seat(2)))
            .unwrap()
            .own_player
            .unwrap()
            .resources
            .counts(),
        &[0, 0, 0, 0, 2]
    );
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
                    seat: seat(2),
                    resources: cost
                },
                ProjectedEvent::SettlementPlaced {
                    seat: seat(2),
                    vertex: vertex(30)
                },
                ProjectedEvent::LongestRouteChanged {
                    previous: Some(seat(0)),
                    holder: None
                }
            ]
        );
    }
    act(&mut engine, 2, Command::EndTurn);
    act(&mut engine, 0, Command::RollDice);
    assert_eq!(
        engine.view(Viewer::Spectator).unwrap().last_dice,
        Some([6, 1])
    );
}
