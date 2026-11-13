//! Purchase cost and availability, shared by command validation and queries.
use crate::game_model::{
    CardId, CardLocation, DevelopmentKind, PlayerId, Position, Resource, Resources, RuleError,
};

pub(crate) fn development_cost() -> Resources {
    Resources::try_from([0, 0, 1, 1, 1]).expect("fixed development-card cost")
}

pub(crate) fn validate_development_purchase(
    position: &Position,
    owner: PlayerId,
) -> Result<(), RuleError> {
    if position.draw_cursor >= 25 {
        return Err(RuleError::EmptyDeck);
    }
    position.hands[owner.index()]
        .checked_sub(&development_cost())
        .map_err(|_| RuleError::InsufficientResources)?;
    Ok(())
}

/// One shared per-card predicate, independent of phase and effect choices.
pub(crate) fn validate_card_play(
    position: &Position,
    owner: PlayerId,
    card: CardId,
    kind: DevelopmentKind,
) -> Result<(), RuleError> {
    if position.played_non_vp
        || card.kind() != kind
        || !matches!(position.card_locations[card.index()], CardLocation::Held { owner: holder, acquired_turn } if holder == owner && acquired_turn < position.turn)
    {
        return Err(RuleError::CardNotPlayable);
    }
    Ok(())
}

pub(crate) fn monopoly_hands(
    position: &Position,
    owner: PlayerId,
    resource: Resource,
) -> ([Resources; 4], [u8; 4]) {
    let mut hands = position.hands;
    let mut transfers = [0; 4];
    for &seat in position.config.seat_order() {
        if seat == owner {
            continue;
        }
        let count = hands[seat.index()].counts()[resource.index()];
        transfers[seat.index()] = count;
        let mut counts = [0; 5];
        counts[resource.index()] = count;
        let bundle = Resources::try_from(counts).expect("bounded opponent hand");
        hands[seat.index()] = hands[seat.index()]
            .checked_sub(&bundle)
            .expect("selected complete resource count");
        hands[owner.index()] = hands[owner.index()]
            .checked_add(&bundle)
            .expect("conserved resource bounds");
    }
    (hands, transfers)
}

/// Validate the approved two-card or sole-card bank choice.
pub(crate) fn validate_invention(bank: &Resources, selection: &Resources) -> Result<(), RuleError> {
    let available = bank.total().min(2);
    if available == 0 {
        return Err(RuleError::BankShortage);
    }
    if selection.total() != available {
        return Err(RuleError::InvalidInput);
    }
    bank.checked_sub(selection)
        .map_err(|_| RuleError::BankShortage)?;
    Ok(())
}

/// One bounded legal choice for card-ID queries, without enumerating bundles.
pub(crate) fn first_invention_choice(bank: &Resources) -> Option<Resources> {
    let mut remaining = bank.total().min(2) as u8;
    if remaining == 0 {
        return None;
    }
    let mut selection = [0; 5];
    for (chosen, &available) in selection.iter_mut().zip(bank.counts()) {
        *chosen = remaining.min(available);
        remaining -= *chosen;
    }
    Some(Resources::try_from(selection).expect("choice bounded by bank"))
}

/// Counts only revealed Knights; the incumbent keeps every tie.
pub(crate) fn reconcile_largest_army(
    position: &Position,
    challenger: PlayerId,
) -> Option<PlayerId> {
    let count = position.played_knights(challenger);
    if count >= 3
        && position
            .largest_army_holder
            .is_none_or(|holder| count > position.played_knights(holder))
    {
        Some(challenger)
    } else {
        position.largest_army_holder
    }
}
