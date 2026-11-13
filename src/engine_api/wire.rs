//! Strict JSON conversion at the external boundary; native rules never parse JSON.
use super::{CommandEnvelope, Engine, Receipt};
use crate::game_model::{
    CardId, Command, EdgeId, HexId, PlayerId, Resource, Resources, RuleError, VertexId,
};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{DeserializeSeed, MapAccess, SeqAccess, Visitor},
    ser::SerializeStruct,
};
use serde_json::{Map, Value};

pub(super) const MAX_BYTES: usize = 65_536;

/// A bounded public diagnostic; input text and private rule facts are never included.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BoundaryError {
    code: &'static str,
    current_revision: u64,
    field: Option<&'static str>,
}
impl BoundaryError {
    /// Stable machine-readable code.
    pub fn code(&self) -> &'static str {
        self.code
    }
    /// Current public revision; standalone decoding/import uses zero.
    pub fn current_revision(&self) -> u64 {
        self.current_revision
    }
    /// Optional static public field, never an input key.
    pub fn field(&self) -> Option<&'static str> {
        self.field
    }
    pub(super) fn new(code: &'static str) -> Self {
        Self {
            code,
            current_revision: 0,
            field: None,
        }
    }
    fn at(mut self, revision: u64) -> Self {
        self.current_revision = revision;
        self
    }
}
impl std::fmt::Display for BoundaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at revision {}", self.code, self.current_revision)
    }
}
impl std::error::Error for BoundaryError {}
pub(super) fn invalid() -> BoundaryError {
    BoundaryError::new("invalidInput")
}

struct StrictValue {
    depth: u8,
}
impl<'de> DeserializeSeed<'de> for StrictValue {
    type Value = Value;
    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Value, D::Error> {
        deserializer.deserialize_any(self)
    }
}
impl<'de> Visitor<'de> for StrictValue {
    type Value = Value;
    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("bounded JSON")
    }
    fn visit_bool<E: serde::de::Error>(self, value: bool) -> Result<Value, E> {
        Ok(Value::Bool(value))
    }
    fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<Value, E> {
        Ok(Value::Number(value.into()))
    }
    fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Value, E> {
        Ok(Value::Number(value.into()))
    }
    fn visit_f64<E: serde::de::Error>(self, value: f64) -> Result<Value, E> {
        serde_json::Number::from_f64(value)
            .map(Value::Number)
            .ok_or_else(|| E::custom("invalid number"))
    }
    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Value, E> {
        Ok(Value::String(value.to_owned()))
    }
    fn visit_string<E: serde::de::Error>(self, value: String) -> Result<Value, E> {
        Ok(Value::String(value))
    }
    fn visit_unit<E: serde::de::Error>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Value, A::Error> {
        if self.depth >= 16 {
            return Err(serde::de::Error::custom("depth limit"));
        }
        let mut values = Vec::new();
        while let Some(value) = seq.next_element_seed(StrictValue {
            depth: self.depth + 1,
        })? {
            values.push(value);
        }
        Ok(Value::Array(values))
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Value, A::Error> {
        if self.depth >= 16 {
            return Err(serde::de::Error::custom("depth limit"));
        }
        let mut values = Map::new();
        while let Some(key) = map.next_key::<String>()? {
            if values.contains_key(&key) {
                return Err(serde::de::Error::custom("duplicate key"));
            }
            let value = map.next_value_seed(StrictValue {
                depth: self.depth + 1,
            })?;
            values.insert(key, value);
        }
        Ok(Value::Object(values))
    }
}
pub(super) fn parse(bytes: &[u8]) -> Result<Value, BoundaryError> {
    if bytes.len() > MAX_BYTES {
        return Err(invalid());
    }
    let mut deserializer = serde_json::Deserializer::from_slice(bytes);
    let value = StrictValue { depth: 0 }
        .deserialize(&mut deserializer)
        .map_err(|_| invalid())?;
    deserializer.end().map_err(|_| invalid())?;
    Ok(value)
}
pub(super) fn object(value: Value) -> Result<Map<String, Value>, BoundaryError> {
    match value {
        Value::Object(map) => Ok(map),
        _ => Err(invalid()),
    }
}
pub(super) fn take(
    map: &mut Map<String, Value>,
    key: &'static str,
) -> Result<Value, BoundaryError> {
    map.remove(key).ok_or_else(invalid)
}
pub(super) fn finish(map: Map<String, Value>) -> Result<(), BoundaryError> {
    if map.is_empty() {
        Ok(())
    } else {
        Err(invalid())
    }
}
pub(super) fn primitive<T: for<'de> Deserialize<'de>>(value: Value) -> Result<T, BoundaryError> {
    serde_json::from_value(value).map_err(|_| invalid())
}
pub(super) fn decimal(value: Value, digits: usize, maximum: u128) -> Result<u128, BoundaryError> {
    let value = value.as_str().ok_or_else(invalid)?;
    if value.is_empty()
        || value.len() > digits
        || !value.bytes().all(|c| c.is_ascii_digit())
        || (value.len() > 1 && value.starts_with('0'))
    {
        return Err(invalid());
    }
    let number = value.parse::<u128>().map_err(|_| invalid())?;
    if number > maximum {
        return Err(invalid());
    }
    Ok(number)
}
pub(super) fn counter(value: Value) -> Result<u64, BoundaryError> {
    decimal(value, 20, u128::from(u64::MAX)).map(|v| v as u64)
}
pub(super) fn seat(value: Value) -> Result<PlayerId, BoundaryError> {
    PlayerId::try_from(primitive::<u8>(value).map_err(|_| BoundaryError::new("invalidSeat"))?)
        .map_err(|_| BoundaryError::new("invalidSeat"))
}
pub(super) fn edge(value: Value) -> Result<EdgeId, BoundaryError> {
    EdgeId::try_from(primitive::<u8>(value).map_err(|_| BoundaryError::new("invalidEdge"))?)
        .map_err(|_| BoundaryError::new("invalidEdge"))
}
pub(super) fn vertex(value: Value) -> Result<VertexId, BoundaryError> {
    VertexId::try_from(primitive::<u8>(value).map_err(|_| BoundaryError::new("invalidVertex"))?)
        .map_err(|_| BoundaryError::new("invalidVertex"))
}
pub(super) fn hex(value: Value) -> Result<HexId, BoundaryError> {
    HexId::try_from(primitive::<u8>(value).map_err(|_| BoundaryError::new("invalidHex"))?)
        .map_err(|_| BoundaryError::new("invalidHex"))
}
pub(super) fn card(value: Value) -> Result<CardId, BoundaryError> {
    CardId::try_from(primitive::<u8>(value).map_err(|_| BoundaryError::new("invalidCard"))?)
        .map_err(|_| BoundaryError::new("invalidCard"))
}
pub(super) fn resource(value: Value) -> Result<Resource, BoundaryError> {
    match value.as_str() {
        Some("brick") => Ok(Resource::Brick),
        Some("wood") => Ok(Resource::Wood),
        Some("wool") => Ok(Resource::Wool),
        Some("wheat") => Ok(Resource::Wheat),
        Some("ore") => Ok(Resource::Ore),
        _ => Err(invalid()),
    }
}
pub(super) fn resources(value: Value) -> Result<Resources, BoundaryError> {
    let mut map = object(value)?;
    let mut counts = [0; 5];
    for (count, key) in counts
        .iter_mut()
        .zip(["brick", "wood", "wool", "wheat", "ore"])
    {
        *count = primitive(take(&mut map, key)?)?;
    }
    finish(map)?;
    Resources::try_from(counts).map_err(|_| invalid())
}
pub(super) fn command(value: Value) -> Result<Command, BoundaryError> {
    let mut map = object(value)?;
    let tag: String = primitive(take(&mut map, "type")?)?;
    let command = match tag.as_str() {
        "placeSetupSettlement" => Command::PlaceSetupSettlement {
            vertex: vertex(take(&mut map, "vertex")?)?,
        },
        "placeSetupRoad" => Command::PlaceSetupRoad {
            edge: edge(take(&mut map, "edge")?)?,
        },
        "rollDice" => Command::RollDice,
        "discardResources" => Command::DiscardResources {
            resources: resources(take(&mut map, "resources")?)?,
        },
        "moveRobber" => Command::MoveRobber {
            hex: hex(take(&mut map, "hex")?)?,
        },
        "chooseRobberVictim" => Command::ChooseRobberVictim {
            victim: seat(take(&mut map, "victim")?)?,
        },
        "placeFreeRoad" => Command::PlaceFreeRoad {
            edge: edge(take(&mut map, "edge")?)?,
        },
        "buildRoad" => Command::BuildRoad {
            edge: edge(take(&mut map, "edge")?)?,
        },
        "buildSettlement" => Command::BuildSettlement {
            vertex: vertex(take(&mut map, "vertex")?)?,
        },
        "upgradeCity" => Command::UpgradeCity {
            vertex: vertex(take(&mut map, "vertex")?)?,
        },
        "buyDevelopmentCard" => Command::BuyDevelopmentCard,
        "playKnight" => Command::PlayKnight {
            card: card(take(&mut map, "card")?)?,
        },
        "playRoadBuilding" => Command::PlayRoadBuilding {
            card: card(take(&mut map, "card")?)?,
        },
        "playMonopoly" => Command::PlayMonopoly {
            card: card(take(&mut map, "card")?)?,
            resource: resource(take(&mut map, "resource")?)?,
        },
        "playInvention" => Command::PlayInvention {
            card: card(take(&mut map, "card")?)?,
            resources: resources(take(&mut map, "resources")?)?,
        },
        "bankTrade" => Command::BankTrade {
            give_resource: resource(take(&mut map, "giveResource")?)?,
            receive_resource: resource(take(&mut map, "receiveResource")?)?,
        },
        "proposeTrade" => Command::ProposeTrade {
            recipient: seat(take(&mut map, "recipient")?)?,
            give: resources(take(&mut map, "give")?)?,
            receive: resources(take(&mut map, "receive")?)?,
        },
        "acceptTrade" => Command::AcceptTrade {
            offer_id: counter(take(&mut map, "offerId")?)?,
        },
        "rejectTrade" => Command::RejectTrade {
            offer_id: counter(take(&mut map, "offerId")?)?,
        },
        "cancelTrade" => Command::CancelTrade {
            offer_id: counter(take(&mut map, "offerId")?)?,
        },
        "endTurn" => Command::EndTurn,
        _ => return Err(invalid()),
    };
    finish(map)?;
    Ok(command)
}
/// Decode strict API version 1. The host must insert or authenticate the acting seat.
/// Standalone failures use revision zero; `Engine::execute_json` supplies its revision.
pub fn decode_command_envelope(bytes: &[u8]) -> Result<CommandEnvelope, BoundaryError> {
    let mut map = object(parse(bytes)?)?;
    let api_version = primitive(take(&mut map, "apiVersion")?)?;
    let actor = seat(take(&mut map, "actor")?)?;
    let expected_revision = counter(take(&mut map, "expectedRevision")?)?;
    let command = command(take(&mut map, "command")?)?;
    finish(map)?;
    if api_version != 1 {
        return Err(BoundaryError::new("unsupportedApiVersion"));
    }
    Ok(CommandEnvelope {
        api_version,
        actor,
        expected_revision,
        command,
    })
}
impl Engine {
    /// Decode and execute an envelope through ordinary native validation and atomic commit.
    /// The host must check/insert actor identity. The returned receipt is host-private;
    /// project it for a viewer before sharing. Never refresh a lost-response envelope and retry.
    pub fn execute_json(&mut self, bytes: &[u8]) -> Result<Receipt, BoundaryError> {
        let envelope = decode_command_envelope(bytes).map_err(|e| e.at(self.state.revision))?;
        self.execute(envelope)
            .map_err(|e| BoundaryError::new(rule_code(e.code)).at(e.current_revision))
    }
}
pub(super) fn rule_code(error: RuleError) -> &'static str {
    match error {
        RuleError::EmptyDeck => "emptyDeck",
        RuleError::CardNotPlayable => "cardNotPlayable",
        RuleError::InvalidTrade => "invalidTrade",
        RuleError::OfferBusy => "offerBusy",
        RuleError::OfferNotFound => "offerNotFound",
        RuleError::OfferUnavailable => "offerUnavailable",
        RuleError::BankShortage => "bankShortage",
        RuleError::NotOwned => "notOwned",
        RuleError::InsufficientResources => "insufficientResources",
        RuleError::NoPieces => "noPieces",
        RuleError::InvalidInput => "invalidInput",
        RuleError::InvalidVictim => "invalidVictim",
        RuleError::UnsupportedApiVersion(_) => "unsupportedApiVersion",
        RuleError::InvalidSeat(_) => "invalidSeat",
        RuleError::StaleRevision => "staleRevision",
        RuleError::GameFinished => "gameFinished",
        RuleError::WrongActor => "wrongActor",
        RuleError::WrongPhase => "wrongPhase",
        RuleError::CounterExhausted => "counterExhausted",
        RuleError::Occupied => "occupied",
        RuleError::DistanceRule => "distanceRule",
        RuleError::InvalidDiscard => "invalidDiscard",
        RuleError::Disconnected => "disconnected",
    }
}
pub(super) struct Wire<'a, T: ?Sized>(pub &'a T);
pub(super) fn resource_name(resource: Resource) -> &'static str {
    match resource {
        Resource::Brick => "brick",
        Resource::Wood => "wood",
        Resource::Wool => "wool",
        Resource::Wheat => "wheat",
        Resource::Ore => "ore",
    }
}
impl Serialize for Wire<'_, Resources> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut fields = s.serialize_struct("Resources", 5)?;
        for (key, count) in ["brick", "wood", "wool", "wheat", "ore"]
            .into_iter()
            .zip(self.0.counts())
        {
            fields.serialize_field(key, count)?;
        }
        fields.end()
    }
}
impl Serialize for Wire<'_, Command> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        macro_rules! fields { ($tag:literal $(, $key:literal => $value:expr)*) => {{
            let mut fields=s.serialize_struct("Command",1usize $(+ {let _=$key;1})*)?;
            fields.serialize_field("type",$tag)?;
            $(fields.serialize_field($key,&$value)?;)*
            fields.end()
        }}; }
        match self.0 {
            Command::PlaceSetupSettlement { vertex } => {
                fields!("placeSetupSettlement","vertex"=>vertex.index())
            }
            Command::PlaceSetupRoad { edge } => fields!("placeSetupRoad","edge"=>edge.index()),
            Command::RollDice => fields!("rollDice"),
            Command::DiscardResources { resources } => {
                fields!("discardResources","resources"=>Wire(resources))
            }
            Command::MoveRobber { hex } => fields!("moveRobber","hex"=>hex.index()),
            Command::ChooseRobberVictim { victim } => {
                fields!("chooseRobberVictim","victim"=>victim.index())
            }
            Command::PlaceFreeRoad { edge } => fields!("placeFreeRoad","edge"=>edge.index()),
            Command::BuildRoad { edge } => fields!("buildRoad","edge"=>edge.index()),
            Command::BuildSettlement { vertex } => {
                fields!("buildSettlement","vertex"=>vertex.index())
            }
            Command::UpgradeCity { vertex } => fields!("upgradeCity","vertex"=>vertex.index()),
            Command::BuyDevelopmentCard => fields!("buyDevelopmentCard"),
            Command::PlayKnight { card } => fields!("playKnight","card"=>card.index()),
            Command::PlayRoadBuilding { card } => fields!("playRoadBuilding","card"=>card.index()),
            Command::PlayMonopoly { card, resource } => {
                fields!("playMonopoly","card"=>card.index(),"resource"=>resource_name(*resource))
            }
            Command::PlayInvention { card, resources } => {
                fields!("playInvention","card"=>card.index(),"resources"=>Wire(resources))
            }
            Command::BankTrade {
                give_resource,
                receive_resource,
            } => {
                fields!("bankTrade","giveResource"=>resource_name(*give_resource),"receiveResource"=>resource_name(*receive_resource))
            }
            Command::ProposeTrade {
                recipient,
                give,
                receive,
            } => {
                fields!("proposeTrade","recipient"=>recipient.index(),"give"=>Wire(give),"receive"=>Wire(receive))
            }
            Command::AcceptTrade { offer_id } => {
                fields!("acceptTrade","offerId"=>offer_id.to_string())
            }
            Command::RejectTrade { offer_id } => {
                fields!("rejectTrade","offerId"=>offer_id.to_string())
            }
            Command::CancelTrade { offer_id } => {
                fields!("cancelTrade","offerId"=>offer_id.to_string())
            }
            Command::EndTurn => fields!("endTurn"),
        }
    }
}
impl Serialize for Wire<'_, CommandEnvelope> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut fields = s.serialize_struct("CommandEnvelope", 4)?;
        fields.serialize_field("apiVersion", &self.0.api_version)?;
        fields.serialize_field("actor", &self.0.actor.index())?;
        fields.serialize_field("expectedRevision", &self.0.expected_revision.to_string())?;
        fields.serialize_field("command", &Wire(&self.0.command))?;
        fields.end()
    }
}
pub(super) fn encode<T: Serialize>(value: &T) -> Result<Vec<u8>, BoundaryError> {
    serde_json::to_vec(value).map_err(|_| invalid())
}
/// Encode a host command in declared field order with exact decimal counters.
pub fn encode_command_envelope(envelope: &CommandEnvelope) -> Result<Vec<u8>, BoundaryError> {
    encode(&Wire(envelope))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wire_parser_enforces_actual_bytes_and_structural_depth() {
        let mut bytes = b"null".to_vec();
        bytes.resize(MAX_BYTES, b' ');
        assert_eq!(parse(&bytes), Ok(Value::Null));
        bytes.push(b' ');
        assert!(parse(&bytes).is_err());
        for depth in [16, 17] {
            let bytes = format!("{}0{}", "[".repeat(depth), "]".repeat(depth));
            assert_eq!(parse(bytes.as_bytes()).is_ok(), depth == 16);
        }
        assert!(parse(b"\"{{[[\"").is_ok());
        for bytes in [&b"\xff"[..], &b"\"\\q\""[..], &b"1e9999999"[..]] {
            assert!(parse(bytes).is_err());
        }
    }
}
