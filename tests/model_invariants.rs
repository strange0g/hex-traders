use hex_traders_engine::{BoardGraph, EdgeId, GraphIdError, HexId, VertexId};

fn hex(index: usize) -> HexId {
    HexId::try_from(u8::try_from(index).unwrap()).unwrap()
}

fn vertex(index: usize) -> VertexId {
    VertexId::try_from(u8::try_from(index).unwrap()).unwrap()
}

fn edge(index: usize) -> EdgeId {
    EdgeId::try_from(u8::try_from(index).unwrap()).unwrap()
}

// Independently hand-traced radius-2 rows and perimeters; see SPEC-game-model.md.
const HEX_VERTICES: [[usize; 6]; 19] = [
    [0, 4, 8, 12, 7, 3],
    [1, 5, 9, 13, 8, 4],
    [2, 6, 10, 14, 9, 5],
    [7, 12, 17, 22, 16, 11],
    [8, 13, 18, 23, 17, 12],
    [9, 14, 19, 24, 18, 13],
    [10, 15, 20, 25, 19, 14],
    [16, 22, 28, 33, 27, 21],
    [17, 23, 29, 34, 28, 22],
    [18, 24, 30, 35, 29, 23],
    [19, 25, 31, 36, 30, 24],
    [20, 26, 32, 37, 31, 25],
    [28, 34, 39, 43, 38, 33],
    [29, 35, 40, 44, 39, 34],
    [30, 36, 41, 45, 40, 35],
    [31, 37, 42, 46, 41, 36],
    [39, 44, 48, 51, 47, 43],
    [40, 45, 49, 52, 48, 44],
    [41, 46, 50, 53, 49, 45],
];

const HEX_EDGES: [[usize; 6]; 19] = [
    [1, 7, 12, 11, 6, 0],
    [3, 8, 14, 13, 7, 2],
    [5, 9, 16, 15, 8, 4],
    [11, 19, 25, 24, 18, 10],
    [13, 20, 27, 26, 19, 12],
    [15, 21, 29, 28, 20, 14],
    [17, 22, 31, 30, 21, 16],
    [24, 34, 40, 39, 33, 23],
    [26, 35, 42, 41, 34, 25],
    [28, 36, 44, 43, 35, 27],
    [30, 37, 46, 45, 36, 29],
    [32, 38, 48, 47, 37, 31],
    [41, 50, 55, 54, 49, 40],
    [43, 51, 57, 56, 50, 42],
    [45, 52, 59, 58, 51, 44],
    [47, 53, 61, 60, 52, 46],
    [56, 63, 67, 66, 62, 55],
    [58, 64, 69, 68, 63, 57],
    [60, 65, 71, 70, 64, 59],
];

const EDGE_VERTICES: [[usize; 2]; 72] = [
    [0, 3],
    [0, 4],
    [1, 4],
    [1, 5],
    [2, 5],
    [2, 6],
    [3, 7],
    [4, 8],
    [5, 9],
    [6, 10],
    [7, 11],
    [7, 12],
    [8, 12],
    [8, 13],
    [9, 13],
    [9, 14],
    [10, 14],
    [10, 15],
    [11, 16],
    [12, 17],
    [13, 18],
    [14, 19],
    [15, 20],
    [16, 21],
    [16, 22],
    [17, 22],
    [17, 23],
    [18, 23],
    [18, 24],
    [19, 24],
    [19, 25],
    [20, 25],
    [20, 26],
    [21, 27],
    [22, 28],
    [23, 29],
    [24, 30],
    [25, 31],
    [26, 32],
    [27, 33],
    [28, 33],
    [28, 34],
    [29, 34],
    [29, 35],
    [30, 35],
    [30, 36],
    [31, 36],
    [31, 37],
    [32, 37],
    [33, 38],
    [34, 39],
    [35, 40],
    [36, 41],
    [37, 42],
    [38, 43],
    [39, 43],
    [39, 44],
    [40, 44],
    [40, 45],
    [41, 45],
    [41, 46],
    [42, 46],
    [43, 47],
    [44, 48],
    [45, 49],
    [46, 50],
    [47, 51],
    [48, 51],
    [48, 52],
    [49, 52],
    [49, 53],
    [50, 53],
];

#[test]
fn game_model_stable_ids_and_clockwise_perimeters_match_independent_fixtures() {
    let graph = BoardGraph::base_game();
    assert_eq!((HexId::COUNT, VertexId::COUNT, EdgeId::COUNT), (19, 54, 72));
    for index in 0..19 {
        assert_eq!(
            graph.hex_vertices(hex(index)).map(VertexId::index),
            HEX_VERTICES[index]
        );
        assert_eq!(
            graph.hex_edges(hex(index)).map(EdgeId::index),
            HEX_EDGES[index]
        );
    }
    for (index, expected) in EDGE_VERTICES.iter().enumerate() {
        assert_eq!(
            &graph.edge_vertices(edge(index)).map(VertexId::index),
            expected
        );
    }
}

#[test]
fn game_model_literal_neighbors_and_coast_include_three_degree_coastal_vertices() {
    let graph = BoardGraph::base_game();
    let hex_neighbors: [&[usize]; 19] = [
        &[1, 3, 4],
        &[0, 2, 4, 5],
        &[1, 5, 6],
        &[0, 4, 7, 8],
        &[0, 1, 3, 5, 8, 9],
        &[1, 2, 4, 6, 9, 10],
        &[2, 5, 10, 11],
        &[3, 8, 12],
        &[3, 4, 7, 9, 12, 13],
        &[4, 5, 8, 10, 13, 14],
        &[5, 6, 9, 11, 14, 15],
        &[6, 10, 15],
        &[7, 8, 13, 16],
        &[8, 9, 12, 14, 16, 17],
        &[9, 10, 13, 15, 17, 18],
        &[10, 11, 14, 18],
        &[12, 13, 17],
        &[13, 14, 16, 18],
        &[14, 15, 17],
    ];
    for (index, expected) in hex_neighbors.iter().enumerate() {
        assert_eq!(
            graph
                .hex_neighbors(hex(index))
                .iter()
                .map(|id| id.index())
                .collect::<Vec<_>>(),
            *expected
        );
    }
    let fixtures = [
        (0, &[0][..], &[3, 4][..], &[0, 1][..]),
        (4, &[0, 1], &[0, 1, 8], &[1, 2, 7]),
        (18, &[4, 5, 9], &[13, 23, 24], &[20, 27, 28]),
        (24, &[5, 9, 10], &[18, 19, 30], &[28, 29, 36]),
        (29, &[8, 9, 13], &[23, 34, 35], &[35, 42, 43]),
        (21, &[7], &[16, 27], &[23, 33]),
        (26, &[11], &[20, 32], &[32, 38]),
        (49, &[17, 18], &[45, 52, 53], &[64, 69, 70]),
        (53, &[18], &[49, 50], &[70, 71]),
    ];
    for (index, hexes, neighbors, edges) in fixtures {
        assert_eq!(
            graph
                .vertex_hexes(vertex(index))
                .iter()
                .map(|id| id.index())
                .collect::<Vec<_>>(),
            hexes
        );
        assert_eq!(
            graph
                .vertex_neighbors(vertex(index))
                .iter()
                .map(|id| id.index())
                .collect::<Vec<_>>(),
            neighbors
        );
        assert_eq!(
            graph
                .vertex_edges(vertex(index))
                .iter()
                .map(|id| id.index())
                .collect::<Vec<_>>(),
            edges
        );
    }
    assert_eq!(graph.edge_hexes(edge(28)), &[hex(5), hex(9)]);
    assert_eq!(graph.edge_hexes(edge(71)), &[hex(18)]);
    assert_eq!(
        (0..54)
            .filter(|&id| graph.is_coastal_vertex(vertex(id)))
            .collect::<Vec<_>>(),
        [
            0, 1, 2, 3, 4, 5, 6, 7, 10, 11, 15, 16, 20, 21, 26, 27, 32, 33, 37, 38, 42, 43, 46, 47,
            48, 49, 50, 51, 52, 53,
        ]
    );
    assert_eq!(
        (0..72)
            .filter(|&id| graph.is_coastal_edge(edge(id)))
            .collect::<Vec<_>>(),
        [
            0, 1, 2, 3, 4, 5, 6, 9, 10, 17, 18, 22, 23, 32, 33, 38, 39, 48, 49, 53, 54, 61, 62, 65,
            66, 67, 68, 69, 70, 71,
        ]
    );
}

fn assert_sorted_unique<T: Ord>(values: &[T]) {
    assert!(values.windows(2).all(|pair| pair[0] < pair[1]));
}

#[test]
fn game_model_every_incidence_is_reciprocal_and_neighbors_are_sorted_unique() {
    let graph = BoardGraph::base_game();
    for index in 0..19 {
        let id = hex(index);
        for &corner in graph.hex_vertices(id) {
            assert!(graph.vertex_hexes(corner).contains(&id));
        }
        for (side, &edge_id) in graph.hex_edges(id).iter().enumerate() {
            assert!(graph.edge_hexes(edge_id).contains(&id));
            let vertices = graph.hex_vertices(id);
            assert!(graph.edge_vertices(edge_id).contains(&vertices[side]));
            assert!(
                graph
                    .edge_vertices(edge_id)
                    .contains(&vertices[(side + 1) % 6])
            );
        }
        assert_sorted_unique(graph.hex_neighbors(id));
        for &neighbor in graph.hex_neighbors(id) {
            assert!(graph.hex_neighbors(neighbor).contains(&id));
        }
    }
    let mut degree_histogram = [0; 4];
    let mut land_histogram = [0; 4];
    for index in 0..54 {
        let id = vertex(index);
        let hexes = graph.vertex_hexes(id);
        let neighbors = graph.vertex_neighbors(id);
        let edges = graph.vertex_edges(id);
        assert_sorted_unique(hexes);
        assert_sorted_unique(neighbors);
        assert_sorted_unique(edges);
        assert_eq!(neighbors.len(), edges.len());
        degree_histogram[neighbors.len()] += 1;
        land_histogram[hexes.len()] += 1;
        for &hex_id in hexes {
            assert!(graph.hex_vertices(hex_id).contains(&id));
        }
        for &neighbor in neighbors {
            assert!(graph.vertex_neighbors(neighbor).contains(&id));
            assert!(
                edges
                    .iter()
                    .any(|&edge_id| graph.edge_vertices(edge_id).contains(&neighbor))
            );
        }
        for &edge_id in edges {
            assert!(graph.edge_vertices(edge_id).contains(&id));
        }
    }
    assert_eq!(degree_histogram, [0, 0, 18, 36]);
    assert_eq!(land_histogram, [0, 18, 12, 24]);
    for index in 0..72 {
        let id = edge(index);
        assert_sorted_unique(graph.edge_vertices(id));
        assert_sorted_unique(graph.edge_hexes(id));
        assert!((1..=2).contains(&graph.edge_hexes(id).len()));
        for &endpoint in graph.edge_vertices(id) {
            assert!(graph.vertex_edges(endpoint).contains(&id));
        }
        for &hex_id in graph.edge_hexes(id) {
            assert!(graph.hex_edges(hex_id).contains(&id));
        }
        if index > 0 {
            assert!(graph.edge_vertices(edge(index - 1)) < graph.edge_vertices(id));
        }
    }
}

#[test]
fn game_model_graph_ids_reject_all_out_of_range_values_before_lookup() {
    for value in 0..=u8::MAX {
        if usize::from(value) < HexId::COUNT {
            assert_eq!(HexId::try_from(value).unwrap().index(), usize::from(value));
        } else {
            assert_eq!(HexId::try_from(value), Err(GraphIdError::Hex(value)));
        }
        if usize::from(value) < VertexId::COUNT {
            assert_eq!(
                VertexId::try_from(value).unwrap().index(),
                usize::from(value)
            );
        } else {
            assert_eq!(VertexId::try_from(value), Err(GraphIdError::Vertex(value)));
        }
        if usize::from(value) < EdgeId::COUNT {
            assert_eq!(EdgeId::try_from(value).unwrap().index(), usize::from(value));
        } else {
            assert_eq!(EdgeId::try_from(value), Err(GraphIdError::Edge(value)));
        }
    }
}

#[test]
fn game_model_queries_borrow_the_same_immutable_topology() {
    let first: &'static BoardGraph = BoardGraph::base_game();
    let second: &'static BoardGraph = BoardGraph::base_game();
    assert!(std::ptr::eq(first, second));
    let vertices: &[VertexId; 6] = first.hex_vertices(hex(0));
    assert!(std::ptr::eq(vertices, second.hex_vertices(hex(0))));
    let neighbors: &[VertexId] = first.vertex_neighbors(vertex(4));
    assert!(std::ptr::eq(neighbors, second.vertex_neighbors(vertex(4))));
}
