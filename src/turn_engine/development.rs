//! Atomic effects of validated development cards.
use crate::game_model::{
    CardId, CardLocation, Event, GameState, PlayerId, Position, Resource, Resources, Transition,
};

pub(super) fn monopoly(
    state: &GameState,
    actor: PlayerId,
    card: CardId,
    resource: Resource,
) -> Transition {
    let mut position = state.position.clone();
    let (hands, transfers) = crate::base_rules::monopoly_hands(&position, actor, resource);
    position.hands = hands;
    spend_progress(&mut position, actor, card);
    Transition {
        position,
        revision: state.revision + 1,
        random: None,
        events: vec![Event::MonopolyPlayed {
            seat: actor,
            card,
            resource,
            transfers,
        }],
    }
}

pub(super) fn invention(
    state: &GameState,
    actor: PlayerId,
    card: CardId,
    resources: Resources,
) -> Transition {
    let mut position = state.position.clone();
    position.bank = position
        .bank
        .checked_sub(&resources)
        .expect("validated bank choice");
    position.hands[actor.index()] = position.hands[actor.index()]
        .checked_add(&resources)
        .expect("conserved resource bounds");
    spend_progress(&mut position, actor, card);
    Transition {
        position,
        revision: state.revision + 1,
        random: None,
        events: vec![Event::InventionPlayed {
            seat: actor,
            card,
            resources,
        }],
    }
}

fn spend_progress(position: &mut Position, actor: PlayerId, card: CardId) {
    position.card_locations[card.index()] = CardLocation::Spent { owner: actor };
    position.played_non_vp = true;
}

pub(super) fn knight(state: &GameState, actor: PlayerId, card: CardId) -> Transition {
    let mut position = state.position.clone();
    let continuation = match position.phase {
        crate::game_model::Phase::PreRoll => crate::game_model::ResumePhase::PreRoll,
        crate::game_model::Phase::Action => crate::game_model::ResumePhase::Action,
        _ => unreachable!("validated Knight phase"),
    };
    position.card_locations[card.index()] = CardLocation::PlayedKnight { owner: actor };
    position.played_non_vp = true;
    let mut events = vec![Event::KnightPlayed { seat: actor, card }];
    let holder = crate::base_rules::reconcile_largest_army(&position, actor);
    if holder != position.largest_army_holder {
        events.push(Event::LargestArmyChanged {
            previous: position.largest_army_holder,
            holder,
        });
        position.largest_army_holder = holder;
    }
    if !super::gameplay::check_victory(&mut position, &mut events) {
        position.phase = crate::game_model::Phase::RobberMove {
            origin: crate::game_model::RobberOrigin::Knight,
            continuation,
        };
        events.push(Event::PhaseChanged {
            phase: position.phase,
        });
    }
    Transition {
        position,
        revision: state.revision + 1,
        random: None,
        events,
    }
}

pub(super) fn road_building(state: &GameState, actor: PlayerId, card: CardId) -> Transition {
    let mut position = state.position.clone();
    let continuation = match position.phase {
        crate::game_model::Phase::PreRoll => crate::game_model::ResumePhase::PreRoll,
        crate::game_model::Phase::Action => crate::game_model::ResumePhase::Action,
        _ => unreachable!("validated Road Building phase"),
    };
    spend_progress(&mut position, actor, card);
    position.phase = crate::game_model::Phase::FreeRoads {
        remaining: 2,
        continuation,
    };
    let events = vec![
        Event::RoadBuildingPlayed { seat: actor, card },
        Event::PhaseChanged {
            phase: position.phase,
        },
    ];
    Transition {
        position,
        revision: state.revision + 1,
        random: None,
        events,
    }
}

pub(super) fn free_road(
    board: &crate::game_model::Board,
    state: &GameState,
    actor: PlayerId,
    edge: crate::game_model::EdgeId,
) -> Transition {
    let mut position = state.position.clone();
    let crate::game_model::Phase::FreeRoads {
        remaining,
        continuation,
    } = position.phase
    else {
        unreachable!("validated free placement phase")
    };
    let mut events = Vec::new();
    super::building::install_road(board, &mut position, actor, edge, &mut events);
    if !super::gameplay::check_victory(&mut position, &mut events) {
        position.phase = if remaining > 1
            && crate::base_rules::validate_road_building(board.graph, &position, actor).is_ok()
        {
            crate::game_model::Phase::FreeRoads {
                remaining: remaining - 1,
                continuation,
            }
        } else {
            match continuation {
                crate::game_model::ResumePhase::PreRoll => crate::game_model::Phase::PreRoll,
                crate::game_model::ResumePhase::Action => crate::game_model::Phase::Action,
            }
        };
        events.push(Event::PhaseChanged {
            phase: position.phase,
        });
    }
    Transition {
        position,
        revision: state.revision + 1,
        random: None,
        events,
    }
}
