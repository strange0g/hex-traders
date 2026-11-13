use hex_traders_engine::{Command, CommandEnvelope, Engine, GameConfig, Viewer};
use serde_json::{Value, json};
fn base() -> Value {
    serde_json::from_slice(include_bytes!("support/snapshot-v1.json")).unwrap()
}
fn bytes(v: &Value) -> Vec<u8> {
    serde_json::to_vec(v).unwrap()
}
fn reject(v: &Value) {
    let e = Engine::from_snapshot(&bytes(v))
        .err()
        .expect("corrupt snapshot accepted");
    assert_eq!(e.code(), "invalidSnapshot");
    assert_eq!(e.current_revision(), 0);
    assert!(!e.to_string().contains("privateSentinel"));
}

#[test]
fn import_complete_fixture_and_resume_next_public_random_command() {
    let snapshot = include_bytes!("support/snapshot-v1.json");
    let mut resumed = Engine::from_snapshot(snapshot).unwrap();
    assert_eq!(resumed.export_snapshot().unwrap(), snapshot);
    let (mut original, _) =
        Engine::create(GameConfig::fixed(&[2, 0, 1]).unwrap(), [0; 32]).unwrap();
    let command = CommandEnvelope {
        api_version: 1,
        actor: original.view(Viewer::Spectator).unwrap().active_player,
        expected_revision: 0,
        command: Command::RollDice,
    };
    assert_eq!(
        original.execute(command).unwrap(),
        resumed.execute(command).unwrap()
    );
    assert_eq!(
        original.export_snapshot().unwrap(),
        resumed.export_snapshot().unwrap()
    );
    let (mut original, _) = Engine::create(
        GameConfig::fixed(&[0, 1, 2]).unwrap(),
        std::array::from_fn(|i| i as u8),
    )
    .unwrap();
    let snapshot = original.export_snapshot().unwrap();
    let mut resumed = Engine::from_snapshot(&snapshot).unwrap();
    assert_eq!(resumed.export_snapshot().unwrap(), snapshot);
    let command = CommandEnvelope {
        api_version: 1,
        actor: original.view(Viewer::Spectator).unwrap().active_player,
        expected_revision: 0,
        command: Command::RollDice,
    };
    assert_eq!(
        original.execute(command).unwrap(),
        resumed.execute(command).unwrap()
    );
    assert_eq!(
        original.export_snapshot().unwrap(),
        resumed.export_snapshot().unwrap()
    );
}

#[test]
fn import_strict_shape_versions_required_nullables_and_unknown_fields() {
    let v = base();
    for pointer in [
        "",
        "/config",
        "/layout",
        "/position",
        "/random",
        "/position/bank",
        "/position/hands/0",
        "/position/phase",
        "/position/cardLocations/0",
        "/position/buildings/31",
        "/layout/ports/0",
        "/layout/ports/0/kind",
    ] {
        let object = v.pointer(pointer).unwrap().as_object().unwrap();
        for key in object.keys() {
            let mut altered = v.clone();
            altered
                .pointer_mut(pointer)
                .unwrap()
                .as_object_mut()
                .unwrap()
                .remove(key);
            reject(&altered);
        }
        let mut altered = v.clone();
        altered
            .pointer_mut(pointer)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert("privateSentinel".into(), json!(1));
        reject(&altered);
        let mut altered = v.clone();
        *altered.pointer_mut(pointer).unwrap() = json!([]);
        reject(&altered);
    }
    for (key, value, code) in [
        ("schemaVersion", json!(2), "unsupportedSchemaVersion"),
        (
            "rulesVersion",
            json!("privateSentinel"),
            "incompatibleRulesVersion",
        ),
        (
            "randomAlgorithm",
            json!("privateSentinel"),
            "incompatibleRandomAlgorithm",
        ),
    ] {
        let mut altered = v.clone();
        altered[key] = value;
        let e = Engine::from_snapshot(&bytes(&altered)).err().unwrap();
        assert_eq!(e.code(), code);
        assert!(!e.to_string().contains("privateSentinel"));
        altered["position"]["turn"] = json!("01");
        reject(&altered);
    }
    for value in [json!(0.0), json!("1"), json!(null), json!(false), json!(-1)] {
        let mut altered = v.clone();
        altered["schemaVersion"] = value;
        reject(&altered);
    }
}

#[test]
fn import_exact_array_lengths_integer_bounds_and_sanitized_native_corruption() {
    let v = base();
    for (pointer, n) in [
        ("/layout/terrain", 19),
        ("/layout/numbers", 19),
        ("/layout/ports", 9),
        ("/layout/framePermutation", 6),
        ("/position/roads", 72),
        ("/position/buildings", 54),
        ("/position/roadLengths", 4),
        ("/position/secondSettlements", 4),
        ("/position/hands", 4),
        ("/position/deck", 25),
        ("/position/cardLocations", 25),
    ] {
        for length in [n - 1, n + 1] {
            let mut altered = v.clone();
            let array = altered
                .pointer_mut(pointer)
                .unwrap()
                .as_array_mut()
                .unwrap();
            array.resize(length, array[0].clone());
            reject(&altered);
        }
    }
    for pointer in ["/position/lastDice", "/position/finalScores"] {
        for length in [1, 3, 5] {
            let mut altered = v.clone();
            *altered.pointer_mut(pointer).unwrap() = json!(vec![1; length]);
            reject(&altered);
        }
    }
    for (pointer, value) in [
        ("/layout/framePermutation/0", json!(0)),
        ("/layout/framePermutation/0", json!(7)),
        ("/position/activePlayer", json!(3)),
        ("/position/startingPlayer", json!(4)),
        ("/position/robber", json!(19)),
        ("/position/secondSettlements/0", json!(54)),
        ("/layout/ports/0/edge", json!(72)),
        ("/position/deck/0", json!(25)),
        ("/position/bank/brick", json!(20)),
        ("/position/bank/brick", json!(1.0)),
        ("/position/bank/brick", json!("1")),
        ("/position/bank/brick", json!(false)),
        ("/position/roadLengths/0", json!(15)),
        ("/position/playedNonVp", json!(true)),
        ("/position/drawCursor", json!(26)),
        ("/position/finalScores", json!([2, 2, 2, 0])),
        (
            "/position/phase",
            json!({"type":"preRoll","privateSentinel":0}),
        ),
        ("/position/phase", json!({"type":"privateSentinel"})),
        ("/position/turn", json!("18446744073709551615")),
    ] {
        let mut altered = v.clone();
        *altered.pointer_mut(pointer).unwrap() = value;
        reject(&altered);
    }
    let mut altered = v.clone();
    altered["position"]["deck"][0] = altered["position"]["deck"][1].clone();
    reject(&altered);
    let mut altered = v.clone();
    altered["layout"]["numbers"]
        .as_array_mut()
        .unwrap()
        .swap(1, 2);
    reject(&altered);
    let mut altered = v.clone();
    altered["layout"]["ports"]
        .as_array_mut()
        .unwrap()
        .swap(0, 1);
    reject(&altered);
    let mut altered = v.clone();
    altered["layout"]["graph"] = json!({"privateSentinel":0});
    reject(&altered);
}

#[test]
fn import_duplicate_keys_size_rng_and_canonical_counter_endpoints() {
    let raw = String::from_utf8(bytes(&base())).unwrap();
    for text in [
        format!("{{\"schemaVersion\":1,{}", &raw[1..]),
        raw.replace("\"seed\":", "\"s\\u0065ed\":\"privateSentinel\",\"seed\":"),
        raw.replace("\"bank\":{", "\"bank\":{\"wood\":0,"),
        format!("{raw} null"),
    ] {
        let e = Engine::from_snapshot(text.as_bytes()).err().unwrap();
        assert_eq!(e.code(), "invalidSnapshot");
        assert!(!e.to_string().contains("privateSentinel"));
    }
    let mut padded = include_bytes!("support/snapshot-v1.json").to_vec();
    padded.resize(65_536, b' ');
    assert!(Engine::from_snapshot(&padded).is_ok());
    padded.push(b' ');
    assert!(Engine::from_snapshot(&padded).is_err());
    for pointer in [
        "/revision",
        "/position/turn",
        "/random/wordPosition",
        "/random/stream",
    ] {
        for value in [
            json!(""),
            json!("01"),
            json!("+1"),
            json!("-1"),
            json!(" 1"),
            json!("١"),
            json!(1),
            json!("18446744073709551616"),
        ] {
            let mut altered = base();
            *altered.pointer_mut(pointer).unwrap() = value;
            if pointer == "/random/wordPosition"
                && altered.pointer(pointer).unwrap() == &json!("18446744073709551616")
            {
                assert!(Engine::from_snapshot(&bytes(&altered)).is_ok());
            } else {
                reject(&altered);
            }
        }
    }
    for position in ["0", "15", "16", "295147905179352825855"] {
        let mut v = base();
        v["random"]["wordPosition"] = json!(position);
        let resumed = Engine::from_snapshot(&bytes(&v)).unwrap();
        let exported: Value = serde_json::from_slice(&resumed.export_snapshot().unwrap()).unwrap();
        assert_eq!(exported["random"]["wordPosition"], position);
    }
    for value in [
        "295147905179352825856",
        "340282366920938463463374607431768211456",
    ] {
        let mut v = base();
        v["random"]["wordPosition"] = json!(value);
        reject(&v);
    }
    for seed in [
        "ab".repeat(31),
        "ab".repeat(33),
        "AB".repeat(32),
        "0g".repeat(32),
        "é".repeat(32),
        "privateSentinel".into(),
    ] {
        let mut v = base();
        v["random"]["seed"] = json!(seed);
        reject(&v);
    }
    let mut v = base();
    v["random"]["stream"] = json!("1");
    reject(&v);
    let mut v = base();
    v["revision"] = json!("18446744073709551615");
    let resumed = Engine::from_snapshot(&bytes(&v)).unwrap();
    assert_eq!(resumed.view(Viewer::Spectator).unwrap().revision, u64::MAX);
}

#[test]
fn import_rng_block_and_period_endpoints_resume_exact_next_dice_and_export() {
    // Independent pinned-rand endpoint goldens; original T27 review is privately preserved.
    for (word, dice, next) in [
        ("0", [3, 2], "2"),
        ("15", [5, 2], "17"),
        ("16", [2, 6], "18"),
        ("295147905179352825855", [4, 3], "1"),
    ] {
        let mut snapshot = base();
        snapshot["random"]["wordPosition"] = json!(word);
        let mut engine = Engine::from_snapshot(&bytes(&snapshot)).unwrap();
        let actor = engine.view(Viewer::Spectator).unwrap().active_player;
        let receipt = engine
            .execute(CommandEnvelope {
                api_version: 1,
                actor,
                expected_revision: 0,
                command: Command::RollDice,
            })
            .unwrap();
        assert_eq!(
            receipt.events[0],
            hex_traders_engine::Event::DiceRolled { seat: actor, dice }
        );
        let exported = engine.export_snapshot().unwrap();
        let value: Value = serde_json::from_slice(&exported).unwrap();
        assert_eq!(value["random"]["wordPosition"], next);
        let mut resumed = Engine::from_snapshot(&exported).unwrap();
        assert_eq!(resumed.export_snapshot().unwrap(), exported);
        if matches!(
            engine.view(Viewer::Spectator).unwrap().phase,
            hex_traders_engine::Phase::RobberMove { .. }
        ) {
            let movement = CommandEnvelope {
                api_version: 1,
                actor,
                expected_revision: 1,
                command: Command::MoveRobber {
                    hex: hex_traders_engine::HexId::try_from(18).unwrap(),
                },
            };
            assert_eq!(
                engine.execute(movement).unwrap(),
                resumed.execute(movement).unwrap()
            );
        }
        assert_eq!(
            engine.view(Viewer::Spectator).unwrap().phase,
            hex_traders_engine::Phase::Action
        );
        {
            let revision = engine.view(Viewer::Spectator).unwrap().revision;
            let end = CommandEnvelope {
                api_version: 1,
                actor,
                expected_revision: revision,
                command: Command::EndTurn,
            };
            assert_eq!(engine.execute(end).unwrap(), resumed.execute(end).unwrap());
            let roll = CommandEnvelope {
                api_version: 1,
                actor: engine.view(Viewer::Spectator).unwrap().active_player,
                expected_revision: revision + 1,
                command: Command::RollDice,
            };
            assert_eq!(
                engine.execute(roll).unwrap(),
                resumed.execute(roll).unwrap()
            );
            assert_eq!(
                engine.export_snapshot().unwrap(),
                resumed.export_snapshot().unwrap()
            );
        }
    }
}
