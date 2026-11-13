//! Host-private record-at-a-time replay; all seeds and holdings are synthetic fixtures.
use hex_traders_engine::{
    Command, CommandEnvelope, Engine, Event, GameConfig, HexId, PlayerId, ReplayVerifier, Viewer,
};
use serde_json::{Value, json};
fn seat(n: u8) -> PlayerId {
    PlayerId::try_from(n).unwrap()
}
fn game(seed: u32) -> Engine {
    let mut bytes = [0; 32];
    bytes[..4].copy_from_slice(&seed.to_le_bytes());
    Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), bytes)
        .unwrap()
        .0
}
fn act(e: &mut Engine, command: Command) -> hex_traders_engine::Receipt {
    let v = e.view(Viewer::Spectator).unwrap();
    e.execute(CommandEnvelope {
        api_version: 1,
        actor: v.active_player,
        expected_revision: v.revision,
        command,
    })
    .unwrap()
}
fn value(bytes: &[u8]) -> Value {
    serde_json::from_slice(bytes).unwrap()
}
fn bytes(v: &Value) -> Vec<u8> {
    serde_json::to_vec(v).unwrap()
}
fn rejects(v: &mut ReplayVerifier, record: &[u8], index: usize, code: &str) {
    let before = v.engine().export_snapshot().unwrap();
    let err = v.push(record).unwrap_err();
    assert_eq!((err.index(), err.code()), (index, code));
    assert_eq!(v.engine().export_snapshot().unwrap(), before);
}
#[test]
fn replay_header_record_and_next_weighted_random_result_match_literal_public_history() {
    // Existing independent T10 seed63: Blue rolls [1,6], steals wheat at H1,
    // then the next dice are [4,3]. This includes a random draw beyond dice.
    let mut e = game(63);
    let header = e.export_replay_header().unwrap();
    let original = e.export_snapshot().unwrap();
    let h = value(&header);
    assert_eq!(h["replayVersion"], 1);
    assert_eq!(h["rulesVersion"], "hex-traders-base-v1");
    assert_eq!(h["randomAlgorithm"], "chacha12-rand010-v1");
    assert_eq!(h["initialSnapshot"], value(&original));
    assert_eq!(header, e.export_replay_header().unwrap());
    let mut replay = ReplayVerifier::from_header(&header).unwrap();
    assert_eq!(replay.engine().export_snapshot().unwrap(), original);
    for command in [
        Command::RollDice,
        Command::MoveRobber {
            hex: HexId::try_from(1).unwrap(),
        },
        Command::EndTurn,
        Command::RollDice,
    ] {
        let r = act(&mut e, command);
        if r.revision == 1 {
            assert_eq!(
                r.events[0],
                Event::DiceRolled {
                    seat: seat(0),
                    dice: [1, 6]
                }
            );
        }
        if r.revision == 2 {
            assert!(matches!(
                r.events[1],
                Event::RobberTheft {
                    resource: Some(hex_traders_engine::Resource::Wheat),
                    ..
                }
            ));
        }
        if r.revision == 4 {
            assert_eq!(
                r.events[0],
                Event::DiceRolled {
                    seat: seat(1),
                    dice: [4, 3]
                }
            );
        }
        let encoded = r.to_replay_record().unwrap();
        if r.revision == 2 {
            let mut wrong = value(&encoded);
            wrong["events"][1]["resource"] = json!("ore");
            rejects(&mut replay, &bytes(&wrong), 1, "replayMismatch");
        }
        assert_eq!(replay.push(&encoded).unwrap(), r);
        assert_eq!(
            replay.engine().export_snapshot().unwrap(),
            e.export_snapshot().unwrap()
        );
        for who in [
            Viewer::Spectator,
            Viewer::Seat(seat(0)),
            Viewer::Seat(seat(1)),
            Viewer::Seat(seat(2)),
        ] {
            assert_eq!(
                replay.engine().project_receipt(&r, who).unwrap(),
                e.project_receipt(&r, who).unwrap()
            );
        }
    }
    assert_eq!(
        replay.into_engine().export_snapshot().unwrap(),
        e.export_snapshot().unwrap()
    );
}
#[test]
fn replay_reports_first_mismatch_without_advancing_and_allows_corrected_same_record() {
    let mut e = game(8);
    let mut replay = ReplayVerifier::from_header(&e.export_replay_header().unwrap()).unwrap();
    let first = act(&mut e, Command::RollDice).to_replay_record().unwrap();
    let expected = value(&first);
    for (field, replacement) in [
        ("previousRevision", json!("1")),
        ("revision", json!("2")),
        ("actor", json!(1)),
        ("command", json!({"type":"endTurn"})),
        ("events", json!([])),
        ("finalScores", json!([10, 0, 0, 0])),
    ] {
        let mut wrong = expected.clone();
        wrong[field] = replacement;
        let code = if field == "actor" {
            "wrongActor"
        } else if field == "command" {
            "wrongPhase"
        } else {
            "replayMismatch"
        };
        rejects(&mut replay, &bytes(&wrong), 0, code);
    }
    let mut wrong = expected.clone();
    wrong["events"].as_array_mut().unwrap().reverse();
    rejects(&mut replay, &bytes(&wrong), 0, "replayMismatch");
    assert_eq!(replay.push(&first).unwrap().revision, 1);
    rejects(&mut replay, &first, 1, "replayMismatch");
    let second = act(&mut e, Command::EndTurn).to_replay_record().unwrap();
    let mut changed = value(&second);
    changed["command"] = json!({"type":"proposeTrade","recipient":1,
        "give":{"brick":1,"wood":0,"wool":0,"wheat":0,"ore":0},
        "receive":{"brick":0,"wood":1,"wool":0,"wheat":0,"ore":0}});
    rejects(&mut replay, &bytes(&changed), 1, "replayMismatch");
    let mut wrong = value(&second);
    wrong["events"][0]["turn"] = json!("99");
    rejects(&mut replay, &bytes(&wrong), 1, "replayMismatch");
    assert_eq!(replay.push(&second).unwrap().revision, 2);
    assert_eq!(
        replay.engine().export_snapshot().unwrap(),
        e.export_snapshot().unwrap()
    );
}
#[test]
fn replay_versions_strict_keys_decimal_counters_and_per_record_bounds_are_enforced() {
    let mut e = game(8);
    let header = e.export_replay_header().unwrap();
    let h = value(&header);
    for (field, replacement, code) in [
        ("replayVersion", json!(2), "unsupportedSchemaVersion"),
        ("rulesVersion", json!("other"), "incompatibleRulesVersion"),
        (
            "randomAlgorithm",
            json!("other"),
            "incompatibleRandomAlgorithm",
        ),
        ("extra", json!(true), "invalidInput"),
        ("initialSnapshot", json!(null), "invalidSnapshot"),
    ] {
        let mut wrong = h.clone();
        wrong[field] = replacement;
        let err = ReplayVerifier::from_header(&bytes(&wrong)).err().unwrap();
        assert_eq!((err.index(), err.code()), (0, code));
    }
    let record = act(&mut e, Command::RollDice).to_replay_record().unwrap();
    let expected = value(&record);
    let mut replay = ReplayVerifier::from_header(&header).unwrap();
    for (field, replacement, code) in [
        ("apiVersion", json!(2), "unsupportedApiVersion"),
        ("rulesVersion", json!("other"), "incompatibleRulesVersion"),
        (
            "randomAlgorithm",
            json!("other"),
            "incompatibleRandomAlgorithm",
        ),
        ("revision", json!("01"), "invalidInput"),
        ("previousRevision", json!(0), "invalidInput"),
        ("actor", json!(4), "invalidSeat"),
        ("events", json!(null), "invalidInput"),
        ("extra", json!(true), "invalidInput"),
    ] {
        let mut wrong = expected.clone();
        wrong[field] = replacement;
        rejects(&mut replay, &bytes(&wrong), 0, code);
    }
    for field in expected.as_object().unwrap().keys() {
        let mut wrong = expected.clone();
        wrong.as_object_mut().unwrap().remove(field);
        rejects(&mut replay, &bytes(&wrong), 0, "invalidInput");
    }
    for raw in [
        br#"{"events":[],"events":[]}"#.as_slice(),
        br#"{"events":[],"\u0065vents":[]}"#.as_slice(),
        b"null",
        b"[]",
        b"{} {}",
    ] {
        rejects(&mut replay, raw, 0, "invalidInput");
        assert_eq!(
            ReplayVerifier::from_header(raw).err().unwrap().code(),
            "invalidInput"
        );
    }
    let mut padded = record.clone();
    padded.resize(65_536, b' ');
    assert_eq!(replay.push(&padded).unwrap().revision, 1);
    padded.push(b' ');
    rejects(&mut replay, &padded, 1, "invalidInput");
    let mut padded_header = header.clone();
    padded_header.resize(65_536, b' ');
    assert!(ReplayVerifier::from_header(&padded_header).is_ok());
    padded_header.push(b' ');
    assert_eq!(
        ReplayVerifier::from_header(&padded_header)
            .err()
            .unwrap()
            .code(),
        "invalidInput"
    );
    for (depth, code) in [(15, "replayMismatch"), (16, "invalidInput")] {
        let mut nested = json!(0);
        for _ in 0..depth {
            nested = json!([nested]);
        }
        let mut wrong = expected.clone();
        wrong["previousRevision"] = json!("1");
        wrong["revision"] = json!("2");
        wrong["command"] = json!({"type":"endTurn"});
        wrong["events"] = nested;
        rejects(&mut replay, &bytes(&wrong), 1, code);
    }
    let deep = format!("{}0{}", "[".repeat(17), "]".repeat(17));
    rejects(&mut replay, deep.as_bytes(), 1, "invalidInput");
    assert_eq!(
        ReplayVerifier::from_header(deep.as_bytes())
            .err()
            .unwrap()
            .code(),
        "invalidInput"
    );
}

#[test]
fn replay_private_record_encoding_stays_distinct_from_original_player_projection() {
    let mut e = game(8);
    let mut receipt = act(&mut e, Command::RollDice);
    receipt.events = vec![
        Event::DevelopmentPurchased {
            seat: seat(1),
            card: hex_traders_engine::CardId::try_from(18).unwrap(),
            remaining: 24,
        },
        Event::Discarded {
            seat: seat(0),
            resources: hex_traders_engine::Resources::try_from([1, 2, 0, 0, 0]).unwrap(),
        },
        Event::RobberTheft {
            thief: seat(0),
            victim: seat(1),
            resource: Some(hex_traders_engine::Resource::Ore),
        },
    ];
    let private = receipt.to_replay_record().unwrap();
    let r = value(&private);
    assert_eq!(r["events"][0]["card"], 18);
    assert_eq!(
        r["events"][1]["resources"],
        json!({"brick":1,"wood":2,"wool":0,"wheat":0,"ore":0})
    );
    assert_eq!(r["events"][2]["resource"], "ore");
    assert_eq!(r["events"][2]["count"], 1);
    let before = e.export_snapshot().unwrap();
    for viewer in [
        Viewer::Spectator,
        Viewer::Seat(seat(0)),
        Viewer::Seat(seat(1)),
        Viewer::Seat(seat(2)),
    ] {
        let projected = e
            .project_receipt(&receipt, viewer)
            .unwrap()
            .to_json()
            .unwrap();
        let v = value(&projected);
        assert_eq!(
            v["events"][0]["card"],
            if viewer == Viewer::Seat(seat(1)) {
                json!(18)
            } else {
                Value::Null
            }
        );
        assert_eq!(
            v["events"][1]["resources"].is_null(),
            viewer != Viewer::Seat(seat(0))
        );
        assert_eq!(
            v["events"][2]["resource"].is_null(),
            !matches!(viewer,Viewer::Seat(s) if s==seat(0)||s==seat(1))
        );
        assert!(v.get("command").is_none());
        assert!(v.get("finalScores").is_none());
    }
    assert_eq!(e.export_snapshot().unwrap(), before);
    act(&mut e, Command::EndTurn);
    assert_eq!(receipt.to_replay_record().unwrap(), private);
    receipt.events = vec![
        Event::DiceRolled {
            seat: seat(0),
            dice: [1, 1]
        };
        4000
    ];
    assert_eq!(
        receipt.to_replay_record().unwrap_err().code(),
        "invalidInput"
    );
}

#[test]
fn replay_maximum_snapshot_counters_nested_strictness_and_semantic_json_equality() {
    let e = game(8);
    let mut snapshot = value(&e.export_snapshot().unwrap());
    snapshot["random"]["wordPosition"] = json!(((1_u128 << 68) - 1).to_string());
    snapshot["revision"] = json!(u64::MAX.to_string());
    let maximal = Engine::from_snapshot(&bytes(&snapshot)).unwrap();
    let header = maximal.export_replay_header().unwrap();
    assert!(header.len() <= 65_536);
    assert_eq!(
        ReplayVerifier::from_header(&header)
            .unwrap()
            .engine()
            .export_snapshot()
            .unwrap(),
        maximal.export_snapshot().unwrap()
    );
    let mut ordinary = game(8);
    let header = ordinary.export_replay_header().unwrap();
    let mut replay = ReplayVerifier::from_header(&header).unwrap();
    let receipt = act(&mut ordinary, Command::RollDice);
    let record = value(&receipt.to_replay_record().unwrap());
    // Parsed maps may use another field order, while exact array/private facts still matter.
    let pretty = serde_json::to_string_pretty(&record).unwrap();
    assert_eq!(replay.push(pretty.as_bytes()).unwrap(), receipt);
    let next = act(&mut ordinary, Command::EndTurn)
        .to_replay_record()
        .unwrap();
    let raw = String::from_utf8(next.clone()).unwrap().replace(
        "\"type\":\"endTurn\"",
        "\"type\":\"endTurn\",\"\\u0074ype\":\"endTurn\"",
    );
    rejects(&mut replay, raw.as_bytes(), 1, "invalidInput");
    let mut wrong = value(&next);
    wrong["events"][0]["extra"] = json!(true);
    rejects(&mut replay, &bytes(&wrong), 1, "replayMismatch");
    let raw = String::from_utf8(header).unwrap().replace(
        "\"schemaVersion\":1",
        "\"schemaVersion\":1,\"schemaVersion\":1",
    );
    assert_eq!(
        ReplayVerifier::from_header(raw.as_bytes())
            .err()
            .unwrap()
            .code(),
        "invalidInput"
    );
    assert_eq!(replay.push(&next).unwrap().revision, 2);
}

#[test]
fn replay_literal_format_one_record_preserves_private_purchase_and_public_projection() {
    // Independent pinned seed8 trace, already sourced by the native public privacy history:
    // Blue [2,4], Red [1,4], first deck card18. Purchase pays wool/wheat/ore once.
    let mut e = game(8);
    let mut replay = ReplayVerifier::from_header(&e.export_replay_header().unwrap()).unwrap();
    for command in [Command::RollDice, Command::EndTurn, Command::RollDice] {
        let r = act(&mut e, command);
        assert_eq!(replay.push(&r.to_replay_record().unwrap()).unwrap(), r);
    }
    let purchase = act(&mut e, Command::BuyDevelopmentCard);
    let literal = include_bytes!("support/replay-v1.json").trim_ascii_end();
    assert_eq!(purchase.to_replay_record().unwrap(), literal);
    assert_eq!(replay.push(literal).unwrap(), purchase);
    for viewer in [
        Viewer::Spectator,
        Viewer::Seat(seat(0)),
        Viewer::Seat(seat(1)),
        Viewer::Seat(seat(2)),
    ] {
        let projected = value(
            &e.project_receipt(&purchase, viewer)
                .unwrap()
                .to_json()
                .unwrap(),
        );
        assert_eq!(
            projected["events"][1]["card"],
            if viewer == Viewer::Seat(seat(1)) {
                json!(18)
            } else {
                Value::Null
            }
        );
        assert!(projected.get("command").is_none());
        assert!(projected.get("random").is_none());
    }
    assert_eq!(
        replay.engine().export_snapshot().unwrap(),
        e.export_snapshot().unwrap()
    );
}
