//! Native commands and bounded legal-action choices.
use super::{Engine, Receipt};
#[cfg(test)]
use super::{ProjectedEvent, ProjectedReceipt, Viewer};
#[cfg(test)]
use crate::game_model::{CardId, Event, Phase};
use crate::{
    game_model::{
        Command, EdgeId, HexId, LegalAction, PlayerId, Resource, Resources, RuleError, VertexId,
    },
    turn_engine::{evaluate, required_discard, validate_action},
};

/// A typed intent submitted with the host-selected actor and observed revision.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CommandEnvelope {
    /// Supported native API version, currently one.
    pub api_version: u32,
    /// Seat selected by the trusted host.
    pub actor: PlayerId,
    /// Must match the current revision exactly.
    pub expected_revision: u64,
    /// Complete choice to validate and execute.
    pub command: Command,
}

/// A player-safe failure; the engine commits nothing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rejection {
    /// Stable rejection reason without private holdings.
    pub code: RuleError,
    /// Current public revision for refreshing a stale view.
    pub current_revision: u64,
}

impl std::fmt::Display for Rejection {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{} at revision {}",
            self.code, self.current_revision
        )
    }
}
impl std::error::Error for Rejection {}

/// Current supported action kinds and bounded choices for one seat.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LegalActions {
    /// Revision used by these predicates.
    pub revision: u64,
    /// Stable action/ID order; an unavailable seat has no choices.
    pub choices: Vec<LegalAction>,
}

impl Engine {
    fn rejection(&self, code: RuleError) -> Rejection {
        Rejection {
            code,
            current_revision: self.state.revision,
        }
    }

    pub(super) fn validate_seat(&self, seat: PlayerId) -> Result<(), Rejection> {
        if seat.index() >= self.state.position.config.seat_order().len() {
            return Err(self.rejection(RuleError::InvalidSeat(seat.index() as u8)));
        }
        Ok(())
    }

    fn validate_command_seats(&self, command: &Command) -> Result<(), Rejection> {
        if let Command::ProposeTrade { recipient, .. } = command {
            self.validate_seat(*recipient)?;
        }
        Ok(())
    }

    /// Commit one validated transition. Version/seat precede revision, then eligibility.
    /// Rejections preserve the complete position and generator.
    pub fn execute(&mut self, envelope: CommandEnvelope) -> Result<Receipt, Rejection> {
        if envelope.api_version != 1 {
            return Err(self.rejection(RuleError::UnsupportedApiVersion(envelope.api_version)));
        }
        self.validate_seat(envelope.actor)?;
        self.validate_command_seats(&envelope.command)?;
        if envelope.expected_revision != self.state.revision {
            return Err(self.rejection(RuleError::StaleRevision));
        }
        let transition = evaluate(&self.board, &self.state, envelope.actor, &envelope.command)
            .map_err(|code| self.rejection(code))?;
        let receipt = Receipt {
            api_version: 1,
            rules_version: "hex-traders-base-v1",
            random_algorithm: "chacha12-rand010-v1",
            actor: envelope.actor,
            command: envelope.command,
            previous_revision: self.state.revision,
            revision: transition.revision,
            events: transition.events,
            final_scores: transition.position.final_scores,
        };
        self.state.position = transition.position;
        self.state.revision = transition.revision;
        if let Some(random) = transition.random {
            self.state.random = random;
        }
        Ok(receipt)
    }

    /// Check one complete choice without RNG draws, mutation or reservation.
    pub fn validate(&self, actor: PlayerId, command: &Command) -> Result<(), Rejection> {
        self.validate_seat(actor)?;
        self.validate_command_seats(command)?;
        validate_action(&self.board, &self.state, actor, command)
            .map(|_| ())
            .map_err(|code| self.rejection(code))
    }

    /// Enumerate supported bounded choices with the execution validator.
    pub fn legal_actions(&self, actor: PlayerId) -> Result<LegalActions, Rejection> {
        self.validate_seat(actor)?;
        let vertices: Vec<_> = (0..VertexId::COUNT as u8)
            .map(|id| VertexId::try_from(id).expect("bounded vertex ID"))
            .filter(|&vertex| {
                self.validate(actor, &Command::PlaceSetupSettlement { vertex })
                    .is_ok()
            })
            .collect();
        let mut choices = Vec::new();
        let cards = self
            .state
            .position
            .held_cards(actor)
            .map(|(card, _)| card)
            .filter(|&card| {
                self.validate(actor, &Command::PlayRoadBuilding { card })
                    .is_ok()
            })
            .collect::<Vec<_>>();
        if !cards.is_empty() {
            choices.push(LegalAction::PlayRoadBuilding { cards });
        }
        let edges = (0..EdgeId::COUNT as u8)
            .map(|id| EdgeId::try_from(id).expect("bounded edge"))
            .filter(|&edge| {
                self.validate(actor, &Command::PlaceFreeRoad { edge })
                    .is_ok()
            })
            .collect::<Vec<_>>();
        if !edges.is_empty() {
            choices.push(LegalAction::PlaceFreeRoad { edges });
        }

        let cards = self
            .state
            .position
            .held_cards(actor)
            .map(|(card, _)| card)
            .filter(|&card| self.validate(actor, &Command::PlayKnight { card }).is_ok())
            .collect::<Vec<_>>();
        if !cards.is_empty() {
            choices.push(LegalAction::PlayKnight { cards });
        }
        if let Some(index) = self.state.position.hands[actor.index()]
            .counts()
            .iter()
            .position(|&count| count > 0)
        {
            let mut give = [0; 5];
            give[index] = 1;
            let mut receive = [0; 5];
            receive[(index + 1) % 5] = 1;
            let give = Resources::try_from(give).expect("one owned resource");
            let receive = Resources::try_from(receive).expect("one distinct resource");
            let mut recipients = self
                .state
                .position
                .config
                .seat_order()
                .iter()
                .copied()
                .filter(|&recipient| {
                    self.validate(
                        actor,
                        &Command::ProposeTrade {
                            recipient,
                            give,
                            receive,
                        },
                    )
                    .is_ok()
                })
                .collect::<Vec<_>>();
            recipients.sort();
            if !recipients.is_empty() {
                choices.push(LegalAction::ProposeTrade { recipients });
            }
        }
        if let Some(offer) = self.state.position.offer {
            if self
                .validate(actor, &Command::AcceptTrade { offer_id: offer.id })
                .is_ok()
            {
                choices.push(LegalAction::AcceptTrade { offer_id: offer.id });
            }
            if self
                .validate(actor, &Command::RejectTrade { offer_id: offer.id })
                .is_ok()
            {
                choices.push(LegalAction::RejectTrade { offer_id: offer.id });
            }
            if self
                .validate(actor, &Command::CancelTrade { offer_id: offer.id })
                .is_ok()
            {
                choices.push(LegalAction::CancelTrade { offer_id: offer.id });
            }
        }
        if self.validate(actor, &Command::BuyDevelopmentCard).is_ok() {
            choices.push(LegalAction::BuyDevelopmentCard);
        }
        let cards = self
            .state
            .position
            .held_cards(actor)
            .map(|(id, _)| id)
            .filter(|&card| {
                self.validate(
                    actor,
                    &Command::PlayMonopoly {
                        card,
                        resource: Resource::Brick,
                    },
                )
                .is_ok()
            })
            .collect::<Vec<_>>();
        if !cards.is_empty() {
            choices.push(LegalAction::PlayMonopoly { cards });
        }
        if let Some(resources) =
            crate::base_rules::first_invention_choice(&self.state.position.bank)
        {
            let cards = self
                .state
                .position
                .held_cards(actor)
                .map(|(id, _)| id)
                .filter(|&card| {
                    self.validate(actor, &Command::PlayInvention { card, resources })
                        .is_ok()
                })
                .collect::<Vec<_>>();
            if !cards.is_empty() {
                choices.push(LegalAction::PlayInvention { cards });
            }
        }
        if let Ok(quantity) = required_discard(&self.state, actor) {
            choices.push(LegalAction::DiscardResources { quantity });
        }
        if self.validate(actor, &Command::EndTurn).is_ok() {
            choices.push(LegalAction::EndTurn);
        }
        if self.validate(actor, &Command::RollDice).is_ok() {
            choices.push(LegalAction::RollDice);
        }
        if !vertices.is_empty() {
            choices.push(LegalAction::PlaceSetupSettlement { vertices });
        }
        let edges: Vec<_> = (0..EdgeId::COUNT as u8)
            .map(|id| EdgeId::try_from(id).expect("bounded edge ID"))
            .filter(|&edge| {
                self.validate(actor, &Command::PlaceSetupRoad { edge })
                    .is_ok()
            })
            .collect();
        if !edges.is_empty() {
            choices.push(LegalAction::PlaceSetupRoad { edges });
        }
        let hexes = (0..HexId::COUNT as u8)
            .map(|id| HexId::try_from(id).expect("bounded hex ID"))
            .filter(|&hex| self.validate(actor, &Command::MoveRobber { hex }).is_ok())
            .collect::<Vec<_>>();
        if !hexes.is_empty() {
            choices.push(LegalAction::MoveRobber { hexes });
        }
        let victims = self
            .state
            .position
            .config
            .seat_order()
            .iter()
            .copied()
            .filter(|&victim| {
                self.validate(actor, &Command::ChooseRobberVictim { victim })
                    .is_ok()
            })
            .collect::<Vec<_>>();
        let mut victims = victims;
        victims.sort();
        if !victims.is_empty() {
            choices.push(LegalAction::ChooseRobberVictim { victims });
        }
        let edges = (0..EdgeId::COUNT as u8)
            .map(|id| EdgeId::try_from(id).expect("bounded edge"))
            .filter(|&edge| self.validate(actor, &Command::BuildRoad { edge }).is_ok())
            .collect::<Vec<_>>();
        if !edges.is_empty() {
            choices.push(LegalAction::BuildRoad { edges });
        }
        let vertices = (0..VertexId::COUNT as u8)
            .map(|id| VertexId::try_from(id).expect("bounded vertex"))
            .filter(|&vertex| {
                self.validate(actor, &Command::BuildSettlement { vertex })
                    .is_ok()
            })
            .collect::<Vec<_>>();
        if !vertices.is_empty() {
            choices.push(LegalAction::BuildSettlement { vertices });
        }
        let vertices = (0..VertexId::COUNT as u8)
            .map(|id| VertexId::try_from(id).expect("bounded vertex"))
            .filter(|&vertex| {
                self.validate(actor, &Command::UpgradeCity { vertex })
                    .is_ok()
            })
            .collect::<Vec<_>>();
        if !vertices.is_empty() {
            choices.push(LegalAction::UpgradeCity { vertices });
        }
        let resources = [
            Resource::Brick,
            Resource::Wood,
            Resource::Wool,
            Resource::Wheat,
            Resource::Ore,
        ];
        let pairs = resources
            .into_iter()
            .flat_map(|give_resource| {
                resources
                    .into_iter()
                    .map(move |receive_resource| (give_resource, receive_resource))
            })
            .filter(|&(give_resource, receive_resource)| {
                self.validate(
                    actor,
                    &Command::BankTrade {
                        give_resource,
                        receive_resource,
                    },
                )
                .is_ok()
            })
            .collect::<Vec<_>>();
        if !pairs.is_empty() {
            choices.push(LegalAction::BankTrade { pairs });
        }
        Ok(LegalActions {
            revision: self.state.revision,
            choices,
        })
    }
}

#[cfg(test)]
#[path = "command_tests.rs"]
mod command_tests;

#[cfg(test)]
#[path = "robber_tests.rs"]
mod robber_tests;

#[cfg(test)]
#[path = "road_tests.rs"]
mod road_tests;

#[cfg(test)]
#[path = "settlement_tests.rs"]
mod settlement_tests;

#[cfg(test)]
#[path = "city_tests.rs"]
mod city_tests;

#[cfg(test)]
#[path = "bank_trade_tests.rs"]
mod bank_trade_tests;

#[cfg(test)]
#[path = "development_tests.rs"]
mod development_tests;

#[cfg(test)]
#[path = "development_effects_tests.rs"]
mod development_effects_tests;

#[cfg(test)]
#[path = "invention_tests.rs"]
mod invention_tests;

#[cfg(test)]
#[path = "knight_tests.rs"]
mod knight_tests;

#[cfg(test)]
#[path = "free_road_tests.rs"]
mod free_road_tests;

#[cfg(test)]
#[path = "victory_tests.rs"]
mod victory_tests;

#[cfg(test)]
#[path = "trade_offer_tests.rs"]
mod trade_offer_tests;

#[cfg(test)]
#[path = "trade_accept_tests.rs"]
mod trade_accept_tests;

#[cfg(test)]
#[path = "native_contract_tests.rs"]
mod native_contract_tests;
