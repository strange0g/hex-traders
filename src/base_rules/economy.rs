//! Pure production and fixed seven obligations.
use crate::game_model::{Board, HexId, Position, Resources};
pub(crate) fn production(board: &Board, position: &Position, total: u8) -> [Resources; 4] {
    let mut claims = [[0u16; 5]; 4];
    for id in 0..HexId::COUNT {
        let hex = HexId::try_from(id as u8).expect("bounded hex ID");
        if board.numbers[id] != Some(total) || hex == position.robber {
            continue;
        }
        let Some(resource) = board.terrain[id].resource() else {
            continue;
        };
        for vertex in board.graph.hex_vertices(hex) {
            if let Some(building) = position.buildings[vertex.index()] {
                claims[building.owner.index()][resource.index()] +=
                    u16::from(building.kind.value());
            }
        }
    }
    let mut allocations = [[0u16; 5]; 4];
    for resource in 0..5 {
        let claim: u16 = claims.iter().map(|hand| hand[resource]).sum();
        let claimants = claims.iter().filter(|hand| hand[resource] != 0).count();
        let stock = u16::from(position.bank.counts()[resource]);
        if claim <= stock || claimants == 1 {
            for seat in 0..4 {
                allocations[seat][resource] = claims[seat][resource].min(stock);
            }
        }
    }
    allocations.map(|counts| Resources::from_wide(counts).expect("allocation bounded by bank"))
}
pub(crate) fn discard_obligations(hands: &[Resources; 4]) -> [u8; 4] {
    hands.map(|hand| {
        let total = hand.total();
        if total > 7 {
            u8::try_from(total / 2).expect("at most ninety-five resource cards")
        } else {
            0
        }
    })
}

pub(crate) fn validate_discard(
    hand: &Resources,
    resources: &Resources,
    quantity: u8,
) -> Result<(), crate::game_model::RuleError> {
    if resources.total() != u16::from(quantity)
        || resources
            .counts()
            .iter()
            .zip(hand.counts())
            .any(|(chosen, held)| chosen > held)
    {
        return Err(crate::game_model::RuleError::InvalidDiscard);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_model::{Building, BuildingKind, CardId, GameConfig, Phase, PlayerId};
    fn fixture() -> (Board, Position) {
        let config = GameConfig::fixed(&[0, 1, 2]).unwrap();
        let setup = super::super::fixed_setup(&config);
        let owner = PlayerId::try_from(0).unwrap();
        let mut position = Position {
            config,
            roads: setup.roads,
            road_lengths: [0; 4],
            longest_route_holder: None,
            largest_army_holder: None,
            buildings: setup.buildings,
            second_settlements: setup.second_settlements,
            robber: setup.robber,
            bank: setup.bank,
            hands: setup.hands,
            offer: None,
            deck: std::array::from_fn(|id| CardId::try_from(id as u8).unwrap()),
            draw_cursor: 0,
            card_locations: [crate::game_model::CardLocation::Undrawn; 25],
            played_non_vp: false,
            final_scores: None,
            starting_player: owner,
            active_player: owner,
            turn: 1,
            phase: Phase::PreRoll,
            last_dice: None,
        };
        position.roads = [None; 72];
        position.buildings = [None; 54];
        position.bank = Resources::try_from([19; 5]).unwrap();
        position.hands = [Resources::default(); 4];
        // Independent source incidence: H1 field8 V1/V8/V9; H15 forest8 V42.
        for (id, owner, kind) in [
            (1, 0, BuildingKind::City),
            (8, 0, BuildingKind::Settlement),
            (9, 1, BuildingKind::Settlement),
            (42, 2, BuildingKind::City),
        ] {
            position.buildings[id] = Some(Building {
                owner: PlayerId::try_from(owner).unwrap(),
                kind,
            });
        }
        (setup.board, position)
    }
    #[test]
    fn base_rules_br010_accumulates_matching_hexes_city_and_robber() {
        let (board, mut position) = fixture();
        assert_eq!(
            production(&board, &position, 8).map(|grant| *grant.counts()),
            [[0, 0, 0, 3, 0], [0, 0, 0, 1, 0], [0, 2, 0, 0, 0], [0; 5]]
        );
        position.robber = HexId::try_from(1).unwrap();
        assert_eq!(
            production(&board, &position, 8).map(|grant| *grant.counts()),
            [[0; 5], [0; 5], [0, 2, 0, 0, 0], [0; 5]]
        );
        assert_eq!(production(&board, &position, 7), [Resources::default(); 4]);
    }
    #[test]
    fn base_rules_br011_shortages_use_distinct_claimants_and_sole_remainder() {
        let (board, mut position) = fixture();
        position.bank = Resources::try_from([19, 19, 19, 1, 19]).unwrap();
        position.hands[2] = Resources::try_from([0, 0, 0, 18, 0]).unwrap();
        assert_eq!(
            production(&board, &position, 8).map(|grant| *grant.counts()),
            [[0; 5], [0; 5], [0, 2, 0, 0, 0], [0; 5]]
        );
        position.buildings[9] = None;
        assert_eq!(
            production(&board, &position, 8).map(|grant| *grant.counts()),
            [[0, 0, 0, 1, 0], [0; 5], [0, 2, 0, 0, 0], [0; 5]]
        );
        position.bank = Resources::try_from([19, 19, 19, 0, 19]).unwrap();
        position.hands[2] = Resources::try_from([0, 0, 0, 19, 0]).unwrap();
        assert_eq!(
            production(&board, &position, 8).map(|grant| *grant.counts()),
            [[0; 5], [0; 5], [0, 2, 0, 0, 0], [0; 5]]
        );
    }
    #[test]
    fn base_rules_br012_obligations_are_fixed_quantity_not_development_cards() {
        let hands = [[0; 5], [7, 0, 0, 0, 0], [8, 0, 0, 0, 0], [9, 8, 2, 0, 0]]
            .map(|counts| Resources::try_from(counts).unwrap());
        assert_eq!(discard_obligations(&hands), [0, 0, 4, 9]);
    }
    #[test]
    fn base_rules_br015_robber_leaves_another_same_resource_hex_producing() {
        let (_, mut position) = fixture();
        // Independent variable seed7943 source order; H2 and H13 are both pasture5.
        let order = [
            17, 6, 18, 0, 15, 11, 8, 5, 10, 9, 7, 2, 13, 4, 12, 14, 16, 3, 1,
        ];
        let terrain = order.map(|i| super::super::CANONICAL_TERRAIN[i]);
        let board = super::super::variable_setup(terrain, [1, 5, 4, 6, 3, 2]).board;
        assert_eq!(
            (board.terrain[2], board.numbers[2]),
            (crate::Terrain::Pasture, Some(5))
        );
        assert_eq!(
            (board.terrain[13], board.numbers[13]),
            (crate::Terrain::Pasture, Some(5))
        );
        position.config = GameConfig::variable(&[0, 1, 2]).unwrap();
        position.buildings = [None; 54];
        for (vertex, owner, kind) in [
            (9, 0, BuildingKind::Settlement),
            (39, 1, BuildingKind::City),
        ] {
            position.buildings[vertex] = Some(Building {
                owner: PlayerId::try_from(owner).unwrap(),
                kind,
            });
        }
        position.robber = HexId::try_from(3).unwrap();
        assert_eq!(
            production(&board, &position, 5).map(|g| *g.counts()),
            [[0, 0, 1, 0, 0], [0, 0, 2, 0, 0], [0; 5], [0; 5]]
        );
        position.robber = HexId::try_from(2).unwrap();
        assert_eq!(
            production(&board, &position, 5).map(|g| *g.counts()),
            [[0; 5], [0, 0, 2, 0, 0], [0; 5], [0; 5]]
        );
    }
}
