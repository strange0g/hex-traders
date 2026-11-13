//! Complete host-private schema-1 export; topology and history are not serialized.
#[path = "snapshot_decode.rs"]
mod decode;
use super::{
    Engine,
    wire::{BoundaryError, Wire, encode},
};
use crate::game_model::{Board, CardLocation, GameConfig, Position};
use serde::{Serialize, Serializer, ser::SerializeStruct};

macro_rules! fields { ($s:ident; $($key:literal => $value:expr),* $(,)?) => {{
    let keys: &[&str] = &[$($key),*];
    let mut fields=$s.serialize_struct("Snapshot",keys.len())?;
    $(fields.serialize_field($key,&$value)?;)*
    fields.end()
}}; }

impl Serialize for Wire<'_, GameConfig> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        fields!(s; "seatOrder"=>Wire(self.0.seat_order()), "setup"=>Wire(&self.0.setup()))
    }
}
impl Serialize for Wire<'_, Board> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let v = self.0;
        fields!(s; "terrain"=>Wire(&v.terrain), "numbers"=>Wire(&v.numbers), "ports"=>Wire(&v.ports), "framePermutation"=>Wire(&v.frame_permutation))
    }
}
impl Serialize for Wire<'_, CardLocation> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            CardLocation::Undrawn => fields!(s; "type"=>"undrawn"),
            CardLocation::Held {
                owner,
                acquired_turn,
            } => {
                fields!(s; "type"=>"held", "owner"=>Wire(owner), "acquiredTurn"=>acquired_turn.to_string())
            }
            CardLocation::Spent { owner } => fields!(s; "type"=>"spent", "owner"=>Wire(owner)),
            CardLocation::PlayedKnight { owner } => {
                fields!(s; "type"=>"playedKnight", "owner"=>Wire(owner))
            }
            CardLocation::RevealedVictory { owner } => {
                fields!(s; "type"=>"revealedVictory", "owner"=>Wire(owner))
            }
        }
    }
}
impl Serialize for Wire<'_, Position> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let v = self.0;
        fields!(s;
            "roads"=>Wire(&v.roads), "buildings"=>Wire(&v.buildings), "roadLengths"=>Wire(&v.road_lengths),
            "longestRouteHolder"=>Wire(&v.longest_route_holder), "largestArmyHolder"=>Wire(&v.largest_army_holder),
            "secondSettlements"=>Wire(&v.second_settlements), "robber"=>Wire(&v.robber), "bank"=>Wire(&v.bank),
            "hands"=>Wire(&v.hands), "offer"=>Wire(&v.offer), "deck"=>Wire(&v.deck), "drawCursor"=>v.draw_cursor,
            "cardLocations"=>Wire(&v.card_locations), "playedNonVp"=>v.played_non_vp, "finalScores"=>Wire(&v.final_scores),
            "startingPlayer"=>Wire(&v.starting_player), "activePlayer"=>Wire(&v.active_player), "turn"=>v.turn.to_string(),
            "phase"=>Wire(&v.phase), "lastDice"=>Wire(&v.last_dice))
    }
}
struct Random<'a>(&'a rand_chacha::ChaCha12Rng);
impl Serialize for Random<'_> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut seed = String::with_capacity(64);
        const HEX: &[u8; 16] = b"0123456789abcdef";
        for byte in self.0.get_seed() {
            seed.push(char::from(HEX[usize::from(byte >> 4)]));
            seed.push(char::from(HEX[usize::from(byte & 15)]));
        }
        // ChaCha's buffered cursor can report just beyond its 68-bit period
        // after a wrapping draw. set_word_pos ignores those high bits; encode
        // the equivalent canonical position so the next snapshot can resume.
        let word_position = self.0.get_word_pos() % (1_u128 << 68);
        fields!(s; "seed"=>seed, "stream"=>self.0.get_stream().to_string(), "wordPosition"=>word_position.to_string())
    }
}
impl Serialize for Wire<'_, Engine> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let v = self.0;
        fields!(s; "schemaVersion"=>1_u32, "rulesVersion"=>"hex-traders-base-v1", "randomAlgorithm"=>"chacha12-rand010-v1",
            "config"=>Wire(&v.state.position.config), "layout"=>Wire(&v.board), "position"=>Wire(&v.state.position),
            "revision"=>v.state.revision.to_string(), "random"=>Random(&v.state.random))
    }
}
impl Engine {
    /// Return the complete host-private snapshot in stable schema-1 field order.
    /// Keep these bytes secret: they contain every hand, deck identity and RNG seed.
    /// This read does not consume randomness, retain history, or perform storage I/O.
    ///
    /// ```
    /// use hex_traders_engine::{Engine, GameConfig};
    /// let (engine, _) = Engine::create(GameConfig::fixed(&[0,1,2])?, [0;32])?;
    /// let host_private_bytes = engine.export_snapshot()?;
    /// assert!(host_private_bytes.len() <= 65_536);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn export_snapshot(&self) -> Result<Vec<u8>, BoundaryError> {
        encode(&Wire(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_model::{
        Building, BuildingKind, CardId, EdgeId, HexId, Phase, PlayerId, Resources, ResumePhase,
        RobberOrigin, TradeOffer, VertexId,
    };
    fn seat(v: u8) -> PlayerId {
        PlayerId::try_from(v).unwrap()
    }
    #[test]
    fn export_every_private_fact_and_pending_phase_without_random_consumption() {
        let (mut engine, _) =
            Engine::create(GameConfig::fixed(&[0, 1, 2, 3]).unwrap(), [0xab; 32]).unwrap();
        let p = &mut engine.state.position;
        p.roads = [Some(seat(3)); EdgeId::COUNT];
        p.buildings = [Some(Building {
            kind: BuildingKind::City,
            owner: seat(3),
        }); VertexId::COUNT];
        p.road_lengths = [12, 13, 14, 15];
        p.longest_route_holder = Some(seat(3));
        p.largest_army_holder = Some(seat(2));
        p.second_settlements = [Some(VertexId::try_from(53).unwrap()), None, None, None];
        p.robber = HexId::try_from(18).unwrap();
        p.bank = Resources::try_from([1, 2, 3, 4, 5]).unwrap();
        p.hands = [Resources::try_from([19; 5]).unwrap(); 4];
        p.offer = Some(TradeOffer {
            id: u64::MAX,
            turn: u64::MAX,
            proposer: seat(3),
            recipient: seat(2),
            give: Resources::try_from([1, 0, 0, 0, 0]).unwrap(),
            receive: Resources::try_from([0, 1, 0, 0, 0]).unwrap(),
        });
        p.deck = std::array::from_fn(|i| CardId::try_from(24 - i as u8).unwrap());
        p.draw_cursor = 25;
        p.card_locations = [CardLocation::Undrawn; 25];
        p.card_locations[0] = CardLocation::Held {
            owner: seat(3),
            acquired_turn: u64::MAX,
        };
        p.card_locations[1] = CardLocation::PlayedKnight { owner: seat(2) };
        p.card_locations[14] = CardLocation::Spent { owner: seat(1) };
        p.card_locations[24] = CardLocation::RevealedVictory { owner: seat(0) };
        p.played_non_vp = true;
        p.final_scores = Some([8, 9, 10, 11]);
        p.starting_player = seat(2);
        p.active_player = seat(3);
        p.turn = u64::MAX;
        p.last_dice = Some([6, 6]);
        engine.state.revision = u64::MAX;
        engine.state.random.set_word_pos((1_u128 << 68) - 1);
        let phases = [
            Phase::SetupSettlement {
                actor: seat(3),
                round: 2,
            },
            Phase::SetupRoad {
                actor: seat(3),
                round: 2,
                settlement: VertexId::try_from(53).unwrap(),
            },
            Phase::PreRoll,
            Phase::Action,
            Phase::Discard {
                obligations: [1, 2, 3, 4],
            },
            Phase::RobberMove {
                origin: RobberOrigin::Seven,
                continuation: ResumePhase::Action,
            },
            Phase::RobberVictim {
                origin: RobberOrigin::Knight,
                continuation: ResumePhase::PreRoll,
                victims: [None, Some(seat(1)), Some(seat(2)), None],
            },
            Phase::FreeRoads {
                remaining: 2,
                continuation: ResumePhase::Action,
            },
            Phase::Finished {
                winner: seat(3),
                score: 12,
            },
        ];
        let expected_phases = serde_json::json!([
            {"type":"setupSettlement","actor":3,"round":2},
            {"type":"setupRoad","actor":3,"round":2,"settlement":53},
            {"type":"preRoll"},{"type":"action"},{"type":"discard","obligations":[1,2,3,4]},
            {"type":"robberMove","origin":"seven","continuation":"action"},
            {"type":"robberVictim","origin":"knight","continuation":"preRoll","victims":[null,1,2,null]},
            {"type":"freeRoads","remaining":2,"continuation":"action"},{"type":"finished","winner":3,"score":12}
        ]);
        for (phase, expected_phase) in phases.into_iter().zip(expected_phases.as_array().unwrap()) {
            engine.state.position.phase = phase;
            let bytes = engine.export_snapshot().unwrap();
            assert_eq!(bytes, engine.export_snapshot().unwrap());
            let value = super::super::wire::parse(&bytes).unwrap();
            assert_eq!(value["revision"], "18446744073709551615");
            assert_eq!(
                value["random"],
                serde_json::json!({"seed":"ab".repeat(32),"stream":"0","wordPosition":"295147905179352825855"})
            );
            let p = &value["position"];
            assert_eq!(p.as_object().unwrap().len(), 20);
            assert_eq!(&p["phase"], expected_phase);
            assert_eq!(p["roads"], serde_json::json!(vec![3; 72]));
            assert!(
                p["buildings"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .all(|v| v == &serde_json::json!({"kind":"city","owner":3}))
            );
            assert_eq!(p["roadLengths"], serde_json::json!([12, 13, 14, 15]));
            assert_eq!(p["longestRouteHolder"], 3);
            assert_eq!(p["largestArmyHolder"], 2);
            assert_eq!(
                p["secondSettlements"],
                serde_json::json!([53, null, null, null])
            );
            assert_eq!(p["robber"], 18);
            assert_eq!(
                p["bank"],
                serde_json::json!({"brick":1,"wood":2,"wool":3,"wheat":4,"ore":5})
            );
            assert!(
                p["hands"].as_array().unwrap().iter().all(|v| v
                    == &serde_json::json!({"brick":19,"wood":19,"wool":19,"wheat":19,"ore":19}))
            );
            assert_eq!(p["offer"]["id"], "18446744073709551615");
            assert_eq!(p["offer"]["turn"], "18446744073709551615");
            assert_eq!(
                p["deck"],
                serde_json::json!((0..25).rev().collect::<Vec<_>>())
            );
            assert_eq!(p["drawCursor"], 25);
            assert_eq!(
                p["cardLocations"][0],
                serde_json::json!({"type":"held","owner":3,"acquiredTurn":"18446744073709551615"})
            );
            assert_eq!(
                p["cardLocations"][1],
                serde_json::json!({"type":"playedKnight","owner":2})
            );
            assert_eq!(
                p["cardLocations"][14],
                serde_json::json!({"type":"spent","owner":1})
            );
            assert_eq!(
                p["cardLocations"][24],
                serde_json::json!({"type":"revealedVictory","owner":0})
            );
            assert_eq!(p["playedNonVp"], true);
            assert_eq!(p["finalScores"], serde_json::json!([8, 9, 10, 11]));
            assert_eq!(p["startingPlayer"], 2);
            assert_eq!(p["activePlayer"], 3);
            assert_eq!(p["turn"], "18446744073709551615");
            assert_eq!(p["lastDice"], serde_json::json!([6, 6]));
            assert_eq!(engine.state.random.get_word_pos(), (1_u128 << 68) - 1);
        }
    }
}
