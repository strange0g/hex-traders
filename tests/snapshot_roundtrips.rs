//! Resume ordinary public histories at every decision and compare the next transition.
use hex_traders_engine::{
    CardId, Command, CommandEnvelope, EdgeId, Engine, Event, GameConfig, HexId, LegalAction, Phase,
    PlayerId, Receipt, ReplayVerifier, Resource, Resources, VertexId, Viewer,
};
use serde_json::Value;
use std::collections::BTreeSet;
fn seat(v: u8) -> PlayerId {
    PlayerId::try_from(v).unwrap()
}
fn id(v: &Value) -> u8 {
    v.as_u64().unwrap().try_into().unwrap()
}
fn bundle(v: &Value) -> Resources {
    Resources::try_from(serde_json::from_value::<[u8; 5]>(v.clone()).unwrap()).unwrap()
}
fn card(v: u8) -> CardId {
    CardId::try_from(v).unwrap()
}
fn phase(p: Phase) -> &'static str {
    match p {
        Phase::SetupSettlement { .. } => "setupSettlement",
        Phase::SetupRoad { .. } => "setupRoad",
        Phase::PreRoll => "preRoll",
        Phase::Action => "action",
        Phase::Discard { .. } => "discard",
        Phase::RobberMove { .. } => "robberMove",
        Phase::RobberVictim { .. } => "robberVictim",
        Phase::FreeRoads { .. } => "freeRoads",
        Phase::Finished { .. } => "finished",
    }
}
fn resume(e: &Engine, seen: &mut BTreeSet<&'static str>) -> Engine {
    let before = e.export_snapshot().unwrap();
    let resumed = Engine::from_snapshot(&before).unwrap_or_else(|err| {
        panic!(
            "{} turn{} revision{}: {err}",
            phase(e.view(Viewer::Spectator).unwrap().phase),
            e.view(Viewer::Spectator).unwrap().turn,
            e.view(Viewer::Spectator).unwrap().revision
        )
    });
    assert_eq!(resumed.export_snapshot().unwrap(), before);
    let view = e.view(Viewer::Spectator).unwrap();
    seen.insert(phase(view.phase));
    for viewer in
        std::iter::once(Viewer::Spectator).chain(view.seat_order.iter().copied().map(Viewer::Seat))
    {
        assert_eq!(
            e.view(viewer).unwrap().to_json().unwrap(),
            resumed.view(viewer).unwrap().to_json().unwrap()
        );
    }
    for s in view.seat_order {
        assert_eq!(
            e.legal_actions(s).unwrap(),
            resumed.legal_actions(s).unwrap()
        );
    }
    assert_eq!(e.export_snapshot().unwrap(), before);
    assert_eq!(resumed.export_snapshot().unwrap(), before);
    resumed
}
fn act(e: &mut Engine, actor: u8, command: Command, seen: &mut BTreeSet<&'static str>) -> Receipt {
    let mut resumed = resume(e, seen);
    let before = e.export_snapshot().unwrap();
    let revision = e.view(Viewer::Spectator).unwrap().revision;
    assert_eq!(
        e.validate(seat(actor), &command),
        resumed.validate(seat(actor), &command)
    );
    assert_eq!(e.export_snapshot().unwrap(), before);
    assert_eq!(resumed.export_snapshot().unwrap(), before);
    let mut replay = ReplayVerifier::from_header(&e.export_replay_header().unwrap()).unwrap();
    let envelope = CommandEnvelope {
        api_version: 1,
        actor: seat(actor),
        expected_revision: revision,
        command,
    };
    let receipt = e.execute(envelope).unwrap();
    let resumed_receipt = resumed.execute(envelope).unwrap();
    assert_eq!(receipt, resumed_receipt);
    let replayed = replay.push(&receipt.to_replay_record().unwrap()).unwrap();
    assert_eq!(receipt, replayed);
    assert_eq!(
        e.export_snapshot().unwrap(),
        replay.engine().export_snapshot().unwrap()
    );
    assert_eq!(
        e.export_snapshot().unwrap(),
        resumed.export_snapshot().unwrap()
    );
    for viewer in std::iter::once(Viewer::Spectator).chain(
        e.view(Viewer::Spectator)
            .unwrap()
            .seat_order
            .into_iter()
            .map(Viewer::Seat),
    ) {
        assert_eq!(
            e.view(viewer).unwrap().to_json().unwrap(),
            replay.engine().view(viewer).unwrap().to_json().unwrap()
        );
        assert_eq!(
            e.project_receipt(&receipt, viewer).unwrap(),
            replay.engine().project_receipt(&replayed, viewer).unwrap()
        );
        assert_eq!(
            e.project_receipt(&receipt, viewer).unwrap(),
            resumed.project_receipt(&resumed_receipt, viewer).unwrap()
        );
    }
    let after = e.export_snapshot().unwrap();
    assert!(e.execute(envelope).is_err());
    assert_eq!(e.export_snapshot().unwrap(), after);
    resume(e, seen);
    receipt
}
fn setup_and_sevens(e: &mut Engine, seen: &mut BTreeSet<&'static str>) {
    loop {
        let view = e.view(Viewer::Spectator).unwrap();
        let actor = view.active_player;
        let command = match view.phase {
            Phase::SetupSettlement { .. } => {
                let choice = e
                    .legal_actions(actor)
                    .unwrap()
                    .choices
                    .into_iter()
                    .find_map(|a| {
                        if let LegalAction::PlaceSetupSettlement { vertices } = a {
                            vertices.first().copied()
                        } else {
                            None
                        }
                    })
                    .unwrap();
                Command::PlaceSetupSettlement { vertex: choice }
            }
            Phase::SetupRoad { .. } => {
                let choice = e
                    .legal_actions(actor)
                    .unwrap()
                    .choices
                    .into_iter()
                    .find_map(|a| {
                        if let LegalAction::PlaceSetupRoad { edges } = a {
                            edges.first().copied()
                        } else {
                            None
                        }
                    })
                    .unwrap();
                Command::PlaceSetupRoad { edge: choice }
            }
            Phase::Discard { obligations } => {
                let at = obligations.iter().position(|&n| n > 0).unwrap();
                let hand = e
                    .view(Viewer::Seat(seat(at as u8)))
                    .unwrap()
                    .own_player
                    .unwrap()
                    .resources;
                let mut remaining = obligations[at];
                let counts = std::array::from_fn(|i| {
                    let take = remaining.min(hand.counts()[i]);
                    remaining -= take;
                    take
                });
                act(
                    e,
                    at as u8,
                    Command::DiscardResources {
                        resources: Resources::try_from(counts).unwrap(),
                    },
                    seen,
                );
                continue;
            }
            Phase::RobberMove { .. } => {
                let hex = (0..19)
                    .map(|i| HexId::try_from(i).unwrap())
                    .find(|&h| {
                        h != view.board.robber
                            && view.board.graph.hex_vertices(h).iter().all(|v| {
                                view.board.buildings[v.index()].is_none_or(|b| b.owner == actor)
                            })
                    })
                    .unwrap();
                Command::MoveRobber { hex }
            }
            _ => break,
        };
        act(e, actor.index() as u8, command, seen);
    }
}

#[test]
fn roundtrip_every_setup_phase_offer_free_road_continuation_and_weighted_theft() {
    let mut seen = BTreeSet::new();
    for n in [3, 4] {
        let order: Vec<_> = (0..n).collect();
        let (mut e, _) = Engine::create(GameConfig::variable(&order).unwrap(), [0; 32]).unwrap();
        setup_and_sevens(&mut e, &mut seen);
        let actor = e.view(Viewer::Spectator).unwrap().active_player.index() as u8;
        act(&mut e, actor, Command::RollDice, &mut seen);
        setup_and_sevens(&mut e, &mut seen);
    }
    let facts: Value =
        serde_json::from_str(include_str!("support/free-road-prefixes.json")).unwrap();
    for c in facts.as_array().unwrap() {
        let n = id(&c["n"]);
        let order: Vec<_> = (0..n).collect();
        let mut seed = [0; 32];
        seed[..4].copy_from_slice(&(c["seed"].as_u64().unwrap() as u32).to_le_bytes());
        let (mut e, _) = Engine::create(GameConfig::fixed(&order).unwrap(), seed).unwrap();
        for row in c["turn_facts"].as_array().unwrap() {
            let actor = id(&row["actor"]);
            let r = act(&mut e, actor, Command::RollDice, &mut seen);
            assert_eq!(
                r.events[0],
                Event::DiceRolled {
                    seat: seat(actor),
                    dice: serde_json::from_value(row["dice"].clone()).unwrap()
                }
            );
            if row["turn"] == 1 {
                act(&mut e, actor, Command::BuyDevelopmentCard, &mut seen);
            }
            act(&mut e, actor, Command::EndTurn, &mut seen);
        }
        let prefix = e.export_snapshot().unwrap();
        for post_roll in [false, true] {
            let mut e = Engine::from_snapshot(&prefix).unwrap();
            if post_roll {
                let receipt = act(&mut e, 0, Command::RollDice, &mut seen);
                assert_eq!(
                    receipt.events[0],
                    Event::DiceRolled {
                        seat: seat(0),
                        dice: serde_json::from_value(c["next_dice"].clone()).unwrap()
                    }
                );
                setup_and_sevens(&mut e, &mut seen);
            }
            act(
                &mut e,
                0,
                Command::PlayRoadBuilding { card: card(14) },
                &mut seen,
            );
            assert!(matches!(
                e.view(Viewer::Spectator).unwrap().phase,
                Phase::FreeRoads { remaining: 2, .. }
            ));
            act(
                &mut e,
                0,
                Command::PlaceFreeRoad {
                    edge: EdgeId::try_from(45).unwrap(),
                },
                &mut seen,
            );
            assert!(matches!(
                e.view(Viewer::Spectator).unwrap().phase,
                Phase::FreeRoads { remaining: 1, .. }
            ));
            act(
                &mut e,
                0,
                Command::PlaceFreeRoad {
                    edge: EdgeId::try_from(44).unwrap(),
                },
                &mut seen,
            );
            assert_eq!(
                e.view(Viewer::Spectator).unwrap().phase,
                if post_roll {
                    Phase::Action
                } else {
                    Phase::PreRoll
                }
            );
        }
    }
    // Source fixed3 seed6 first dice are seven; the offer survives save/resume.
    let mut seed = [0; 32];
    seed[0] = 6;
    let (mut e, _) = Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), seed).unwrap();
    let actor = e.view(Viewer::Spectator).unwrap().active_player.index() as u8;
    act(&mut e, actor, Command::RollDice, &mut seen);
    setup_and_sevens(&mut e, &mut seen);
    let view = e.view(Viewer::Spectator).unwrap();
    if view.phase == Phase::Action {
        let own = e
            .view(Viewer::Seat(seat(actor)))
            .unwrap()
            .own_player
            .unwrap()
            .resources;
        let give_at = own.counts().iter().position(|&n| n > 0).unwrap();
        let mut give = [0; 5];
        give[give_at] = 1;
        let mut receive = [0; 5];
        receive[(give_at + 1) % 5] = 1;
        act(
            &mut e,
            actor,
            Command::ProposeTrade {
                recipient: seat((actor + 1) % 3),
                give: Resources::try_from(give).unwrap(),
                receive: Resources::try_from(receive).unwrap(),
            },
            &mut seen,
        );
        let offer = e.view(Viewer::Spectator).unwrap().offer.unwrap();
        act(
            &mut e,
            actor,
            Command::CancelTrade { offer_id: offer.id },
            &mut seen,
        );
    }
    // Independent T10 samplers: zero, automatic and selected weighted theft.
    for (seed_byte, actor, hex, victim, resource, next_dice) in [
        (9, 2, 18, None, None, [5, 1]),
        (63, 0, 1, None, Some(Resource::Wheat), [4, 3]),
        (35, 1, 10, Some(0), Some(Resource::Ore), [3, 6]),
    ] {
        let mut seed = [0; 32];
        seed[0] = seed_byte;
        let (mut e, _) = Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), seed).unwrap();
        act(&mut e, actor, Command::RollDice, &mut seen);
        let movement = act(
            &mut e,
            actor,
            Command::MoveRobber {
                hex: HexId::try_from(hex).unwrap(),
            },
            &mut seen,
        );
        let receipt = if let Some(victim) = victim {
            assert!(matches!(
                e.view(Viewer::Spectator).unwrap().phase,
                Phase::RobberVictim { .. }
            ));
            act(
                &mut e,
                actor,
                Command::ChooseRobberVictim {
                    victim: seat(victim),
                },
                &mut seen,
            )
        } else {
            movement
        };
        let theft = receipt.events.iter().find_map(|event| {
            if let Event::RobberTheft { resource, .. } = event {
                Some(*resource)
            } else {
                None
            }
        });
        assert_eq!(theft, resource.map(Some));
        act(&mut e, actor, Command::EndTurn, &mut seen);
        let next = e.view(Viewer::Spectator).unwrap().active_player.index() as u8;
        let receipt = act(&mut e, next, Command::RollDice, &mut seen);
        assert_eq!(
            receipt.events[0],
            Event::DiceRolled {
                seat: seat(next),
                dice: next_dice
            }
        );
    }
    assert!(
        seen.contains("setupSettlement")
            && seen.contains("setupRoad")
            && seen.contains("freeRoads")
            && seen.contains("robberVictim")
    );
}

#[test]
fn roundtrip_frozen_complete_card_award_and_finished_histories() {
    let facts: Value = serde_json::from_str(include_str!("support/victory-prefixes.json")).unwrap();
    let mut seen = BTreeSet::new();
    let mut checked_opponent_reveal = false;
    for c in facts.as_array().unwrap() {
        let mut seed = [0; 32];
        seed[..4].copy_from_slice(&(c["seed"].as_u64().unwrap() as u32).to_le_bytes());
        let (mut e, _) = Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), seed).unwrap();
        let rows = c["records"].as_array().unwrap();
        for (index, row) in rows.iter().enumerate() {
            let actor = id(&row["actor"]);
            if !row["preroll"].as_array().unwrap().is_empty() {
                act(
                    &mut e,
                    actor,
                    Command::PlayKnight {
                        card: card(id(&row["preroll"][0])),
                    },
                    &mut seen,
                );
                act(
                    &mut e,
                    actor,
                    Command::MoveRobber {
                        hex: HexId::try_from(id(&row["preroll"][1])).unwrap(),
                    },
                    &mut seen,
                );
            }
            let r = act(&mut e, actor, Command::RollDice, &mut seen);
            assert_eq!(
                r.events[0],
                Event::DiceRolled {
                    seat: seat(actor),
                    dice: serde_json::from_value(row["dice"].clone()).unwrap()
                }
            );
            for (actor, resources) in row["discard"].as_object().unwrap() {
                act(
                    &mut e,
                    actor.parse().unwrap(),
                    Command::DiscardResources {
                        resources: bundle(resources),
                    },
                    &mut seen,
                );
            }
            if let Some(h) = row.get("seven_move") {
                act(
                    &mut e,
                    actor,
                    Command::MoveRobber {
                        hex: HexId::try_from(id(h)).unwrap(),
                    },
                    &mut seen,
                );
            }
            for a in row["actions"].as_array().unwrap() {
                let command = match a[0].as_str().unwrap() {
                    "buy" => Command::BuyDevelopmentCard,
                    "city" => Command::UpgradeCity {
                        vertex: VertexId::try_from(id(&a[1])).unwrap(),
                    },
                    "road" => Command::BuildRoad {
                        edge: EdgeId::try_from(id(&a[1])).unwrap(),
                    },
                    "trade" => Command::BankTrade {
                        give_resource: resources()[usize::from(id(&a[1]))],
                        receive_resource: resources()[usize::from(id(&a[2]))],
                    },
                    _ => unreachable!(),
                };
                act(&mut e, actor, command, &mut seen);
            }
            if index + 1 < rows.len() {
                act(&mut e, actor, Command::EndTurn, &mut seen);
            }
        }
        match c["mode"].as_str().unwrap() {
            "purchase" => {}
            "knight" => {
                act(&mut e, 2, Command::EndTurn, &mut seen);
                act(&mut e, 0, Command::PlayKnight { card: card(0) }, &mut seen);
            }
            "freeRoad" => {
                act(
                    &mut e,
                    0,
                    Command::PlayRoadBuilding { card: card(14) },
                    &mut seen,
                );
                act(
                    &mut e,
                    0,
                    Command::PlaceFreeRoad {
                        edge: EdgeId::try_from(51).unwrap(),
                    },
                    &mut seen,
                );
            }
            _ => unreachable!(),
        }
        assert_eq!(
            e.view(Viewer::Spectator).unwrap().phase,
            Phase::Finished {
                winner: seat(0),
                score: 10
            }
        );
        resume(&e, &mut seen);
        let original = e.export_snapshot().unwrap();
        let finished = snapshot_value(&e);
        for (pointer, bad) in [
            ("/position/phase/winner", serde_json::json!(1)),
            ("/position/phase/score", serde_json::json!(9)),
            ("/position/phase/score", serde_json::json!(11)),
            ("/position/finalScores", serde_json::json!(null)),
            ("/position/finalScores/0", serde_json::json!(9)),
        ] {
            let mut value = finished.clone();
            *value.pointer_mut(pointer).unwrap() = bad;
            invalid_snapshot(&value);
        }
        if let Some(index) =
            (20..25).find(|&i| finished["position"]["cardLocations"][i]["type"] == "undrawn")
        {
            let mut value = finished.clone();
            let start = value["position"]["startingPlayer"].as_u64().unwrap();
            let acquired = (1 + 3 - start) % 3 + 1;
            moved_card(
                &mut value,
                index,
                serde_json::json!({"type":"held","owner":1,"acquiredTurn":acquired.to_string()}),
            );
            value["position"]["finalScores"][1] =
                serde_json::json!(value["position"]["finalScores"][1].as_u64().unwrap() + 1);
            assert!(Engine::from_snapshot(&serde_json::to_vec(&value).unwrap()).is_ok());
            value["position"]["cardLocations"][index] =
                serde_json::json!({"type":"revealedVictory","owner":1});
            invalid_snapshot(&value);
            checked_opponent_reveal = true;
        }
        assert_eq!(e.export_snapshot().unwrap(), original);
    }
    assert!(checked_opponent_reveal);
    for tag in [
        "preRoll",
        "action",
        "discard",
        "robberMove",
        "freeRoads",
        "finished",
    ] {
        assert!(seen.contains(tag), "missing {tag}");
    }
}
fn resources() -> [Resource; 5] {
    [
        Resource::Brick,
        Resource::Wood,
        Resource::Wool,
        Resource::Wheat,
        Resource::Ore,
    ]
}

fn snapshot_value(e: &Engine) -> Value {
    serde_json::from_slice(&e.export_snapshot().unwrap()).unwrap()
}
fn invalid_snapshot(value: &Value) {
    let error = Engine::from_snapshot(&serde_json::to_vec(value).unwrap())
        .err()
        .expect("corrupt native candidate accepted");
    assert_eq!(error.code(), "invalidSnapshot");
}
fn moved_card(value: &mut Value, id: usize, location: Value) {
    let cursor = value["position"]["drawCursor"].as_u64().unwrap() as usize;
    let deck = value["position"]["deck"].as_array_mut().unwrap();
    let at = deck
        .iter()
        .position(|v| v.as_u64() == Some(id as u64))
        .unwrap();
    deck.swap(at, cursor);
    value["position"]["drawCursor"] = serde_json::json!(cursor + 1);
    value["position"]["cardLocations"][id] = location;
}
#[test]
fn roundtrip_native_award_ties_and_corrupt_pending_card_offer_candidates() {
    use serde_json::json;
    let (e, _) = Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), [0; 32]).unwrap();
    let original = e.export_snapshot().unwrap();
    let mut value = snapshot_value(&e);
    for id in [0, 6, 10, 18, 23] {
        value["position"]["roads"][id] = json!(2);
    }
    for id in [61, 65, 71, 70, 69] {
        value["position"]["roads"][id] = json!(0);
    }
    value["position"]["roadLengths"] = json!([5, 1, 5, 0]);
    for holder in [json!(null), json!(0), json!(2)] {
        value["position"]["longestRouteHolder"] = holder;
        let resumed = Engine::from_snapshot(&serde_json::to_vec(&value).unwrap()).unwrap();
        assert_eq!(snapshot_value(&resumed), value);
    }
    value["position"]["longestRouteHolder"] = json!(1);
    invalid_snapshot(&value);
    assert_eq!(e.export_snapshot().unwrap(), original);

    let mut army = snapshot_value(&e);
    for id in 0..6 {
        moved_card(
            &mut army,
            id,
            json!({"type":"playedKnight","owner":if id<3{2}else{0}}),
        );
    }
    invalid_snapshot(&army);
    for holder in [0, 2] {
        army["position"]["largestArmyHolder"] = json!(holder);
        assert!(Engine::from_snapshot(&serde_json::to_vec(&army).unwrap()).is_ok());
    }
    army["position"]["largestArmyHolder"] = json!(1);
    invalid_snapshot(&army);

    let mut held = snapshot_value(&e);
    held["position"]["phase"] = json!({"type":"action"});
    held["position"]["lastDice"] = json!([3, 4]);
    held["revision"] = json!("1");
    moved_card(
        &mut held,
        14,
        json!({"type":"held","owner":2,"acquiredTurn":"1"}),
    );
    assert!(Engine::from_snapshot(&serde_json::to_vec(&held).unwrap()).is_ok());
    for (pointer, bad) in [
        ("/position/cardLocations/14/owner", json!(3)),
        ("/position/cardLocations/14/owner", json!(0)),
        ("/position/cardLocations/14/acquiredTurn", json!("0")),
        ("/position/cardLocations/14/acquiredTurn", json!("2")),
        ("/position/drawCursor", json!(0)),
        ("/position/turn", json!("2")),
        ("/revision", json!("0")),
        ("/position/playedNonVp", json!(true)),
    ] {
        let mut bad_value = held.clone();
        *bad_value.pointer_mut(pointer).unwrap() = bad;
        invalid_snapshot(&bad_value);
    }
    let mut free = held;
    free["position"]["cardLocations"][14] = json!({"type":"spent","owner":2});
    free["position"]["playedNonVp"] = json!(true);
    free["position"]["phase"] = json!({"type":"freeRoads","remaining":2,"continuation":"action"});
    assert!(Engine::from_snapshot(&serde_json::to_vec(&free).unwrap()).is_ok());
    for (pointer, bad) in [
        ("/position/phase/remaining", json!(0)),
        ("/position/phase/remaining", json!(3)),
        ("/position/phase/continuation", json!("preRoll")),
        ("/position/playedNonVp", json!(false)),
        (
            "/position/cardLocations/14",
            json!({"type":"held","owner":2,"acquiredTurn":"1"}),
        ),
    ] {
        let mut bad_value = free.clone();
        *bad_value.pointer_mut(pointer).unwrap() = bad;
        invalid_snapshot(&bad_value);
    }
    let mut offer = snapshot_value(&e);
    offer["position"]["phase"] = json!({"type":"action"});
    offer["position"]["lastDice"] = json!([1, 1]);
    offer["revision"] = json!("1");
    offer["position"]["offer"] = json!({"id":"1","turn":"1","proposer":2,"recipient":0,"give":{"brick":1,"wood":0,"wool":0,"wheat":0,"ore":0},"receive":{"brick":0,"wood":0,"wool":0,"wheat":0,"ore":1}});
    assert!(Engine::from_snapshot(&serde_json::to_vec(&offer).unwrap()).is_ok());
    for (pointer, bad) in [
        ("/position/offer/id", json!("0")),
        ("/position/offer/id", json!("2")),
        ("/position/offer/turn", json!("2")),
        ("/position/offer/recipient", json!(2)),
        ("/position/offer/recipient", json!(3)),
        ("/position/offer/give/brick", json!(2)),
        ("/position/offer/receive/brick", json!(1)),
    ] {
        let mut bad_value = offer.clone();
        *bad_value.pointer_mut(pointer).unwrap() = bad;
        invalid_snapshot(&bad_value);
    }
    let mut seed = [0; 32];
    seed[0] = 12;
    let (variable, _) = Engine::create(GameConfig::variable(&[0, 1, 2]).unwrap(), seed).unwrap();
    let initial = snapshot_value(&variable);
    for bad in [0, 7] {
        let mut value = initial.clone();
        value["layout"]["framePermutation"][0] = json!(bad);
        invalid_snapshot(&value);
    }
    let mut value = initial.clone();
    value["layout"]["numbers"]
        .as_array_mut()
        .unwrap()
        .swap(0, 1);
    invalid_snapshot(&value);
    let mut value = initial.clone();
    value["layout"]["ports"].as_array_mut().unwrap().swap(0, 1);
    invalid_snapshot(&value);
    let mut value = initial;
    value["position"]["phase"]["round"] = json!(3);
    invalid_snapshot(&value);
    assert_eq!(e.export_snapshot().unwrap(), original);
}
