//! Free placement validation and atomic setup completion.

use crate::{
    base_rules::{starting_resources, validate_setup_road, validate_setup_settlement},
    game_model::{
        Board, Building, Command, Event, GameState, Phase, PlayerId, RuleError, Transition,
    },
};

pub(super) fn validate_placement(
    board: &Board,
    state: &GameState,
    command: &Command,
) -> Result<(), RuleError> {
    match (state.position.phase, command) {
        (Phase::SetupSettlement { .. }, Command::PlaceSetupSettlement { vertex }) => {
            validate_setup_settlement(board, &state.position.buildings, *vertex)?;
        }
        (Phase::SetupRoad { settlement, .. }, Command::PlaceSetupRoad { edge }) => {
            validate_setup_road(board, &state.position.roads, settlement, *edge)?;
        }
        _ => return Err(RuleError::WrongPhase),
    }
    Ok(())
}

pub(super) fn apply_validated(
    board: &Board,
    state: &GameState,
    actor: PlayerId,
    command: &Command,
) -> Result<Transition, RuleError> {
    let mut position = state.position.clone();
    let mut events = Vec::new();
    match (position.phase, command) {
        (Phase::SetupSettlement { round, .. }, Command::PlaceSetupSettlement { vertex }) => {
            position.buildings[vertex.index()] = Some(Building {
                owner: actor,
                kind: crate::game_model::BuildingKind::Settlement,
            });
            if round == 2 {
                position.second_settlements[actor.index()] = Some(*vertex);
            }
            position.phase = Phase::SetupRoad {
                actor,
                round,
                settlement: *vertex,
            };
            events.push(Event::SettlementPlaced {
                seat: actor,
                vertex: *vertex,
            });
            events.push(Event::PhaseChanged {
                phase: position.phase,
            });
        }
        (Phase::SetupRoad { round, .. }, Command::PlaceSetupRoad { edge }) => {
            position.roads[edge.index()] = Some(actor);
            events.push(Event::RoadPlaced {
                seat: actor,
                edge: *edge,
            });
            super::refresh_routes(board, &mut position, &[actor], &mut events);
            let order = position.config.seat_order();
            let current = order
                .iter()
                .position(|&seat| seat == actor)
                .expect("validated actor");
            let start = order
                .iter()
                .position(|&seat| seat == position.starting_player)
                .expect("validated starter");
            let offset = (current + order.len() - start) % order.len();
            if round == 1 {
                let next = if offset + 1 == order.len() {
                    actor
                } else {
                    order[(current + 1) % order.len()]
                };
                position.active_player = next;
                position.phase = Phase::SetupSettlement {
                    actor: next,
                    round: if offset + 1 == order.len() { 2 } else { 1 },
                };
            } else if offset != 0 {
                let next = order[(current + order.len() - 1) % order.len()];
                position.active_player = next;
                position.phase = Phase::SetupSettlement {
                    actor: next,
                    round: 2,
                };
            } else {
                for &seat in order {
                    let vertex = position.second_settlements[seat.index()]
                        .expect("completed second settlement");
                    let resources = starting_resources(board, vertex);
                    position.hands[seat.index()] = resources;
                    position.bank = position
                        .bank
                        .checked_sub(&resources)
                        .expect("starting resources fit bank");
                    events.push(Event::StartingResources { seat, resources });
                }
                position.turn = 1;
                position.active_player = position.starting_player;
                position.phase = Phase::PreRoll;
                events.push(Event::TurnStarted {
                    seat: position.active_player,
                    turn: 1,
                });
            }
            events.push(Event::PhaseChanged {
                phase: position.phase,
            });
        }
        _ => unreachable!("validated phase and command"),
    }
    Ok(Transition {
        position,
        revision: state.revision + 1,
        random: None,
        events,
    })
}
