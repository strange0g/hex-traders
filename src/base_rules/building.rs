//! Shared placement legality and fixed paid-construction costs.
use crate::game_model::{
    Board, BoardGraph, Building, BuildingKind, EdgeId, PlayerId, Position, Resources, RuleError,
    VertexId,
};

pub(crate) fn settlement_cost() -> Resources {
    Resources::try_from([1, 1, 1, 1, 0]).expect("fixed settlement cost")
}

pub(crate) fn validate_paid_settlement(
    board: &Board,
    position: &Position,
    owner: PlayerId,
    vertex: VertexId,
) -> Result<(), RuleError> {
    super::validate_setup_settlement(board, &position.buildings, vertex)?;
    if position
        .buildings
        .iter()
        .flatten()
        .filter(|building| building.owner == owner && building.kind == BuildingKind::Settlement)
        .count()
        >= 5
    {
        return Err(RuleError::NoPieces);
    }
    if !board
        .graph
        .vertex_edges(vertex)
        .iter()
        .any(|edge| position.roads[edge.index()] == Some(owner))
    {
        return Err(RuleError::Disconnected);
    }
    position.hands[owner.index()]
        .checked_sub(&settlement_cost())
        .map_err(|_| RuleError::InsufficientResources)?;
    Ok(())
}

pub(crate) fn road_cost() -> Resources {
    Resources::try_from([1, 1, 0, 0, 0]).expect("fixed road cost")
}

pub(crate) fn city_cost() -> Resources {
    Resources::try_from([0, 0, 0, 2, 3]).expect("fixed city cost")
}

pub(crate) fn validate_city_upgrade(
    position: &Position,
    owner: PlayerId,
    vertex: VertexId,
) -> Result<(), RuleError> {
    if position.buildings[vertex.index()]
        != Some(Building {
            owner,
            kind: BuildingKind::Settlement,
        })
    {
        return Err(RuleError::NotOwned);
    }
    if position
        .buildings
        .iter()
        .flatten()
        .filter(|building| building.owner == owner && building.kind == BuildingKind::City)
        .count()
        >= 4
    {
        return Err(RuleError::NoPieces);
    }
    position.hands[owner.index()]
        .checked_sub(&city_cost())
        .map_err(|_| RuleError::InsufficientResources)?;
    Ok(())
}

pub(crate) fn validate_paid_road(
    graph: &BoardGraph,
    position: &Position,
    owner: PlayerId,
    edge: EdgeId,
) -> Result<(), RuleError> {
    validate_road_placement(graph, &position.roads, &position.buildings, owner, edge)?;
    position.hands[owner.index()]
        .checked_sub(&road_cost())
        .map_err(|_| RuleError::InsufficientResources)?;
    Ok(())
}

pub(crate) fn validate_road_placement(
    graph: &BoardGraph,
    roads: &[Option<PlayerId>; EdgeId::COUNT],
    buildings: &[Option<Building>; VertexId::COUNT],
    owner: PlayerId,
    edge: EdgeId,
) -> Result<(), RuleError> {
    if roads[edge.index()].is_some() {
        return Err(RuleError::Occupied);
    }
    if roads.iter().filter(|&&seat| seat == Some(owner)).count() >= 15 {
        return Err(RuleError::NoPieces);
    }
    if !graph
        .edge_vertices(edge)
        .iter()
        .any(|&vertex| match buildings[vertex.index()] {
            Some(building) => building.owner == owner,
            None => graph
                .vertex_edges(vertex)
                .iter()
                .any(|edge| roads[edge.index()] == Some(owner)),
        })
    {
        return Err(RuleError::Disconnected);
    }
    Ok(())
}

/// Activation and continuation share the same ordinary placement validator.
pub(crate) fn validate_road_building(
    graph: &BoardGraph,
    position: &Position,
    owner: PlayerId,
) -> Result<(), RuleError> {
    if position
        .roads
        .iter()
        .filter(|&&seat| seat == Some(owner))
        .count()
        >= 15
    {
        return Err(RuleError::NoPieces);
    }
    if !(0..EdgeId::COUNT as u8).any(|id| {
        validate_road_placement(
            graph,
            &position.roads,
            &position.buildings,
            owner,
            EdgeId::try_from(id).expect("bounded edge"),
        )
        .is_ok()
    }) {
        return Err(RuleError::Disconnected);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_model::BuildingKind;
    #[test]
    fn base_rules_br017_empty_connected_owner_and_blocked_endpoints() {
        let graph = BoardGraph::base_game();
        let owner = PlayerId::try_from(0).unwrap();
        let opponent = PlayerId::try_from(1).unwrap();
        let mut roads = [None; 72];
        let mut buildings = [None; 54];
        let target = EdgeId::try_from(18).unwrap();
        assert_eq!(
            validate_road_placement(graph, &roads, &buildings, owner, target),
            Err(RuleError::Disconnected)
        );
        buildings[3] = Some(Building {
            owner,
            kind: BuildingKind::Settlement,
        });
        roads[6] = Some(owner);
        roads[10] = Some(owner);
        assert_eq!(
            validate_road_placement(graph, &roads, &buildings, owner, target),
            Ok(())
        );
        buildings[11] = Some(Building {
            owner: opponent,
            kind: BuildingKind::Settlement,
        });
        assert_eq!(
            validate_road_placement(graph, &roads, &buildings, owner, target),
            Err(RuleError::Disconnected)
        );
        roads[23] = Some(opponent);
        assert_eq!(
            validate_road_placement(graph, &roads, &buildings, owner, target),
            Err(RuleError::Disconnected)
        );
        roads[23] = Some(owner);
        assert_eq!(
            validate_road_placement(graph, &roads, &buildings, owner, target),
            Ok(())
        );
        // Opponent E24 and own E23 share empty V16; only a building blocks it.
        roads[24] = Some(opponent);
        assert_eq!(
            validate_road_placement(graph, &roads, &buildings, owner, target),
            Ok(())
        );
        roads[23] = None;
        buildings[11].as_mut().unwrap().owner = owner;
        assert_eq!(
            validate_road_placement(graph, &roads, &buildings, owner, target),
            Ok(())
        );
        for occupant in [owner, opponent] {
            roads[18] = Some(occupant);
            assert_eq!(
                validate_road_placement(graph, &roads, &buildings, owner, target),
                Err(RuleError::Occupied)
            );
        }
    }
    #[test]
    fn base_rules_br017_fifteen_roads_exhaust_inventory() {
        let graph = BoardGraph::base_game();
        let owner = PlayerId::try_from(0).unwrap();
        let mut roads = [None; 72];
        for id in [0, 1, 2, 3, 4, 5, 9, 17, 22, 32, 38, 48, 53, 61, 65] {
            roads[id] = Some(owner);
        }
        assert_eq!(
            validate_road_placement(
                graph,
                &roads,
                &[None; 54],
                owner,
                EdgeId::try_from(6).unwrap()
            ),
            Err(RuleError::NoPieces)
        );
    }
}
