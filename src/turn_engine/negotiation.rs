//! Public bilateral offers and their successful-transition lifetime.
use crate::game_model::{
    Command, Event, GameState, Phase, PlayerId, RuleError, TradeOffer, Transition,
};

pub(super) fn validate(
    state: &GameState,
    actor: PlayerId,
    command: &Command,
) -> Result<(), RuleError> {
    let p = &state.position;
    match *command {
        Command::ProposeTrade {
            recipient,
            give,
            receive,
        } => {
            if actor != p.active_player && recipient != p.active_player {
                return Err(RuleError::WrongActor);
            }
            if p.phase != Phase::Action {
                return Err(RuleError::WrongPhase);
            }
            if actor == recipient {
                return Err(RuleError::InvalidTrade);
            }
            if let Some(offer) = p.offer
                && actor != p.active_player
                && actor != offer.proposer
                && actor != offer.recipient
            {
                return Err(RuleError::OfferBusy);
            }
            crate::base_rules::validate_offer_terms(&p.hands[actor.index()], &give, &receive)
        }
        Command::AcceptTrade { offer_id }
        | Command::RejectTrade { offer_id }
        | Command::CancelTrade { offer_id } => {
            if p.phase != Phase::Action {
                return Err(RuleError::WrongPhase);
            }
            let offer = p.offer.ok_or(RuleError::OfferNotFound)?;
            let required = if matches!(command, Command::CancelTrade { .. }) {
                offer.proposer
            } else {
                offer.recipient
            };
            if actor != required {
                return Err(RuleError::WrongActor);
            }
            if offer.id != offer_id || offer.turn != p.turn {
                return Err(RuleError::OfferNotFound);
            }
            if matches!(command, Command::AcceptTrade { .. }) {
                crate::base_rules::validate_offer_terms(
                    &p.hands[actor.index()],
                    &offer.receive,
                    &offer.give,
                )
                .map_err(|_| RuleError::OfferUnavailable)?;
            }
            Ok(())
        }
        _ => unreachable!("negotiation command"),
    }
}

pub(super) fn apply(
    state: &GameState,
    actor: PlayerId,
    command: &Command,
) -> Result<Transition, RuleError> {
    let mut position = state.position.clone();
    let mut events = Vec::new();
    if matches!(command, Command::AcceptTrade { .. }) {
        let offer = position.offer.expect("validated current offer");
        let (proposer, recipient) = crate::base_rules::calculate_offer_exchange(&position, offer)?;
        position.hands[offer.proposer.index()] = proposer;
        position.hands[offer.recipient.index()] = recipient;
        events.push(Event::TradeCompleted { offer });
    }
    if let Some(old) = position.offer.take() {
        events.push(Event::TradeOfferClosed { offer_id: old.id });
    }
    if let Command::ProposeTrade {
        recipient,
        give,
        receive,
    } = *command
    {
        let offer = TradeOffer {
            id: state.revision + 1,
            turn: position.turn,
            proposer: actor,
            recipient,
            give,
            receive,
        };
        position.offer = Some(offer);
        events.push(Event::TradeOfferCreated { offer });
    }
    Ok(Transition {
        position,
        revision: state.revision + 1,
        random: None,
        events,
    })
}

/// Close an unchanged offer once after a successful action, before awards or phase facts.
pub(super) fn close_expired(state: &GameState, next: &mut Transition) {
    let Some(offer) = state.position.offer else {
        return;
    };
    if next.position.offer != Some(offer) {
        return;
    }
    let changed = [offer.proposer, offer.recipient]
        .into_iter()
        .any(|seat| state.position.hands[seat.index()] != next.position.hands[seat.index()]);
    if !changed && next.position.phase == Phase::Action && next.position.turn == offer.turn {
        return;
    }
    next.position.offer = None;
    let index = next
        .events
        .iter()
        .position(|event| {
            matches!(
                event,
                Event::LongestRouteChanged { .. }
                    | Event::LargestArmyChanged { .. }
                    | Event::VictoryPointsRevealed { .. }
                    | Event::GameFinished { .. }
                    | Event::TurnStarted { .. }
                    | Event::PhaseChanged { .. }
            )
        })
        .unwrap_or(next.events.len());
    next.events
        .insert(index, Event::TradeOfferClosed { offer_id: offer.id });
}
