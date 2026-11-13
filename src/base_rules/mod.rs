//! Pure source-verified setup calculations.

use crate::game_model::{
    Board, BoardGraph, Building, EdgeId, GameConfig, HexId, PlayerId, Port, PortKind, Position,
    Resource, Resources, RuleError, Terrain, VertexId,
};

// Version 1 shuffle inputs follow graph IDs and sorted printed template IDs.
pub(crate) const CANONICAL_TERRAIN: [Terrain; HexId::COUNT] = {
    use Terrain::{Desert, Field, Forest, Hills, Mountain, Pasture};
    [
        Desert, Field, Pasture, Hills, Pasture, Forest, Mountain, Forest, Field, Hills, Forest,
        Pasture, Field, Hills, Mountain, Forest, Mountain, Field, Pasture,
    ]
};
pub(crate) const CANONICAL_FRAMES: [u8; 6] = [1, 2, 3, 4, 5, 6];
// First settlement/road, then designated second settlement/road, in source role order.
pub(crate) const FIXED_PRESETS: [(usize, usize, usize, usize); 4] = [
    (31, 46, 40, 57),
    (39, 55, 9, 14),
    (19, 21, 17, 25),
    (41, 59, 29, 42),
];

pub(crate) struct InitialSetup {
    pub board: Board,
    pub roads: [Option<PlayerId>; EdgeId::COUNT],
    pub buildings: [Option<Building>; VertexId::COUNT],
    pub second_settlements: [Option<VertexId>; 4],
    pub robber: HexId,
    pub bank: Resources,
    pub hands: [Resources; 4],
}

pub(crate) fn fixed_setup(config: &GameConfig) -> InitialSetup {
    let graph = BoardGraph::base_game();
    // Source: CN3081 v6.250401 pp.4–5; independently captured in tests/support/setup.json.
    let terrain = CANONICAL_TERRAIN;
    let numbers = [
        None,
        Some(8),
        Some(11),
        Some(6),
        Some(3),
        Some(4),
        Some(9),
        Some(10),
        Some(5),
        Some(12),
        Some(11),
        Some(5),
        Some(2),
        Some(9),
        Some(4),
        Some(8),
        Some(6),
        Some(3),
        Some(10),
    ];
    let board = Board {
        graph,
        terrain,
        numbers,
        ports: frame_ports(CANONICAL_FRAMES),
        frame_permutation: CANONICAL_FRAMES,
    };
    let mut roads = [None; EdgeId::COUNT];
    let mut buildings = [None; VertexId::COUNT];
    let mut second_settlements = [None; 4];
    let mut hands = [Resources::default(); 4];
    for (&owner, (first, first_road, second, second_road)) in
        config.seat_order().iter().zip(FIXED_PRESETS)
    {
        for id in [first, second] {
            buildings[id] = Some(Building {
                owner,
                kind: crate::game_model::BuildingKind::Settlement,
            });
        }
        for id in [first_road, second_road] {
            roads[id] = Some(owner);
        }
        let vertex = VertexId::try_from(second as u8).expect("source settlement ID");
        second_settlements[owner.index()] = Some(vertex);
        hands[owner.index()] = starting_resources(&board, vertex);
    }
    let bank = std::array::from_fn(|resource| {
        let distributed: u16 = hands
            .iter()
            .map(|hand| u16::from(hand.counts()[resource]))
            .sum();
        u8::try_from(
            19_u16
                .checked_sub(distributed)
                .expect("source bank covers starting hands"),
        )
        .expect("bank fits u8")
    });
    InitialSetup {
        board,
        roads,
        buildings,
        second_settlements,
        robber: HexId::try_from(0).expect("source robber ID"),
        bank: Resources::try_from(bank).expect("source bank bounds"),
        hands,
    }
}

pub(crate) fn public_score(position: &Position, owner: PlayerId) -> u8 {
    position
        .buildings
        .iter()
        .flatten()
        .filter(|building| building.owner == owner)
        .map(|building| building.kind.value())
        .sum::<u8>()
        + 2 * u8::from(position.longest_route_holder == Some(owner))
        + 2 * u8::from(position.largest_army_holder == Some(owner))
        + position
            .card_locations
            .iter()
            .filter(|&&location| {
                location == crate::game_model::CardLocation::RevealedVictory { owner }
            })
            .count() as u8
}

pub(crate) fn current_score(position: &Position, owner: PlayerId) -> u8 {
    public_score(position, owner)
        + position
            .held_cards(owner)
            .filter(|(card, _)| card.kind() == crate::game_model::DevelopmentKind::VictoryPoint)
            .count() as u8
}

pub(crate) fn validate_setup_settlement(
    board: &Board,
    buildings: &[Option<Building>; VertexId::COUNT],
    vertex: VertexId,
) -> Result<(), RuleError> {
    if buildings[vertex.index()].is_some() {
        return Err(RuleError::Occupied);
    }
    if board
        .graph
        .vertex_neighbors(vertex)
        .iter()
        .any(|neighbor| buildings[neighbor.index()].is_some())
    {
        return Err(RuleError::DistanceRule);
    }
    Ok(())
}

pub(crate) fn validate_setup_road(
    board: &Board,
    roads: &[Option<PlayerId>; EdgeId::COUNT],
    settlement: VertexId,
    edge: EdgeId,
) -> Result<(), RuleError> {
    if roads[edge.index()].is_some() {
        return Err(RuleError::Occupied);
    }
    if !board.graph.edge_vertices(edge).contains(&settlement) {
        return Err(RuleError::Disconnected);
    }
    Ok(())
}

pub(crate) fn starting_resources(board: &Board, second_settlement: VertexId) -> Resources {
    let mut counts = [0u16; 5];
    for hex in board.graph.vertex_hexes(second_settlement) {
        if let Some(resource) = board.terrain[hex.index()].resource() {
            counts[resource.index()] += 1;
        }
    }
    Resources::from_wide(counts).expect("at most three adjacent hexes")
}

pub(crate) fn frame_ports(frames: [u8; 6]) -> [Port; 9] {
    use PortKind::{Generic, Special};
    // Printed templates retain their spacing along each clockwise five-edge segment.
    let segments = [
        [0, 1, 2, 3, 4],
        [5, 9, 17, 22, 32],
        [38, 48, 53, 61, 65],
        [71, 70, 69, 68, 67],
        [66, 62, 54, 49, 39],
        [33, 23, 18, 10, 6],
    ];
    let templates: [&[(usize, PortKind)]; 6] = [
        &[(0, Generic), (3, Special(Resource::Wool))],
        &[(2, Generic)],
        &[(0, Generic), (3, Special(Resource::Brick))],
        &[(2, Special(Resource::Wood))],
        &[(0, Generic), (3, Special(Resource::Wheat))],
        &[(2, Special(Resource::Ore))],
    ];
    let mut ports = [Port {
        edge: EdgeId::try_from(0).expect("source edge"),
        kind: Generic,
    }; 9];
    let mut next = 0;
    for (segment, frame) in segments.into_iter().zip(frames) {
        for &(offset, kind) in templates[usize::from(frame - 1)] {
            ports[next] = Port {
                edge: EdgeId::try_from(segment[offset]).expect("source port ID"),
                kind,
            };
            next += 1;
        }
    }
    ports
}

pub(crate) fn variable_numbers(terrain: &[Terrain; HexId::COUNT]) -> [Option<u8>; HexId::COUNT] {
    let spiral = [
        2, 1, 0, 3, 7, 12, 16, 17, 18, 15, 11, 6, 5, 4, 8, 13, 14, 10, 9,
    ];
    let values = [5, 2, 6, 3, 8, 10, 9, 12, 11, 4, 8, 10, 9, 4, 5, 6, 3, 11];
    let mut numbers = [None; HexId::COUNT];
    for (hex, number) in spiral
        .into_iter()
        .filter(|&hex| terrain[hex] != Terrain::Desert)
        .zip(values)
    {
        numbers[hex] = Some(number);
    }
    numbers
}

pub(crate) fn variable_setup(terrain: [Terrain; HexId::COUNT], frames: [u8; 6]) -> InitialSetup {
    let robber = terrain
        .iter()
        .position(|&hex| hex == Terrain::Desert)
        .expect("canonical desert");
    InitialSetup {
        board: Board {
            graph: BoardGraph::base_game(),
            terrain,
            numbers: variable_numbers(&terrain),
            ports: frame_ports(frames),
            frame_permutation: frames,
        },
        roads: [None; EdgeId::COUNT],
        buildings: [None; VertexId::COUNT],
        second_settlements: [None; 4],
        robber: HexId::try_from(robber as u8).expect("bounded hex"),
        bank: Resources::try_from([19; 5]).expect("initial supply"),
        hands: [Resources::default(); 4],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_model::{GameConfig, PortKind, Resource, Terrain};

    #[test]
    fn base_rules_br008_second_settlement_grants_count_adjacent_hexes_only() {
        let mut setup = fixed_setup(&GameConfig::fixed(&[2, 0, 3, 1]).unwrap());
        // Independently traced hex incidence in tests/model_invariants.rs and source presets.
        let cases = [
            (0, [0, 0, 0, 0, 0]),  // Coast with desert only.
            (4, [0, 0, 0, 1, 0]),  // Coast: desert plus one field.
            (21, [0, 1, 0, 0, 0]), // One coastal forest.
            (49, [0, 0, 1, 1, 0]), // Two coastal hexes.
            (29, [2, 0, 0, 1, 0]), // Two hills both grant brick.
            (40, [1, 0, 0, 1, 1]), // Blue designated second settlement.
            (9, [0, 1, 1, 1, 0]),
            (17, [1, 0, 1, 1, 0]),
        ];
        for (vertex, expected) in cases {
            assert_eq!(
                starting_resources(&setup.board, VertexId::try_from(vertex).unwrap()).counts(),
                &expected
            );
        }
        assert_eq!(
            starting_resources(&setup.board, VertexId::try_from(31).unwrap()).counts(),
            &[0, 2, 1, 0, 0]
        );
        assert_ne!(
            starting_resources(&setup.board, VertexId::try_from(31).unwrap()),
            setup.hands[2]
        );
        setup.board.numbers = [None; HexId::COUNT];
        assert_eq!(
            starting_resources(&setup.board, VertexId::try_from(40).unwrap()).counts(),
            &[1, 0, 0, 1, 1]
        );
    }

    #[test]
    fn base_rules_br006_source_sample_skips_desert_without_consuming_a_letter() {
        let source: serde_json::Value =
            serde_json::from_str(include_str!("../../tests/support/setup.json")).unwrap();
        let mut terrain = CANONICAL_TERRAIN;
        terrain.swap(0, 14);
        let setup = variable_setup(terrain, CANONICAL_FRAMES);
        assert_eq!(
            serde_json::json!(setup.board.numbers),
            source["variable"]["sourcePage11Sample"]["numberByHex"]
        );
        assert_eq!(setup.robber.index(), 14);
        assert_eq!(setup.board.frame_permutation, [1, 2, 3, 4, 5, 6]);
        assert_eq!(setup.bank.counts(), &[19; 5]);
        assert!(setup.hands.iter().all(|hand| hand.counts() == &[0; 5]));
        assert!(setup.roads.iter().all(Option::is_none));
        assert!(setup.buildings.iter().all(Option::is_none));
        for desert in 0..HexId::COUNT {
            let mut terrain = CANONICAL_TERRAIN;
            terrain.swap(0, desert);
            let numbers = variable_numbers(&terrain);
            assert_eq!(numbers[desert], None);
            assert_eq!(numbers.iter().flatten().count(), 18);
            let ordered: Vec<_> = source["variable"]["spiralHexes"]
                .as_array()
                .unwrap()
                .iter()
                .filter_map(|hex| numbers[hex.as_u64().unwrap() as usize])
                .collect();
            assert_eq!(
                serde_json::json!(ordered),
                source["variable"]["numberValues"]
            );
        }
    }

    #[test]
    fn fixed_creation_layout_and_role_mapping_match_source_fixture() {
        let source: serde_json::Value =
            serde_json::from_str(include_str!("../../tests/support/setup.json")).unwrap();
        for order in [&[2, 0, 1][..], &[2, 0, 3, 1][..]] {
            let config = GameConfig::fixed(order).unwrap();
            let setup = fixed_setup(&config);
            let names = setup.board.terrain.map(|terrain| match terrain {
                Terrain::Hills => "hills",
                Terrain::Forest => "forest",
                Terrain::Pasture => "pasture",
                Terrain::Field => "field",
                Terrain::Mountain => "mountain",
                Terrain::Desert => "desert",
            });
            assert_eq!(serde_json::json!(names), source["fixed"]["terrainByHex"]);
            assert_eq!(
                serde_json::json!(setup.board.numbers),
                source["fixed"]["numberByHex"]
            );
            assert_eq!(setup.robber.index(), 0);
            for (role, &seat) in config.seat_order().iter().enumerate() {
                let preset = &source["fixed"]["presets"][role];
                for name in ["firstSettlement", "secondSettlement"] {
                    assert_eq!(
                        setup.buildings[preset[name].as_u64().unwrap() as usize]
                            .unwrap()
                            .owner,
                        seat
                    );
                }
                for name in ["firstRoad", "secondRoad"] {
                    assert_eq!(
                        setup.roads[preset[name].as_u64().unwrap() as usize],
                        Some(seat)
                    );
                }
                assert_eq!(
                    serde_json::json!(setup.hands[seat.index()].counts()),
                    preset["startingHand"]
                );
            }
            assert_eq!(setup.buildings.iter().flatten().count(), order.len() * 2);
            assert_eq!(setup.roads.iter().flatten().count(), order.len() * 2);
            let expected_bank = if order.len() == 3 {
                [17, 18, 17, 16, 18]
            } else {
                [15, 18, 17, 15, 18]
            };
            assert_eq!(setup.bank.counts(), &expected_bank);
            let resources = [
                Resource::Brick,
                Resource::Wood,
                Resource::Wool,
                Resource::Wheat,
                Resource::Ore,
            ];
            for (port, expected) in setup
                .board
                .ports
                .iter()
                .zip(source["fixed"]["portsClockwise"].as_array().unwrap())
            {
                assert_eq!(port.edge.index() as u64, expected["edge"].as_u64().unwrap());
                let (kind, rate) = match port.kind {
                    PortKind::Generic => ("generic", 3),
                    PortKind::Special(resource) => (
                        ["brick", "wood", "wool", "wheat", "ore"]
                            [resources.iter().position(|&r| r == resource).unwrap()],
                        2,
                    ),
                };
                assert_eq!(expected["kind"], kind);
                assert_eq!(expected["rate"], rate);
                assert_eq!(
                    serde_json::json!(
                        setup
                            .board
                            .graph
                            .edge_vertices(port.edge)
                            .map(|id| id.index())
                    ),
                    expected["vertices"]
                );
            }
        }
    }
}

#[cfg(test)]
mod setup_route_tests {
    use super::*;
    #[test]
    fn base_rules_br008_setup_route_zero_one_two_and_blocker() {
        let setup = fixed_setup(&GameConfig::fixed(&[0, 1, 2]).unwrap());
        let owner = PlayerId::try_from(0).unwrap();
        let mut roads = [None; EdgeId::COUNT];
        let mut buildings = [None; VertexId::COUNT];
        assert_eq!(
            longest_route_length(setup.board.graph, &roads, &buildings, owner),
            0
        );
        roads[1] = Some(owner);
        assert_eq!(
            longest_route_length(setup.board.graph, &roads, &buildings, owner),
            1
        );
        roads[23] = Some(owner);
        assert_eq!(
            longest_route_length(setup.board.graph, &roads, &buildings, owner),
            1
        );
        roads[23] = None;
        roads[2] = Some(owner);
        assert_eq!(
            longest_route_length(setup.board.graph, &roads, &buildings, owner),
            2
        );
        buildings[4] = Some(Building {
            owner,
            kind: crate::game_model::BuildingKind::Settlement,
        });
        assert_eq!(
            longest_route_length(setup.board.graph, &roads, &buildings, owner),
            2
        );
        buildings[4] = Some(Building {
            kind: crate::game_model::BuildingKind::Settlement,
            owner: PlayerId::try_from(1).unwrap(),
        });
        assert_eq!(
            longest_route_length(setup.board.graph, &roads, &buildings, owner),
            1
        );
    }
}

mod economy;
pub(crate) use economy::{discard_obligations, production, validate_discard};

mod robber;
pub(crate) use robber::{card_at_index, robber_victims};

mod longest_road;
pub(crate) use longest_road::{longest_route_length, reconcile_longest_route};
#[cfg(test)]
pub(crate) use longest_road::{reset_route_searches, route_searches};

mod building;
pub(crate) use building::{
    city_cost, road_cost, settlement_cost, validate_city_upgrade, validate_paid_road,
    validate_paid_settlement, validate_road_building, validate_road_placement,
};

mod trade;
pub(crate) use trade::{
    BankExchange, calculate_bank_trade, calculate_offer_exchange, validate_offer_terms,
};

mod development;
pub(crate) use development::{
    development_cost, first_invention_choice, monopoly_hands, reconcile_largest_army,
    validate_card_play, validate_development_purchase, validate_invention,
};
