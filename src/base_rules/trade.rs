//! One checked maritime exchange; building ownership grants the best port rate.
use crate::game_model::{
    Board, PlayerId, PortKind, Position, Resource, Resources, RuleError, TradeOffer,
};

pub(crate) struct BankExchange {
    pub rate: u8,
    pub hand: Resources,
    pub bank: Resources,
}

pub(crate) fn calculate_bank_trade(
    board: &Board,
    position: &Position,
    owner: PlayerId,
    give: Resource,
    receive: Resource,
) -> Result<BankExchange, RuleError> {
    if give == receive {
        return Err(RuleError::InvalidTrade);
    }
    let rate = board
        .ports
        .iter()
        .filter(|port| {
            board.graph.edge_vertices(port.edge).iter().any(|vertex| {
                position.buildings[vertex.index()].is_some_and(|building| building.owner == owner)
            })
        })
        .map(|port| match port.kind {
            PortKind::Generic => 3,
            PortKind::Special(resource) if resource == give => 2,
            PortKind::Special(_) => 4,
        })
        .min()
        .unwrap_or(4);
    let mut paid = [0; 5];
    paid[give.index()] = rate;
    let paid = Resources::try_from(paid).expect("fixed maritime rate bounds");
    let mut gained = [0; 5];
    gained[receive.index()] = 1;
    let gained = Resources::try_from(gained).expect("one received resource");
    let hand = position.hands[owner.index()]
        .checked_sub(&paid)
        .map_err(|_| RuleError::InsufficientResources)?;
    let bank = position
        .bank
        .checked_sub(&gained)
        .map_err(|_| RuleError::BankShortage)?;
    Ok(BankExchange {
        rate,
        hand: hand
            .checked_add(&gained)
            .expect("conserved incoming resource bounds"),
        bank: bank
            .checked_add(&paid)
            .expect("conserved returned resource bounds"),
    })
}

/// Validate positive, disjoint public terms and the proposer's own payment only.
pub(crate) fn validate_offer_terms(
    hand: &Resources,
    give: &Resources,
    receive: &Resources,
) -> Result<(), RuleError> {
    if give.total() == 0
        || receive.total() == 0
        || give
            .counts()
            .iter()
            .zip(receive.counts())
            .any(|(&a, &b)| a != 0 && b != 0)
    {
        return Err(RuleError::InvalidTrade);
    }
    hand.checked_sub(give)
        .map_err(|_| RuleError::InsufficientResources)?;
    Ok(())
}

/// Privately recheck both holdings and prepare both sides without partial transfer.
pub(crate) fn calculate_offer_exchange(
    position: &Position,
    offer: TradeOffer,
) -> Result<(Resources, Resources), RuleError> {
    let proposer = position.hands[offer.proposer.index()];
    let recipient = position.hands[offer.recipient.index()];
    validate_offer_terms(&proposer, &offer.give, &offer.receive)
        .map_err(|_| RuleError::OfferUnavailable)?;
    validate_offer_terms(&recipient, &offer.receive, &offer.give)
        .map_err(|_| RuleError::OfferUnavailable)?;
    let proposer = proposer
        .checked_sub(&offer.give)
        .and_then(|hand| hand.checked_add(&offer.receive))
        .map_err(|_| RuleError::OfferUnavailable)?;
    let recipient = recipient
        .checked_sub(&offer.receive)
        .and_then(|hand| hand.checked_add(&offer.give))
        .map_err(|_| RuleError::OfferUnavailable)?;
    Ok((proposer, recipient))
}
