//! Atomic paid construction/purchases, affected awards and active-player victory.
use crate::{
    base_rules::{city_cost, development_cost, road_cost, settlement_cost},
    game_model::{
        Board, Building, BuildingKind, CardLocation, Command, EdgeId, Event, GameState, PlayerId,
        Position, RuleError, Transition,
    },
};

pub(super) fn apply_validated(
    board: &Board,
    state: &GameState,
    actor: PlayerId,
    command: &Command,
) -> Result<Transition, RuleError> {
    let mut position = state.position.clone();
    let cost = match command {
        Command::BuyDevelopmentCard => development_cost(),
        Command::BuildRoad { .. } => road_cost(),
        Command::BuildSettlement { .. } => settlement_cost(),
        Command::UpgradeCity { .. } => city_cost(),
        _ => unreachable!("validated construction command"),
    };
    position.hands[actor.index()] = position.hands[actor.index()]
        .checked_sub(&cost)
        .expect("validated construction cost");
    position.bank = position
        .bank
        .checked_add(&cost)
        .expect("conserved bank bounds");
    let mut events = vec![Event::ConstructionPaid {
        seat: actor,
        resources: cost,
    }];
    match *command {
        Command::BuyDevelopmentCard => {
            // Schema 1 draws index zero first; purchase consumes no random words.
            let card = position.deck[usize::from(position.draw_cursor)];
            position.draw_cursor += 1;
            position.card_locations[card.index()] = CardLocation::Held {
                owner: actor,
                acquired_turn: position.turn,
            };
            events.push(Event::DevelopmentPurchased {
                seat: actor,
                card,
                remaining: 25 - position.draw_cursor,
            });
        }
        Command::BuildRoad { edge } => {
            install_road(board, &mut position, actor, edge, &mut events);
        }
        Command::BuildSettlement { vertex } => {
            position.buildings[vertex.index()] = Some(Building {
                owner: actor,
                kind: BuildingKind::Settlement,
            });
            events.push(Event::SettlementPlaced {
                seat: actor,
                vertex,
            });
            let owners = position
                .config
                .seat_order()
                .iter()
                .copied()
                .filter(|&owner| {
                    owner != actor
                        && board
                            .graph
                            .vertex_edges(vertex)
                            .iter()
                            .filter(|edge| position.roads[edge.index()] == Some(owner))
                            .count()
                            >= 2
                })
                .collect::<Vec<_>>();
            super::refresh_routes(board, &mut position, &owners, &mut events);
        }
        Command::UpgradeCity { vertex } => {
            position.buildings[vertex.index()]
                .as_mut()
                .expect("validated owned settlement")
                .kind = BuildingKind::City;
            events.push(Event::CityUpgraded {
                seat: actor,
                vertex,
            });
        }
        _ => unreachable!("validated construction command"),
    }
    super::gameplay::check_victory(&mut position, &mut events);
    Ok(Transition {
        position,
        revision: state.revision + 1,
        random: None,
        events,
    })
}

/// Paid and free roads install and reconcile the same authoritative road graph.
pub(super) fn install_road(
    board: &Board,
    position: &mut Position,
    actor: PlayerId,
    edge: EdgeId,
    events: &mut Vec<Event>,
) {
    position.roads[edge.index()] = Some(actor);
    events.push(Event::RoadPlaced { seat: actor, edge });
    super::refresh_routes(board, position, &[actor], events);
}
