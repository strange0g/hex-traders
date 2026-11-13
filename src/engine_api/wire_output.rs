//! Borrowed JSON encoders for the native viewer-safe allowlists.
#[cfg(test)]
#[path = "wire_output_tests.rs"]
mod tests;
use super::wire::{BoundaryError, Wire, encode, resource_name, rule_code};
use super::{
    BoardView, DevelopmentCardView, GameView, LegalActions, OwnPlayerView,
    ProjectedCreationReceipt, ProjectedEvent, ProjectedReceipt, PublicPlayerView, Rejection,
};
use crate::game_model::{
    BoardGraph, Building, BuildingKind, CardId, CreationEvent, DevelopmentKind, EdgeId, HexId,
    LegalAction, Phase, PlayerId, Port, PortKind, Resource, ResumePhase, RobberOrigin, SetupMode,
    Terrain, TradeOffer, VertexId,
};
use serde::{
    Serialize, Serializer,
    ser::{SerializeSeq, SerializeStruct},
};

macro_rules! fields { ($s:ident; $($key:literal => $value:expr),* $(,)?) => {{
    let keys: &[&str] = &[$($key),*];
    let mut fields=$s.serialize_struct("Wire",keys.len())?;
    $(fields.serialize_field($key,&$value)?;)*
    fields.end()
}}; }
macro_rules! scalar { ($($kind:ty),*) => { $(impl Serialize for Wire<'_,$kind> { fn serialize<S:Serializer>(&self,s:S)->Result<S::Ok,S::Error> { self.0.serialize(s) } })* }; }
scalar!(u8, bool, String, str);
macro_rules! id { ($($kind:ty),*) => { $(impl Serialize for Wire<'_,$kind> { fn serialize<S:Serializer>(&self,s:S)->Result<S::Ok,S::Error> { self.0.index().serialize(s) } })* }; }
id!(PlayerId, CardId, EdgeId, HexId, VertexId);
impl<T> Serialize for Wire<'_, [T]>
where
    for<'a> Wire<'a, T>: Serialize,
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut seq = s.serialize_seq(Some(self.0.len()))?;
        for v in self.0 {
            seq.serialize_element(&Wire(v))?;
        }
        seq.end()
    }
}
impl<T, const N: usize> Serialize for Wire<'_, [T; N]>
where
    for<'a> Wire<'a, T>: Serialize,
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        Wire(&self.0[..]).serialize(s)
    }
}
impl<T> Serialize for Wire<'_, Vec<T>>
where
    for<'a> Wire<'a, T>: Serialize,
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        Wire(&self.0[..]).serialize(s)
    }
}
impl<T> Serialize for Wire<'_, Option<T>>
where
    for<'a> Wire<'a, T>: Serialize,
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            Some(v) => s.serialize_some(&Wire(v)),
            None => s.serialize_none(),
        }
    }
}
impl<T: ?Sized> Serialize for Wire<'_, &T>
where
    for<'a> Wire<'a, T>: Serialize,
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        Wire(*self.0).serialize(s)
    }
}
impl Serialize for Wire<'_, (Resource, Resource)> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut seq = s.serialize_seq(Some(2))?;
        seq.serialize_element(&Wire(&self.0.0))?;
        seq.serialize_element(&Wire(&self.0.1))?;
        seq.end()
    }
}
impl Serialize for Wire<'_, Resource> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        resource_name(*self.0).serialize(s)
    }
}
impl Serialize for Wire<'_, Terrain> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            Terrain::Hills => s.serialize_str("hills"),
            Terrain::Forest => s.serialize_str("forest"),
            Terrain::Pasture => s.serialize_str("pasture"),
            Terrain::Field => s.serialize_str("field"),
            Terrain::Mountain => s.serialize_str("mountain"),
            Terrain::Desert => s.serialize_str("desert"),
        }
    }
}
impl Serialize for Wire<'_, BuildingKind> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            BuildingKind::Settlement => s.serialize_str("settlement"),
            BuildingKind::City => s.serialize_str("city"),
        }
    }
}
impl Serialize for Wire<'_, DevelopmentKind> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            DevelopmentKind::Knight => s.serialize_str("knight"),
            DevelopmentKind::RoadBuilding => s.serialize_str("roadBuilding"),
            DevelopmentKind::Monopoly => s.serialize_str("monopoly"),
            DevelopmentKind::Invention => s.serialize_str("invention"),
            DevelopmentKind::VictoryPoint => s.serialize_str("victoryPoint"),
        }
    }
}
impl Serialize for Wire<'_, SetupMode> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            SetupMode::Fixed => s.serialize_str("fixed"),
            SetupMode::Variable => s.serialize_str("variable"),
        }
    }
}
impl Serialize for Wire<'_, ResumePhase> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            ResumePhase::PreRoll => s.serialize_str("preRoll"),
            ResumePhase::Action => s.serialize_str("action"),
        }
    }
}
impl Serialize for Wire<'_, RobberOrigin> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            RobberOrigin::Knight => s.serialize_str("knight"),
            RobberOrigin::Seven => s.serialize_str("seven"),
        }
    }
}
impl Serialize for Wire<'_, Building> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let v = self.0;
        fields!(s;"kind" => Wire(&v.kind),"owner" => Wire(&v.owner))
    }
}
impl Serialize for Wire<'_, Port> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let v = self.0;
        fields!(s;"edge" => Wire(&v.edge),"kind" => Wire(&v.kind))
    }
}
impl Serialize for Wire<'_, TradeOffer> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let v = self.0;
        fields!(s;"id" => v.id.to_string(),"turn" => v.turn.to_string(),"proposer" => Wire(&v.proposer),"recipient" => Wire(&v.recipient),"give" => Wire(&v.give),"receive" => Wire(&v.receive))
    }
}
impl Serialize for Wire<'_, BoardView> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let v = self.0;
        fields!(s;"graph" => Wire(&v.graph),"terrain" => Wire(&v.terrain),"numbers" => Wire(&v.numbers),"ports" => Wire(&v.ports),"roads" => Wire(&v.roads),"buildings" => Wire(&v.buildings),"robber" => Wire(&v.robber))
    }
}
impl Serialize for Wire<'_, PublicPlayerView> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let v = self.0;
        fields!(s;"seat" => Wire(&v.seat),"resourceCount" => Wire(&v.resource_count),"roadsRemaining" => Wire(&v.roads_remaining),"settlementsRemaining" => Wire(&v.settlements_remaining),"citiesRemaining" => Wire(&v.cities_remaining),"developmentCardCount" => Wire(&v.development_card_count),"spentProgress" => Wire(&v.spent_progress),"playedKnights" => Wire(&v.played_knights),"publicScore" => Wire(&v.public_score),"longestRouteLength" => Wire(&v.longest_route_length))
    }
}
impl Serialize for Wire<'_, DevelopmentCardView> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let v = self.0;
        fields!(s;"id" => Wire(&v.id),"kind" => Wire(&v.kind),"acquiredTurn" => v.acquired_turn.to_string(),"acquiredOnEarlierTurn" => Wire(&v.acquired_on_earlier_turn))
    }
}
impl Serialize for Wire<'_, OwnPlayerView> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let v = self.0;
        fields!(s;"seat" => Wire(&v.seat),"resources" => Wire(&v.resources),"trueScore" => Wire(&v.true_score),"developmentCards" => Wire(&v.development_cards))
    }
}
impl Serialize for Wire<'_, GameView> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let v = self.0;
        fields!(s;"revision" => v.revision.to_string(),"board" => Wire(&v.board),"setup" => Wire(&v.setup),"seatOrder" => Wire(&v.seat_order),"turnOrder" => Wire(&v.turn_order),"startingPlayer" => Wire(&v.starting_player),"activePlayer" => Wire(&v.active_player),"turn" => v.turn.to_string(),"phase" => Wire(&v.phase),"lastDice" => Wire(&v.last_dice),"longestRouteHolder" => Wire(&v.longest_route_holder),"largestArmyHolder" => Wire(&v.largest_army_holder),"bank" => Wire(&v.bank),"offer" => Wire(&v.offer),"developmentDeckRemaining" => Wire(&v.development_deck_remaining),"revealedVictoryCards" => Wire(&v.revealed_victory_cards),"players" => Wire(&v.players),"ownPlayer" => Wire(&v.own_player))
    }
}
impl Serialize for Wire<'_, LegalActions> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let v = self.0;
        fields!(s;"revision" => v.revision.to_string(),"choices" => Wire(&v.choices))
    }
}
impl Serialize for Wire<'_, ProjectedReceipt> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let v = self.0;
        fields!(s;"revision" => v.revision.to_string(),"events" => Wire(&v.events))
    }
}
impl Serialize for Wire<'_, ProjectedCreationReceipt> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let v = self.0;
        fields!(s;"revision" => v.revision.to_string(),"events" => Wire(&v.events))
    }
}
impl Serialize for Wire<'_, PortKind> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            PortKind::Generic => fields!(s;"type" => "generic"),
            PortKind::Special(resource) => fields!(s;"type"=>"special","resource"=>Wire(resource)),
        }
    }
}
impl Serialize for Wire<'_, Phase> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            Phase::SetupSettlement { actor, round } => {
                fields!(s;"type" => "setupSettlement" ,"actor" => Wire(actor),"round" => Wire(round))
            }
            Phase::SetupRoad {
                actor,
                round,
                settlement,
            } => {
                fields!(s;"type" => "setupRoad" ,"actor" => Wire(actor),"round" => Wire(round),"settlement" => Wire(settlement))
            }
            Phase::PreRoll => fields!(s;"type" => "preRoll"),
            Phase::Action => fields!(s;"type" => "action"),
            Phase::Discard { obligations } => {
                fields!(s;"type" => "discard" ,"obligations" => Wire(obligations))
            }
            Phase::RobberMove {
                origin,
                continuation,
            } => {
                fields!(s;"type" => "robberMove" ,"origin" => Wire(origin),"continuation" => Wire(continuation))
            }
            Phase::RobberVictim {
                origin,
                continuation,
                victims,
            } => {
                fields!(s;"type" => "robberVictim" ,"origin" => Wire(origin),"continuation" => Wire(continuation),"victims" => Wire(victims))
            }
            Phase::FreeRoads {
                remaining,
                continuation,
            } => {
                fields!(s;"type" => "freeRoads" ,"remaining" => Wire(remaining),"continuation" => Wire(continuation))
            }
            Phase::Finished { winner, score } => {
                fields!(s;"type" => "finished" ,"winner" => Wire(winner),"score" => Wire(score))
            }
        }
    }
}
impl Serialize for Wire<'_, CreationEvent> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            CreationEvent::GameCreated { setup, seat_order } => {
                fields!(s;"type" => "gameCreated" ,"setup" => Wire(setup),"seatOrder" => Wire(seat_order))
            }
            CreationEvent::SettlementPlaced { seat, vertex } => {
                fields!(s;"type" => "settlementPlaced" ,"seat" => Wire(seat),"vertex" => Wire(vertex))
            }
            CreationEvent::RoadPlaced { seat, edge } => {
                fields!(s;"type" => "roadPlaced" ,"seat" => Wire(seat),"edge" => Wire(edge))
            }
            CreationEvent::StartingResources { seat, resources } => {
                fields!(s;"type" => "startingResources" ,"seat" => Wire(seat),"resources" => Wire(resources))
            }
            CreationEvent::StarterRolled { seat, dice } => {
                fields!(s;"type" => "starterRolled" ,"seat" => Wire(seat),"dice" => Wire(dice))
            }
            CreationEvent::TurnStarted { seat, turn } => {
                fields!(s;"type" => "turnStarted" ,"seat" => Wire(seat),"turn" => turn.to_string())
            }
            CreationEvent::SetupStarted { phase } => {
                fields!(s;"type" => "setupStarted" ,"phase" => Wire(phase))
            }
        }
    }
}
impl Serialize for Wire<'_, ProjectedEvent> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            ProjectedEvent::TradeOfferCreated { offer } => {
                fields!(s;"type" => "tradeOfferCreated" ,"offer" => Wire(offer))
            }
            ProjectedEvent::TradeCompleted { offer } => {
                fields!(s;"type" => "tradeCompleted" ,"offer" => Wire(offer))
            }
            ProjectedEvent::TradeOfferClosed { offer_id } => {
                fields!(s;"type" => "tradeOfferClosed" ,"offerId" => offer_id.to_string())
            }
            ProjectedEvent::RoadBuildingPlayed { seat, card } => {
                fields!(s;"type" => "roadBuildingPlayed" ,"seat" => Wire(seat),"card" => Wire(card))
            }
            ProjectedEvent::KnightPlayed { seat, card } => {
                fields!(s;"type" => "knightPlayed" ,"seat" => Wire(seat),"card" => Wire(card))
            }
            ProjectedEvent::LargestArmyChanged { previous, holder } => {
                fields!(s;"type" => "largestArmyChanged" ,"previous" => Wire(previous),"holder" => Wire(holder))
            }
            ProjectedEvent::DevelopmentPurchased {
                seat,
                card,
                remaining,
            } => {
                fields!(s;"type" => "developmentPurchased" ,"seat" => Wire(seat),"card" => Wire(card),"remaining" => Wire(remaining))
            }
            ProjectedEvent::MonopolyPlayed {
                seat,
                card,
                resource,
                transfers,
            } => {
                fields!(s;"type" => "monopolyPlayed" ,"seat" => Wire(seat),"card" => Wire(card),"resource" => Wire(resource),"transfers" => Wire(transfers))
            }
            ProjectedEvent::InventionPlayed {
                seat,
                card,
                resources,
            } => {
                fields!(s;"type" => "inventionPlayed" ,"seat" => Wire(seat),"card" => Wire(card),"resources" => Wire(resources))
            }
            ProjectedEvent::VictoryPointsRevealed { winner, cards } => {
                fields!(s;"type" => "victoryPointsRevealed" ,"winner" => Wire(winner),"cards" => Wire(cards))
            }
            ProjectedEvent::BankTraded {
                seat,
                give_resource,
                receive_resource,
                rate,
            } => {
                fields!(s;"type" => "bankTraded" ,"seat" => Wire(seat),"giveResource" => Wire(give_resource),"receiveResource" => Wire(receive_resource),"rate" => Wire(rate))
            }
            ProjectedEvent::CityUpgraded { seat, vertex } => {
                fields!(s;"type" => "cityUpgraded" ,"seat" => Wire(seat),"vertex" => Wire(vertex))
            }
            ProjectedEvent::ConstructionPaid { seat, resources } => {
                fields!(s;"type" => "constructionPaid" ,"seat" => Wire(seat),"resources" => Wire(resources))
            }
            ProjectedEvent::LongestRouteChanged { previous, holder } => {
                fields!(s;"type" => "longestRouteChanged" ,"previous" => Wire(previous),"holder" => Wire(holder))
            }
            ProjectedEvent::RobberMoved { seat, hex } => {
                fields!(s;"type" => "robberMoved" ,"seat" => Wire(seat),"hex" => Wire(hex))
            }
            ProjectedEvent::RobberTheft {
                thief,
                victim,
                count,
                resource,
            } => {
                fields!(s;"type" => "robberTheft" ,"thief" => Wire(thief),"victim" => Wire(victim),"count" => Wire(count),"resource" => Wire(resource))
            }
            ProjectedEvent::Discarded {
                seat,
                count,
                resources,
            } => {
                fields!(s;"type" => "discarded" ,"seat" => Wire(seat),"count" => Wire(count),"resources" => Wire(resources))
            }
            ProjectedEvent::GameFinished { winner, score } => {
                fields!(s;"type" => "gameFinished" ,"winner" => Wire(winner),"score" => Wire(score))
            }
            ProjectedEvent::DiceRolled { seat, dice } => {
                fields!(s;"type" => "diceRolled" ,"seat" => Wire(seat),"dice" => Wire(dice))
            }
            ProjectedEvent::Production { seat, resources } => {
                fields!(s;"type" => "production" ,"seat" => Wire(seat),"resources" => Wire(resources))
            }
            ProjectedEvent::RoadPlaced { seat, edge } => {
                fields!(s;"type" => "roadPlaced" ,"seat" => Wire(seat),"edge" => Wire(edge))
            }
            ProjectedEvent::StartingResources { seat, resources } => {
                fields!(s;"type" => "startingResources" ,"seat" => Wire(seat),"resources" => Wire(resources))
            }
            ProjectedEvent::TurnStarted { seat, turn } => {
                fields!(s;"type" => "turnStarted" ,"seat" => Wire(seat),"turn" => turn.to_string())
            }
            ProjectedEvent::SettlementPlaced { seat, vertex } => {
                fields!(s;"type" => "settlementPlaced" ,"seat" => Wire(seat),"vertex" => Wire(vertex))
            }
            ProjectedEvent::PhaseChanged { phase } => {
                fields!(s;"type" => "phaseChanged" ,"phase" => Wire(phase))
            }
        }
    }
}
impl Serialize for Wire<'_, LegalAction> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            LegalAction::ProposeTrade { recipients } => {
                fields!(s;"type" => "proposeTrade" ,"recipients" => Wire(recipients))
            }
            LegalAction::AcceptTrade { offer_id } => {
                fields!(s;"type" => "acceptTrade" ,"offerId" => offer_id.to_string())
            }
            LegalAction::RejectTrade { offer_id } => {
                fields!(s;"type" => "rejectTrade" ,"offerId" => offer_id.to_string())
            }
            LegalAction::CancelTrade { offer_id } => {
                fields!(s;"type" => "cancelTrade" ,"offerId" => offer_id.to_string())
            }
            LegalAction::PlayRoadBuilding { cards } => {
                fields!(s;"type" => "playRoadBuilding" ,"cards" => Wire(cards))
            }
            LegalAction::PlaceFreeRoad { edges } => {
                fields!(s;"type" => "placeFreeRoad" ,"edges" => Wire(edges))
            }
            LegalAction::PlayKnight { cards } => {
                fields!(s;"type" => "playKnight" ,"cards" => Wire(cards))
            }
            LegalAction::BuyDevelopmentCard => fields!(s;"type" => "buyDevelopmentCard"),
            LegalAction::PlayMonopoly { cards } => {
                fields!(s;"type" => "playMonopoly" ,"cards" => Wire(cards))
            }
            LegalAction::PlayInvention { cards } => {
                fields!(s;"type" => "playInvention" ,"cards" => Wire(cards))
            }
            LegalAction::BankTrade { pairs } => {
                fields!(s;"type" => "bankTrade" ,"pairs" => Wire(pairs))
            }
            LegalAction::UpgradeCity { vertices } => {
                fields!(s;"type" => "upgradeCity" ,"vertices" => Wire(vertices))
            }
            LegalAction::BuildSettlement { vertices } => {
                fields!(s;"type" => "buildSettlement" ,"vertices" => Wire(vertices))
            }
            LegalAction::BuildRoad { edges } => {
                fields!(s;"type" => "buildRoad" ,"edges" => Wire(edges))
            }
            LegalAction::MoveRobber { hexes } => {
                fields!(s;"type" => "moveRobber" ,"hexes" => Wire(hexes))
            }
            LegalAction::ChooseRobberVictim { victims } => {
                fields!(s;"type" => "chooseRobberVictim" ,"victims" => Wire(victims))
            }
            LegalAction::DiscardResources { quantity } => {
                fields!(s;"type" => "discardResources" ,"quantity" => Wire(quantity))
            }
            LegalAction::EndTurn => fields!(s;"type" => "endTurn"),
            LegalAction::RollDice => fields!(s;"type" => "rollDice"),
            LegalAction::PlaceSetupRoad { edges } => {
                fields!(s;"type" => "placeSetupRoad" ,"edges" => Wire(edges))
            }
            LegalAction::PlaceSetupSettlement { vertices } => {
                fields!(s;"type" => "placeSetupSettlement" ,"vertices" => Wire(vertices))
            }
        }
    }
}
impl Serialize for Wire<'_, BoardGraph> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let hex_vertices: [_; 19] = std::array::from_fn(|i| {
            self.0
                .hex_vertices(HexId::try_from(i as u8).expect("bounded graph ID"))
        });
        let hex_edges: [_; 19] = std::array::from_fn(|i| {
            self.0
                .hex_edges(HexId::try_from(i as u8).expect("bounded graph ID"))
        });
        let hex_neighbors: [_; 19] = std::array::from_fn(|i| {
            self.0
                .hex_neighbors(HexId::try_from(i as u8).expect("bounded graph ID"))
        });
        let vertex_hexes: [_; 54] = std::array::from_fn(|i| {
            self.0
                .vertex_hexes(VertexId::try_from(i as u8).expect("bounded graph ID"))
        });
        let vertex_neighbors: [_; 54] = std::array::from_fn(|i| {
            self.0
                .vertex_neighbors(VertexId::try_from(i as u8).expect("bounded graph ID"))
        });
        let vertex_edges: [_; 54] = std::array::from_fn(|i| {
            self.0
                .vertex_edges(VertexId::try_from(i as u8).expect("bounded graph ID"))
        });
        let edge_vertices: [_; 72] = std::array::from_fn(|i| {
            self.0
                .edge_vertices(EdgeId::try_from(i as u8).expect("bounded graph ID"))
        });
        let edge_hexes: [_; 72] = std::array::from_fn(|i| {
            self.0
                .edge_hexes(EdgeId::try_from(i as u8).expect("bounded graph ID"))
        });
        fields!(s;"hexVertices"=>Wire(&hex_vertices),"hexEdges"=>Wire(&hex_edges),"hexNeighbors"=>Wire(&hex_neighbors),"vertexHexes"=>Wire(&vertex_hexes),"vertexNeighbors"=>Wire(&vertex_neighbors),"vertexEdges"=>Wire(&vertex_edges),"edgeVertices"=>Wire(&edge_vertices),"edgeHexes"=>Wire(&edge_hexes))
    }
}
impl Serialize for Wire<'_, Rejection> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        fields!(s;"code"=>rule_code(self.0.code),"currentRevision"=>self.0.current_revision.to_string())
    }
}
impl Serialize for Wire<'_, BoundaryError> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut fields = s.serialize_struct(
            "BoundaryError",
            if self.0.field().is_some() { 3 } else { 2 },
        )?;
        fields.serialize_field("code", self.0.code())?;
        fields.serialize_field("currentRevision", &self.0.current_revision().to_string())?;
        if let Some(field) = self.0.field() {
            fields.serialize_field("field", field)?;
        }
        fields.end()
    }
}
impl GameView {
    /// Encode this native allowlist in declared camelCase field order.
    /// This operation does not mutate a game or consume randomness.
    pub fn to_json(&self) -> Result<Vec<u8>, BoundaryError> {
        encode(&Wire(self))
    }
}
impl LegalActions {
    /// Encode this native allowlist in declared camelCase field order.
    /// This operation does not mutate a game or consume randomness.
    pub fn to_json(&self) -> Result<Vec<u8>, BoundaryError> {
        encode(&Wire(self))
    }
}
impl ProjectedReceipt {
    /// Encode this native allowlist in declared camelCase field order.
    /// This operation does not mutate a game or consume randomness.
    pub fn to_json(&self) -> Result<Vec<u8>, BoundaryError> {
        encode(&Wire(self))
    }
}
impl ProjectedCreationReceipt {
    /// Encode this native allowlist in declared camelCase field order.
    /// This operation does not mutate a game or consume randomness.
    pub fn to_json(&self) -> Result<Vec<u8>, BoundaryError> {
        encode(&Wire(self))
    }
}
impl Rejection {
    /// Encode this native allowlist in declared camelCase field order.
    /// This operation does not mutate a game or consume randomness.
    pub fn to_json(&self) -> Result<Vec<u8>, BoundaryError> {
        encode(&Wire(self))
    }
}
impl BoundaryError {
    /// Encode this native allowlist in declared camelCase field order.
    /// This operation does not mutate a game or consume randomness.
    pub fn to_json(&self) -> Result<Vec<u8>, BoundaryError> {
        encode(&Wire(self))
    }
}
