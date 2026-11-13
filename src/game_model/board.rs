use std::{collections::BTreeSet, sync::LazyLock};

use super::{EdgeId, HexId, VertexId};

static BASE_GAME_GRAPH: LazyLock<BoardGraph> = LazyLock::new(BoardGraph::build);
const CORNERS: [(i8, i8); 6] = [(0, -2), (1, -1), (1, 1), (0, 2), (-1, 1), (-1, -1)];

/// Immutable radius-2 island topology, shared by all base games.
///
/// Terrain, numbers and ports belong to each game's separate layout. Hex
/// perimeters run clockwise from the top; every neighbor list is sorted by ID.
#[derive(Debug)]
pub struct BoardGraph {
    hex_vertices: [[VertexId; 6]; HexId::COUNT],
    hex_edges: [[EdgeId; 6]; HexId::COUNT],
    hex_neighbors: [Vec<HexId>; HexId::COUNT],
    vertex_hexes: [Vec<HexId>; VertexId::COUNT],
    vertex_neighbors: [Vec<VertexId>; VertexId::COUNT],
    vertex_edges: [Vec<EdgeId>; VertexId::COUNT],
    edge_vertices: [[VertexId; 2]; EdgeId::COUNT],
    edge_hexes: [Vec<HexId>; EdgeId::COUNT],
}

impl BoardGraph {
    /// Borrow the topology constructed once for the process; no layout is shared.
    pub fn base_game() -> &'static Self {
        &BASE_GAME_GRAPH
    }

    /// Borrow the six corners clockwise from this hex's top corner.
    pub fn hex_vertices(&self, id: HexId) -> &[VertexId; 6] {
        &self.hex_vertices[id.index()]
    }

    /// Borrow the six sides corresponding to the clockwise corner perimeter.
    pub fn hex_edges(&self, id: HexId) -> &[EdgeId; 6] {
        &self.hex_edges[id.index()]
    }

    /// Borrow adjacent land hexes in ascending ID order.
    pub fn hex_neighbors(&self, id: HexId) -> &[HexId] {
        &self.hex_neighbors[id.index()]
    }

    /// Borrow the one to three land hexes touching a vertex, sorted by ID.
    pub fn vertex_hexes(&self, id: VertexId) -> &[HexId] {
        &self.vertex_hexes[id.index()]
    }

    /// Borrow neighboring vertices in ascending ID order, including the coast.
    pub fn vertex_neighbors(&self, id: VertexId) -> &[VertexId] {
        &self.vertex_neighbors[id.index()]
    }

    /// Borrow incident edges in ascending ID order.
    pub fn vertex_edges(&self, id: VertexId) -> &[EdgeId] {
        &self.vertex_edges[id.index()]
    }

    /// Borrow the distinct endpoints in ascending vertex ID order.
    pub fn edge_vertices(&self, id: EdgeId) -> &[VertexId; 2] {
        &self.edge_vertices[id.index()]
    }

    /// Borrow the one or two land hexes touching an edge, sorted by ID.
    pub fn edge_hexes(&self, id: EdgeId) -> &[HexId] {
        &self.edge_hexes[id.index()]
    }

    /// Whether a vertex touches sea; a coastal vertex may have three neighbors.
    pub fn is_coastal_vertex(&self, id: VertexId) -> bool {
        self.vertex_hexes(id).len() < 3
    }

    /// Whether an edge touches just one land hex, making it eligible for a port.
    pub fn is_coastal_edge(&self, id: EdgeId) -> bool {
        self.edge_hexes(id).len() == 1
    }

    fn build() -> Self {
        let mut hexes = Vec::new();
        for r in -2_i8..=2 {
            for q in -2_i8..=2 {
                if q.abs().max(r.abs()).max((q + r).abs()) <= 2 {
                    hexes.push((q, r));
                }
            }
        }
        let hexes: [(i8, i8); HexId::COUNT] = hexes.try_into().expect("radius-2 hex count");
        let coordinates: Vec<_> = hexes
            .iter()
            .flat_map(|&(q, r)| CORNERS.map(|(x, y)| (3 * r + y, 2 * q + r + x)))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        assert_eq!(coordinates.len(), VertexId::COUNT, "radius-2 vertex count");
        let hex_vertices = hexes.map(|(q, r)| {
            CORNERS.map(|(x, y)| {
                let index = coordinates
                    .binary_search(&(3 * r + y, 2 * q + r + x))
                    .expect("generated corner exists");
                VertexId(index as u8)
            })
        });
        let edges: Vec<_> = hex_vertices
            .iter()
            .flat_map(|vertices| {
                (0..6).map(|side| {
                    let first = vertices[side];
                    let second = vertices[(side + 1) % 6];
                    [first.min(second), first.max(second)]
                })
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let edge_vertices: [[VertexId; 2]; EdgeId::COUNT] =
            edges.try_into().expect("radius-2 edge count");
        let hex_edges = hex_vertices.map(|vertices| {
            std::array::from_fn(|side| {
                let first = vertices[side];
                let second = vertices[(side + 1) % 6];
                let index = edge_vertices
                    .binary_search(&[first.min(second), first.max(second)])
                    .expect("generated side exists");
                EdgeId(index as u8)
            })
        });
        let mut graph = Self {
            hex_vertices,
            hex_edges,
            hex_neighbors: std::array::from_fn(|_| Vec::new()),
            vertex_hexes: std::array::from_fn(|_| Vec::new()),
            vertex_neighbors: std::array::from_fn(|_| Vec::new()),
            vertex_edges: std::array::from_fn(|_| Vec::new()),
            edge_vertices,
            edge_hexes: std::array::from_fn(|_| Vec::new()),
        };
        for (index, vertices) in graph.hex_vertices.iter().enumerate() {
            let id = HexId(index as u8);
            for vertex in vertices {
                graph.vertex_hexes[vertex.index()].push(id);
            }
            for edge in graph.hex_edges[index] {
                graph.edge_hexes[edge.index()].push(id);
            }
        }
        for (index, &[first, second]) in graph.edge_vertices.iter().enumerate() {
            let id = EdgeId(index as u8);
            graph.vertex_neighbors[first.index()].push(second);
            graph.vertex_neighbors[second.index()].push(first);
            graph.vertex_edges[first.index()].push(id);
            graph.vertex_edges[second.index()].push(id);
            if let [first_hex, second_hex] = graph.edge_hexes[index].as_slice() {
                graph.hex_neighbors[first_hex.index()].push(*second_hex);
                graph.hex_neighbors[second_hex.index()].push(*first_hex);
            }
        }
        for neighbors in &mut graph.vertex_neighbors {
            neighbors.sort_unstable();
        }
        for neighbors in &mut graph.hex_neighbors {
            neighbors.sort_unstable();
        }
        graph
    }
}
