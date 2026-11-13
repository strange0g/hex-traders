use hex_traders_engine::{Command, CommandEnvelope, Engine, GameConfig, Viewer};

#[test]
fn export_preserves_seed_byte_order_and_variable_source_frame_identity() {
    let (engine, _) = Engine::create(
        GameConfig::fixed(&[0, 1, 2]).unwrap(),
        std::array::from_fn(|i| i as u8),
    )
    .unwrap();
    let value: serde_json::Value =
        serde_json::from_slice(&engine.export_snapshot().unwrap()).unwrap();
    assert_eq!(
        value["random"]["seed"],
        "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"
    );
    let mut seed = [0; 32];
    seed[0] = 12;
    let (variable, _) = Engine::create(GameConfig::variable(&[2, 0, 1]).unwrap(), seed).unwrap();
    let value: serde_json::Value =
        serde_json::from_slice(&variable.export_snapshot().unwrap()).unwrap();
    assert_eq!(value["config"]["setup"], "variable");
    // Independently pinned layout: T06 seed12 goldens in creation.rs.
    assert_eq!(
        value["layout"]["framePermutation"],
        serde_json::json!([3, 5, 6, 1, 2, 4])
    );
    assert_eq!(
        value["layout"]["numbers"],
        serde_json::json!([
            6, 2, 5, 3, 9, 10, null, 8, 4, 11, 3, 8, 10, 5, 6, 4, 9, 12, 11
        ])
    );
    assert_eq!(
        value["layout"]["terrain"],
        serde_json::json!([
            "hills", "pasture", "field", "forest", "mountain", "pasture", "desert", "hills",
            "forest", "field", "forest", "mountain", "pasture", "forest", "field", "field",
            "pasture", "hills", "mountain"
        ])
    );
    assert_eq!(
        value["layout"]["ports"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| p["edge"].as_u64().unwrap())
            .collect::<Vec<_>>(),
        vec![0, 3, 5, 22, 53, 71, 68, 54, 18]
    );
}

#[test]
fn export_is_complete_stable_host_private_and_bounded() {
    let (mut engine, _) = Engine::create(GameConfig::fixed(&[2, 0, 1]).unwrap(), [0; 32]).unwrap();
    let before = engine.export_snapshot().unwrap();
    assert_eq!(before, include_bytes!("support/snapshot-v1.json"));
    assert_eq!(before, engine.export_snapshot().unwrap());
    assert!(before.len() <= 65_536);
    let value: serde_json::Value = serde_json::from_slice(&before).unwrap();
    assert_eq!(value["schemaVersion"], 1);
    assert_eq!(value["rulesVersion"], "hex-traders-base-v1");
    assert_eq!(value["randomAlgorithm"], "chacha12-rand010-v1");
    assert_eq!(
        value["config"],
        serde_json::json!({"seatOrder":[2,0,1],"setup":"fixed"})
    );
    assert_eq!(value["revision"], "0");
    assert_eq!(value["random"]["seed"], "00".repeat(32));
    assert_eq!(value["random"]["stream"], "0");
    assert_eq!(value["layout"]["terrain"].as_array().unwrap().len(), 19);
    assert_eq!(value["layout"]["numbers"].as_array().unwrap().len(), 19);
    assert_eq!(value["layout"]["ports"].as_array().unwrap().len(), 9);
    assert_eq!(
        value["layout"]["framePermutation"],
        serde_json::json!([1, 2, 3, 4, 5, 6])
    );
    assert!(value["layout"].get("graph").is_none());
    assert_eq!(value["position"]["roads"].as_array().unwrap().len(), 72);
    assert_eq!(value["position"]["buildings"].as_array().unwrap().len(), 54);
    assert_eq!(value["position"]["hands"].as_array().unwrap().len(), 4);
    assert_eq!(value["position"]["deck"].as_array().unwrap().len(), 25);
    assert_eq!(
        value["position"]["cardLocations"].as_array().unwrap().len(),
        25
    );
    assert_eq!(value["position"]["offer"], serde_json::Value::Null);
    assert_eq!(value["position"]["finalScores"], serde_json::Value::Null);
    assert!(value["position"].get("history").is_none());
    let public = serde_json::from_slice::<serde_json::Value>(
        &engine.view(Viewer::Spectator).unwrap().to_json().unwrap(),
    )
    .unwrap();
    assert!(public.get("random").is_none());
    assert!(public.get("position").is_none());
    let actor = engine.view(Viewer::Spectator).unwrap().active_player;
    let rejected = CommandEnvelope {
        api_version: 1,
        actor,
        expected_revision: 1,
        command: Command::RollDice,
    };
    assert!(engine.execute(rejected).is_err());
    assert_eq!(before, engine.export_snapshot().unwrap());
    let receipt = engine
        .execute(CommandEnvelope {
            expected_revision: 0,
            ..rejected
        })
        .unwrap();
    assert_eq!(receipt.revision, 1);
    assert_ne!(before, engine.export_snapshot().unwrap());
}
