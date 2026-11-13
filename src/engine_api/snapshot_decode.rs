//! Strict bounded candidate conversion; no candidate is exposed before native validation.
use super::super::{
    Engine,
    wire::{
        self, BoundaryError, counter, decimal, finish, object, parse, primitive, resource,
        resources, seat, take,
    },
};
use crate::game_model::{
    Board, BoardGraph, Building, BuildingKind, CardLocation, GameConfig, GameState, Phase, Port,
    PortKind, Position, ResumePhase, RobberOrigin, SetupMode, Terrain,
};
use rand::SeedableRng;
use serde_json::Value;

type Result<T> = std::result::Result<T, BoundaryError>;
fn invalid() -> BoundaryError {
    BoundaryError::new("invalidSnapshot")
}
fn array<T, const N: usize>(value: Value, decode: fn(Value) -> Result<T>) -> Result<[T; N]> {
    let Value::Array(values) = value else {
        return Err(invalid());
    };
    if values.len() != N {
        return Err(invalid());
    }
    values
        .into_iter()
        .map(decode)
        .collect::<Result<Vec<_>>>()?
        .try_into()
        .map_err(|_| invalid())
}
fn optional<T>(value: Value, decode: fn(Value) -> Result<T>) -> Result<Option<T>> {
    if value.is_null() {
        Ok(None)
    } else {
        decode(value).map(Some)
    }
}
fn config(value: Value) -> Result<GameConfig> {
    let mut m = object(value)?;
    let seats: Vec<u8> = primitive(take(&mut m, "seatOrder")?)?;
    let setup = match take(&mut m, "setup")?.as_str() {
        Some("fixed") => SetupMode::Fixed,
        Some("variable") => SetupMode::Variable,
        _ => return Err(invalid()),
    };
    finish(m)?;
    match setup {
        SetupMode::Fixed => GameConfig::fixed(&seats),
        SetupMode::Variable => GameConfig::variable(&seats),
    }
    .map_err(|_| invalid())
}
fn terrain(value: Value) -> Result<Terrain> {
    match value.as_str() {
        Some("hills") => Ok(Terrain::Hills),
        Some("forest") => Ok(Terrain::Forest),
        Some("pasture") => Ok(Terrain::Pasture),
        Some("field") => Ok(Terrain::Field),
        Some("mountain") => Ok(Terrain::Mountain),
        Some("desert") => Ok(Terrain::Desert),
        _ => Err(invalid()),
    }
}
fn port(value: Value) -> Result<Port> {
    let mut m = object(value)?;
    let edge = wire::edge(take(&mut m, "edge")?)?;
    let mut kind = object(take(&mut m, "kind")?)?;
    let tag: String = primitive(take(&mut kind, "type")?)?;
    let k = match tag.as_str() {
        "generic" => PortKind::Generic,
        "special" => PortKind::Special(resource(take(&mut kind, "resource")?)?),
        _ => return Err(invalid()),
    };
    finish(kind)?;
    finish(m)?;
    Ok(Port { edge, kind: k })
}
fn board(value: Value) -> Result<Board> {
    let mut m = object(value)?;
    let board = Board {
        graph: BoardGraph::base_game(),
        terrain: array(take(&mut m, "terrain")?, terrain)?,
        numbers: array(take(&mut m, "numbers")?, |v| optional(v, primitive::<u8>))?,
        ports: array(take(&mut m, "ports")?, port)?,
        frame_permutation: array(take(&mut m, "framePermutation")?, primitive::<u8>)?,
    };
    finish(m)?;
    Ok(board)
}
fn building(value: Value) -> Result<Building> {
    let mut m = object(value)?;
    let owner = seat(take(&mut m, "owner")?)?;
    let kind = match take(&mut m, "kind")?.as_str() {
        Some("settlement") => BuildingKind::Settlement,
        Some("city") => BuildingKind::City,
        _ => return Err(invalid()),
    };
    finish(m)?;
    Ok(Building { owner, kind })
}
fn offer(value: Value) -> Result<crate::TradeOffer> {
    let mut m = object(value)?;
    let offer = crate::TradeOffer {
        id: counter(take(&mut m, "id")?)?,
        turn: counter(take(&mut m, "turn")?)?,
        proposer: seat(take(&mut m, "proposer")?)?,
        recipient: seat(take(&mut m, "recipient")?)?,
        give: resources(take(&mut m, "give")?)?,
        receive: resources(take(&mut m, "receive")?)?,
    };
    finish(m)?;
    Ok(offer)
}
fn location(value: Value) -> Result<CardLocation> {
    let mut m = object(value)?;
    let tag: String = primitive(take(&mut m, "type")?)?;
    let v = match tag.as_str() {
        "undrawn" => CardLocation::Undrawn,
        "held" => CardLocation::Held {
            owner: seat(take(&mut m, "owner")?)?,
            acquired_turn: counter(take(&mut m, "acquiredTurn")?)?,
        },
        "spent" => CardLocation::Spent {
            owner: seat(take(&mut m, "owner")?)?,
        },
        "playedKnight" => CardLocation::PlayedKnight {
            owner: seat(take(&mut m, "owner")?)?,
        },
        "revealedVictory" => CardLocation::RevealedVictory {
            owner: seat(take(&mut m, "owner")?)?,
        },
        _ => return Err(invalid()),
    };
    finish(m)?;
    Ok(v)
}
fn resume(value: Value) -> Result<ResumePhase> {
    match value.as_str() {
        Some("preRoll") => Ok(ResumePhase::PreRoll),
        Some("action") => Ok(ResumePhase::Action),
        _ => Err(invalid()),
    }
}
fn origin(value: Value) -> Result<RobberOrigin> {
    match value.as_str() {
        Some("seven") => Ok(RobberOrigin::Seven),
        Some("knight") => Ok(RobberOrigin::Knight),
        _ => Err(invalid()),
    }
}
fn phase(value: Value) -> Result<Phase> {
    let mut m = object(value)?;
    let tag: String = primitive(take(&mut m, "type")?)?;
    let phase = match tag.as_str() {
        "setupSettlement" => Phase::SetupSettlement {
            actor: seat(take(&mut m, "actor")?)?,
            round: primitive(take(&mut m, "round")?)?,
        },
        "setupRoad" => Phase::SetupRoad {
            actor: seat(take(&mut m, "actor")?)?,
            round: primitive(take(&mut m, "round")?)?,
            settlement: wire::vertex(take(&mut m, "settlement")?)?,
        },
        "preRoll" => Phase::PreRoll,
        "action" => Phase::Action,
        "discard" => Phase::Discard {
            obligations: array(take(&mut m, "obligations")?, primitive::<u8>)?,
        },
        "robberMove" => Phase::RobberMove {
            origin: origin(take(&mut m, "origin")?)?,
            continuation: resume(take(&mut m, "continuation")?)?,
        },
        "robberVictim" => Phase::RobberVictim {
            origin: origin(take(&mut m, "origin")?)?,
            continuation: resume(take(&mut m, "continuation")?)?,
            victims: array(take(&mut m, "victims")?, |v| optional(v, seat))?,
        },
        "freeRoads" => Phase::FreeRoads {
            remaining: primitive(take(&mut m, "remaining")?)?,
            continuation: resume(take(&mut m, "continuation")?)?,
        },
        "finished" => Phase::Finished {
            winner: seat(take(&mut m, "winner")?)?,
            score: primitive(take(&mut m, "score")?)?,
        },
        _ => return Err(invalid()),
    };
    finish(m)?;
    Ok(phase)
}
fn position(value: Value, config: GameConfig) -> Result<Position> {
    let mut m = object(value)?;
    let p = Position {
        config,
        roads: array(take(&mut m, "roads")?, |v| optional(v, seat))?,
        buildings: array(take(&mut m, "buildings")?, |v| optional(v, building))?,
        road_lengths: array(take(&mut m, "roadLengths")?, primitive::<u8>)?,
        longest_route_holder: optional(take(&mut m, "longestRouteHolder")?, seat)?,
        largest_army_holder: optional(take(&mut m, "largestArmyHolder")?, seat)?,
        second_settlements: array(take(&mut m, "secondSettlements")?, |v| {
            optional(v, wire::vertex)
        })?,
        robber: wire::hex(take(&mut m, "robber")?)?,
        bank: resources(take(&mut m, "bank")?)?,
        hands: array(take(&mut m, "hands")?, resources)?,
        offer: optional(take(&mut m, "offer")?, offer)?,
        deck: array(take(&mut m, "deck")?, wire::card)?,
        draw_cursor: primitive(take(&mut m, "drawCursor")?)?,
        card_locations: array(take(&mut m, "cardLocations")?, location)?,
        played_non_vp: primitive(take(&mut m, "playedNonVp")?)?,
        final_scores: optional(take(&mut m, "finalScores")?, |v| array(v, primitive::<u8>))?,
        starting_player: seat(take(&mut m, "startingPlayer")?)?,
        active_player: seat(take(&mut m, "activePlayer")?)?,
        turn: counter(take(&mut m, "turn")?)?,
        phase: phase(take(&mut m, "phase")?)?,
        last_dice: optional(take(&mut m, "lastDice")?, |v| array(v, primitive::<u8>))?,
    };
    finish(m)?;
    Ok(p)
}
fn random(value: Value) -> Result<rand_chacha::ChaCha12Rng> {
    let mut m = object(value)?;
    let seed: String = primitive(take(&mut m, "seed")?)?;
    if seed.len() != 64
        || !seed
            .bytes()
            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
    {
        return Err(invalid());
    }
    let stream = counter(take(&mut m, "stream")?)?;
    if stream != 0 {
        return Err(invalid());
    }
    let word = decimal(take(&mut m, "wordPosition")?, 21, (1_u128 << 68) - 1)?;
    finish(m)?;
    let mut bytes = [0; 32];
    fn digit(c: u8) -> u8 {
        if c.is_ascii_digit() {
            c - b'0'
        } else {
            c - b'a' + 10
        }
    }
    for (byte, pair) in bytes.iter_mut().zip(seed.as_bytes().as_chunks::<2>().0) {
        *byte = digit(pair[0]) * 16 + digit(pair[1]);
    }
    let mut random = rand_chacha::ChaCha12Rng::from_seed(bytes);
    random.set_word_pos(word);
    Ok(random)
}
fn candidate(bytes: &[u8]) -> Result<Engine> {
    let mut m = object(parse(bytes)?)?;
    let schema: u32 = primitive(take(&mut m, "schemaVersion")?)?;
    let rules: String = primitive(take(&mut m, "rulesVersion")?)?;
    let algorithm: String = primitive(take(&mut m, "randomAlgorithm")?)?;
    let config = config(take(&mut m, "config")?)?;
    let board = board(take(&mut m, "layout")?)?;
    let position = position(take(&mut m, "position")?, config)?;
    let revision = counter(take(&mut m, "revision")?)?;
    let random = random(take(&mut m, "random")?)?;
    finish(m)?;
    if schema != 1 {
        return Err(BoundaryError::new("unsupportedSchemaVersion"));
    }
    if rules != "hex-traders-base-v1" {
        return Err(BoundaryError::new("incompatibleRulesVersion"));
    }
    if algorithm != "chacha12-rand010-v1" {
        return Err(BoundaryError::new("incompatibleRandomAlgorithm"));
    }
    let state = GameState {
        position,
        revision,
        random,
    };
    crate::turn_engine::validate_game(&board, &state).map_err(|_| invalid())?;
    Ok(Engine { board, state })
}
impl Engine {
    /// Construct a separate engine from bounded host-private schema-1 bytes.
    /// Every field and native invariant is checked before the engine is returned.
    /// Failure cannot replace or mutate any existing engine. Topology is rebuilt;
    /// validation establishes current facts, not proof of honest past play.
    ///
    /// ```
    /// use hex_traders_engine::{Engine,GameConfig};
    /// let (original,_) = Engine::create(GameConfig::variable(&[0,1,2])?,[0;32])?;
    /// let bytes=original.export_snapshot()?;
    /// let resumed=Engine::from_snapshot(&bytes)?;
    /// assert_eq!(resumed.export_snapshot()?,bytes);
    /// # Ok::<(),Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_snapshot(bytes: &[u8]) -> std::result::Result<Self, BoundaryError> {
        candidate(bytes).map_err(|e| match e.code() {
            "unsupportedSchemaVersion"
            | "incompatibleRulesVersion"
            | "incompatibleRandomAlgorithm" => e,
            _ => invalid(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_model::VertexId;
    use serde_json::json;
    fn player() -> crate::PlayerId {
        crate::PlayerId::try_from(3).unwrap()
    }
    #[test]
    fn import_enum_payloads_nullable_ids_and_every_branch_are_exact_and_strict() {
        let phases = [
            (
                json!({"type":"setupSettlement","actor":3,"round":2}),
                Phase::SetupSettlement {
                    actor: player(),
                    round: 2,
                },
            ),
            (
                json!({"type":"setupRoad","actor":3,"round":2,"settlement":53}),
                Phase::SetupRoad {
                    actor: player(),
                    round: 2,
                    settlement: VertexId::try_from(53).unwrap(),
                },
            ),
            (json!({"type":"preRoll"}), Phase::PreRoll),
            (json!({"type":"action"}), Phase::Action),
            (
                json!({"type":"discard","obligations":[0,1,2,3]}),
                Phase::Discard {
                    obligations: [0, 1, 2, 3],
                },
            ),
            (
                json!({"type":"robberMove","origin":"seven","continuation":"action"}),
                Phase::RobberMove {
                    origin: RobberOrigin::Seven,
                    continuation: ResumePhase::Action,
                },
            ),
            (
                json!({"type":"robberVictim","origin":"knight","continuation":"preRoll","victims":[null,null,null,3]}),
                Phase::RobberVictim {
                    origin: RobberOrigin::Knight,
                    continuation: ResumePhase::PreRoll,
                    victims: [None, None, None, Some(player())],
                },
            ),
            (
                json!({"type":"freeRoads","remaining":1,"continuation":"action"}),
                Phase::FreeRoads {
                    remaining: 1,
                    continuation: ResumePhase::Action,
                },
            ),
            (
                json!({"type":"finished","winner":3,"score":12}),
                Phase::Finished {
                    winner: player(),
                    score: 12,
                },
            ),
        ];
        for (value, expected) in phases {
            assert_eq!(phase(value.clone()), Ok(expected));
            for key in value.as_object().unwrap().keys() {
                let mut bad = value.clone();
                bad.as_object_mut().unwrap().remove(key);
                assert!(phase(bad).is_err());
            }
            let mut bad = value;
            bad["privateSentinel"] = json!(0);
            assert!(phase(bad).is_err());
        }
        for (tag, key) in [("discard", "obligations"), ("robberVictim", "victims")] {
            for n in [3, 5] {
                let mut bad = if tag == "discard" {
                    json!({"type":"discard"})
                } else {
                    json!({"type":"robberVictim","origin":"seven","continuation":"action"})
                };
                bad[key] = json!(vec![0; n]);
                assert!(phase(bad).is_err());
            }
        }
        let locations = [
            (json!({"type":"undrawn"}), CardLocation::Undrawn),
            (
                json!({"type":"held","owner":3,"acquiredTurn":"18446744073709551615"}),
                CardLocation::Held {
                    owner: player(),
                    acquired_turn: u64::MAX,
                },
            ),
            (
                json!({"type":"spent","owner":3}),
                CardLocation::Spent { owner: player() },
            ),
            (
                json!({"type":"playedKnight","owner":3}),
                CardLocation::PlayedKnight { owner: player() },
            ),
            (
                json!({"type":"revealedVictory","owner":3}),
                CardLocation::RevealedVictory { owner: player() },
            ),
        ];
        for (value, expected) in locations {
            assert_eq!(location(value.clone()), Ok(expected));
            for key in value.as_object().unwrap().keys() {
                let mut bad = value.clone();
                bad.as_object_mut().unwrap().remove(key);
                assert!(location(bad).is_err());
            }
            let mut bad = value;
            bad["privateSentinel"] = json!(0);
            assert!(location(bad).is_err());
        }
        for kind in ["settlement", "city"] {
            let value = json!({"kind":kind,"owner":3});
            let b = building(value.clone()).unwrap();
            assert_eq!(b.owner, player());
            assert_eq!(
                b.kind,
                if kind == "city" {
                    BuildingKind::City
                } else {
                    BuildingKind::Settlement
                }
            );
            let mut bad = value;
            bad["privateSentinel"] = json!(0);
            assert!(building(bad).is_err());
        }
        assert!(location(json!([])).is_err());
        assert!(location(json!({"type":"privateSentinel"})).is_err());
        assert!(
            phase(json!({"type":"robberMove","origin":"privateSentinel","continuation":"action"}))
                .is_err()
        );
        assert!(
            phase(json!({"type":"freeRoads","remaining":1,"continuation":"privateSentinel"}))
                .is_err()
        );
        assert!(building(json!({"kind":"privateSentinel","owner":3})).is_err());
        for decimal in ["0", "18446744073709551615"] {
            let value = json!({"id":decimal,"turn":decimal,"proposer":3,"recipient":2,"give":{"brick":1,"wood":0,"wool":0,"wheat":0,"ore":0},"receive":{"brick":0,"wood":2,"wool":0,"wheat":0,"ore":0}});
            assert_eq!(
                offer(value.clone()),
                Ok(crate::TradeOffer {
                    id: decimal.parse().unwrap(),
                    turn: decimal.parse().unwrap(),
                    proposer: player(),
                    recipient: crate::PlayerId::try_from(2).unwrap(),
                    give: crate::Resources::try_from([1, 0, 0, 0, 0]).unwrap(),
                    receive: crate::Resources::try_from([0, 2, 0, 0, 0]).unwrap()
                })
            );
            for key in value.as_object().unwrap().keys() {
                let mut bad = value.clone();
                bad.as_object_mut().unwrap().remove(key);
                assert!(offer(bad).is_err());
            }
            let mut bad = value;
            bad["privateSentinel"] = json!(0);
            assert!(offer(bad).is_err());
        }
        assert!(offer(json!([])).is_err());
    }
}
