//! Public 4:1/3:1/2:1 histories; fixed-source claims and standalone RNG literals.
use super::{edge, seat, vertex};
use hex_traders_engine::{
    Command, CommandEnvelope, Engine, Event, GameConfig, LegalAction, ProjectedEvent, Receipt,
    Resource, RuleError, Viewer,
};
const KINDS: [Resource; 5] = [
    Resource::Brick,
    Resource::Wood,
    Resource::Wool,
    Resource::Wheat,
    Resource::Ore,
];
fn game(seed_value: u32) -> Engine {
    let mut seed = [0; 32];
    seed[..4].copy_from_slice(&seed_value.to_le_bytes());
    Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), seed)
        .unwrap()
        .0
}
fn act(engine: &mut Engine, actor: u8, command: Command) -> Receipt {
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
fn pairs(engine: &Engine, actor: u8, give: Resource) {
    let expected = KINDS
        .into_iter()
        .filter(|&receive| receive != give)
        .map(|receive| (give, receive))
        .collect::<Vec<_>>();
    assert!(
        engine
            .legal_actions(seat(actor))
            .unwrap()
            .choices
            .contains(&LegalAction::BankTrade {
                pairs: expected.clone()
            })
    );
    for offered in KINDS {
        for receive in KINDS {
            assert_eq!(
                engine
                    .validate(
                        seat(actor),
                        &Command::BankTrade {
                            give_resource: offered,
                            receive_resource: receive
                        }
                    )
                    .is_ok(),
                offered == give && receive != give
            );
        }
    }
}
fn projections(engine: &Engine, receipt: &Receipt, give: Resource, receive: Resource, rate: u8) {
    for viewer in [
        Viewer::Spectator,
        Viewer::Seat(seat(0)),
        Viewer::Seat(seat(1)),
        Viewer::Seat(seat(2)),
    ] {
        assert_eq!(
            engine.project_receipt(receipt, viewer).unwrap().events,
            vec![ProjectedEvent::BankTraded {
                seat: receipt.actor,
                give_resource: give,
                receive_resource: receive,
                rate
            }]
        );
        assert_eq!(
            engine.project_receipt(receipt, viewer).unwrap().revision,
            receipt.revision
        );
    }
}
fn funded_blue() -> Engine {
    // Seed50456 source claims: four nines, three wood rolls and one five fund the path.
    let mut engine = game(50456);
    for (index, (actor, dice)) in [
        (2, [4, 1]),
        (0, [6, 5]),
        (1, [2, 6]),
        (2, [5, 3]),
        (0, [6, 3]),
        (1, [4, 5]),
        (2, [4, 5]),
        (0, [6, 3]),
    ]
    .into_iter()
    .enumerate()
    {
        act(&mut engine, actor, Command::RollDice);
        assert_eq!(
            engine.view(Viewer::Spectator).unwrap().last_dice,
            Some(dice)
        );
        if index < 7 {
            act(&mut engine, actor, Command::EndTurn);
        }
    }
    assert_eq!(
        engine.view(Viewer::Spectator).unwrap().bank.counts(),
        &[9, 14, 15, 13, 14]
    );
    assert_eq!(
        engine
            .view(Viewer::Seat(seat(0)))
            .unwrap()
            .own_player
            .unwrap()
            .resources
            .counts(),
        &[5, 3, 1, 1, 1]
    );
    engine
}
#[test]
fn bank_trade_public_default_four_for_one_preserves_next_dice() {
    let mut engine = funded_blue();
    pairs(&engine, 0, Resource::Brick);
    let board = engine.view(Viewer::Spectator).unwrap().board;
    let receipt = act(
        &mut engine,
        0,
        Command::BankTrade {
            give_resource: Resource::Brick,
            receive_resource: Resource::Ore,
        },
    );
    assert_eq!(
        receipt.events,
        vec![Event::BankTraded {
            seat: seat(0),
            give_resource: Resource::Brick,
            receive_resource: Resource::Ore,
            rate: 4
        }]
    );
    assert_eq!(
        engine.view(Viewer::Spectator).unwrap().bank.counts(),
        &[13, 14, 15, 13, 13]
    );
    assert_eq!(
        engine
            .view(Viewer::Seat(seat(0)))
            .unwrap()
            .own_player
            .unwrap()
            .resources
            .counts(),
        &[1, 3, 1, 1, 2]
    );
    assert_eq!(
        engine.view(Viewer::Spectator).unwrap().board.roads,
        board.roads
    );
    assert_eq!(
        engine.view(Viewer::Spectator).unwrap().board.buildings,
        board.buildings
    );
    act(&mut engine, 0, Command::EndTurn);
    act(&mut engine, 1, Command::RollDice);
    assert_eq!(
        engine.view(Viewer::Spectator).unwrap().last_dice,
        Some([2, 6])
    );
    assert_eq!(
        engine.view(Viewer::Spectator).unwrap().bank.counts(),
        &[13, 13, 15, 12, 13]
    );
    projections(&engine, &receipt, Resource::Brick, Resource::Ore, 4);
}
#[test]
fn bank_trade_public_new_special_port_uses_two_for_one_in_the_same_phase() {
    let mut engine = funded_blue();
    for id in [47, 53] {
        act(&mut engine, 0, Command::BuildRoad { edge: edge(id) });
    }
    let revision = engine.view(Viewer::Spectator).unwrap().revision;
    let before = format!("{:?}", engine.view(Viewer::Spectator).unwrap());
    assert_eq!(
        engine
            .execute(CommandEnvelope {
                api_version: 1,
                actor: seat(0),
                expected_revision: revision,
                command: Command::BankTrade {
                    give_resource: Resource::Brick,
                    receive_resource: Resource::Ore
                }
            })
            .unwrap_err()
            .code,
        RuleError::InsufficientResources
    );
    assert_eq!(
        format!("{:?}", engine.view(Viewer::Spectator).unwrap()),
        before
    );
    // Brick port E61 endpoint V42 is two road steps from Blue V31.
    act(
        &mut engine,
        0,
        Command::BuildSettlement { vertex: vertex(42) },
    );
    pairs(&engine, 0, Resource::Brick);
    let receipt = act(
        &mut engine,
        0,
        Command::BankTrade {
            give_resource: Resource::Brick,
            receive_resource: Resource::Ore,
        },
    );
    assert_eq!(
        receipt.events,
        vec![Event::BankTraded {
            seat: seat(0),
            give_resource: Resource::Brick,
            receive_resource: Resource::Ore,
            rate: 2
        }]
    );
    assert_eq!(
        engine.view(Viewer::Spectator).unwrap().bank.counts(),
        &[14, 17, 16, 14, 13]
    );
    assert_eq!(
        engine
            .view(Viewer::Seat(seat(0)))
            .unwrap()
            .own_player
            .unwrap()
            .resources
            .counts(),
        &[0, 0, 0, 0, 2]
    );
    act(&mut engine, 0, Command::EndTurn);
    act(&mut engine, 1, Command::RollDice);
    assert_eq!(
        engine.view(Viewer::Spectator).unwrap().last_dice,
        Some([2, 6])
    );
    assert_eq!(
        engine.view(Viewer::Spectator).unwrap().bank.counts(),
        &[14, 15, 16, 13, 13]
    );
    projections(&engine, &receipt, Resource::Brick, Resource::Ore, 2);
}
#[test]
fn bank_trade_public_new_generic_port_uses_three_for_one() {
    let mut engine = game(27147);
    let dice = [
        [3, 1],
        [3, 6],
        [6, 4],
        [5, 6],
        [1, 3],
        [2, 2],
        [6, 5],
        [5, 4],
        [3, 5],
        [5, 5],
        [4, 4],
        [3, 6],
        [5, 4],
        [3, 1],
        [2, 6],
        [4, 5],
        [2, 1],
        [1, 1],
    ];
    for (index, dice) in dice.into_iter().enumerate() {
        let actor = ((index + 2) % 3) as u8;
        act(&mut engine, actor, Command::RollDice);
        assert_eq!(
            engine.view(Viewer::Spectator).unwrap().last_dice,
            Some(dice)
        );
        if index < 17 {
            act(&mut engine, actor, Command::EndTurn);
        }
    }
    assert_eq!(
        engine.view(Viewer::Spectator).unwrap().bank.counts(),
        &[7, 3, 14, 11, 9]
    );
    assert_eq!(
        engine
            .view(Viewer::Seat(seat(1)))
            .unwrap()
            .own_player
            .unwrap()
            .resources
            .counts(),
        &[5, 5, 3, 5, 0]
    );
    // Red V9-E14-V13-E13-V8-E7-V4-E1-V0-E0-V3; E0 is generic.
    for id in [13, 7, 1, 0] {
        act(&mut engine, 1, Command::BuildRoad { edge: edge(id) });
    }
    act(
        &mut engine,
        1,
        Command::BuildSettlement { vertex: vertex(3) },
    );
    pairs(&engine, 1, Resource::Wheat);
    let receipt = act(
        &mut engine,
        1,
        Command::BankTrade {
            give_resource: Resource::Wheat,
            receive_resource: Resource::Ore,
        },
    );
    assert_eq!(
        receipt.events,
        vec![Event::BankTraded {
            seat: seat(1),
            give_resource: Resource::Wheat,
            receive_resource: Resource::Ore,
            rate: 3
        }]
    );
    assert_eq!(
        engine.view(Viewer::Spectator).unwrap().bank.counts(),
        &[12, 8, 15, 15, 8]
    );
    assert_eq!(
        engine
            .view(Viewer::Seat(seat(1)))
            .unwrap()
            .own_player
            .unwrap()
            .resources
            .counts(),
        &[0, 0, 2, 1, 1]
    );
    act(&mut engine, 1, Command::EndTurn);
    act(&mut engine, 2, Command::RollDice);
    assert_eq!(
        engine.view(Viewer::Spectator).unwrap().last_dice,
        Some([3, 5])
    );
    assert_eq!(
        engine.view(Viewer::Spectator).unwrap().bank.counts(),
        &[12, 7, 15, 14, 8]
    );
    projections(&engine, &receipt, Resource::Wheat, Resource::Ore, 3);
}
