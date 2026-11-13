//! Public setup traces: independently captured from CN3081 v6.250401 pp.5/12.
//! Stable incidence and resource provenance: support/setup-source.md.
//! Literal expectations were checked without engine imports; no runtime /tmp input.
use hex_traders_engine::{
    Command, CommandEnvelope, EdgeId, Engine, GameConfig, LegalAction, Phase, PlayerId, Resources,
    RuleError, VertexId, Viewer,
};
#[path = "game_sequences/bank_trade.rs"]
mod bank_trade;
#[path = "game_sequences/city.rs"]
mod city;
#[path = "game_sequences/development.rs"]
mod development;
#[path = "game_sequences/full_games.rs"]
mod full_games;
#[path = "game_sequences/invention.rs"]
mod invention;
#[path = "game_sequences/knight.rs"]
mod knight;
#[path = "game_sequences/monopoly.rs"]
mod monopoly;
#[path = "game_sequences/road.rs"]
mod road;
#[path = "game_sequences/settlement.rs"]
mod settlement;
#[path = "game_sequences/trade_accept.rs"]
mod trade_accept;
#[path = "game_sequences/trade_offer.rs"]
mod trade_offer;
#[path = "game_sequences/verified_game.rs"]
mod verified_game;

fn seat(id: u8) -> PlayerId {
    PlayerId::try_from(id).unwrap()
}
fn vertex(id: u8) -> VertexId {
    VertexId::try_from(id).unwrap()
}
fn edge(id: u8) -> EdgeId {
    EdgeId::try_from(id).unwrap()
}
struct SetupStep {
    actor: u8,
    command: Command,
    phase: Phase,
    turn: u64,
    routes: Vec<u8>,
    counts: Vec<u8>,
    bank: [u8; 5],
    stocks: Vec<[u8; 3]>,
}
fn run_setup(
    first_byte: u8,
    order: &[u8],
    starter: u8,
    turn_order: &[u8],
    steps: Vec<SetupStep>,
    hands: &[[u8; 5]],
    robber: usize,
) {
    let mut seed = [0; 32];
    seed[0] = first_byte;
    let (mut engine, _) = Engine::create(GameConfig::variable(order).unwrap(), seed).unwrap();
    let initial = engine.view(Viewer::Spectator).unwrap();
    assert_eq!(initial.starting_player, seat(starter));
    assert_eq!(
        initial.turn_order,
        turn_order.iter().map(|&id| seat(id)).collect::<Vec<_>>()
    );
    let final_step = steps.len() - 1;
    let mut buildings = [None; 54];
    let mut roads = [None; 72];
    for (index, expected) in steps.into_iter().enumerate() {
        let actor = seat(expected.actor);
        let revision = index as u64;
        let before = engine.view(Viewer::Spectator).unwrap();
        let chosen = engine.legal_actions(actor).unwrap();
        assert_eq!(chosen.revision, revision);
        assert!(
            chosen
                .choices
                .iter()
                .any(|action| match (action, expected.command) {
                    (
                        LegalAction::PlaceSetupSettlement { vertices },
                        Command::PlaceSetupSettlement { vertex },
                    ) => vertices.contains(&vertex),
                    (LegalAction::PlaceSetupRoad { edges }, Command::PlaceSetupRoad { edge }) =>
                        edges.contains(&edge),
                    _ => false,
                })
        );
        // Literal negative examples: wrong first actor, nonincident E2, occupied V0/E1, adjacent V4.
        let negative = match index {
            0 => Some((
                seat((expected.actor + 1) % order.len() as u8),
                expected.command,
                RuleError::WrongActor,
            )),
            1 => Some((
                actor,
                Command::PlaceSetupRoad { edge: edge(2) },
                RuleError::Disconnected,
            )),
            2 => Some((
                actor,
                Command::PlaceSetupSettlement { vertex: vertex(0) },
                RuleError::Occupied,
            )),
            _ if index == final_step => Some((
                actor,
                Command::PlaceSetupRoad { edge: edge(1) },
                RuleError::Occupied,
            )),
            _ => None,
        };
        if let Some((bad_actor, bad_command, error)) = negative {
            let private_before: Vec<_> = order
                .iter()
                .map(|&id| engine.view(Viewer::Seat(seat(id))).unwrap().own_player)
                .collect();
            assert_eq!(
                engine
                    .execute(CommandEnvelope {
                        api_version: 1,
                        actor: bad_actor,
                        expected_revision: revision,
                        command: bad_command
                    })
                    .unwrap_err()
                    .code,
                error
            );
            assert_eq!(
                format!("{:?}", engine.view(Viewer::Spectator).unwrap()),
                format!("{before:?}")
            );
            assert_eq!(
                order
                    .iter()
                    .map(|&id| engine.view(Viewer::Seat(seat(id))).unwrap().own_player)
                    .collect::<Vec<_>>(),
                private_before
            );
        }
        if index == 2 {
            assert_eq!(
                engine
                    .validate(actor, &Command::PlaceSetupSettlement { vertex: vertex(4) })
                    .unwrap_err()
                    .code,
                RuleError::DistanceRule
            );
        }
        assert_eq!(engine.validate(actor, &expected.command), Ok(()));
        let receipt = engine
            .execute(CommandEnvelope {
                api_version: 1,
                actor,
                expected_revision: revision,
                command: expected.command,
            })
            .unwrap();
        assert_eq!(
            (receipt.previous_revision, receipt.revision),
            (revision, revision + 1)
        );
        let after = engine.view(Viewer::Spectator).unwrap();
        assert_eq!(after.phase, expected.phase);
        assert_eq!(
            after.active_player,
            match expected.phase {
                Phase::SetupSettlement { actor, .. } | Phase::SetupRoad { actor, .. } => actor,
                Phase::PreRoll => seat(starter),
                _ => panic!("setup-only phase"),
            }
        );
        assert_eq!(after.turn, expected.turn);
        assert_eq!(after.bank, Resources::try_from(expected.bank).unwrap());
        assert_eq!(after.revision, revision + 1);
        assert_eq!(after.last_dice, None);
        assert_eq!(after.board.robber.index(), robber);
        assert_eq!(after.development_deck_remaining, 25);
        match expected.command {
            Command::PlaceSetupSettlement { vertex } => buildings[vertex.index()] = Some(actor),
            Command::PlaceSetupRoad { edge } => roads[edge.index()] = Some(actor),
            _ => panic!("setup-only fixture"),
        }
        assert_eq!(
            after
                .board
                .buildings
                .map(|building| building.map(|building| building.owner)),
            buildings
        );
        assert_eq!(after.board.roads, roads);
        for player in &after.players {
            let id = player.seat.index();
            assert_eq!(player.longest_route_length, expected.routes[id]);
            assert_eq!(player.resource_count, expected.counts[id]);
            assert_eq!(
                [
                    player.roads_remaining,
                    player.settlements_remaining,
                    player.cities_remaining
                ],
                expected.stocks[id]
            );
            assert_eq!(player.development_card_count, 0);
            let own = engine
                .view(Viewer::Seat(player.seat))
                .unwrap()
                .own_player
                .unwrap();
            assert_eq!(
                own.resources.counts(),
                if expected.turn == 0 {
                    &[0; 5]
                } else {
                    &hands[id]
                }
            );
            if expected.turn == 1 {
                assert_eq!((player.public_score, own.true_score), (2, 2));
            }
        }
    }
}

#[test]
fn setup_3_seat_literal_trace() {
    run_setup(
        12,
        &[2, 0, 1],
        0,
        &[0, 1, 2],
        vec![
            SetupStep {
                actor: 0,
                command: Command::PlaceSetupSettlement { vertex: vertex(0) },
                phase: Phase::SetupRoad {
                    actor: seat(0),
                    round: 1,
                    settlement: vertex(0),
                },
                turn: 0,
                routes: vec![0, 0, 0],
                counts: vec![0, 0, 0],
                bank: [19, 19, 19, 19, 19],
                stocks: vec![[15, 4, 4], [15, 5, 4], [15, 5, 4]],
            },
            SetupStep {
                actor: 0,
                command: Command::PlaceSetupRoad { edge: edge(1) },
                phase: Phase::SetupSettlement {
                    actor: seat(1),
                    round: 1,
                },
                turn: 0,
                routes: vec![1, 0, 0],
                counts: vec![0, 0, 0],
                bank: [19, 19, 19, 19, 19],
                stocks: vec![[14, 4, 4], [15, 5, 4], [15, 5, 4]],
            },
            SetupStep {
                actor: 1,
                command: Command::PlaceSetupSettlement { vertex: vertex(21) },
                phase: Phase::SetupRoad {
                    actor: seat(1),
                    round: 1,
                    settlement: vertex(21),
                },
                turn: 0,
                routes: vec![1, 0, 0],
                counts: vec![0, 0, 0],
                bank: [19, 19, 19, 19, 19],
                stocks: vec![[14, 4, 4], [15, 4, 4], [15, 5, 4]],
            },
            SetupStep {
                actor: 1,
                command: Command::PlaceSetupRoad { edge: edge(23) },
                phase: Phase::SetupSettlement {
                    actor: seat(2),
                    round: 1,
                },
                turn: 0,
                routes: vec![1, 1, 0],
                counts: vec![0, 0, 0],
                bank: [19, 19, 19, 19, 19],
                stocks: vec![[14, 4, 4], [14, 4, 4], [15, 5, 4]],
            },
            SetupStep {
                actor: 2,
                command: Command::PlaceSetupSettlement { vertex: vertex(51) },
                phase: Phase::SetupRoad {
                    actor: seat(2),
                    round: 1,
                    settlement: vertex(51),
                },
                turn: 0,
                routes: vec![1, 1, 0],
                counts: vec![0, 0, 0],
                bank: [19, 19, 19, 19, 19],
                stocks: vec![[14, 4, 4], [14, 4, 4], [15, 4, 4]],
            },
            SetupStep {
                actor: 2,
                command: Command::PlaceSetupRoad { edge: edge(66) },
                phase: Phase::SetupSettlement {
                    actor: seat(2),
                    round: 2,
                },
                turn: 0,
                routes: vec![1, 1, 1],
                counts: vec![0, 0, 0],
                bank: [19, 19, 19, 19, 19],
                stocks: vec![[14, 4, 4], [14, 4, 4], [14, 4, 4]],
            },
            SetupStep {
                actor: 2,
                command: Command::PlaceSetupSettlement { vertex: vertex(14) },
                phase: Phase::SetupRoad {
                    actor: seat(2),
                    round: 2,
                    settlement: vertex(14),
                },
                turn: 0,
                routes: vec![1, 1, 1],
                counts: vec![0, 0, 0],
                bank: [19, 19, 19, 19, 19],
                stocks: vec![[14, 4, 4], [14, 4, 4], [14, 3, 4]],
            },
            SetupStep {
                actor: 2,
                command: Command::PlaceSetupRoad { edge: edge(16) },
                phase: Phase::SetupSettlement {
                    actor: seat(1),
                    round: 2,
                },
                turn: 0,
                routes: vec![1, 1, 1],
                counts: vec![0, 0, 0],
                bank: [19, 19, 19, 19, 19],
                stocks: vec![[14, 4, 4], [14, 4, 4], [13, 3, 4]],
            },
            SetupStep {
                actor: 1,
                command: Command::PlaceSetupSettlement { vertex: vertex(35) },
                phase: Phase::SetupRoad {
                    actor: seat(1),
                    round: 2,
                    settlement: vertex(35),
                },
                turn: 0,
                routes: vec![1, 1, 1],
                counts: vec![0, 0, 0],
                bank: [19, 19, 19, 19, 19],
                stocks: vec![[14, 4, 4], [14, 3, 4], [13, 3, 4]],
            },
            SetupStep {
                actor: 1,
                command: Command::PlaceSetupRoad { edge: edge(51) },
                phase: Phase::SetupSettlement {
                    actor: seat(0),
                    round: 2,
                },
                turn: 0,
                routes: vec![1, 1, 1],
                counts: vec![0, 0, 0],
                bank: [19, 19, 19, 19, 19],
                stocks: vec![[14, 4, 4], [13, 3, 4], [13, 3, 4]],
            },
            SetupStep {
                actor: 0,
                command: Command::PlaceSetupSettlement { vertex: vertex(1) },
                phase: Phase::SetupRoad {
                    actor: seat(0),
                    round: 2,
                    settlement: vertex(1),
                },
                turn: 0,
                routes: vec![1, 1, 1],
                counts: vec![0, 0, 0],
                bank: [19, 19, 19, 19, 19],
                stocks: vec![[14, 3, 4], [13, 3, 4], [13, 3, 4]],
            },
            SetupStep {
                actor: 0,
                command: Command::PlaceSetupRoad { edge: edge(2) },
                phase: Phase::PreRoll,
                turn: 1,
                routes: vec![2, 1, 1],
                counts: vec![1, 3, 2],
                bank: [19, 18, 17, 16, 19],
                stocks: vec![[13, 3, 4], [13, 3, 4], [13, 3, 4]],
            },
        ],
        &[[0, 0, 1, 0, 0], [0, 1, 0, 2, 0], [0, 0, 1, 1, 0]],
        6,
    );
}

#[test]
fn setup_4_seat_literal_trace() {
    run_setup(
        6,
        &[2, 0, 3, 1],
        2,
        &[2, 0, 3, 1],
        vec![
            SetupStep {
                actor: 2,
                command: Command::PlaceSetupSettlement { vertex: vertex(0) },
                phase: Phase::SetupRoad {
                    actor: seat(2),
                    round: 1,
                    settlement: vertex(0),
                },
                turn: 0,
                routes: vec![0, 0, 0, 0],
                counts: vec![0, 0, 0, 0],
                bank: [19, 19, 19, 19, 19],
                stocks: vec![[15, 5, 4], [15, 5, 4], [15, 4, 4], [15, 5, 4]],
            },
            SetupStep {
                actor: 2,
                command: Command::PlaceSetupRoad { edge: edge(1) },
                phase: Phase::SetupSettlement {
                    actor: seat(0),
                    round: 1,
                },
                turn: 0,
                routes: vec![0, 0, 1, 0],
                counts: vec![0, 0, 0, 0],
                bank: [19, 19, 19, 19, 19],
                stocks: vec![[15, 5, 4], [15, 5, 4], [14, 4, 4], [15, 5, 4]],
            },
            SetupStep {
                actor: 0,
                command: Command::PlaceSetupSettlement { vertex: vertex(53) },
                phase: Phase::SetupRoad {
                    actor: seat(0),
                    round: 1,
                    settlement: vertex(53),
                },
                turn: 0,
                routes: vec![0, 0, 1, 0],
                counts: vec![0, 0, 0, 0],
                bank: [19, 19, 19, 19, 19],
                stocks: vec![[15, 4, 4], [15, 5, 4], [14, 4, 4], [15, 5, 4]],
            },
            SetupStep {
                actor: 0,
                command: Command::PlaceSetupRoad { edge: edge(71) },
                phase: Phase::SetupSettlement {
                    actor: seat(3),
                    round: 1,
                },
                turn: 0,
                routes: vec![1, 0, 1, 0],
                counts: vec![0, 0, 0, 0],
                bank: [19, 19, 19, 19, 19],
                stocks: vec![[14, 4, 4], [15, 5, 4], [14, 4, 4], [15, 5, 4]],
            },
            SetupStep {
                actor: 3,
                command: Command::PlaceSetupSettlement { vertex: vertex(45) },
                phase: Phase::SetupRoad {
                    actor: seat(3),
                    round: 1,
                    settlement: vertex(45),
                },
                turn: 0,
                routes: vec![1, 0, 1, 0],
                counts: vec![0, 0, 0, 0],
                bank: [19, 19, 19, 19, 19],
                stocks: vec![[14, 4, 4], [15, 5, 4], [14, 4, 4], [15, 4, 4]],
            },
            SetupStep {
                actor: 3,
                command: Command::PlaceSetupRoad { edge: edge(64) },
                phase: Phase::SetupSettlement {
                    actor: seat(1),
                    round: 1,
                },
                turn: 0,
                routes: vec![1, 0, 1, 1],
                counts: vec![0, 0, 0, 0],
                bank: [19, 19, 19, 19, 19],
                stocks: vec![[14, 4, 4], [15, 5, 4], [14, 4, 4], [14, 4, 4]],
            },
            SetupStep {
                actor: 1,
                command: Command::PlaceSetupSettlement { vertex: vertex(27) },
                phase: Phase::SetupRoad {
                    actor: seat(1),
                    round: 1,
                    settlement: vertex(27),
                },
                turn: 0,
                routes: vec![1, 0, 1, 1],
                counts: vec![0, 0, 0, 0],
                bank: [19, 19, 19, 19, 19],
                stocks: vec![[14, 4, 4], [15, 4, 4], [14, 4, 4], [14, 4, 4]],
            },
            SetupStep {
                actor: 1,
                command: Command::PlaceSetupRoad { edge: edge(39) },
                phase: Phase::SetupSettlement {
                    actor: seat(1),
                    round: 2,
                },
                turn: 0,
                routes: vec![1, 1, 1, 1],
                counts: vec![0, 0, 0, 0],
                bank: [19, 19, 19, 19, 19],
                stocks: vec![[14, 4, 4], [14, 4, 4], [14, 4, 4], [14, 4, 4]],
            },
            SetupStep {
                actor: 1,
                command: Command::PlaceSetupSettlement { vertex: vertex(26) },
                phase: Phase::SetupRoad {
                    actor: seat(1),
                    round: 2,
                    settlement: vertex(26),
                },
                turn: 0,
                routes: vec![1, 1, 1, 1],
                counts: vec![0, 0, 0, 0],
                bank: [19, 19, 19, 19, 19],
                stocks: vec![[14, 4, 4], [14, 3, 4], [14, 4, 4], [14, 4, 4]],
            },
            SetupStep {
                actor: 1,
                command: Command::PlaceSetupRoad { edge: edge(38) },
                phase: Phase::SetupSettlement {
                    actor: seat(3),
                    round: 2,
                },
                turn: 0,
                routes: vec![1, 1, 1, 1],
                counts: vec![0, 0, 0, 0],
                bank: [19, 19, 19, 19, 19],
                stocks: vec![[14, 4, 4], [13, 3, 4], [14, 4, 4], [14, 4, 4]],
            },
            SetupStep {
                actor: 3,
                command: Command::PlaceSetupSettlement { vertex: vertex(39) },
                phase: Phase::SetupRoad {
                    actor: seat(3),
                    round: 2,
                    settlement: vertex(39),
                },
                turn: 0,
                routes: vec![1, 1, 1, 1],
                counts: vec![0, 0, 0, 0],
                bank: [19, 19, 19, 19, 19],
                stocks: vec![[14, 4, 4], [13, 3, 4], [14, 4, 4], [14, 3, 4]],
            },
            SetupStep {
                actor: 3,
                command: Command::PlaceSetupRoad { edge: edge(56) },
                phase: Phase::SetupSettlement {
                    actor: seat(0),
                    round: 2,
                },
                turn: 0,
                routes: vec![1, 1, 1, 1],
                counts: vec![0, 0, 0, 0],
                bank: [19, 19, 19, 19, 19],
                stocks: vec![[14, 4, 4], [13, 3, 4], [14, 4, 4], [13, 3, 4]],
            },
            SetupStep {
                actor: 0,
                command: Command::PlaceSetupSettlement { vertex: vertex(12) },
                phase: Phase::SetupRoad {
                    actor: seat(0),
                    round: 2,
                    settlement: vertex(12),
                },
                turn: 0,
                routes: vec![1, 1, 1, 1],
                counts: vec![0, 0, 0, 0],
                bank: [19, 19, 19, 19, 19],
                stocks: vec![[14, 3, 4], [13, 3, 4], [14, 4, 4], [13, 3, 4]],
            },
            SetupStep {
                actor: 0,
                command: Command::PlaceSetupRoad { edge: edge(19) },
                phase: Phase::SetupSettlement {
                    actor: seat(2),
                    round: 2,
                },
                turn: 0,
                routes: vec![1, 1, 1, 1],
                counts: vec![0, 0, 0, 0],
                bank: [19, 19, 19, 19, 19],
                stocks: vec![[13, 3, 4], [13, 3, 4], [14, 4, 4], [13, 3, 4]],
            },
            SetupStep {
                actor: 2,
                command: Command::PlaceSetupSettlement { vertex: vertex(1) },
                phase: Phase::SetupRoad {
                    actor: seat(2),
                    round: 2,
                    settlement: vertex(1),
                },
                turn: 0,
                routes: vec![1, 1, 1, 1],
                counts: vec![0, 0, 0, 0],
                bank: [19, 19, 19, 19, 19],
                stocks: vec![[13, 3, 4], [13, 3, 4], [14, 3, 4], [13, 3, 4]],
            },
            SetupStep {
                actor: 2,
                command: Command::PlaceSetupRoad { edge: edge(2) },
                phase: Phase::PreRoll,
                turn: 1,
                routes: vec![1, 1, 2, 1],
                counts: vec![3, 1, 0, 3],
                bank: [18, 18, 18, 17, 17],
                stocks: vec![[13, 3, 4], [13, 3, 4], [13, 3, 4], [13, 3, 4]],
            },
        ],
        &[
            [0, 1, 0, 2, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0],
            [1, 0, 0, 0, 2],
        ],
        1,
    );
}

#[test]
fn turn_fixed_first_roll_matches_independent_dice_and_source_production() {
    // Standalone pinned-dependency oracle: seed0 deck+starter consume10 words,
    // starter2; first gameplay dice [2,3]. Source H8 field5 and H11 pasture5.
    let (mut engine, _) = Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), [0; 32]).unwrap();
    assert_eq!(
        engine.legal_actions(seat(2)).unwrap().choices,
        vec![LegalAction::RollDice]
    );
    let receipt = engine
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(2),
            expected_revision: 0,
            command: Command::RollDice,
        })
        .unwrap();
    let public = engine.view(Viewer::Spectator).unwrap();
    assert_eq!(public.phase, Phase::Action);
    assert_eq!(public.last_dice, Some([2, 3]));
    assert_eq!((public.revision, public.turn), (1, 1));
    // Source H8 field5 touches Orange V17; H11 pasture5 touches Blue V31.
    let hands = [[1, 0, 1, 1, 1], [0, 1, 1, 1, 0], [1, 0, 1, 2, 0]];
    for (id, hand) in hands.into_iter().enumerate() {
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
    assert_eq!(receipt.revision, 1);
    assert_eq!(
        engine
            .execute(CommandEnvelope {
                api_version: 1,
                actor: seat(2),
                expected_revision: 0,
                command: Command::RollDice
            })
            .unwrap_err()
            .code,
        RuleError::StaleRevision
    );
}

#[test]
fn turn_end_wraps_supplied_order_without_consuming_next_dice() {
    let (mut engine, _) = Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), [0; 32]).unwrap();
    engine
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(2),
            expected_revision: 0,
            command: Command::RollDice,
        })
        .unwrap();
    let hands: Vec<_> = (0..3)
        .map(|id| {
            engine
                .view(Viewer::Seat(seat(id)))
                .unwrap()
                .own_player
                .unwrap()
                .resources
        })
        .collect();
    engine
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(2),
            expected_revision: 1,
            command: Command::EndTurn,
        })
        .unwrap();
    let public = engine.view(Viewer::Spectator).unwrap();
    assert_eq!(
        (
            public.active_player,
            public.turn,
            public.phase,
            public.last_dice
        ),
        (seat(0), 2, Phase::PreRoll, None)
    );
    assert_eq!(public.revision, 2);
    assert_eq!(
        (0..3)
            .map(|id| engine
                .view(Viewer::Seat(seat(id)))
                .unwrap()
                .own_player
                .unwrap()
                .resources)
            .collect::<Vec<_>>(),
        hands
    );
    // Independent next pair after the first seed0 gameplay pair is [2,1].
    engine
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(0),
            expected_revision: 2,
            command: Command::RollDice,
        })
        .unwrap();
    assert_eq!(
        engine.view(Viewer::Spectator).unwrap().last_dice,
        Some([2, 1])
    );
    engine
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(0),
            expected_revision: 3,
            command: Command::EndTurn,
        })
        .unwrap();
    let public = engine.view(Viewer::Spectator).unwrap();
    assert_eq!(
        (public.active_player, public.turn, public.phase),
        (seat(1), 3, Phase::PreRoll)
    );
}

#[test]
fn discard_public_arrival_orders_keep_fixed_obligations_until_last() {
    // Independent pinned RNG/source incidence calculation, fixed seats [0,1,2],
    // seed12; all stock covers
    // claims before the first seven. Final hands/bank and discards are literal.
    let pairs = [
        [5, 1],
        [3, 3],
        [2, 2],
        [4, 6],
        [4, 6],
        [3, 2],
        [1, 2],
        [2, 6],
        [3, 2],
        [3, 3],
        [4, 4],
        [6, 6],
        [4, 5],
        [5, 3],
        [5, 1],
        [3, 5],
        [5, 1],
        [6, 5],
        [1, 4],
        [6, 1],
    ];
    let before_hands = [[2, 5, 3, 2, 2], [1, 2, 2, 5, 5], [6, 2, 2, 4, 1]];
    let discards = [[2, 5, 0, 0, 0], [0, 0, 0, 5, 2], [6, 1, 0, 0, 0]];
    for arrival in [[2, 0, 1], [1, 0, 2]] {
        let mut seed = [0; 32];
        seed[0] = 12;
        let (mut engine, _) = Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), seed).unwrap();
        for (turn, dice) in pairs.into_iter().enumerate() {
            let actor = seat((turn % 3) as u8);
            let revision = (turn * 2) as u64;
            engine
                .execute(CommandEnvelope {
                    api_version: 1,
                    actor,
                    expected_revision: revision,
                    command: Command::RollDice,
                })
                .unwrap();
            assert_eq!(
                engine.view(Viewer::Spectator).unwrap().last_dice,
                Some(dice)
            );
            if turn < 19 {
                engine
                    .execute(CommandEnvelope {
                        api_version: 1,
                        actor,
                        expected_revision: revision + 1,
                        command: Command::EndTurn,
                    })
                    .unwrap();
            }
        }
        let mut obligations = [7, 7, 7, 0];
        let public = engine.view(Viewer::Spectator).unwrap();
        assert_eq!(
            (public.phase, public.bank, public.revision),
            (
                Phase::Discard { obligations },
                Resources::try_from([10, 10, 12, 8, 11]).unwrap(),
                39
            )
        );
        for id in 0..3 {
            assert_eq!(
                engine
                    .view(Viewer::Seat(seat(id)))
                    .unwrap()
                    .own_player
                    .unwrap()
                    .resources
                    .counts(),
                &before_hands[id as usize]
            );
        }
        for (index, id) in arrival.into_iter().enumerate() {
            let actor = seat(id);
            let revision = 39 + index as u64;
            assert_eq!(
                engine.legal_actions(actor).unwrap().choices,
                vec![LegalAction::DiscardResources { quantity: 7 }]
            );
            let before = format!("{:?}", engine.view(Viewer::Spectator).unwrap());
            assert!(
                engine
                    .execute(CommandEnvelope {
                        api_version: 1,
                        actor,
                        expected_revision: revision,
                        command: Command::EndTurn
                    })
                    .is_err()
            );
            assert_eq!(
                format!("{:?}", engine.view(Viewer::Spectator).unwrap()),
                before
            );
            let command = Command::DiscardResources {
                resources: Resources::try_from(discards[id as usize]).unwrap(),
            };
            assert_eq!(engine.validate(actor, &command), Ok(()));
            let receipt = engine
                .execute(CommandEnvelope {
                    api_version: 1,
                    actor,
                    expected_revision: revision,
                    command,
                })
                .unwrap();
            assert_eq!(receipt.revision, revision + 1);
            obligations[id as usize] = 0;
            assert_eq!(
                engine.legal_actions(actor).unwrap().choices,
                if index == 2 && id == 1 {
                    vec![LegalAction::MoveRobber {
                        hexes: (1..19)
                            .map(|value| hex_traders_engine::HexId::try_from(value).unwrap())
                            .collect(),
                    }]
                } else {
                    vec![]
                }
            );
            let public = engine.view(Viewer::Spectator).unwrap();
            assert_eq!(
                public.phase,
                if index == 2 {
                    Phase::RobberMove {
                        origin: hex_traders_engine::RobberOrigin::Seven,
                        continuation: hex_traders_engine::ResumePhase::Action,
                    }
                } else {
                    Phase::Discard { obligations }
                }
            );
            assert!(
                engine
                    .execute(CommandEnvelope {
                        api_version: 1,
                        actor,
                        expected_revision: revision + 1,
                        command
                    })
                    .is_err()
            );
        }
        for (id, hand) in [[0, 0, 3, 2, 2], [1, 2, 2, 0, 3], [0, 1, 2, 4, 1]]
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
        assert_eq!(
            engine.view(Viewer::Spectator).unwrap().bank.counts(),
            &[18, 16, 12, 13, 13]
        );
    }
}

#[test]
fn discard_four_seat_trace_excludes_exactly_seven_and_preserves_arrival_order() {
    // Independent seed220 oracle uses fixed supplied order [2,0,3,1].
    let rolls = [
        (3, [6, 3]),
        (1, [4, 1]),
        (2, [3, 2]),
        (0, [6, 2]),
        (3, [3, 2]),
        (1, [2, 5]),
    ];
    for arrival in [[1, 2], [2, 1]] {
        let mut seed = [0; 32];
        seed[0] = 220;
        let (mut engine, _) =
            Engine::create(GameConfig::fixed(&[2, 0, 3, 1]).unwrap(), seed).unwrap();
        assert_eq!(
            engine.view(Viewer::Spectator).unwrap().starting_player,
            seat(3)
        );
        for (index, (id, dice)) in rolls.into_iter().enumerate() {
            let revision = (index * 2) as u64;
            engine
                .execute(CommandEnvelope {
                    api_version: 1,
                    actor: seat(id),
                    expected_revision: revision,
                    command: Command::RollDice,
                })
                .unwrap();
            assert_eq!(
                engine.view(Viewer::Spectator).unwrap().last_dice,
                Some(dice)
            );
            if index < 5 {
                engine
                    .execute(CommandEnvelope {
                        api_version: 1,
                        actor: seat(id),
                        expected_revision: revision + 1,
                        command: Command::EndTurn,
                    })
                    .unwrap();
            }
        }
        let mut obligations = [0, 4, 4, 0];
        let before = [
            [1, 1, 1, 2, 0],
            [3, 1, 0, 4, 0],
            [2, 1, 3, 1, 1],
            [1, 0, 1, 4, 1],
        ];
        let public = engine.view(Viewer::Spectator).unwrap();
        assert_eq!(public.phase, Phase::Discard { obligations });
        assert_eq!(public.bank.counts(), &[12, 16, 14, 8, 17]);
        for (id, hand) in before.into_iter().enumerate() {
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
        assert_eq!(
            public
                .players
                .iter()
                .find(|player| player.seat == seat(3))
                .unwrap()
                .resource_count,
            7
        );
        assert!(engine.legal_actions(seat(3)).unwrap().choices.is_empty());
        for (index, id) in arrival.into_iter().enumerate() {
            let actor = seat(id);
            let revision = 11 + index as u64;
            assert_eq!(
                engine.legal_actions(actor).unwrap().choices,
                vec![LegalAction::DiscardResources { quantity: 4 }]
            );
            let bundle = if id == 1 {
                [3, 1, 0, 0, 0]
            } else {
                [2, 1, 1, 0, 0]
            };
            engine
                .execute(CommandEnvelope {
                    api_version: 1,
                    actor,
                    expected_revision: revision,
                    command: Command::DiscardResources {
                        resources: Resources::try_from(bundle).unwrap(),
                    },
                })
                .unwrap();
            obligations[id as usize] = 0;
            assert_eq!(
                engine.view(Viewer::Spectator).unwrap().phase,
                if index == 0 {
                    Phase::Discard { obligations }
                } else {
                    Phase::RobberMove {
                        origin: hex_traders_engine::RobberOrigin::Seven,
                        continuation: hex_traders_engine::ResumePhase::Action,
                    }
                }
            );
        }
        assert_eq!(
            engine.view(Viewer::Spectator).unwrap().bank.counts(),
            &[17, 18, 15, 8, 17]
        );
        let after = [
            [1, 1, 1, 2, 0],
            [0, 0, 0, 4, 0],
            [0, 0, 2, 1, 1],
            [1, 0, 1, 4, 1],
        ];
        for (id, hand) in after.into_iter().enumerate() {
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
    }
}

#[test]
fn robber_public_zero_automatic_and_chosen_paths_keep_exact_hands_and_next_dice() {
    use hex_traders_engine::{Event, HexId, ProjectedEvent, Resource, ResumePhase, RobberOrigin};
    // Independent pinned-rand program: deck 0..24, starter rolls, first dice,
    // actual-card index and next dice. Source and output retained in T10c evidence.
    let cases = [
        (
            9,
            2,
            [6, 1],
            18,
            None,
            None,
            [[1, 0, 0, 1, 1], [0, 1, 1, 1, 0], [1, 0, 1, 1, 0]],
            [5, 1],
        ),
        (
            63,
            0,
            [1, 6],
            1,
            Some(1),
            Some(Resource::Wheat),
            [[1, 0, 0, 2, 1], [0, 1, 1, 0, 0], [1, 0, 1, 1, 0]],
            [4, 3],
        ),
        (
            35,
            1,
            [2, 5],
            10,
            Some(0),
            Some(Resource::Ore),
            [[1, 0, 0, 1, 0], [0, 1, 1, 1, 1], [1, 0, 1, 1, 0]],
            [3, 6],
        ),
    ];
    for (first, active, dice, destination, victim, stolen, hands, next_dice) in cases {
        let mut seed = [0; 32];
        seed[0] = first;
        let (mut engine, _) = Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), seed).unwrap();
        let actor = seat(active);
        assert_eq!(engine.view(Viewer::Spectator).unwrap().active_player, actor);
        engine
            .execute(CommandEnvelope {
                api_version: 1,
                actor,
                expected_revision: 0,
                command: Command::RollDice,
            })
            .unwrap();
        let before = engine.view(Viewer::Spectator).unwrap();
        assert_eq!(before.last_dice, Some(dice));
        assert_eq!(
            before.phase,
            Phase::RobberMove {
                origin: RobberOrigin::Seven,
                continuation: ResumePhase::Action
            }
        );
        assert_eq!(
            engine.legal_actions(actor).unwrap().choices,
            vec![LegalAction::MoveRobber {
                hexes: (1..19).map(|id| HexId::try_from(id).unwrap()).collect()
            }]
        );
        assert_eq!(
            engine
                .execute(CommandEnvelope {
                    api_version: 1,
                    actor,
                    expected_revision: 1,
                    command: Command::MoveRobber {
                        hex: HexId::try_from(0).unwrap()
                    }
                })
                .unwrap_err()
                .code,
            RuleError::InvalidInput
        );
        assert_eq!(engine.view(Viewer::Spectator).unwrap().revision, 1);
        let receipt = engine
            .execute(CommandEnvelope {
                api_version: 1,
                actor,
                expected_revision: 1,
                command: Command::MoveRobber {
                    hex: HexId::try_from(destination).unwrap(),
                },
            })
            .unwrap();
        assert_eq!(
            receipt.events[0],
            Event::RobberMoved {
                seat: actor,
                hex: HexId::try_from(destination).unwrap()
            }
        );
        let theft_receipt = if destination == 10 {
            let phase = Phase::RobberVictim {
                origin: RobberOrigin::Seven,
                continuation: ResumePhase::Action,
                victims: [Some(seat(0)), None, Some(seat(2)), None],
            };
            assert_eq!(
                receipt.events,
                vec![
                    Event::RobberMoved {
                        seat: actor,
                        hex: HexId::try_from(10).unwrap()
                    },
                    Event::PhaseChanged { phase }
                ]
            );
            assert_eq!(engine.view(Viewer::Spectator).unwrap().phase, phase);
            assert_eq!(
                engine.legal_actions(actor).unwrap().choices,
                vec![LegalAction::ChooseRobberVictim {
                    victims: vec![seat(0), seat(2)]
                }]
            );
            assert_eq!(
                engine
                    .execute(CommandEnvelope {
                        api_version: 1,
                        actor,
                        expected_revision: 2,
                        command: Command::ChooseRobberVictim { victim: actor }
                    })
                    .unwrap_err()
                    .code,
                RuleError::InvalidVictim
            );
            engine
                .execute(CommandEnvelope {
                    api_version: 1,
                    actor,
                    expected_revision: 2,
                    command: Command::ChooseRobberVictim { victim: seat(0) },
                })
                .unwrap()
        } else {
            receipt
        };
        let public = engine.view(Viewer::Spectator).unwrap();
        assert_eq!(public.phase, Phase::Action);
        assert_eq!(public.board.robber, HexId::try_from(destination).unwrap());
        assert_eq!(public.board.roads, before.board.roads);
        assert_eq!(public.board.buildings, before.board.buildings);
        assert_eq!(public.bank.counts(), &[17, 18, 17, 16, 18]);
        for id in 0..3 {
            assert_eq!(
                engine
                    .view(Viewer::Seat(seat(id)))
                    .unwrap()
                    .own_player
                    .unwrap()
                    .resources
                    .counts(),
                &hands[id as usize]
            );
        }
        for viewer in [
            Viewer::Spectator,
            Viewer::Seat(seat(0)),
            Viewer::Seat(seat(1)),
            Viewer::Seat(seat(2)),
        ] {
            let projected = engine.project_receipt(&theft_receipt, viewer).unwrap();
            if let Some(victim) = victim {
                let resource = if matches!(viewer, Viewer::Seat(owner) if owner == actor || owner == seat(victim))
                {
                    stolen
                } else {
                    None
                };
                assert!(projected.events.contains(&ProjectedEvent::RobberTheft {
                    thief: actor,
                    victim: seat(victim),
                    count: 1,
                    resource
                }));
            } else {
                assert!(
                    !projected
                        .events
                        .iter()
                        .any(|event| matches!(event, ProjectedEvent::RobberTheft { .. }))
                );
            }
        }
        let revision = public.revision;
        engine
            .execute(CommandEnvelope {
                api_version: 1,
                actor,
                expected_revision: revision,
                command: Command::EndTurn,
            })
            .unwrap();
        let next = seat((active + 1) % 3);
        engine
            .execute(CommandEnvelope {
                api_version: 1,
                actor: next,
                expected_revision: revision + 1,
                command: Command::RollDice,
            })
            .unwrap();
        assert_eq!(
            engine.view(Viewer::Spectator).unwrap().last_dice,
            Some(next_dice)
        );
    }
}

#[path = "game_sequences/free_road.rs"]
mod free_road;

#[path = "game_sequences/victory.rs"]
mod victory;
