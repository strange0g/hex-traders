//! Commit the sole validated exchange without another rate calculation or RNG draw.
use crate::{
    base_rules::BankExchange,
    game_model::{Event, GameState, PlayerId, Resource, Transition},
};

pub(super) fn apply_validated(
    state: &GameState,
    actor: PlayerId,
    give_resource: Resource,
    receive_resource: Resource,
    exchange: BankExchange,
) -> Transition {
    let mut position = state.position.clone();
    position.hands[actor.index()] = exchange.hand;
    position.bank = exchange.bank;
    Transition {
        position,
        revision: state.revision + 1,
        random: None,
        events: vec![Event::BankTraded {
            seat: actor,
            give_resource,
            receive_resource,
            rate: exchange.rate,
        }],
    }
}
