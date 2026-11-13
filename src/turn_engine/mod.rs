//! Turn sequencing and game creation.

mod creation;
mod development;
mod invariants;
mod negotiation;
pub(crate) use invariants::validate_game;

pub(crate) use creation::{create_game, validate_created_game};

mod building;
mod gameplay;
mod robber;
mod setup;
mod trade;
use crate::game_model::{Board, Command, GameState, Phase, PlayerId, RuleError, Transition};

/// Read-only validation also prepares a bank exchange for execution to consume once.
pub(crate) fn validate_action(
    board: &Board,
    state: &GameState,
    actor: PlayerId,
    command: &Command,
) -> Result<Option<crate::base_rules::BankExchange>, RuleError> {
    if matches!(state.position.phase, Phase::Finished { .. }) {
        return Err(RuleError::GameFinished);
    }
    if let Command::DiscardResources { resources } = command {
        let quantity = required_discard(state, actor)?;
        return crate::base_rules::validate_discard(
            &state.position.hands[actor.index()],
            resources,
            quantity,
        )
        .map(|()| None);
    }
    if matches!(
        command,
        Command::ProposeTrade { .. }
            | Command::AcceptTrade { .. }
            | Command::RejectTrade { .. }
            | Command::CancelTrade { .. }
    ) {
        negotiation::validate(state, actor, command)?;
        state
            .revision
            .checked_add(1)
            .ok_or(RuleError::CounterExhausted)?;
        return Ok(None);
    }
    if actor != state.position.active_player {
        return Err(RuleError::WrongActor);
    }
    let mut exchange = None;
    match command {
        Command::PlayRoadBuilding { card } => {
            if !matches!(state.position.phase, Phase::PreRoll | Phase::Action) {
                return Err(RuleError::WrongPhase);
            }
            crate::base_rules::validate_card_play(
                &state.position,
                actor,
                *card,
                crate::game_model::DevelopmentKind::RoadBuilding,
            )?;
            crate::base_rules::validate_road_building(board.graph, &state.position, actor)?;
        }
        Command::PlaceFreeRoad { edge } => {
            if !matches!(state.position.phase, Phase::FreeRoads { .. }) {
                return Err(RuleError::WrongPhase);
            }
            crate::base_rules::validate_road_placement(
                board.graph,
                &state.position.roads,
                &state.position.buildings,
                actor,
                *edge,
            )?;
        }
        Command::PlayKnight { card } => {
            if !matches!(state.position.phase, Phase::PreRoll | Phase::Action) {
                return Err(RuleError::WrongPhase);
            }
            crate::base_rules::validate_card_play(
                &state.position,
                actor,
                *card,
                crate::game_model::DevelopmentKind::Knight,
            )?;
        }
        Command::BuyDevelopmentCard => {
            if state.position.phase != Phase::Action {
                return Err(RuleError::WrongPhase);
            }
            crate::base_rules::validate_development_purchase(&state.position, actor)?;
        }
        Command::PlayInvention { card, resources } => {
            if !matches!(state.position.phase, Phase::PreRoll | Phase::Action) {
                return Err(RuleError::WrongPhase);
            }
            crate::base_rules::validate_card_play(
                &state.position,
                actor,
                *card,
                crate::game_model::DevelopmentKind::Invention,
            )?;
            crate::base_rules::validate_invention(&state.position.bank, resources)?;
        }
        Command::PlayMonopoly { card, .. } => {
            if !matches!(state.position.phase, Phase::PreRoll | Phase::Action) {
                return Err(RuleError::WrongPhase);
            }
            crate::base_rules::validate_card_play(
                &state.position,
                actor,
                *card,
                crate::game_model::DevelopmentKind::Monopoly,
            )?;
        }
        Command::BankTrade {
            give_resource,
            receive_resource,
        } => {
            if state.position.phase != Phase::Action {
                return Err(RuleError::WrongPhase);
            }
            exchange = Some(crate::base_rules::calculate_bank_trade(
                board,
                &state.position,
                actor,
                *give_resource,
                *receive_resource,
            )?);
        }
        Command::DiscardResources { .. } => unreachable!("discard validated above"),
        Command::ProposeTrade { .. }
        | Command::AcceptTrade { .. }
        | Command::RejectTrade { .. }
        | Command::CancelTrade { .. } => unreachable!("negotiation validated above"),
        Command::MoveRobber { .. } | Command::ChooseRobberVictim { .. } => {
            robber::validate(state, command)?;
        }
        Command::BuildRoad { edge } => {
            if state.position.phase != Phase::Action {
                return Err(RuleError::WrongPhase);
            }
            crate::base_rules::validate_paid_road(board.graph, &state.position, actor, *edge)?;
        }
        Command::BuildSettlement { vertex } => {
            if state.position.phase != Phase::Action {
                return Err(RuleError::WrongPhase);
            }
            crate::base_rules::validate_paid_settlement(board, &state.position, actor, *vertex)?;
        }
        Command::UpgradeCity { vertex } => {
            if state.position.phase != Phase::Action {
                return Err(RuleError::WrongPhase);
            }
            crate::base_rules::validate_city_upgrade(&state.position, actor, *vertex)?;
        }
        Command::EndTurn => {
            if state.position.phase != Phase::Action {
                return Err(RuleError::WrongPhase);
            }
            state
                .position
                .turn
                .checked_add(1)
                .ok_or(RuleError::CounterExhausted)?;
        }
        Command::RollDice => {
            if state.position.phase != Phase::PreRoll {
                return Err(RuleError::WrongPhase);
            }
        }
        Command::PlaceSetupSettlement { .. } | Command::PlaceSetupRoad { .. } => {
            setup::validate_placement(board, state, command)?
        }
    }
    state
        .revision
        .checked_add(1)
        .ok_or(RuleError::CounterExhausted)?;
    Ok(exchange)
}

pub(crate) fn evaluate(
    board: &Board,
    state: &GameState,
    actor: PlayerId,
    command: &Command,
) -> Result<Transition, RuleError> {
    let exchange = validate_action(board, state, actor, command)?;
    let mut next = match command {
        Command::ProposeTrade { .. }
        | Command::AcceptTrade { .. }
        | Command::RejectTrade { .. }
        | Command::CancelTrade { .. } => negotiation::apply(state, actor, command),
        Command::PlayRoadBuilding { card } => Ok(development::road_building(state, actor, *card)),
        Command::PlaceFreeRoad { edge } => Ok(development::free_road(board, state, actor, *edge)),
        Command::PlayKnight { card } => Ok(development::knight(state, actor, *card)),
        Command::PlayInvention { card, resources } => {
            Ok(development::invention(state, actor, *card, *resources))
        }
        Command::PlayMonopoly { card, resource } => {
            Ok(development::monopoly(state, actor, *card, *resource))
        }
        Command::BankTrade {
            give_resource,
            receive_resource,
        } => Ok(trade::apply_validated(
            state,
            actor,
            *give_resource,
            *receive_resource,
            exchange.expect("validated bank trade supplies its checked exchange"),
        )),
        Command::DiscardResources { resources } => gameplay::discard(state, actor, *resources),
        Command::MoveRobber { .. } | Command::ChooseRobberVictim { .. } => {
            robber::apply_validated(board, state, actor, command)
        }
        Command::BuyDevelopmentCard
        | Command::BuildRoad { .. }
        | Command::BuildSettlement { .. }
        | Command::UpgradeCity { .. } => building::apply_validated(board, state, actor, command),
        Command::EndTurn => gameplay::end_turn(state),
        Command::RollDice => gameplay::apply_validated(board, state, actor),
        Command::PlaceSetupSettlement { .. } | Command::PlaceSetupRoad { .. } => {
            setup::apply_validated(board, state, actor, command)
        }
    }?;
    negotiation::close_expired(state, &mut next);
    Ok(next)
}

pub(crate) fn required_discard(state: &GameState, actor: PlayerId) -> Result<u8, RuleError> {
    let Phase::Discard { obligations } = state.position.phase else {
        return Err(RuleError::WrongPhase);
    };
    let quantity = obligations[actor.index()];
    if quantity == 0 {
        return Err(RuleError::WrongActor);
    }
    state
        .revision
        .checked_add(1)
        .ok_or(RuleError::CounterExhausted)?;
    Ok(quantity)
}

pub(crate) fn refresh_routes(
    board: &Board,
    position: &mut crate::game_model::Position,
    owners: &[PlayerId],
    events: &mut Vec<crate::game_model::Event>,
) {
    for &owner in owners {
        position.road_lengths[owner.index()] = crate::base_rules::longest_route_length(
            board.graph,
            &position.roads,
            &position.buildings,
            owner,
        );
    }
    let holder = crate::base_rules::reconcile_longest_route(
        &position.road_lengths,
        position.longest_route_holder,
    );
    if holder != position.longest_route_holder {
        events.push(crate::game_model::Event::LongestRouteChanged {
            previous: position.longest_route_holder,
            holder,
        });
        position.longest_route_holder = holder;
    }
}
