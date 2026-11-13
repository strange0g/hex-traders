//! Independent fixed source incidence and standalone pinned rand/chacha sequence.
use super::{seat, vertex};
use hex_traders_engine::{
    BuildingKind, Command, CommandEnvelope, Engine, Event, GameConfig, LegalAction, Phase,
    ProjectedEvent, Resources, Viewer,
};
fn act(engine: &mut Engine, actor: u8, command: Command) -> hex_traders_engine::Receipt {
    let expected_revision = engine.view(Viewer::Spectator).unwrap().revision;
    engine
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(actor),
            expected_revision,
            command,
        })
        .unwrap()
}
#[test]
fn city_public_upgrade_returns_stock_and_next_roll_produces_two() {
    // Oracle seed8433: Blue V40 at H14/mountain4 produces ore, H17/field3 wheat.
    let mut seed = [0; 32];
    seed[..4].copy_from_slice(&8433u32.to_le_bytes());
    let (mut engine, _) = Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), seed).unwrap();
    for (index, (actor, dice)) in [(0, [2, 2]), (1, [3, 1]), (2, [2, 1]), (0, [2, 4])]
        .into_iter()
        .enumerate()
    {
        act(&mut engine, actor, Command::RollDice);
        assert_eq!(
            engine.view(Viewer::Spectator).unwrap().last_dice,
            Some(dice)
        );
        if index < 3 {
            act(&mut engine, actor, Command::EndTurn);
        }
    }
    let before = engine.view(Viewer::Spectator).unwrap();
    assert_eq!(
        (before.turn, before.revision, before.phase),
        (4, 7, Phase::Action)
    );
    assert_eq!(before.bank.counts(), &[16, 14, 16, 15, 15]);
    assert_eq!(
        engine
            .view(Viewer::Seat(seat(0)))
            .unwrap()
            .own_player
            .unwrap()
            .resources
            .counts(),
        &[1, 0, 0, 2, 3]
    );
    let candidates = engine.legal_actions(seat(0)).unwrap();
    assert!(candidates.choices.contains(&LegalAction::UpgradeCity {
        vertices: vec![vertex(31), vertex(40)]
    }));
    for id in 0..54 {
        assert_eq!(
            engine
                .validate(seat(0), &Command::UpgradeCity { vertex: vertex(id) })
                .is_ok(),
            [31, 40].contains(&id)
        );
    }
    let receipt = act(&mut engine, 0, Command::UpgradeCity { vertex: vertex(40) });
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
    let after = engine.view(Viewer::Spectator).unwrap();
    assert_eq!(after.board.buildings[40].unwrap().kind, BuildingKind::City);
    assert_eq!(after.board.roads, before.board.roads);
    assert_eq!(after.longest_route_holder, before.longest_route_holder);
    assert_eq!(after.bank.counts(), &[16, 14, 16, 17, 18]);
    assert_eq!(
        (
            after.players[0].settlements_remaining,
            after.players[0].cities_remaining,
            after.players[0].public_score
        ),
        (4, 3, 3)
    );
    assert_eq!(
        after.players[0].longest_route_length,
        before.players[0].longest_route_length
    );
    assert_eq!(
        engine
            .view(Viewer::Seat(seat(0)))
            .unwrap()
            .own_player
            .unwrap()
            .resources
            .counts(),
        &[1, 0, 0, 0, 0]
    );
    act(&mut engine, 0, Command::EndTurn);
    let production = act(&mut engine, 1, Command::RollDice);
    assert_eq!(
        engine.view(Viewer::Spectator).unwrap().last_dice,
        Some([2, 2])
    );
    assert_eq!(
        production.events,
        vec![
            Event::DiceRolled {
                seat: seat(1),
                dice: [2, 2]
            },
            Event::Production {
                seat: seat(0),
                resources: Resources::try_from([0, 0, 0, 0, 2]).unwrap()
            },
            Event::Production {
                seat: seat(1),
                resources: Resources::try_from([0, 1, 0, 0, 0]).unwrap()
            },
            Event::Production {
                seat: seat(2),
                resources: Resources::try_from([0, 1, 0, 0, 0]).unwrap()
            },
            Event::PhaseChanged {
                phase: Phase::Action
            }
        ]
    );
    assert_eq!(
        engine.view(Viewer::Spectator).unwrap().bank.counts(),
        &[16, 12, 16, 17, 16]
    );
    assert_eq!(
        engine
            .view(Viewer::Seat(seat(0)))
            .unwrap()
            .own_player
            .unwrap()
            .resources
            .counts(),
        &[1, 0, 0, 0, 2]
    );
    for viewer in [
        Viewer::Spectator,
        Viewer::Seat(seat(0)),
        Viewer::Seat(seat(1)),
        Viewer::Seat(seat(2)),
    ] {
        let projected = engine.project_receipt(&receipt, viewer).unwrap();
        assert_eq!(projected.revision, 8);
        assert_eq!(
            projected.events,
            vec![
                ProjectedEvent::ConstructionPaid {
                    seat: seat(0),
                    resources: cost
                },
                ProjectedEvent::CityUpgraded {
                    seat: seat(0),
                    vertex: vertex(40)
                }
            ]
        );
    }
}
