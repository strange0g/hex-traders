//! Creation and explicitly seeded randomness.

use rand::{
    SeedableRng,
    distr::{Distribution, Uniform},
    seq::SliceRandom,
};
use rand_chacha::ChaCha12Rng;

use crate::{
    base_rules::{
        CANONICAL_FRAMES, CANONICAL_TERRAIN, fixed_setup, longest_route_length, starting_resources,
        variable_setup,
    },
    game_model::{
        Board, CardId, CreationEvent, EdgeId, GameConfig, GameState, ModelError, Phase, PlayerId,
        Position, SetupMode, Terrain, VertexId,
    },
};

pub(crate) struct CreatedGame {
    pub board: Board,
    pub state: GameState,
    pub events: Vec<CreationEvent>,
}

pub(crate) fn create_game(config: GameConfig, seed: [u8; 32]) -> CreatedGame {
    let mut random = ChaCha12Rng::from_seed(seed);
    let setup = match config.setup() {
        SetupMode::Fixed => fixed_setup(&config),
        SetupMode::Variable => {
            let mut terrain = CANONICAL_TERRAIN;
            terrain.shuffle(&mut random);
            let mut frames = CANONICAL_FRAMES;
            frames.shuffle(&mut random);
            variable_setup(terrain, frames)
        }
    };
    let mut deck =
        std::array::from_fn(|index| CardId::try_from(index as u8).expect("canonical card ID"));
    // rand's shuffle calls random_range internally; the pinned unbiased feature is required.
    // https://docs.rs/rand/0.10.3/src/rand/seq/increasing_uniform.rs.html
    deck.shuffle(&mut random);
    let mut events = vec![CreationEvent::GameCreated {
        setup: config.setup(),
        seat_order: config.seat_order().to_vec(),
    }];
    if config.setup() == SetupMode::Fixed {
        for &seat in config.seat_order() {
            for (index, building) in setup.buildings.iter().enumerate() {
                if building.is_some_and(|building| building.owner == seat) {
                    events.push(CreationEvent::SettlementPlaced {
                        seat,
                        vertex: VertexId::try_from(index as u8).expect("bounded vertex"),
                    });
                }
            }
            for (index, &owner) in setup.roads.iter().enumerate() {
                if owner == Some(seat) {
                    events.push(CreationEvent::RoadPlaced {
                        seat,
                        edge: EdgeId::try_from(index as u8).expect("bounded edge"),
                    });
                }
            }
        }
        for &seat in config.seat_order() {
            events.push(CreationEvent::StartingResources {
                seat,
                resources: setup.hands[seat.index()],
            });
        }
    }
    let starting_player = select_starter(config.seat_order(), &mut random, &mut events);
    let (phase, turn) = match config.setup() {
        SetupMode::Fixed => (Phase::PreRoll, 1),
        SetupMode::Variable => (
            Phase::SetupSettlement {
                actor: starting_player,
                round: 1,
            },
            0,
        ),
    };
    let mut state = GameState {
        position: Position {
            road_lengths: std::array::from_fn(|id| {
                longest_route_length(
                    setup.board.graph,
                    &setup.roads,
                    &setup.buildings,
                    PlayerId::try_from(id as u8).expect("bounded seat"),
                )
            }),
            longest_route_holder: None,
            largest_army_holder: None,
            config,
            roads: setup.roads,
            buildings: setup.buildings,
            second_settlements: setup.second_settlements,
            robber: setup.robber,
            bank: setup.bank,
            hands: setup.hands,
            offer: None,
            deck,
            draw_cursor: 0,
            card_locations: [crate::game_model::CardLocation::Undrawn; 25],
            played_non_vp: false,
            final_scores: None,
            starting_player,
            active_player: starting_player,
            turn,
            phase,
            last_dice: None,
        },
        random,
        revision: 0,
    };
    state.position.longest_route_holder =
        crate::base_rules::reconcile_longest_route(&state.position.road_lengths, None);
    events.push(match state.position.config.setup() {
        SetupMode::Fixed => CreationEvent::TurnStarted {
            seat: starting_player,
            turn,
        },
        SetupMode::Variable => CreationEvent::SetupStarted { phase },
    });
    CreatedGame {
        board: setup.board,
        state,
        events,
    }
}

pub(super) fn roll_dice(random: &mut ChaCha12Rng) -> [u8; 2] {
    // Uniform::sample uses unbiased rejection sampling even with default features disabled.
    // https://docs.rs/rand/0.10.3/src/rand/distr/uniform_int.rs.html
    let die = Uniform::new_inclusive(1_u8, 6).expect("nonempty constant die range");
    [die.sample(random), die.sample(random)]
}

fn select_starter(
    seats: &[PlayerId],
    random: &mut ChaCha12Rng,
    events: &mut Vec<CreationEvent>,
) -> PlayerId {
    let mut contenders = seats.to_vec();
    loop {
        let mut highest = 0;
        let mut leaders = Vec::new();
        for &seat in &contenders {
            let dice = roll_dice(random);
            events.push(CreationEvent::StarterRolled { seat, dice });
            let sum = dice[0] + dice[1];
            if sum > highest {
                highest = sum;
                leaders.clear();
            }
            if sum == highest {
                leaders.push(seat);
            }
        }
        if leaders.len() == 1 {
            return leaders[0];
        }
        contenders = leaders;
    }
}

pub(crate) fn validate_created_game(board: &Board, state: &GameState) -> Result<(), ModelError> {
    let p = &state.position;
    let invalid = ModelError::InvalidInitialState;
    super::validate_game(board, state).map_err(|_| invalid)?;
    if state.revision != 0
        || p.last_dice.is_some()
        || p.starting_player != p.active_player
        || board.terrain[p.robber.index()] != Terrain::Desert
        || p.draw_cursor != 0
        || p.played_non_vp
        || p.final_scores.is_some()
        || p.offer.is_some()
        || p.largest_army_holder.is_some()
    {
        return Err(invalid);
    }
    let (phase, turn, count) = match p.config.setup() {
        SetupMode::Fixed => {
            for &seat in p.config.seat_order() {
                let second = p.second_settlements[seat.index()].ok_or(invalid)?;
                if p.hands[seat.index()] != starting_resources(board, second) {
                    return Err(invalid);
                }
            }
            (Phase::PreRoll, 1, 2)
        }
        SetupMode::Variable => (
            Phase::SetupSettlement {
                actor: p.starting_player,
                round: 1,
            },
            0,
            0,
        ),
    };
    if p.phase != phase || p.turn != turn {
        return Err(invalid);
    }
    for &seat in p.config.seat_order() {
        if p.roads.iter().filter(|&&s| s == Some(seat)).count() != count
            || p.buildings
                .iter()
                .flatten()
                .filter(|b| b.owner == seat)
                .count()
                != count
        {
            return Err(invalid);
        }
    }
    if p.buildings
        .iter()
        .flatten()
        .any(|b| b.kind != crate::game_model::BuildingKind::Settlement)
    {
        return Err(invalid);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_model::{Building, DevelopmentKind, GameConfig, Phase, PortKind, Resources};

    #[test]
    fn creation_rejects_valid_count_wrong_fixed_token_layout() {
        let mut game = create_game(GameConfig::fixed(&[0, 1, 2]).unwrap(), [0; 32]);
        game.board.numbers.swap(1, 2);
        assert_eq!(
            validate_created_game(&game.board, &game.state),
            Err(ModelError::InvalidInitialState)
        );
    }

    #[test]
    fn creation_second_settlement_facts_match_source_and_reject_corrupt_grants() {
        for order in [&[2, 0, 1][..], &[2, 0, 3, 1][..]] {
            let fixed = create_game(GameConfig::fixed(order).unwrap(), [0; 32]);
            for (&seat, vertex) in fixed
                .state
                .position
                .config
                .seat_order()
                .iter()
                .zip([40, 9, 17, 29])
            {
                assert_eq!(
                    fixed.state.position.second_settlements[seat.index()],
                    Some(VertexId::try_from(vertex).unwrap())
                );
            }
            validate_created_game(&fixed.board, &fixed.state).unwrap();
            let mut empty = create_game(GameConfig::variable(order).unwrap(), [0; 32]);
            assert_eq!(empty.state.position.second_settlements, [None; 4]);
            empty.state.position.second_settlements[0] = Some(VertexId::try_from(0).unwrap());
            assert_eq!(
                validate_created_game(&empty.board, &empty.state),
                Err(ModelError::InvalidInitialState)
            );
        }
        let mutations: [fn(&mut CreatedGame); 8] = [
            |game| game.state.position.second_settlements[2] = None,
            |game| game.state.position.second_settlements[2] = Some(VertexId::try_from(0).unwrap()),
            |game| game.state.position.second_settlements[2] = Some(VertexId::try_from(9).unwrap()),
            |game| {
                game.state.position.second_settlements[2] = Some(VertexId::try_from(31).unwrap())
            },
            |game| game.state.position.second_settlements[3] = Some(VertexId::try_from(9).unwrap()),
            |game| {
                game.state.position.buildings[40].as_mut().unwrap().owner =
                    PlayerId::try_from(0).unwrap()
            },
            |game| {
                game.state.position.hands[2] = Resources::try_from([0, 1, 0, 1, 1]).unwrap();
                game.state.position.bank = Resources::try_from([18, 17, 17, 16, 18]).unwrap();
            },
            |game| {
                game.state.position.buildings[31].as_mut().unwrap().kind =
                    crate::game_model::BuildingKind::City
            },
        ];
        for (index, mutate) in mutations.iter().enumerate() {
            let mut game = create_game(GameConfig::fixed(&[2, 0, 1]).unwrap(), [0; 32]);
            mutate(&mut game);
            assert_eq!(
                validate_created_game(&game.board, &game.state),
                Err(ModelError::InvalidInitialState),
                "second settlement corruption {index}"
            );
        }
    }

    #[test]
    fn fixed_creation_complete_position_has_initial_stocks_deck_and_phase() {
        for order in [&[2, 0, 1][..], &[2, 0, 3, 1][..]] {
            let created = create_game(GameConfig::fixed(order).unwrap(), [0; 32]);
            let position = &created.state.position;
            assert_eq!(created.state.revision, 0);
            assert_eq!(position.turn, 1);
            assert_eq!(position.phase, Phase::PreRoll);
            assert_eq!(position.last_dice, None);
            assert_eq!(position.active_player, position.starting_player);
            let mut deck = position.deck.map(|card| card.index());
            deck.sort_unstable();
            assert_eq!(deck, std::array::from_fn(|i| i));
            let mut kinds = [0; 5];
            for card in position.deck {
                kinds[card.kind() as usize] += 1;
            }
            assert_eq!(kinds[DevelopmentKind::Knight as usize], 14);
            assert_eq!(kinds, [14, 2, 2, 2, 5]);
            for &seat in position.config.seat_order() {
                let roads = position
                    .roads
                    .iter()
                    .filter(|&&owner| owner == Some(seat))
                    .count();
                let buildings = position
                    .buildings
                    .iter()
                    .flatten()
                    .filter(|b| b.owner == seat)
                    .count();
                assert_eq!((15 - roads, 5 - buildings, 4), (13, 3, 4));
                assert_eq!(buildings, 2);
            }
            assert_eq!(created.state.random.get_seed(), [0; 32]);
            assert_eq!(created.state.random.get_stream(), 0);
            validate_created_game(&created.board, &created.state).unwrap();
        }
    }

    #[test]
    fn fixed_creation_rng_matches_independent_unbiased_golden_traces() {
        struct Golden {
            seed: u8,
            seats: &'static [u8],
            rolls: &'static [(usize, [u8; 2])],
            starter: usize,
            word_position: u128,
            next: [u8; 2],
        }
        // Independently recorded using the pinned rand APIs; tiers were hand-traced.
        let cases = [
            Golden {
                seed: 6,
                seats: &[2, 0, 1],
                rolls: &[
                    (2, [1, 6]),
                    (0, [2, 1]),
                    (1, [6, 1]),
                    (2, [3, 3]),
                    (1, [1, 1]),
                ],
                starter: 2,
                word_position: 14,
                next: [4, 1],
            },
            Golden {
                seed: 6,
                seats: &[2, 0, 3, 1],
                rolls: &[
                    (2, [1, 6]),
                    (0, [2, 1]),
                    (3, [6, 1]),
                    (1, [3, 3]),
                    (2, [1, 1]),
                    (3, [4, 1]),
                ],
                starter: 3,
                word_position: 16,
                next: [3, 4],
            },
            Golden {
                seed: 21,
                seats: &[2, 0, 1],
                rolls: &[
                    (2, [4, 6]),
                    (0, [1, 4]),
                    (1, [6, 4]),
                    (2, [2, 5]),
                    (1, [3, 4]),
                    (2, [3, 4]),
                    (1, [6, 5]),
                ],
                starter: 1,
                word_position: 18,
                next: [6, 2],
            },
            Golden {
                seed: 21,
                seats: &[2, 0, 3, 1],
                rolls: &[
                    (2, [4, 6]),
                    (0, [1, 4]),
                    (3, [6, 4]),
                    (1, [2, 5]),
                    (2, [3, 4]),
                    (3, [3, 4]),
                    (2, [6, 5]),
                    (3, [6, 2]),
                ],
                starter: 2,
                word_position: 20,
                next: [5, 2],
            },
        ];
        for case in cases {
            let mut seed = [0; 32];
            seed[0] = case.seed;
            let created = create_game(GameConfig::fixed(case.seats).unwrap(), seed);
            let expected_deck = if case.seed == 6 {
                [
                    12, 0, 4, 5, 17, 8, 11, 24, 19, 6, 20, 18, 16, 21, 1, 3, 23, 10, 22, 13, 2, 14,
                    15, 9, 7,
                ]
            } else {
                [
                    23, 8, 16, 17, 10, 7, 9, 18, 1, 6, 22, 15, 2, 24, 13, 20, 3, 11, 4, 12, 19, 21,
                    5, 0, 14,
                ]
            };
            assert_eq!(
                created.state.position.deck.map(|card| card.index()),
                expected_deck
            );
            let rolls: Vec<_> = created
                .events
                .iter()
                .filter_map(|event| match event {
                    CreationEvent::StarterRolled { seat, dice } => Some((seat.index(), *dice)),
                    _ => None,
                })
                .collect();
            assert_eq!(rolls, case.rolls);
            assert_eq!(created.state.position.starting_player.index(), case.starter);
            assert_eq!(created.state.random.get_word_pos(), case.word_position);
            let mut next = created.state.random.clone();
            assert_eq!(roll_dice(&mut next), case.next);
            assert_eq!(next.get_word_pos(), case.word_position + 2);
            assert_eq!(created.state.random.get_word_pos(), case.word_position);
        }
    }

    #[test]
    fn fixed_creation_invariants_reject_corrupt_initial_game_facts() {
        let mutations: [fn(&mut CreatedGame); 27] = [
            |game| game.state.revision = 1,
            |game| game.state.position.turn = 0,
            |game| game.state.position.last_dice = Some([3, 4]),
            |game| {
                game.state.position.active_player = PlayerId::try_from(
                    ((game.state.position.starting_player.index() + 1) % 3) as u8,
                )
                .unwrap()
            },
            |game| {
                game.state.position.active_player = PlayerId::try_from(3).unwrap();
                game.state.position.starting_player = PlayerId::try_from(3).unwrap();
            },
            |game| game.state.random.set_stream(1),
            |game| game.state.position.robber = crate::game_model::HexId::try_from(1).unwrap(),
            |game| game.state.position.bank = Resources::default(),
            |game| game.board.terrain[1] = Terrain::Forest,
            |game| game.board.numbers[0] = Some(2),
            |game| game.board.numbers[1] = Some(7),
            |game| game.board.numbers[1] = Some(12),
            |game| game.board.ports[0].edge = EdgeId::try_from(7).unwrap(),
            |game| game.board.ports[0].edge = game.board.ports[1].edge,
            |game| game.board.ports[0].kind = PortKind::Special(crate::Resource::Ore),
            |game| game.state.position.deck[0] = game.state.position.deck[1],
            |game| game.state.position.draw_cursor = 1,
            |game| {
                game.state.position.card_locations[0] = crate::game_model::CardLocation::Held {
                    owner: PlayerId::try_from(0).unwrap(),
                    acquired_turn: 1,
                }
            },
            |game| {
                game.state.position.offer = Some(crate::game_model::TradeOffer {
                    id: 1,
                    turn: 1,
                    proposer: PlayerId::try_from(0).unwrap(),
                    recipient: PlayerId::try_from(1).unwrap(),
                    give: Resources::try_from([1, 0, 0, 0, 0]).unwrap(),
                    receive: Resources::try_from([0, 1, 0, 0, 0]).unwrap(),
                })
            },
            |game| game.state.position.final_scores = Some([0; 4]),
            |game| game.state.position.played_non_vp = true,
            |game| game.state.position.roads[46] = Some(PlayerId::try_from(3).unwrap()),
            |game| {
                game.state.position.buildings[31] = Some(Building {
                    kind: crate::game_model::BuildingKind::Settlement,
                    owner: PlayerId::try_from(3).unwrap(),
                })
            },
            |game| {
                game.state.position.buildings[0] = Some(Building {
                    kind: crate::game_model::BuildingKind::Settlement,
                    owner: PlayerId::try_from(0).unwrap(),
                });
                game.state.position.buildings[4] = game.state.position.buildings[0];
            },
            |game| game.state.position.roads[0] = Some(PlayerId::try_from(0).unwrap()),
            |game| {
                game.state.position.roads[46] = None;
                game.state.position.buildings[31] = None;
            },
            |game| {
                game.state.position.hands[3] = Resources::try_from([1, 0, 0, 0, 0]).unwrap();
                game.state.position.bank = Resources::try_from([16, 18, 17, 16, 18]).unwrap();
            },
        ];
        for (index, mutate) in mutations.iter().enumerate() {
            let mut game = create_game(GameConfig::fixed(&[2, 0, 1]).unwrap(), [0; 32]);
            mutate(&mut game);
            assert_eq!(
                validate_created_game(&game.board, &game.state),
                Err(ModelError::InvalidInitialState),
                "corruption {index}"
            );
        }
    }

    #[test]
    fn variable_creation_rng_matches_all_eight_independent_layout_and_tie_goldens() {
        // Independent pinned-RNG oracle and hand-traced tiers; retained literals, no engine imports.
        let goldens: serde_json::Value = serde_json::from_str(r#"{
  "layouts": {
    "6": {
      "terrain": ["field","desert","pasture","forest","field","field","pasture","forest","hills","mountain","pasture","pasture","mountain","mountain","field","hills","hills","forest","forest"],
      "frames": [4,5,2,3,1,6],
      "ports": [[2,"wood"],[5,"generic"],[22,"wheat"],[53,"generic"],[71,"generic"],[68,"brick"],[66,"generic"],[49,"wool"],[18,"ore"]],
      "numbers": [2,null,5,6,9,10,8,3,4,11,3,4,8,5,6,11,10,9,12],
      "robber": 1,
      "deck": [2,11,0,6,10,4,19,13,7,21,20,8,14,16,17,15,22,23,24,12,1,5,9,18,3]
    },
    "21": {
      "terrain": ["desert","field","mountain","field","forest","forest","hills","pasture","field","mountain","forest","forest","pasture","mountain","hills","field","hills","pasture","pasture"],
      "frames": [5,1,2,3,6,4],
      "ports": [[0,"generic"],[3,"wheat"],[5,"generic"],[22,"wool"],[53,"generic"],[71,"generic"],[68,"brick"],[54,"ore"],[18,"wood"]],
      "numbers": [null,2,5,6,9,10,8,3,4,11,3,4,8,5,6,11,10,9,12],
      "robber": 0,
      "deck": [17,10,3,15,7,22,1,20,21,14,11,18,5,16,12,6,0,8,4,2,24,13,19,23,9]
    },
    "8": {
      "terrain": ["pasture","field","forest","hills","mountain","forest","field","forest","forest","pasture","field","hills","pasture","mountain","desert","mountain","pasture","field","hills"],
      "frames": [4,6,5,1,3,2],
      "ports": [[2,"wood"],[17,"ore"],[38,"generic"],[61,"wheat"],[71,"generic"],[68,"wool"],[66,"generic"],[49,"brick"],[18,"generic"]],
      "numbers": [6,2,5,3,4,9,10,8,5,11,3,8,10,6,null,4,9,12,11],
      "robber": 14,
      "deck": [20,16,8,18,9,7,6,4,10,23,2,12,11,19,22,0,3,14,1,21,15,5,17,24,13]
    },
    "12": {
      "terrain": ["hills","pasture","field","forest","mountain","pasture","desert","hills","forest","field","forest","mountain","pasture","forest","field","field","pasture","hills","mountain"],
      "frames": [3,5,6,1,2,4],
      "ports": [[0,"generic"],[3,"brick"],[5,"generic"],[22,"wheat"],[53,"ore"],[71,"generic"],[68,"wool"],[54,"generic"],[18,"wood"]],
      "numbers": [6,2,5,3,9,10,null,8,4,11,3,8,10,5,6,4,9,12,11],
      "robber": 6,
      "deck": [10,6,19,7,11,2,24,0,16,4,23,20,14,3,9,18,22,13,21,12,8,1,5,15,17]
    },
    "17": {
      "terrain": ["mountain","field","field","pasture","forest","mountain","pasture","mountain","desert","forest","hills","forest","hills","field","forest","hills","pasture","field","pasture"],
      "frames": [3,2,6,5,1,4],
      "ports": [[0,"generic"],[3,"brick"],[17,"generic"],[53,"ore"],[71,"generic"],[68,"wheat"],[66,"generic"],[49,"wool"],[18,"wood"]],
      "numbers": [6,2,5,3,4,9,10,8,null,11,3,8,10,5,6,4,9,12,11],
      "robber": 8,
      "deck": [22,21,13,7,20,5,17,6,2,18,19,4,12,23,11,10,9,1,24,16,8,0,15,14,3]
    }
  },
  "cases": [
    {"seed":6,"seats":[2,0,1],"rounds":[[[2,[6,1]],[0,[3,3]],[1,[1,1]]]],"starter":2,"position":14,"next":[4,1]},
    {"seed":6,"seats":[2,0,3,1],"rounds":[[[2,[6,1]],[0,[3,3]],[3,[1,1]],[1,[4,1]]]],"starter":2,"position":16,"next":[3,4]},
    {"seed":21,"seats":[2,0,1],"rounds":[[[2,[4,6]],[0,[4,2]],[1,[5,3]]]],"starter":2,"position":13,"next":[4,3]},
    {"seed":21,"seats":[2,0,3,1],"rounds":[[[2,[4,6]],[0,[4,2]],[3,[5,3]],[1,[4,3]]]],"starter":2,"position":15,"next":[4,6]},
    {"seed":8,"seats":[2,0,1],"rounds":[[[2,[1,5]],[0,[2,4]],[1,[1,4]]],[[2,[1,4]],[0,[2,1]]]],"starter":2,"position":18,"next":[2,6]},
    {"seed":8,"seats":[2,0,3,1],"rounds":[[[2,[1,5]],[0,[2,4]],[3,[1,4]],[1,[1,4]]],[[2,[2,1]],[0,[2,6]]]],"starter":0,"position":20,"next":[5,6]},
    {"seed":12,"seats":[2,0,1],"rounds":[[[2,[5,3]],[0,[2,6]],[1,[1,6]]],[[2,[5,1]],[0,[3,3]]],[[2,[2,2]],[0,[4,6]]]],"starter":0,"position":21,"next":[4,6]},
    {"seed":17,"seats":[2,0,3,1],"rounds":[[[2,[1,6]],[0,[5,6]],[3,[3,4]],[1,[5,6]]],[[0,[6,5]],[1,[5,6]]],[[0,[1,5]],[1,[3,2]]]],"starter":0,"position":24,"next":[5,6]}
  ]
}"#).unwrap();
        for case in goldens["cases"].as_array().unwrap() {
            let mut seed = [0; 32];
            seed[0] = case["seed"].as_u64().unwrap() as u8;
            let order: Vec<_> = case["seats"]
                .as_array()
                .unwrap()
                .iter()
                .map(|seat| seat.as_u64().unwrap() as u8)
                .collect();
            let created = create_game(GameConfig::variable(&order).unwrap(), seed);
            let expected = &goldens["layouts"][seed[0].to_string()];
            let terrain = created.board.terrain.map(|terrain| match terrain {
                Terrain::Hills => "hills",
                Terrain::Forest => "forest",
                Terrain::Pasture => "pasture",
                Terrain::Field => "field",
                Terrain::Mountain => "mountain",
                Terrain::Desert => "desert",
            });
            assert_eq!(serde_json::json!(terrain), expected["terrain"]);
            assert_eq!(
                serde_json::json!(created.board.frame_permutation),
                expected["frames"]
            );
            let ports = created.board.ports.map(|port| {
                let kind = match port.kind {
                    PortKind::Generic => "generic",
                    PortKind::Special(resource) => {
                        ["brick", "wood", "wool", "wheat", "ore"][resource.index()]
                    }
                };
                (port.edge.index(), kind)
            });
            assert_eq!(serde_json::json!(ports), expected["ports"]);
            assert_eq!(
                serde_json::json!(created.board.numbers),
                expected["numbers"]
            );
            let position = &created.state.position;
            assert_eq!(
                serde_json::json!(position.robber.index()),
                expected["robber"]
            );
            assert_eq!(
                serde_json::json!(position.deck.map(|card| card.index())),
                expected["deck"]
            );
            let rolls: Vec<_> = created
                .events
                .iter()
                .filter_map(|event| match event {
                    CreationEvent::StarterRolled { seat, dice } => {
                        Some(serde_json::json!([seat.index(), dice]))
                    }
                    _ => None,
                })
                .collect();
            let expected_rolls: Vec<_> = case["rounds"]
                .as_array()
                .unwrap()
                .iter()
                .flat_map(|round| round.as_array().unwrap().iter().cloned())
                .collect();
            assert_eq!(rolls, expected_rolls);
            assert_eq!(created.events.len(), rolls.len() + 2);
            assert_eq!(
                serde_json::json!(position.starting_player.index()),
                case["starter"]
            );
            assert_eq!(position.active_player, position.starting_player);
            let phase = Phase::SetupSettlement {
                actor: position.starting_player,
                round: 1,
            };
            assert_eq!(position.phase, phase);
            assert_eq!(
                created.events.last(),
                Some(&CreationEvent::SetupStarted { phase })
            );
            assert_eq!(position.turn, 0);
            assert_eq!(created.state.revision, 0);
            assert_eq!(position.last_dice, None);
            assert_eq!(position.bank.counts(), &[19; 5]);
            assert!(position.hands.iter().all(|hand| hand.counts() == &[0; 5]));
            assert!(position.roads.iter().all(Option::is_none));
            assert!(position.buildings.iter().all(Option::is_none));
            let word_position = u128::from(case["position"].as_u64().unwrap());
            assert_eq!(created.state.random.get_word_pos(), word_position);
            let mut next = created.state.random.clone();
            assert_eq!(serde_json::json!(roll_dice(&mut next)), case["next"]);
            assert_eq!(next.get_word_pos(), word_position + 2);
            assert_eq!(created.state.random.get_word_pos(), word_position);
            validate_created_game(&created.board, &created.state).unwrap();
        }
    }

    #[test]
    fn variable_creation_invariants_reject_wrong_setup_payload_and_source_identity() {
        let mutations: [fn(&mut CreatedGame); 11] = [
            |game| game.state.position.phase = Phase::PreRoll,
            |game| {
                game.state.position.phase = Phase::SetupSettlement {
                    actor: PlayerId::try_from(1).unwrap(),
                    round: 1,
                }
            },
            |game| {
                game.state.position.phase = Phase::SetupSettlement {
                    actor: game.state.position.starting_player,
                    round: 2,
                }
            },
            |game| game.state.position.turn = 1,
            |game| {
                game.state.position.hands[0] = Resources::try_from([1, 0, 0, 0, 0]).unwrap();
                game.state.position.bank = Resources::try_from([18, 19, 19, 19, 19]).unwrap();
            },
            |game| {
                game.state.position.buildings[0] = Some(Building {
                    kind: crate::game_model::BuildingKind::Settlement,
                    owner: PlayerId::try_from(0).unwrap(),
                })
            },
            |game| game.board.frame_permutation[0] = 0,
            |game| game.board.frame_permutation[0] = game.board.frame_permutation[1],
            |game| game.board.frame_permutation.swap(0, 1),
            |game| game.board.numbers.swap(0, 2),
            |game| game.board.ports[0].kind = PortKind::Generic,
        ];
        for (index, mutate) in mutations.iter().enumerate() {
            let mut seed = [0; 32];
            seed[0] = 6;
            let mut game = create_game(GameConfig::variable(&[2, 0, 1]).unwrap(), seed);
            mutate(&mut game);
            assert_eq!(
                validate_created_game(&game.board, &game.state),
                Err(ModelError::InvalidInitialState),
                "variable corruption {index}"
            );
        }
    }
}
