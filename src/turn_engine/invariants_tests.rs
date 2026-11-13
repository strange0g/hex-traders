use super::*;
use crate::{
    game_model::{Building, Command, GameConfig, HexId, Resources},
    turn_engine::{create_game, evaluate},
};

fn apply(board: &Board, state: &mut GameState, command: Command) {
    let next = evaluate(board, state, state.position.active_player, &command).unwrap();
    state.position = next.position;
    state.revision = next.revision;
    if let Some(random) = next.random {
        state.random = random;
    }
}
fn draw(p: &mut Position, id: u8, location: CardLocation) {
    let at = p
        .deck
        .iter()
        .position(|c| c.index() == usize::from(id))
        .unwrap();
    p.deck.swap(usize::from(p.draw_cursor), at);
    p.draw_cursor += 1;
    p.card_locations[usize::from(id)] = location;
}
#[test]
fn invariant_creation_and_public_progressed_setup_and_turns_are_valid() {
    for config in [
        GameConfig::fixed(&[2, 0, 1]).unwrap(),
        GameConfig::fixed(&[2, 0, 3, 1]).unwrap(),
        GameConfig::variable(&[2, 0, 1]).unwrap(),
        GameConfig::variable(&[2, 0, 3, 1]).unwrap(),
    ] {
        let mut game = create_game(config, [0; 32]);
        validate_game(&game.board, &game.state).unwrap();
        loop {
            let command = match game.state.position.phase {
                Phase::SetupSettlement { .. } => Command::PlaceSetupSettlement {
                    vertex: (0..54)
                        .map(|i| VertexId::try_from(i).unwrap())
                        .find(|&v| {
                            validate_setup_settlement(
                                &game.board,
                                &game.state.position.buildings,
                                v,
                            )
                            .is_ok()
                        })
                        .unwrap(),
                },
                Phase::SetupRoad { settlement, .. } => Command::PlaceSetupRoad {
                    edge: *game
                        .board
                        .graph
                        .vertex_edges(settlement)
                        .iter()
                        .find(|&&e| {
                            validate_setup_road(
                                &game.board,
                                &game.state.position.roads,
                                settlement,
                                e,
                            )
                            .is_ok()
                        })
                        .unwrap(),
                },
                _ => break,
            };
            apply(&game.board, &mut game.state, command);
            validate_game(&game.board, &game.state).unwrap();
        }
        for _ in 0..12 {
            apply(&game.board, &mut game.state, Command::RollDice);
            validate_game(&game.board, &game.state).unwrap();
            if matches!(game.state.position.phase, Phase::RobberMove { .. }) {
                let hex = (0..19)
                    .map(|i| HexId::try_from(i).unwrap())
                    .find(|&h| {
                        h != game.state.position.robber
                            && robber_victims(
                                game.board.graph,
                                &game.state.position.buildings,
                                game.state.position.active_player,
                                h,
                            )
                            .iter()
                            .flatten()
                            .count()
                                == 0
                    })
                    .unwrap();
                apply(&game.board, &mut game.state, Command::MoveRobber { hex });
                validate_game(&game.board, &game.state).unwrap();
            }
            if game.state.position.phase != Phase::Action {
                break;
            }
            apply(&game.board, &mut game.state, Command::EndTurn);
            validate_game(&game.board, &game.state).unwrap();
        }
    }
}
#[test]
fn invariant_rejects_independent_layout_resource_owner_piece_card_and_cache_corruption() {
    type Mutation = fn(&mut Board, &mut GameState);
    let mutations: [Mutation; 22] = [
        |b, _| b.numbers.swap(1, 2),
        |b, _| b.terrain.swap(1, 2),
        |b, _| b.frame_permutation[0] = 0,
        |b, _| b.frame_permutation[0] = 7,
        |b, _| b.frame_permutation.swap(0, 1),
        |b, _| b.ports.swap(0, 1),
        |_, s| s.position.bank = Resources::default(),
        |_, s| s.position.hands[3] = Resources::try_from([1, 0, 0, 0, 0]).unwrap(),
        |_, s| s.position.roads[0] = Some(seat(3)),
        |_, s| {
            s.position.buildings[0] = Some(Building {
                kind: BuildingKind::Settlement,
                owner: seat(3),
            })
        },
        |_, s| *s.position.roads.iter_mut().find(|s| s.is_some()).unwrap() = None,
        |_, s| s.position.second_settlements[0] = None,
        |_, s| s.position.road_lengths[0] = 15,
        |_, s| s.position.deck[0] = s.position.deck[1],
        |_, s| s.position.draw_cursor = 26,
        |_, s| s.position.draw_cursor = 1,
        |_, s| s.position.starting_player = seat(3),
        |_, s| s.position.active_player = seat(3),
        |_, s| s.position.turn = 2,
        |_, s| s.position.last_dice = Some([0, 7]),
        |_, s| s.position.played_non_vp = true,
        |_, s| s.position.final_scores = Some([0; 4]),
    ];
    for (index, mutate) in mutations.into_iter().enumerate() {
        let mut g = create_game(GameConfig::fixed(&[0, 1, 2]).unwrap(), [0; 32]);
        mutate(&mut g.board, &mut g.state);
        assert_eq!(
            validate_game(&g.board, &g.state),
            Err(InvalidState),
            "corruption {index}"
        );
    }
}
#[test]
fn invariant_card_acquisition_pending_cards_discards_and_offer_rules() {
    let mut g = create_game(GameConfig::fixed(&[0, 1, 2]).unwrap(), [0; 32]);
    let active = g.state.position.active_player;
    g.state.position.phase = Phase::Action;
    g.state.position.last_dice = Some([3, 4]);
    g.state.revision = 1;
    draw(
        &mut g.state.position,
        14,
        CardLocation::Held {
            owner: active,
            acquired_turn: 1,
        },
    );
    validate_game(&g.board, &g.state).unwrap();
    let valid = g.state.position.clone();
    for (owner, turn) in [
        (active, 0),
        (active, 2),
        (seat((active.index() + 1) % 3), 1),
    ] {
        g.state.position.card_locations[14] = CardLocation::Held {
            owner,
            acquired_turn: turn,
        };
        assert!(validate_game(&g.board, &g.state).is_err());
    }
    g.state.position = valid.clone();
    g.state.position.last_dice = None;
    g.state.position.phase = Phase::PreRoll;
    assert!(validate_game(&g.board, &g.state).is_err());
    g.state.position = valid.clone();
    g.state.position.card_locations[14] = CardLocation::PlayedKnight { owner: active };
    assert!(validate_game(&g.board, &g.state).is_err());
    g.state.position = valid;
    g.state.position.card_locations[14] = CardLocation::Spent { owner: active };
    g.state.position.played_non_vp = true;
    g.state.position.phase = Phase::FreeRoads {
        remaining: 2,
        continuation: ResumePhase::Action,
    };
    validate_game(&g.board, &g.state).unwrap();
    g.state.position.phase = Phase::FreeRoads {
        remaining: 0,
        continuation: ResumePhase::Action,
    };
    assert!(validate_game(&g.board, &g.state).is_err());
    g.state.position.phase = Phase::Discard {
        obligations: [0; 4],
    };
    assert!(validate_game(&g.board, &g.state).is_err());
    g.state.position.hands = [Resources::default(); 4];
    g.state.position.hands[0] = Resources::try_from([10, 0, 0, 0, 0]).unwrap();
    g.state.position.hands[1] = Resources::try_from([0, 10, 0, 0, 0]).unwrap();
    g.state.position.bank = Resources::try_from([9, 9, 19, 19, 19]).unwrap();
    // Zero means fulfilled: seat0 legitimately keeps ten cards after an earlier discard.
    g.state.position.phase = Phase::Discard {
        obligations: [0, 5, 0, 0],
    };
    validate_game(&g.board, &g.state).unwrap();
    g.state.position.phase = Phase::Discard {
        obligations: [0, 4, 0, 0],
    };
    assert!(validate_game(&g.board, &g.state).is_err());
    g.state.position.phase = Phase::Action;
    g.state.position.offer = Some(crate::TradeOffer {
        id: 1,
        turn: 1,
        proposer: seat(0),
        recipient: active,
        give: Resources::try_from([1, 0, 0, 0, 0]).unwrap(),
        receive: Resources::try_from([0, 0, 1, 0, 0]).unwrap(),
    });
    if active == seat(0) {
        g.state.position.offer.as_mut().unwrap().recipient = seat(1);
    }
    validate_game(&g.board, &g.state).unwrap(); // Recipient need not be able to fund it.
    g.state.position.offer.as_mut().unwrap().id = 2;
    assert!(validate_game(&g.board, &g.state).is_err());
}

#[test]
fn invariant_army_ties_robber_victims_completion_and_overflow_safe_counters() {
    let mut g = create_game(GameConfig::fixed(&[0, 1, 2]).unwrap(), [0; 32]);
    let active = g.state.position.active_player;
    let other = seat((active.index() + 1) % 3);
    for id in 0..3 {
        draw(
            &mut g.state.position,
            id,
            CardLocation::PlayedKnight { owner: active },
        );
    }
    for id in 3..6 {
        draw(
            &mut g.state.position,
            id,
            CardLocation::PlayedKnight { owner: other },
        );
    }
    assert!(validate_game(&g.board, &g.state).is_err()); // Army cannot be unclaimed at qualifying tie.
    g.state.position.largest_army_holder = Some(active);
    validate_game(&g.board, &g.state).unwrap();
    g.state.position.largest_army_holder = Some(other);
    validate_game(&g.board, &g.state).unwrap();
    g.state.position.largest_army_holder = Some(seat((active.index() + 2) % 3));
    assert!(validate_game(&g.board, &g.state).is_err());
    g.state.position.largest_army_holder = Some(active);
    g.state.position.played_non_vp = true;
    g.state.position.phase = Phase::RobberMove {
        origin: RobberOrigin::Knight,
        continuation: ResumePhase::PreRoll,
    };
    validate_game(&g.board, &g.state).unwrap();
    let hex = (0..19)
        .map(|i| HexId::try_from(i).unwrap())
        .find(|&h| {
            robber_victims(g.board.graph, &g.state.position.buildings, active, h)
                .iter()
                .flatten()
                .count()
                > 1
        })
        .unwrap();
    g.state.position.robber = hex;
    let victims = robber_victims(g.board.graph, &g.state.position.buildings, active, hex);
    g.state.position.phase = Phase::RobberVictim {
        origin: RobberOrigin::Knight,
        continuation: ResumePhase::PreRoll,
        victims,
    };
    validate_game(&g.board, &g.state).unwrap();
    g.state.position.phase = Phase::RobberVictim {
        origin: RobberOrigin::Knight,
        continuation: ResumePhase::PreRoll,
        victims: [None; 4],
    };
    assert!(validate_game(&g.board, &g.state).is_err());
    g.state.position.phase = Phase::RobberMove {
        origin: RobberOrigin::Seven,
        continuation: ResumePhase::PreRoll,
    };
    assert!(validate_game(&g.board, &g.state).is_err());
    g.state.position.phase = Phase::PreRoll;
    g.state.revision = u64::MAX;
    validate_game(&g.board, &g.state).unwrap();
    g.state.position.turn = u64::MAX;
    assert!(validate_game(&g.board, &g.state).is_err());

    let mut g = create_game(GameConfig::fixed(&[0, 1, 2]).unwrap(), [0; 32]);
    let winner = g.state.position.active_player;
    for b in g.state.position.buildings.iter_mut().flatten() {
        if b.owner == winner {
            b.kind = BuildingKind::City;
        }
    }
    for id in 0..3 {
        draw(
            &mut g.state.position,
            id,
            CardLocation::PlayedKnight { owner: winner },
        );
    }
    g.state.position.largest_army_holder = Some(winner);
    for id in 20..24 {
        draw(
            &mut g.state.position,
            id,
            CardLocation::Held {
                owner: winner,
                acquired_turn: 1,
            },
        );
    }
    g.state.position.last_dice = Some([1, 1]);
    g.state.position.phase = Phase::Action;
    g.state.position.turn = 4;
    g.state.revision = 7;
    assert_eq!(current_score(&g.state.position, winner), 10);
    assert!(validate_game(&g.board, &g.state).is_err());
    // The same ten points remain legal while their owner waits off-turn.
    let seats = g.state.position.config.seat_order();
    let at = seats.iter().position(|&s| s == winner).unwrap();
    let next = seats[(at + 1) % 3];
    g.state.position.active_player = next;
    g.state.position.turn = 2;
    g.state.revision = 3;
    validate_game(&g.board, &g.state).unwrap();
    g.state.position.active_player = winner;
    g.state.position.turn = 4;
    g.state.revision = 7;
    for id in 20..24 {
        g.state.position.card_locations[id] = CardLocation::RevealedVictory { owner: winner };
    }
    g.state.position.phase = Phase::Finished { winner, score: 10 };
    g.state.position.final_scores = Some(std::array::from_fn(|id| {
        if id < 3 {
            current_score(&g.state.position, seat(id))
        } else {
            0
        }
    }));
    validate_game(&g.board, &g.state).unwrap();
    g.state.position.final_scores.as_mut().unwrap()[3] = 1;
    assert!(validate_game(&g.board, &g.state).is_err());
    g.state.position.final_scores.as_mut().unwrap()[3] = 0;
    g.state.position.card_locations[20] = CardLocation::RevealedVictory { owner: next };
    assert!(validate_game(&g.board, &g.state).is_err());
}

#[test]
fn invariant_spent_kinds_and_qualifying_route_tie_with_no_holder() {
    for id in [0, 13, 20, 24] {
        let mut g = create_game(GameConfig::fixed(&[0, 1, 2]).unwrap(), [0; 32]);
        let active = g.state.position.active_player;
        draw(
            &mut g.state.position,
            id,
            CardLocation::Spent { owner: active },
        );
        assert_eq!(
            validate_game(&g.board, &g.state),
            Err(InvalidState),
            "invalid spent card {id}"
        );
    }
    let mut g = create_game(GameConfig::fixed(&[0, 1, 2]).unwrap(), [0; 32]);
    // Independent endpoint literal witnesses: 0–3–7–11–16–21 and
    // 42–46–50–53–49–52. Both trails contain exactly five distinct edges.
    for id in [0, 6, 10, 18, 23] {
        g.state.position.roads[id] = Some(seat(2));
    }
    for id in [61, 65, 71, 70, 69] {
        g.state.position.roads[id] = Some(seat(0));
    }
    g.state.position.road_lengths = [5, 1, 5, 0];
    g.state.position.longest_route_holder = None;
    assert_eq!(g.state.position.active_player, seat(2));
    validate_game(&g.board, &g.state).unwrap();
    for holder in [seat(0), seat(2)] {
        g.state.position.longest_route_holder = Some(holder);
        validate_game(&g.board, &g.state).unwrap();
    }
    g.state.position.longest_route_holder = Some(seat(1));
    assert!(validate_game(&g.board, &g.state).is_err());
}

#[test]
fn invariant_setup_rejects_two_roads_paired_to_the_same_settlement() {
    let mut g = create_game(GameConfig::variable(&[0, 1, 2]).unwrap(), [0; 32]);
    loop {
        let completed = g
            .state
            .position
            .second_settlements
            .iter()
            .enumerate()
            .find_map(|(id, &v)| {
                v.filter(|_| {
                    g.state
                        .position
                        .roads
                        .iter()
                        .filter(|&&owner| owner == Some(seat(id)))
                        .count()
                        == 2
                })
                .map(|v| (seat(id), v))
            });
        if let Some((owner, second)) = completed {
            assert!(matches!(
                g.state.position.phase,
                Phase::SetupSettlement { .. }
            ));
            validate_game(&g.board, &g.state).unwrap();
            let first = g
                .state
                .position
                .buildings
                .iter()
                .enumerate()
                .find_map(|(id, &v)| {
                    (id != second.index() && v.is_some_and(|b| b.owner == owner))
                        .then(|| VertexId::try_from(id as u8).unwrap())
                })
                .unwrap();
            let old = *g
                .board
                .graph
                .vertex_edges(second)
                .iter()
                .find(|&&edge| g.state.position.roads[edge.index()] == Some(owner))
                .unwrap();
            let new = *g
                .board
                .graph
                .vertex_edges(first)
                .iter()
                .find(|&&edge| g.state.position.roads[edge.index()].is_none())
                .unwrap();
            g.state.position.roads[old.index()] = None;
            g.state.position.roads[new.index()] = Some(owner);
            g.state.position.road_lengths = std::array::from_fn(|id| {
                longest_route_length(
                    g.board.graph,
                    &g.state.position.roads,
                    &g.state.position.buildings,
                    seat(id),
                )
            });
            assert!(
                validate_game(&g.board, &g.state).is_err(),
                "validator accepted two setup roads at first settlement and none at second"
            );
            break;
        }
        let command = match g.state.position.phase {
            Phase::SetupSettlement { .. } => Command::PlaceSetupSettlement {
                vertex: (0..54)
                    .map(|i| VertexId::try_from(i).unwrap())
                    .find(|&v| {
                        validate_setup_settlement(&g.board, &g.state.position.buildings, v).is_ok()
                    })
                    .unwrap(),
            },
            Phase::SetupRoad { settlement, .. } => Command::PlaceSetupRoad {
                edge: *g
                    .board
                    .graph
                    .vertex_edges(settlement)
                    .iter()
                    .find(|&&e| {
                        validate_setup_road(&g.board, &g.state.position.roads, settlement, e)
                            .is_ok()
                    })
                    .unwrap(),
            },
            _ => panic!("setup ended before a second pair was complete"),
        };
        apply(&g.board, &mut g.state, command);
    }
}
