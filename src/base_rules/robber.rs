//! Public adjacency and actual-card indexing; randomness belongs to turn-engine.
use crate::game_model::{BoardGraph, Building, HexId, PlayerId, Resource, Resources, VertexId};

pub(crate) fn robber_victims(
    graph: &BoardGraph,
    buildings: &[Option<Building>; VertexId::COUNT],
    actor: PlayerId,
    hex: HexId,
) -> [Option<PlayerId>; 4] {
    let mut victims = [None; 4];
    for vertex in graph.hex_vertices(hex) {
        if let Some(building) = buildings[vertex.index()]
            && building.owner != actor
        {
            victims[building.owner.index()] = Some(building.owner);
        }
    }
    victims
}

pub(crate) fn card_at_index(hand: &Resources, index: u8) -> Option<Resource> {
    let mut cumulative = 0u16;
    for (resource, &count) in [
        Resource::Brick,
        Resource::Wood,
        Resource::Wool,
        Resource::Wheat,
        Resource::Ore,
    ]
    .into_iter()
    .zip(hand.counts())
    {
        cumulative += u16::from(count);
        if u16::from(index) < cumulative {
            return Some(resource);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn base_rules_br014_indexes_cards_in_fixed_weighted_order() {
        let hand = Resources::try_from([1, 3, 0, 1, 2]).unwrap();
        let expected = [
            Resource::Brick,
            Resource::Wood,
            Resource::Wood,
            Resource::Wood,
            Resource::Wheat,
            Resource::Ore,
            Resource::Ore,
        ];
        for (index, resource) in expected.into_iter().enumerate() {
            assert_eq!(card_at_index(&hand, index as u8), Some(resource));
        }
        assert_eq!(card_at_index(&hand, 7), None);
        assert_eq!(card_at_index(&Resources::default(), 0), None);
        let full = Resources::try_from([19; 5]).unwrap();
        assert_eq!(card_at_index(&full, 94), Some(Resource::Ore));
        assert_eq!(card_at_index(&full, 95), None);
    }
}
