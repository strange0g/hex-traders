//! Host receipts and explicit viewer-safe projections of original facts.
use super::{Engine, Viewer, command::Rejection};
use crate::game_model::{
    CardId, Command, EdgeId, Event, HexId, Phase, PlayerId, Resource, Resources, TradeOffer,
    VertexId,
};

/// Accepted host-private command and resolved facts; project before sharing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Receipt {
    /// Native API contract version.
    pub api_version: u32,
    /// Rules edition and digital policies.
    pub rules_version: &'static str,
    /// Pinned generator and sampling version.
    pub random_algorithm: &'static str,
    /// Accepted actor.
    pub actor: PlayerId,
    /// Accepted intent; may contain private choices.
    pub command: Command,
    /// Revision before this command.
    pub previous_revision: u64,
    /// Revision after this command.
    pub revision: u64,
    /// Ordered host facts.
    pub events: Vec<Event>,
    /// Final true scores by seat ID only on completion; unused seats are zero. Host-private.
    pub final_scores: Option<[u8; 4]>,
}

/// Explicit safe facts, distinct from host-private command events.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectedEvent {
    /// A public bilateral proposal was created.
    TradeOfferCreated { offer: TradeOffer },
    /// Both public offer bundles were exchanged atomically.
    TradeCompleted { offer: TradeOffer },
    /// A public proposal closed without an affordability explanation.
    TradeOfferClosed { offer_id: u64 },
    /// Public progress identity and start of mandatory free placements.
    RoadBuildingPlayed { seat: PlayerId, card: CardId },
    /// Public revealed Knight identity.
    KnightPlayed { seat: PlayerId, card: CardId },
    /// Public Army acquisition or transfer; ties keep the holder.
    LargestArmyChanged {
        previous: Option<PlayerId>,
        holder: Option<PlayerId>,
    },
    /// Buyer/deck count are public; canonical identity (and its kind) is buyer-only.
    DevelopmentPurchased {
        seat: PlayerId,
        card: Option<CardId>,
        remaining: u8,
    },
    /// Public Monopoly identity, selected resource, and transfers by seat ID.
    MonopolyPlayed {
        seat: PlayerId,
        card: CardId,
        resource: Resource,
        transfers: [u8; 4],
    },
    /// Public Invention identity and bank resources taken.
    InventionPlayed {
        seat: PlayerId,
        card: CardId,
        resources: Resources,
    },
    /// Only the winner's VP identities, revealed immediately before completion.
    VictoryPointsRevealed {
        winner: PlayerId,
        cards: [Option<CardId>; 5],
    },
    /// Public completed one-card bank exchange at the best owned port rate.
    BankTraded {
        seat: PlayerId,
        give_resource: Resource,
        receive_resource: Resource,
        rate: u8,
    },
    /// Public replacement of one settlement with a city at the same vertex.
    CityUpgraded { seat: PlayerId, vertex: VertexId },
    /// Public fixed construction or card-purchase cost returned to the bank.
    ConstructionPaid {
        seat: PlayerId,
        resources: Resources,
    },
    /// Public transfer or return of the road award.
    LongestRouteChanged {
        previous: Option<PlayerId>,
        holder: Option<PlayerId>,
    },
    /// Public relocation destination.
    RobberMoved { seat: PlayerId, hex: HexId },
    /// Parties/count are public; the exact kind is available only to the parties.
    RobberTheft {
        thief: PlayerId,
        victim: PlayerId,
        count: u8,
        resource: Option<Resource>,
    },
    /// Count is public; exact types are available only to this discarding seat.
    Discarded {
        seat: PlayerId,
        count: u8,
        resources: Option<Resources>,
    },
    /// Immediate victory at the first winning owner-turn decision.
    GameFinished { winner: PlayerId, score: u8 },
    /// Resolved public gameplay dice.
    DiceRolled { seat: PlayerId, dice: [u8; 2] },
    /// Public allocation from board production.
    Production {
        seat: PlayerId,
        resources: Resources,
    },
    /// Public free road placement.
    RoadPlaced { seat: PlayerId, edge: EdgeId },
    /// Public second-settlement resource allocation.
    StartingResources {
        seat: PlayerId,
        resources: Resources,
    },
    /// The first ordinary turn began.
    TurnStarted { seat: PlayerId, turn: u64 },
    /// Public settlement placement.
    SettlementPlaced { seat: PlayerId, vertex: VertexId },
    /// Next public decision phase.
    PhaseChanged { phase: Phase },
}

/// Safe response at the original receipt's revision.
///
/// ```compile_fail
/// use hex_traders_engine::ProjectedReceipt;
/// let projected = ProjectedReceipt { revision: 1, events: vec![] };
/// let raw_command = projected.command;
/// ```
/// ```compile_fail
/// use hex_traders_engine::ProjectedReceipt;
/// let projected = ProjectedReceipt { revision: 1, events: vec![] };
/// let private_scores = projected.final_scores;
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectedReceipt {
    /// Revision at acceptance, even when the engine has since advanced.
    pub revision: u64,
    /// Ordered safe facts for the selected viewer.
    pub events: Vec<ProjectedEvent>,
}

impl Engine {
    /// Project the original receipt's facts, without revealing its raw command.
    pub fn project_receipt(
        &self,
        receipt: &Receipt,
        viewer: Viewer,
    ) -> Result<ProjectedReceipt, Rejection> {
        if let Viewer::Seat(seat) = viewer {
            self.validate_seat(seat)?;
        }
        let events = receipt
            .events
            .iter()
            .map(|&event| projected_event(event, Some(viewer)))
            .collect();
        Ok(ProjectedReceipt {
            revision: receipt.revision,
            events,
        })
    }
}

// None is reserved for the host-private replay encoder; public callers always supply a viewer.
pub(super) fn projected_event(event: Event, viewer: Option<Viewer>) -> ProjectedEvent {
    match event {
        Event::TradeOfferCreated { offer } => ProjectedEvent::TradeOfferCreated { offer },
        Event::TradeCompleted { offer } => ProjectedEvent::TradeCompleted { offer },
        Event::TradeOfferClosed { offer_id } => ProjectedEvent::TradeOfferClosed { offer_id },
        Event::RoadBuildingPlayed { seat, card } => {
            ProjectedEvent::RoadBuildingPlayed { seat, card }
        }
        Event::KnightPlayed { seat, card } => ProjectedEvent::KnightPlayed { seat, card },
        Event::LargestArmyChanged { previous, holder } => {
            ProjectedEvent::LargestArmyChanged { previous, holder }
        }
        Event::DevelopmentPurchased {
            seat,
            card,
            remaining,
        } => ProjectedEvent::DevelopmentPurchased {
            seat,
            remaining,
            card: if viewer.is_none() || viewer == Some(Viewer::Seat(seat)) {
                Some(card)
            } else {
                None
            },
        },
        Event::MonopolyPlayed {
            seat,
            card,
            resource,
            transfers,
        } => ProjectedEvent::MonopolyPlayed {
            seat,
            card,
            resource,
            transfers,
        },
        Event::InventionPlayed {
            seat,
            card,
            resources,
        } => ProjectedEvent::InventionPlayed {
            seat,
            card,
            resources,
        },
        Event::VictoryPointsRevealed { winner, cards } => {
            ProjectedEvent::VictoryPointsRevealed { winner, cards }
        }
        Event::BankTraded {
            seat,
            give_resource,
            receive_resource,
            rate,
        } => ProjectedEvent::BankTraded {
            seat,
            give_resource,
            receive_resource,
            rate,
        },
        Event::CityUpgraded { seat, vertex } => ProjectedEvent::CityUpgraded { seat, vertex },
        Event::ConstructionPaid { seat, resources } => {
            ProjectedEvent::ConstructionPaid { seat, resources }
        }
        Event::LongestRouteChanged { previous, holder } => {
            ProjectedEvent::LongestRouteChanged { previous, holder }
        }
        Event::RobberMoved { seat, hex } => ProjectedEvent::RobberMoved { seat, hex },
        Event::RobberTheft {
            thief,
            victim,
            resource,
        } => ProjectedEvent::RobberTheft {
            thief,
            victim,
            count: u8::from(resource.is_some()),
            resource: if viewer.is_none()
                || viewer == Some(Viewer::Seat(thief))
                || viewer == Some(Viewer::Seat(victim))
            {
                resource
            } else {
                None
            },
        },
        Event::Discarded { seat, resources } => ProjectedEvent::Discarded {
            seat,
            count: u8::try_from(resources.total()).expect("at most ninety-five resource cards"),
            resources: if viewer.is_none() || viewer == Some(Viewer::Seat(seat)) {
                Some(resources)
            } else {
                None
            },
        },
        Event::GameFinished { winner, score } => ProjectedEvent::GameFinished { winner, score },
        Event::DiceRolled { seat, dice } => ProjectedEvent::DiceRolled { seat, dice },
        Event::Production { seat, resources } => ProjectedEvent::Production { seat, resources },
        Event::RoadPlaced { seat, edge } => ProjectedEvent::RoadPlaced { seat, edge },
        Event::StartingResources { seat, resources } => {
            ProjectedEvent::StartingResources { seat, resources }
        }
        Event::TurnStarted { seat, turn } => ProjectedEvent::TurnStarted { seat, turn },
        Event::SettlementPlaced { seat, vertex } => {
            ProjectedEvent::SettlementPlaced { seat, vertex }
        }
        Event::PhaseChanged { phase } => ProjectedEvent::PhaseChanged { phase },
    }
}

#[cfg(test)]
#[path = "receipt_tests.rs"]
mod receipt_tests;
