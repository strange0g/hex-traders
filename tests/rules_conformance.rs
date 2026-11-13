use hex_traders_engine::{BoardGraph, EdgeId, HexId, VertexId};
use serde_json::Value;
use std::collections::BTreeMap;

fn fixture() -> Value {
    serde_json::from_str(include_str!("support/setup.json")).unwrap()
}

fn number(value: &Value) -> u8 {
    u8::try_from(value.as_u64().unwrap()).unwrap()
}

fn vertex(value: &Value) -> VertexId {
    VertexId::try_from(number(value)).unwrap()
}

fn edge(value: &Value) -> EdgeId {
    EdgeId::try_from(number(value)).unwrap()
}

#[test]
fn base_rules_br001_br005_fixed_pieces_and_hands_fixture() {
    let data = fixture();
    let fixed = &data["fixed"];
    let graph = BoardGraph::base_game();
    let presets = fixed["presets"].as_array().unwrap();
    let expected = [
        ("Blue", 31, 46, 40, 57, [1, 0, 0, 1, 1]),
        ("Red", 39, 55, 9, 14, [0, 1, 1, 1, 0]),
        ("Orange", 19, 21, 17, 25, [1, 0, 1, 1, 0]),
        ("White", 41, 59, 29, 42, [2, 0, 0, 1, 0]),
    ];
    assert_eq!(presets.len(), expected.len());
    assert_eq!(fixed["threePlayerOmitRole"], "White");
    assert_eq!(presets.iter().filter(|p| p["role"] != "White").count(), 3);
    let mut occupied = Vec::new();
    for (preset, (role, first, first_road, second, second_road, hand)) in
        presets.iter().zip(expected)
    {
        assert_eq!(preset["role"], role);
        assert_eq!(number(&preset["firstSettlement"]), first);
        assert_eq!(number(&preset["firstRoad"]), first_road);
        assert_eq!(number(&preset["secondSettlement"]), second);
        assert_eq!(number(&preset["secondRoad"]), second_road);
        for (settlement, road) in [
            ("firstSettlement", "firstRoad"),
            ("secondSettlement", "secondRoad"),
        ] {
            let id = vertex(&preset[settlement]);
            assert!(graph.edge_vertices(edge(&preset[road])).contains(&id));
            assert!(
                occupied
                    .iter()
                    .all(|other| !graph.vertex_neighbors(id).contains(other))
            );
            assert!(!occupied.contains(&id));
            occupied.push(id);
        }
        let mut resources = [0_u8; 5];
        for hex in graph.vertex_hexes(vertex(&preset["secondSettlement"])) {
            let index = match fixed["terrainByHex"][hex.index()].as_str().unwrap() {
                "hills" => Some(0),
                "forest" => Some(1),
                "pasture" => Some(2),
                "field" => Some(3),
                "mountain" => Some(4),
                "desert" => None,
                _ => panic!("unexpected terrain"),
            };
            if let Some(index) = index {
                resources[index] += 1;
            }
        }
        assert_eq!(resources, hand);
        assert_eq!(preset["startingHand"], serde_json::json!(hand));
    }
}

#[test]
fn base_rules_br002_fixed_terrain_fixture() {
    let data = fixture();
    let terrain = data["fixed"]["terrainByHex"].as_array().unwrap();
    assert_eq!(
        data["fixed"]["terrainByHex"],
        serde_json::json!([
            "desert", "field", "pasture", "hills", "pasture", "forest", "mountain", "forest",
            "field", "hills", "forest", "pasture", "field", "hills", "mountain", "forest",
            "mountain", "field", "pasture"
        ])
    );
    let mut counts = BTreeMap::new();
    for kind in terrain {
        *counts.entry(kind.as_str().unwrap()).or_insert(0) += 1;
    }
    assert_eq!(
        counts,
        BTreeMap::from([
            ("desert", 1),
            ("field", 4),
            ("forest", 4),
            ("hills", 3),
            ("mountain", 3),
            ("pasture", 4)
        ])
    );
}

#[test]
fn base_rules_br003_fixed_numbers_fixture() {
    let data = fixture();
    let fixed = &data["fixed"];
    assert_eq!(
        fixed["numberByHex"],
        serde_json::json!([
            null, 8, 11, 6, 3, 4, 9, 10, 5, 12, 11, 5, 2, 9, 4, 8, 6, 3, 10
        ])
    );
    assert_eq!(fixed["robberHex"], 0);
    for (terrain, token) in fixed["terrainByHex"]
        .as_array()
        .unwrap()
        .iter()
        .zip(fixed["numberByHex"].as_array().unwrap())
    {
        assert_eq!(terrain == "desert", token.is_null());
    }
    let mut counts = BTreeMap::new();
    for token in fixed["numberByHex"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|v| !v.is_null())
    {
        *counts.entry(number(token)).or_insert(0) += 1;
    }
    assert_eq!(
        counts,
        BTreeMap::from([
            (2, 1),
            (3, 2),
            (4, 2),
            (5, 2),
            (6, 2),
            (8, 2),
            (9, 2),
            (10, 2),
            (11, 2),
            (12, 1)
        ])
    );
}

#[test]
fn base_rules_br004_fixed_ports_fixture() {
    let data = fixture();
    let graph = BoardGraph::base_game();
    let ports = data["fixed"]["portsClockwise"].as_array().unwrap();
    let expected: [(u8, [usize; 2], &str, u8); 9] = [
        (0, [0, 3], "generic", 3),
        (3, [1, 5], "wool", 2),
        (17, [10, 15], "generic", 3),
        (38, [26, 32], "generic", 3),
        (61, [42, 46], "brick", 2),
        (69, [49, 52], "wood", 2),
        (66, [47, 51], "generic", 3),
        (49, [33, 38], "wheat", 2),
        (18, [11, 16], "ore", 2),
    ];
    assert_eq!(ports.len(), expected.len());
    for (port, (id, endpoints, kind, rate)) in ports.iter().zip(expected) {
        assert_eq!(number(&port["edge"]), id);
        assert_eq!(port["vertices"], serde_json::json!(endpoints));
        assert_eq!(port["kind"], kind);
        assert_eq!(number(&port["rate"]), rate);
        let id = edge(&port["edge"]);
        assert!(graph.is_coastal_edge(id));
        assert_eq!(graph.edge_vertices(id).map(|v| v.index()), endpoints);
    }
}

#[test]
fn base_rules_br006_frame_templates_fixture() {
    let data = fixture();
    let variable = &data["variable"];
    let segments = variable["segments"].as_array().unwrap();
    let expected_segments = [
        [0, 1, 2, 3, 4],
        [5, 9, 17, 22, 32],
        [38, 48, 53, 61, 65],
        [71, 70, 69, 68, 67],
        [66, 62, 54, 49, 39],
        [33, 23, 18, 10, 6],
    ];
    let expected_templates = [
        vec![(0, "generic"), (3, "wool")],
        vec![(2, "generic")],
        vec![(0, "generic"), (3, "brick")],
        vec![(2, "wood")],
        vec![(0, "generic"), (3, "wheat")],
        vec![(2, "ore")],
    ];
    let graph = BoardGraph::base_game();
    let mut previous = VertexId::try_from(3).unwrap();
    let mut edges = Vec::new();
    for (segment, expected) in segments.iter().zip(expected_segments) {
        assert_eq!(segment["edges"], serde_json::json!(expected));
        for value in segment["edges"].as_array().unwrap() {
            let id = edge(value);
            assert!(graph.is_coastal_edge(id));
            assert!(!edges.contains(&id));
            edges.push(id);
            let [a, b] = *graph.edge_vertices(id);
            previous = if a == previous {
                b
            } else {
                assert_eq!(b, previous);
                a
            };
        }
    }
    assert_eq!(previous, VertexId::try_from(3).unwrap());
    assert_eq!(edges.len(), 30);
    assert_eq!(
        variable["coastEdgesClockwise"],
        serde_json::json!(expected_segments.concat())
    );
    let templates = variable["templates"].as_array().unwrap();
    assert_eq!(templates.len(), 6);
    let mut ports = Vec::new();
    for (slot, (template, expected)) in templates.iter().zip(expected_templates).enumerate() {
        assert_eq!(number(&template["id"]) as usize, slot + 1);
        let actual = template["ports"].as_array().unwrap();
        assert_eq!(actual.len(), expected.len());
        for (port, (offset, kind)) in actual.iter().zip(expected) {
            assert_eq!(number(&port["offset"]), offset);
            assert_eq!(port["kind"], kind);
            ports.push((number(&segments[slot]["edges"][usize::from(offset)]), kind));
        }
    }
    assert_eq!(
        variable["fixedFramePermutation"],
        serde_json::json!([1, 2, 3, 4, 5, 6])
    );
    let fixed: Vec<_> = data["fixed"]["portsClockwise"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| (number(&p["edge"]), p["kind"].as_str().unwrap()))
        .collect();
    assert_eq!(ports, fixed);
}

#[test]
fn base_rules_br006_number_spiral_fixture() {
    let data = fixture();
    let variable = &data["variable"];
    assert_eq!(variable["numberLetters"], "ABCDEFGHIJKLMNOPQR");
    assert_eq!(
        variable["numberValues"],
        serde_json::json!([5, 2, 6, 3, 8, 10, 9, 12, 11, 4, 8, 10, 9, 4, 5, 6, 3, 11])
    );
    assert_eq!(
        variable["spiralHexes"],
        serde_json::json!([
            2, 1, 0, 3, 7, 12, 16, 17, 18, 15, 11, 6, 5, 4, 8, 13, 14, 10, 9
        ])
    );
    assert_eq!(variable["skipDesertWithoutConsumingLetter"], true);
    let graph = BoardGraph::base_game();
    let spiral: Vec<_> = variable["spiralHexes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|n| HexId::try_from(number(n)).unwrap())
        .collect();
    assert_eq!(spiral.len(), HexId::COUNT);
    for (index, hex) in spiral.iter().enumerate() {
        assert!(!spiral[..index].contains(hex));
        if index > 0 {
            assert!(graph.hex_neighbors(spiral[index - 1]).contains(hex));
        }
    }
    let values = variable["numberValues"].as_array().unwrap();
    for desert in &spiral {
        let mut board = [None; HexId::COUNT];
        for (hex, token) in spiral.iter().filter(|hex| *hex != desert).zip(values) {
            board[hex.index()] = Some(number(token));
        }
        assert_eq!(board.iter().filter(|v| v.is_none()).count(), 1);
        assert_eq!(board[desert.index()], None);
        if desert.index() == 14 {
            assert_eq!(
                serde_json::json!(board),
                variable["sourcePage11Sample"]["numberByHex"]
            );
            assert_eq!(
                serde_json::json!(board),
                serde_json::json!([
                    6, 2, 5, 3, 4, 9, 10, 8, 5, 11, 3, 8, 10, 6, null, 4, 9, 12, 11
                ])
            );
        }
    }
}

fn valid_route_witness(case: &Value, roads: &[Value], vertices: &[Value]) -> bool {
    let graph = BoardGraph::base_game();
    let owned = case["roads"].as_array().unwrap();
    let blockers = case["opponent_building_vertices"].as_array().unwrap();
    let distinct: std::collections::BTreeSet<_> = roads.iter().map(number).collect();
    roads.len() == case["expected_length"].as_u64().unwrap() as usize
        && vertices.len() == roads.len() + 1
        && distinct.len() == roads.len()
        && roads.iter().all(|road| owned.contains(road))
        && roads.iter().zip(vertices.windows(2)).all(|(road, pair)| {
            let endpoints = graph.edge_vertices(edge(road));
            endpoints.contains(&vertex(&pair[0]))
                && endpoints.contains(&vertex(&pair[1]))
                && pair[0] != pair[1]
        })
        && !vertices
            .windows(3)
            .any(|triple| blockers.contains(&triple[1]))
}

#[test]
fn longest_route_independent_endpoints_and_manual_witnesses_match_public_graph() {
    let data: Value = serde_json::from_str(include_str!("support/longest-route.json")).unwrap();
    assert_eq!(
        data["provenance"]["production_graph_or_route_solver_used"],
        false
    );
    let graph = BoardGraph::base_game();
    for (id, endpoints) in data["edge_vertices"].as_array().unwrap().iter().enumerate() {
        assert_eq!(
            graph
                .edge_vertices(EdgeId::try_from(id as u8).unwrap())
                .map(|id| id.index()),
            [
                endpoints[0].as_u64().unwrap() as usize,
                endpoints[1].as_u64().unwrap() as usize
            ]
        );
    }
    let cases = data["route_cases"].as_array().unwrap();
    assert_eq!(cases.len(), 15);
    for case in cases {
        assert!(
            valid_route_witness(
                case,
                case["witness"]["road_ids"].as_array().unwrap(),
                case["witness"]["vertex_ids"].as_array().unwrap()
            ),
            "{}",
            case["name"]
        );
        assert!(!case["upper_bound_reason"].as_str().unwrap().is_empty());
    }
    // Deliberately invalid controls: count an edge twice or traverse a blocker internally.
    let mut cycle = cases
        .iter()
        .find(|case| case["name"] == "hex_zero_cycle")
        .unwrap()
        .clone();
    cycle["expected_length"] = serde_json::json!(7);
    assert!(!valid_route_witness(
        &cycle,
        serde_json::json!([1, 0, 6, 11, 12, 7, 7])
            .as_array()
            .unwrap(),
        serde_json::json!([4, 0, 3, 7, 12, 8, 4, 8])
            .as_array()
            .unwrap()
    ));
    let mut blocked = cases
        .iter()
        .find(|case| case["name"] == "opponent_blocks_middle")
        .unwrap()
        .clone();
    blocked["expected_length"] = serde_json::json!(5);
    assert!(!valid_route_witness(
        &blocked,
        serde_json::json!([0, 6, 10, 18, 23]).as_array().unwrap(),
        serde_json::json!([0, 3, 7, 11, 16, 21]).as_array().unwrap()
    ));
}
