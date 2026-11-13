//! Public native contract: errors, non-mutating queries and revision races.
use hex_traders_engine::{
    CardId, Command, CommandEnvelope, Engine, GameConfig, GameView, LegalAction, PlayerId,
    Resources, RuleError, Viewer,
};

fn seat(id: u8) -> PlayerId {
    PlayerId::try_from(id).unwrap()
}

fn assert_same_view(a: &GameView, b: &GameView) {
    assert_eq!(
        (
            a.revision,
            a.setup,
            &a.seat_order,
            &a.turn_order,
            a.starting_player,
            a.active_player,
            a.turn,
            a.phase,
            a.last_dice
        ),
        (
            b.revision,
            b.setup,
            &b.seat_order,
            &b.turn_order,
            b.starting_player,
            b.active_player,
            b.turn,
            b.phase,
            b.last_dice
        )
    );
    assert_eq!(
        (
            a.longest_route_holder,
            a.largest_army_holder,
            a.bank,
            a.offer,
            a.development_deck_remaining,
            a.revealed_victory_cards,
            &a.players,
            &a.own_player
        ),
        (
            b.longest_route_holder,
            b.largest_army_holder,
            b.bank,
            b.offer,
            b.development_deck_remaining,
            b.revealed_victory_cards,
            &b.players,
            &b.own_player
        )
    );
    assert_eq!(
        (
            a.board.terrain,
            a.board.numbers,
            a.board.ports,
            a.board.roads,
            a.board.buildings,
            a.board.robber
        ),
        (
            b.board.terrain,
            b.board.numbers,
            b.board.ports,
            b.board.roads,
            b.board.buildings,
            b.board.robber
        )
    );
    assert!(std::ptr::eq(a.board.graph, b.board.graph));
}

fn fixed() -> Engine {
    let mut seed = [0; 32];
    seed[0] = 8;
    Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), seed)
        .unwrap()
        .0
}

#[test]
fn release_identity_reaches_native_snapshot_and_replay_contracts() {
    use hex_traders_engine::ReplayVerifier;
    use serde_json::{Value, json};
    let rules = "hex-traders-base-v1";
    let (mut engine, creation) =
        Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), [0; 32]).unwrap();
    assert_eq!(creation.rules_version, rules);
    let viewer = Viewer::Spectator;
    let view = engine.view(viewer).unwrap();
    let header = engine.export_replay_header().unwrap();
    let mut replay = ReplayVerifier::from_header(&header).unwrap();
    let receipt = engine
        .execute(CommandEnvelope {
            api_version: 1,
            actor: view.active_player,
            expected_revision: view.revision,
            command: Command::RollDice,
        })
        .unwrap();
    assert_eq!(receipt.rules_version, rules);
    let record = receipt.to_replay_record().unwrap();
    let snapshot = engine.export_snapshot().unwrap();
    let outputs = [header.clone(), record.clone(), snapshot.clone()];
    for output in outputs {
        let value: Value = serde_json::from_slice(&output).unwrap();
        assert_eq!(value["rulesVersion"], rules);
        assert_eq!(value["randomAlgorithm"], "chacha12-rand010-v1");
    }
    assert_eq!(replay.push(&record).unwrap(), receipt);
    assert_eq!(replay.engine().export_snapshot().unwrap(), snapshot);
    assert_eq!(
        Engine::from_snapshot(&snapshot)
            .unwrap()
            .export_snapshot()
            .unwrap(),
        snapshot
    );
    let before = replay.engine().export_snapshot().unwrap();
    let mut invalid: Value = serde_json::from_slice(&record).unwrap();
    invalid["rulesVersion"] = json!("unknown-base-v1");
    assert_eq!(
        replay
            .push(&serde_json::to_vec(&invalid).unwrap())
            .unwrap_err()
            .code(),
        "incompatibleRulesVersion"
    );
    assert_eq!(replay.engine().export_snapshot().unwrap(), before);
    let mut invalid: Value = serde_json::from_slice(&snapshot).unwrap();
    invalid["rulesVersion"] = json!("unknown-base-v1");
    assert_eq!(
        Engine::from_snapshot(&serde_json::to_vec(&invalid).unwrap())
            .err()
            .unwrap()
            .code(),
        "incompatibleRulesVersion"
    );
}

#[test]
fn native_rejections_have_precedence_preserve_views_and_do_not_change_next_roll() {
    let mut engine = fixed();
    let mut control = fixed();
    let before = (0..3)
        .map(|id| engine.view(Viewer::Seat(seat(id))).unwrap())
        .collect::<Vec<_>>();
    let original = engine.view(Viewer::Spectator).unwrap();
    assert_eq!(original.active_player, seat(0));
    let illegal_trade = Command::ProposeTrade {
        recipient: seat(3),
        give: Resources::try_from([1, 0, 0, 0, 0]).unwrap(),
        receive: Resources::try_from([0, 1, 0, 0, 0]).unwrap(),
    };
    let bad_card = Command::PlayKnight {
        card: CardId::try_from(0).unwrap(),
    };
    let cases = [
        (2, 3, 1, illegal_trade, RuleError::UnsupportedApiVersion(2)),
        (1, 3, 1, illegal_trade, RuleError::InvalidSeat(3)),
        (1, 0, 1, illegal_trade, RuleError::InvalidSeat(3)),
        (1, 1, 1, Command::EndTurn, RuleError::StaleRevision),
        (1, 1, 0, Command::EndTurn, RuleError::WrongActor),
        (1, 0, 0, Command::EndTurn, RuleError::WrongPhase),
        (1, 0, 0, bad_card, RuleError::CardNotPlayable),
    ];
    for (version, actor, revision, command, expected) in cases {
        let error = engine
            .execute(CommandEnvelope {
                api_version: version,
                actor: seat(actor),
                expected_revision: revision,
                command,
            })
            .unwrap_err();
        assert_eq!((error.code, error.current_revision), (expected, 0));
        assert_same_view(&engine.view(Viewer::Spectator).unwrap(), &original);
        for id in 0..3 {
            assert_same_view(
                &engine.view(Viewer::Seat(seat(id))).unwrap(),
                &before[id as usize],
            );
        }
    }
    assert_eq!(
        engine.validate(seat(0), &bad_card).unwrap_err().code,
        RuleError::CardNotPlayable
    );
    let actions = engine.legal_actions(seat(0)).unwrap();
    assert_eq!(actions.revision, 0);
    assert!(actions.choices.contains(&LegalAction::RollDice));
    let envelope = CommandEnvelope {
        api_version: 1,
        actor: seat(0),
        expected_revision: 0,
        command: Command::RollDice,
    };
    let receipt = engine.execute(envelope).unwrap();
    assert_eq!((receipt.previous_revision, receipt.revision), (0, 1));
    assert_eq!(receipt, control.execute(envelope).unwrap());
    let retry = engine.execute(envelope).unwrap_err();
    assert_eq!(
        (retry.code, retry.current_revision),
        (RuleError::StaleRevision, 1)
    );
    assert_same_view(
        &engine.view(Viewer::Spectator).unwrap(),
        &control.view(Viewer::Spectator).unwrap(),
    );
}

#[test]
fn native_setup_candidates_validate_and_execute_at_the_observed_revision() {
    let config = GameConfig::variable(&[0, 1, 2]).unwrap();
    let seed = [9; 32];
    let (engine, _) = Engine::create(config.clone(), seed).unwrap();
    let actor = engine.view(Viewer::Spectator).unwrap().active_player;
    let choices = engine.legal_actions(actor).unwrap();
    assert_eq!(choices.revision, 0);
    let vertices = match choices.choices.as_slice() {
        [LegalAction::PlaceSetupSettlement { vertices }] => vertices,
        other => panic!("unexpected setup choices: {other:?}"),
    };
    assert_eq!(vertices.len(), 54);
    for &vertex in vertices {
        let command = Command::PlaceSetupSettlement { vertex };
        assert_eq!(engine.validate(actor, &command), Ok(()));
        let (mut copy, _) = Engine::create(config.clone(), seed).unwrap();
        let receipt = copy
            .execute(CommandEnvelope {
                api_version: 1,
                actor,
                expected_revision: choices.revision,
                command,
            })
            .unwrap();
        assert_eq!((receipt.previous_revision, receipt.revision), (0, 1));
    }
    assert_eq!(engine.view(Viewer::Spectator).unwrap().revision, 0);
}
#[test]
fn wire_commands_round_trip_and_reject_real_byte_shape_traps() {
    use hex_traders_engine::{decode_command_envelope, encode_command_envelope};
    let commands = [
        r#"{"type":"placeSetupSettlement","vertex":53}"#,
        r#"{"type":"placeSetupRoad","edge":71}"#,
        r#"{"type":"rollDice"}"#,
        r#"{"type":"discardResources","resources":{"brick":0,"wood":19,"wool":0,"wheat":0,"ore":0}}"#,
        r#"{"type":"moveRobber","hex":18}"#,
        r#"{"type":"chooseRobberVictim","victim":3}"#,
        r#"{"type":"placeFreeRoad","edge":71}"#,
        r#"{"type":"buildRoad","edge":0}"#,
        r#"{"type":"buildSettlement","vertex":0}"#,
        r#"{"type":"upgradeCity","vertex":0}"#,
        r#"{"type":"buyDevelopmentCard"}"#,
        r#"{"type":"playKnight","card":13}"#,
        r#"{"type":"playRoadBuilding","card":14}"#,
        r#"{"type":"playMonopoly","card":16,"resource":"wool"}"#,
        r#"{"type":"playInvention","card":18,"resources":{"brick":0,"wood":1,"wool":0,"wheat":1,"ore":0}}"#,
        r#"{"type":"bankTrade","giveResource":"brick","receiveResource":"ore"}"#,
        r#"{"type":"proposeTrade","recipient":3,"give":{"brick":1,"wood":0,"wool":0,"wheat":0,"ore":0},"receive":{"brick":0,"wood":0,"wool":0,"wheat":1,"ore":0}}"#,
        r#"{"type":"acceptTrade","offerId":"18446744073709551615"}"#,
        r#"{"type":"rejectTrade","offerId":"0"}"#,
        r#"{"type":"cancelTrade","offerId":"12"}"#,
        r#"{"type":"endTurn"}"#,
    ];
    let id = |value| hex_traders_engine::CardId::try_from(value).unwrap();
    let edge = |value| hex_traders_engine::EdgeId::try_from(value).unwrap();
    let vertex = |value| hex_traders_engine::VertexId::try_from(value).unwrap();
    let resources = |counts| hex_traders_engine::Resources::try_from(counts).unwrap();
    let expected = [
        Command::PlaceSetupSettlement { vertex: vertex(53) },
        Command::PlaceSetupRoad { edge: edge(71) },
        Command::RollDice,
        Command::DiscardResources {
            resources: resources([0, 19, 0, 0, 0]),
        },
        Command::MoveRobber {
            hex: hex_traders_engine::HexId::try_from(18).unwrap(),
        },
        Command::ChooseRobberVictim {
            victim: PlayerId::try_from(3).unwrap(),
        },
        Command::PlaceFreeRoad { edge: edge(71) },
        Command::BuildRoad { edge: edge(0) },
        Command::BuildSettlement { vertex: vertex(0) },
        Command::UpgradeCity { vertex: vertex(0) },
        Command::BuyDevelopmentCard,
        Command::PlayKnight { card: id(13) },
        Command::PlayRoadBuilding { card: id(14) },
        Command::PlayMonopoly {
            card: id(16),
            resource: hex_traders_engine::Resource::Wool,
        },
        Command::PlayInvention {
            card: id(18),
            resources: resources([0, 1, 0, 1, 0]),
        },
        Command::BankTrade {
            give_resource: hex_traders_engine::Resource::Brick,
            receive_resource: hex_traders_engine::Resource::Ore,
        },
        Command::ProposeTrade {
            recipient: PlayerId::try_from(3).unwrap(),
            give: resources([1, 0, 0, 0, 0]),
            receive: resources([0, 0, 0, 1, 0]),
        },
        Command::AcceptTrade { offer_id: u64::MAX },
        Command::RejectTrade { offer_id: 0 },
        Command::CancelTrade { offer_id: 12 },
        Command::EndTurn,
    ];
    for (command, expected) in commands.into_iter().zip(expected) {
        let bytes = format!(
            r#"{{"apiVersion":1,"actor":0,"expectedRevision":"18446744073709551615","command":{command}}}"#
        );
        let envelope = decode_command_envelope(bytes.as_bytes()).unwrap();
        assert_eq!(envelope.command, expected);
        assert_eq!(envelope.expected_revision, u64::MAX);
        assert_eq!(
            decode_command_envelope(&encode_command_envelope(&envelope).unwrap()),
            Ok(envelope)
        );
    }
    let valid =
        r#"{"apiVersion":1,"actor":0,"expectedRevision":"0","command":{"type":"rollDice"}}"#;
    for bad in [
        valid.replace("\"apiVersion\":1", "\"apiVersion\":1,\"apiVersion\":1"),
        valid.replace("\"rollDice\"", "\"rollDice\",\"privateSentinel\":true"),
        valid.replace("\"rollDice\"", "\"rollDice\",\"dice\":[6,6]"),
        valid.replace("\"0\"", "\"01\""),
        valid.replace("\"0\"", "\"18446744073709551616\""),
        valid.replace("\"actor\":0", "\"actor\":-0"),
        valid.replace("\"actor\":0", "\"actor\":1.0"),
        valid.replace("\"actor\":0", "\"actor\":1e0"),
        "[1,0,\"0\",{\"type\":\"rollDice\"}]".to_owned(),
        format!("{valid} null"),
    ] {
        let error = decode_command_envelope(bad.as_bytes()).unwrap_err();
        assert!(!format!("{error:?} {error}").contains("privateSentinel"));
    }
    let nested = r#"{"apiVersion":1,"actor":0,"expectedRevision":"0","command":{"type":"discardResources","resources":{"brick":0,"wood":1,"w\u006fod":2,"wool":0,"wheat":0,"ore":0}}}"#;
    assert!(decode_command_envelope(nested.as_bytes()).is_err());
}

#[test]
fn wire_ranges_shapes_canonical_counters_and_version_precedence() {
    use hex_traders_engine::decode_command_envelope;
    let envelope = |command: &str| {
        format!(r#"{{"apiVersion":1,"actor":0,"expectedRevision":"0","command":{command}}}"#)
    };
    for (tag, field, limit) in [
        ("buildRoad", "edge", 72),
        ("buildSettlement", "vertex", 54),
        ("moveRobber", "hex", 19),
        ("playKnight", "card", 25),
        ("chooseRobberVictim", "victim", 4),
    ] {
        let valid = envelope(&format!(r#"{{"type":"{tag}","{field}":{}}}"#, limit - 1));
        assert!(decode_command_envelope(valid.as_bytes()).is_ok());
        for value in [
            limit.to_string(),
            "-1".into(),
            "-0".into(),
            "1.0".into(),
            "1e0".into(),
            "true".into(),
            "null".into(),
            "\"0\"".into(),
        ] {
            let bad = envelope(&format!(r#"{{"type":"{tag}","{field}":{value}}}"#));
            assert!(decode_command_envelope(bad.as_bytes()).is_err(), "{bad}");
        }
    }
    let valid = envelope(r#"{"type":"rollDice"}"#);
    for counter in ["", "+1", "-1", " 1", "01", "١", "18446744073709551616"] {
        assert!(
            decode_command_envelope(valid.replace("\"0\"", &format!("\"{counter}\"")).as_bytes())
                .is_err()
        );
    }
    for bad in [
        valid.replace("\"apiVersion\":1", "\"apiVersion\":1,\"extra\":0"),
        envelope("[\"rollDice\"]"),
        envelope(r#"{"type":"discardResources","resources":[0,0,0,0,0]}"#),
        envelope(
            r#"{"type":"discardResources","resources":{"brick":20,"wood":0,"wool":0,"wheat":0,"ore":0}}"#,
        ),
        envelope(
            r#"{"type":"discardResources","resources":{"brick":1.0,"wood":0,"wool":0,"wheat":0,"ore":0}}"#,
        ),
        envelope(
            r#"{"type":"discardResources","resources":{"brick":0,"wood":0,"wool":0,"wheat":0,"ore":0,"extra":0}}"#,
        ),
        envelope(
            r#"{"type":"discardResources","resources":{"brick":0,"wood":0,"wool":0,"wheat":0}}"#,
        ),
        envelope(r#"{"type":"playMonopoly","card":16,"resource":"privateSentinel"}"#),
    ] {
        assert!(decode_command_envelope(bad.as_bytes()).is_err());
    }
    let unsupported = valid.replace("\"apiVersion\":1", "\"apiVersion\":2");
    assert_eq!(
        decode_command_envelope(unsupported.as_bytes())
            .unwrap_err()
            .code(),
        "unsupportedApiVersion"
    );
    assert_eq!(
        decode_command_envelope(
            unsupported
                .replace("\"rollDice\"", "\"rollDice\",\"extra\":0")
                .as_bytes()
        )
        .unwrap_err()
        .code(),
        "invalidInput"
    );
}

#[test]
fn wire_execute_uses_native_commit_and_lost_response_retry() {
    use hex_traders_engine::encode_command_envelope;
    let (mut engine, _) = Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), [0; 32]).unwrap();
    let view = engine.view(Viewer::Spectator).unwrap();
    let envelope = CommandEnvelope {
        api_version: 1,
        actor: view.active_player,
        expected_revision: view.revision,
        command: Command::RollDice,
    };
    let bytes = encode_command_envelope(&envelope).unwrap();
    let receipt = engine.execute_json(&bytes).unwrap();
    assert_eq!(receipt.revision, 1);
    let error = engine.execute_json(&bytes).unwrap_err();
    assert_eq!(error.code(), "staleRevision");
    assert_eq!(error.current_revision(), 1);
    assert_eq!(engine.view(Viewer::Spectator).unwrap().revision, 1);
}
