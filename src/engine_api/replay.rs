//! Host-private format-1 replay, bounded one header/accepted record at a time.
use super::{
    CommandEnvelope, Engine, Receipt,
    receipt::projected_event,
    wire::{self, BoundaryError, Wire},
};
use crate::game_model::GameState;
use serde::{
    Serialize, Serializer,
    ser::{SerializeSeq, SerializeStruct},
};
use serde_json::{Map, Value};
use std::io::{self, Write};

/// The first rejected or mismatching replay record; never includes private input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReplayError {
    index: usize,
    code: &'static str,
}
impl ReplayError {
    /// Zero-based accepted-record index. Header failures also use zero.
    pub fn index(&self) -> usize {
        self.index
    }
    /// Static boundary, rule, or replayMismatch code; no command/state is echoed.
    pub fn code(&self) -> &'static str {
        self.code
    }
}
impl std::fmt::Display for ReplayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at replay record {}", self.code, self.index)
    }
}
impl std::error::Error for ReplayError {}

// Bound allocation while serializing a caller-owned Receipt, whose event vector can be arbitrary.
struct BoundedBytes(Vec<u8>);
impl Write for BoundedBytes {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() > wire::MAX_BYTES - self.0.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "replay size limit",
            ));
        }
        self.0.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
fn encode<T: Serialize>(value: &T) -> Result<Vec<u8>, BoundaryError> {
    let mut bytes = BoundedBytes(Vec::new());
    serde_json::to_writer(&mut bytes, value).map_err(|_| wire::invalid())?;
    wire::parse(&bytes.0)?; // Export must satisfy the same structural bound as import.
    Ok(bytes.0)
}
struct Header<'a>(&'a Engine);
impl Serialize for Header<'_> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut fields = s.serialize_struct("ReplayHeader", 4)?;
        fields.serialize_field("replayVersion", &1_u32)?;
        fields.serialize_field("rulesVersion", &"hex-traders-base-v1")?;
        fields.serialize_field("randomAlgorithm", &"chacha12-rand010-v1")?;
        fields.serialize_field("initialSnapshot", &Wire(self.0))?;
        fields.end()
    }
}
struct PrivateEvents<'a>(&'a [crate::Event]);
impl Serialize for PrivateEvents<'_> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut seq = s.serialize_seq(Some(self.0.len()))?;
        for &event in self.0 {
            seq.serialize_element(&Wire(&projected_event(event, None)))?;
        }
        seq.end()
    }
}
impl Serialize for Wire<'_, Receipt> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let r = self.0;
        let mut fields = s.serialize_struct("ReplayRecord", 9)?;
        fields.serialize_field("apiVersion", &r.api_version)?;
        fields.serialize_field("rulesVersion", &r.rules_version)?;
        fields.serialize_field("randomAlgorithm", &r.random_algorithm)?;
        fields.serialize_field("actor", &Wire(&r.actor))?;
        fields.serialize_field("command", &Wire(&r.command))?;
        fields.serialize_field("previousRevision", &r.previous_revision.to_string())?;
        fields.serialize_field("revision", &r.revision.to_string())?;
        fields.serialize_field("events", &PrivateEvents(&r.events))?;
        fields.serialize_field("finalScores", &Wire(&r.final_scores))?;
        fields.end()
    }
}
impl Engine {
    /// Encode a format-1 header containing this complete host-private initial snapshot.
    /// The caller stores this header and accepted records separately and controls stream length.
    /// Each encoded unit is at most 64 KiB and container depth 16. No history or I/O is retained.
    pub fn export_replay_header(&self) -> Result<Vec<u8>, BoundaryError> {
        encode(&Header(self))
    }
}
impl Receipt {
    /// Encode one accepted format-1 record, including private command/event facts and final scores.
    /// Keep it host-private; use ordinary projected receipts for player activity history.
    /// Fabricated receipts exceeding 64 KiB/depth 16 reject rather than exporting invalid records.
    pub fn to_replay_record(&self) -> Result<Vec<u8>, BoundaryError> {
        encode(&Wire(self))
    }
}
fn version(
    map: &mut Map<String, Value>,
    key: &'static str,
    expected: &str,
    code: &'static str,
) -> Result<(), BoundaryError> {
    let actual: String = wire::primitive(wire::take(map, key)?)?;
    if actual == expected {
        Ok(())
    } else {
        Err(BoundaryError::new(code))
    }
}
fn versions(map: &mut Map<String, Value>) -> Result<(), BoundaryError> {
    version(
        map,
        "rulesVersion",
        "hex-traders-base-v1",
        "incompatibleRulesVersion",
    )?;
    version(
        map,
        "randomAlgorithm",
        "chacha12-rand010-v1",
        "incompatibleRandomAlgorithm",
    )
}
fn header(bytes: &[u8]) -> Result<Engine, BoundaryError> {
    let mut map = wire::object(wire::parse(bytes)?)?;
    let format: u32 = wire::primitive(wire::take(&mut map, "replayVersion")?)?;
    if format != 1 {
        return Err(BoundaryError::new("unsupportedSchemaVersion"));
    }
    versions(&mut map)?;
    let snapshot = wire::take(&mut map, "initialSnapshot")?;
    wire::finish(map)?;
    Engine::from_snapshot(&wire::encode(&snapshot)?)
}
fn envelope(expected: &Value) -> Result<(CommandEnvelope, u64), BoundaryError> {
    let mut map = wire::object(expected.clone())?;
    let api_version: u32 = wire::primitive(wire::take(&mut map, "apiVersion")?)?;
    if api_version != 1 {
        return Err(BoundaryError::new("unsupportedApiVersion"));
    }
    versions(&mut map)?;
    let actor = wire::seat(wire::take(&mut map, "actor")?)?;
    let command = wire::command(wire::take(&mut map, "command")?)?;
    let expected_revision = wire::counter(wire::take(&mut map, "previousRevision")?)?;
    let revision = wire::counter(wire::take(&mut map, "revision")?)?;
    if !wire::take(&mut map, "events")?.is_array() {
        return Err(wire::invalid());
    }
    wire::take(&mut map, "finalScores")?;
    wire::finish(map)?;
    Ok((
        CommandEnvelope {
            api_version,
            actor,
            expected_revision,
            command,
        },
        revision,
    ))
}
/// A host-private verifier retaining only the current game and next zero-based record index.
/// Feed bounded records individually; the host controls how many records it reads.
/// A failure preserves the last verified snapshot/index, so the corrected record can be retried.
///
/// ```
/// use hex_traders_engine::{Command,CommandEnvelope,Engine,GameConfig,ReplayVerifier,Viewer};
/// let (mut engine,_) = Engine::create(GameConfig::fixed(&[0,1,2])?,[0;32])?;
/// let mut replay = ReplayVerifier::from_header(&engine.export_replay_header()?)?;
/// let view = engine.view(Viewer::Spectator)?;
/// let receipt = engine.execute(CommandEnvelope {api_version:1,actor:view.active_player,
///     expected_revision:view.revision,command:Command::RollDice})?;
/// assert_eq!(replay.push(&receipt.to_replay_record()?)?,receipt);
/// assert_eq!(replay.into_engine().export_snapshot()?,engine.export_snapshot()?);
/// # Ok::<(),Box<dyn std::error::Error>>(())
/// ```
pub struct ReplayVerifier {
    engine: Engine,
    index: usize,
}
impl ReplayVerifier {
    /// Validate bounded format/version/header data and construct a separate initial engine.
    /// Header failures use index zero; no partially imported engine is returned.
    pub fn from_header(bytes: &[u8]) -> Result<Self, ReplayError> {
        header(bytes)
            .map(|engine| Self { engine, index: 0 })
            .map_err(|e| ReplayError {
                index: 0,
                code: e.code(),
            })
    }
    /// Verify one accepted record through ordinary execution and exact private event/revision equality.
    /// JSON field order and whitespace may differ; arrays, counters, and private facts must match.
    /// Rejected commands are never advanced into accepted replay history.
    pub fn push(&mut self, bytes: &[u8]) -> Result<Receipt, ReplayError> {
        let fail = |code| ReplayError {
            index: self.index,
            code,
        };
        let next = self
            .index
            .checked_add(1)
            .ok_or_else(|| fail("counterExhausted"))?;
        let expected = wire::parse(bytes).map_err(|e| fail(e.code()))?;
        let (envelope, revision) = envelope(&expected).map_err(|e| fail(e.code()))?;
        if envelope.expected_revision != self.engine.state.revision
            || envelope.expected_revision.checked_add(1) != Some(revision)
        {
            return Err(fail("replayMismatch"));
        }
        let before = GameState {
            position: self.engine.state.position.clone(),
            random: self.engine.state.random.clone(),
            revision: self.engine.state.revision,
        };
        let receipt = self
            .engine
            .execute(envelope)
            .map_err(|e| fail(wire::rule_code(e.code)))?;
        let actual = receipt.to_replay_record().and_then(|v| wire::parse(&v));
        match actual {
            Ok(value) if value == expected => {
                self.index = next;
                Ok(receipt)
            }
            other => {
                self.engine.state = before;
                Err(fail(other.err().map_or("replayMismatch", |e| e.code())))
            }
        }
    }
    /// Borrow the last fully verified game for host inspection; this does not expose private internals.
    pub fn engine(&self) -> &Engine {
        &self.engine
    }
    /// Finish inspection and take the last verified engine, without retaining replay history.
    pub fn into_engine(self) -> Engine {
        self.engine
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replay_writer_rejects_crossing_the_bound_without_growing() {
        let mut bytes = BoundedBytes(Vec::new());
        assert_eq!(
            bytes.write(&vec![0; wire::MAX_BYTES - 1]).unwrap(),
            wire::MAX_BYTES - 1
        );
        assert!(bytes.write(&[1, 2]).is_err());
        assert_eq!(bytes.0.len(), wire::MAX_BYTES - 1);
        assert_eq!(bytes.write(&[3]).unwrap(), 1);
        assert_eq!(bytes.0.len(), wire::MAX_BYTES);
        assert!(bytes.write(&[4]).is_err());
        assert_eq!(bytes.0.len(), wire::MAX_BYTES);
        bytes.flush().unwrap();
    }
}
