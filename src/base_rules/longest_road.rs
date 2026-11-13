//! Exact bounded trails. Vertices may repeat; edges may not.
use crate::game_model::{BoardGraph, Building, EdgeId, PlayerId, VertexId};

#[cfg(test)]
thread_local! { static SEARCHES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }
#[cfg(test)]
pub(crate) fn reset_route_searches() {
    SEARCHES.set(0);
}
#[cfg(test)]
pub(crate) fn route_searches() -> usize {
    SEARCHES.get()
}

pub(crate) fn longest_route_length(
    graph: &BoardGraph,
    roads: &[Option<PlayerId>; EdgeId::COUNT],
    buildings: &[Option<Building>; VertexId::COUNT],
    owner: PlayerId,
) -> u8 {
    #[cfg(test)]
    SEARCHES.set(SEARCHES.get() + 1);
    let mut best = 0;
    for id in 0..VertexId::COUNT as u8 {
        best = best.max(walk(
            graph,
            roads,
            buildings,
            owner,
            VertexId::try_from(id).expect("bounded vertex"),
            0,
            0,
        ));
    }
    best
}

fn walk(
    graph: &BoardGraph,
    roads: &[Option<PlayerId>; EdgeId::COUNT],
    buildings: &[Option<Building>; VertexId::COUNT],
    owner: PlayerId,
    vertex: VertexId,
    visited: u128,
    length: u8,
) -> u8 {
    if length == 15
        || (length != 0
            && buildings[vertex.index()].is_some_and(|building| building.owner != owner))
    {
        return length;
    }
    let mut best = length;
    for &edge in graph.vertex_edges(vertex) {
        let bit = 1u128 << edge.index();
        if roads[edge.index()] == Some(owner) && visited & bit == 0 {
            let endpoints = graph.edge_vertices(edge);
            let next = if endpoints[0] == vertex {
                endpoints[1]
            } else {
                endpoints[0]
            };
            best = best.max(walk(
                graph,
                roads,
                buildings,
                owner,
                next,
                visited | bit,
                length + 1,
            ));
        }
    }
    best
}

pub(crate) fn reconcile_longest_route(
    lengths: &[u8; 4],
    incumbent: Option<PlayerId>,
) -> Option<PlayerId> {
    let maximum = *lengths.iter().max().expect("four bounded seat lengths");
    if maximum < 5 {
        return None;
    }
    if incumbent.is_some_and(|seat| lengths[seat.index()] == maximum) {
        return incumbent;
    }
    let mut leaders = lengths
        .iter()
        .enumerate()
        .filter(|(_, length)| **length == maximum)
        .map(|(id, _)| PlayerId::try_from(id as u8).expect("bounded seat"));
    let first = leaders.next();
    if leaders.next().is_none() {
        first
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_model::GameConfig;
    #[test]
    fn base_rules_br034_independent_exact_trails_and_blocked_endpoints() {
        let fixtures: serde_json::Value =
            serde_json::from_str(include_str!("../../tests/support/longest-route.json")).unwrap();
        let setup = super::super::fixed_setup(&GameConfig::fixed(&[0, 1, 2]).unwrap());
        let owner = PlayerId::try_from(0).unwrap();
        let opponent = PlayerId::try_from(1).unwrap();
        let cases = fixtures["route_cases"].as_array().unwrap();
        assert_eq!(cases.len(), 15);
        for case in cases {
            let mut roads = [None; EdgeId::COUNT];
            let mut buildings = [None; VertexId::COUNT];
            for road in case["roads"].as_array().unwrap() {
                roads[road.as_u64().unwrap() as usize] = Some(owner);
            }
            for (key, seat) in [
                ("opponent_building_vertices", opponent),
                ("own_building_vertices", owner),
            ] {
                for vertex in case[key].as_array().unwrap() {
                    buildings[vertex.as_u64().unwrap() as usize] = Some(Building {
                        owner: seat,
                        kind: crate::game_model::BuildingKind::Settlement,
                    });
                }
            }
            assert_eq!(
                longest_route_length(setup.board.graph, &roads, &buildings, owner),
                case["expected_length"].as_u64().unwrap() as u8,
                "{}",
                case["name"]
            );
        }
    }
}

#[cfg(test)]
mod award_tests {
    use super::*;
    #[test]
    fn base_rules_br035_incumbent_threshold_ties_takeover_and_supply_literals() {
        let data: serde_json::Value =
            serde_json::from_str(include_str!("../../tests/support/longest-route.json")).unwrap();
        let mut count = 0;
        for sequence in data["award_sequences"].as_array().unwrap() {
            let mut holder = sequence["initial_holder"]
                .as_u64()
                .map(|id| PlayerId::try_from(id as u8).unwrap());
            for step in sequence["steps"].as_array().unwrap() {
                let lengths =
                    std::array::from_fn(|id| step["route_lengths"][id].as_u64().unwrap() as u8);
                let expected = step["expected_holder"]
                    .as_u64()
                    .map(|id| PlayerId::try_from(id as u8).unwrap());
                assert_eq!(
                    holder.map(|seat| seat.index()),
                    step["prior_holder"].as_u64().map(|id| id as usize)
                );
                holder = reconcile_longest_route(&lengths, holder);
                assert_eq!(holder, expected, "{}", step["name"]);
                count += 1;
            }
        }
        assert_eq!(count, 16);
    }
}
