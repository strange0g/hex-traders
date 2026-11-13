//! Mutation families stay malformed while configuration/values shrink; no real private data.
use super::*;
use hex_traders_engine::{Command, CommandEnvelope, LegalAction, decode_command_envelope};
use serde_json::json;

fn bad_snapshots(original: &Value, n: u32) -> Vec<Vec<u8>> {
    let mut cases = Vec::new();
    for family in 0..12 {
        let mut v = original.clone();
        match family {
            0 => {
                let names = ["brick", "wood", "wool", "wheat", "ore"];
                let r = names[n as usize % 5];
                v["position"]["bank"][r] = json!((count(&v["position"]["bank"][r]) + 1) % 20);
            }
            1 => {
                v["position"]["deck"][n as usize % 25] =
                    v["position"]["deck"][((n % 25 + 1) % 25) as usize].clone()
            }
            2 => v["position"]["turn"] = json!(format!("0{n}")),
            3 => v["revision"] = json!(n),
            4 => v["random"]["wordPosition"] = json!(((1_u128 << 68) + u128::from(n)).to_string()),
            5 => v["random"]["stream"] = json!("1"),
            6 => v["position"]["activePlayer"] = json!(4 + n % 100),
            7 => v["position"]["phase"]["privateSyntheticSentinel"] = json!(n),
            8 => {
                v["position"]["cardLocations"][n as usize % 25] =
                    json!({"type":"held","owner":0,"acquiredTurn":"0"})
            }
            9 => v["position"]["roads"]
                .as_array_mut()
                .unwrap()
                .pop()
                .map(|_| ())
                .unwrap(),
            10 => {
                v["position"]["phase"] =
                    json!({"type":"freeRoads","remaining":0,"continuation":"action"})
            }
            _ => v["privateSyntheticSentinel"] = json!(n),
        }
        cases.push(serde_json::to_vec(&v).unwrap());
    }
    let text = serde_json::to_string(original).unwrap();
    cases.push(
        text.replacen(
            "\"schemaVersion\":1",
            "\"schemaVersion\":1,\"schemaVersion\":1",
            1,
        )
        .into_bytes(),
    );
    cases.push(
        text.replacen(
            "\"schemaVersion\":1",
            "\"schemaVersion\":1,\"schema\\u0056ersion\":1",
            1,
        )
        .into_bytes(),
    );
    cases
}
fn bad_envelopes(actor: usize, revision: u64, n: u32) -> Vec<Vec<u8>> {
    let base = json!({"apiVersion":1,"actor":actor,"expectedRevision":revision.to_string(),"command":{"type":"rollDice"}});
    let mut cases = Vec::new();
    for family in 0..11 {
        let mut v = base.clone();
        match family {
            0 => v["apiVersion"] = json!(2 + n % 100),
            1 => v["actor"] = json!(4 + n % 100),
            2 => v["expectedRevision"] = json!(n),
            3 => v["expectedRevision"] = json!(format!("0{n}")),
            4 => v["command"]["type"] = json!(format!("privateSyntheticSentinel{n}")),
            5 => v["command"]["privateSyntheticSentinel"] = json!(n),
            6 => v["command"] = json!([n]),
            7 => v["actor"] = json!(1.5),
            8 => v["command"] = json!({"type":"buildRoad","edge":72+n%100}),
            9 => {
                v["command"] = json!({"type":"discardResources","resources":{"brick":20+n%100,"wood":0,"wool":0,"wheat":0,"ore":0}})
            }
            _ => v
                .as_object_mut()
                .unwrap()
                .remove("actor")
                .map(|_| ())
                .unwrap(),
        }
        cases.push(serde_json::to_vec(&v).unwrap());
    }
    let text = serde_json::to_string(&base).unwrap();
    cases.push(
        text.replacen("\"apiVersion\":1", "\"apiVersion\":1,\"apiVersion\":1", 1)
            .into_bytes(),
    );
    cases.push(
        text.replacen(
            "\"apiVersion\":1",
            "\"apiVersion\":1,\"api\\u0056ersion\":1",
            1,
        )
        .into_bytes(),
    );
    cases.push(format!("{}0{}", "[".repeat(17), "]".repeat(17)).into_bytes());
    let mut padded = serde_json::to_vec(&base).unwrap();
    padded.resize(65_537, b' ');
    cases.push(padded);
    cases
}
proptest! {
    #![proptest_config(ProptestConfig { failure_persistence: Some(Box::new(FileFailurePersistence::Direct("tests/property_sequences.proptest-regressions"))), cases: 64, max_shrink_iters: 2048, rng_seed: RngSeed::Fixed(0xca7a0032), .. ProptestConfig::default() })]
    #[test]
    fn property_bounded_malformed_snapshots_and_wire_preserve_original_continuation(
        four in any::<bool>(), variable in any::<bool>(), seed in any::<u32>(), rotation in 0_u8..4, n in any::<u32>(), noise in prop::collection::vec(any::<u8>(),0..257)
    ) {
        let mut e=create(four,variable,seed,rotation);
        let creation=e.view(Viewer::Spectator).unwrap();
        let first=e.legal_actions(creation.active_player).unwrap().choices.into_iter().find_map(|action|match action {
            LegalAction::RollDice=>Some(Command::RollDice),
            LegalAction::PlaceSetupSettlement{vertices}=>Some(Command::PlaceSetupSettlement{vertex:vertices[0]}),
            _=>None,
        }).unwrap();
        e.execute(CommandEnvelope{api_version:1,actor:creation.active_player,expected_revision:creation.revision,command:first}).unwrap();
        let before=e.export_snapshot().unwrap();
        let snapshot:Value=serde_json::from_slice(&before).unwrap();
        let view=e.view(Viewer::Spectator).unwrap();
        let mut control=Engine::from_snapshot(&before).unwrap();
        let mut malformed=bad_snapshots(&snapshot,n);
        // A deliberate invalid UTF-8 prefix cannot accidentally become a valid snapshot.
        malformed.push([vec![255],noise.clone()].concat());
        let mut padded=before.clone();padded.resize(65_537,b' ');malformed.push(padded);
        for bytes in malformed {
            assert!(bytes.len()<=65_537);
            let error=Engine::from_snapshot(&bytes).err().expect("malformed snapshot accepted");
            assert!(!error.to_string().contains("privateSyntheticSentinel"));
            assert_eq!(error.current_revision(),0);
            assert_eq!(e.export_snapshot().unwrap(),before);
        }
        let mut malformed=bad_envelopes(view.active_player.index(),view.revision,n);
        malformed.push([vec![255],noise].concat());
        for bytes in malformed {
            assert!(bytes.len()<=65_537);
            let decoded=decode_command_envelope(&bytes).unwrap_err();
            assert_eq!(decoded.current_revision(),0);
            let error=e.execute_json(&bytes).unwrap_err();
            assert_eq!(error.current_revision(),view.revision);
            assert_eq!(error.code(),decoded.code());
            assert!(!error.to_string().contains("privateSyntheticSentinel"));
            assert_eq!(e.export_snapshot().unwrap(),before);
        }
        let (actor,command)=view.seat_order.iter().find_map(|&seat| {
            e.legal_actions(seat).unwrap().choices.first().map(|action| (seat,super::commands::supplied(&e,seat,action,n as usize)))
        }).unwrap();
        let envelope=CommandEnvelope{api_version:1,actor,expected_revision:view.revision,command};
        assert_eq!(e.execute(envelope).unwrap(),control.execute(envelope).unwrap());
        assert_eq!(e.export_snapshot().unwrap(),control.export_snapshot().unwrap());
        facts(&e);
    }
}
