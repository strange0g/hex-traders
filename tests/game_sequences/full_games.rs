//! Complete source-frozen fixed/variable games; all accepted commands replay and resume.
use super::verified_game::VerifiedGame;
use hex_traders_engine::{
    BuildingKind, CardId, Command, CommandEnvelope, CreationEvent, DevelopmentKind, EdgeId, Engine,
    Event, GameConfig, HexId, LegalAction, Phase, PlayerId, PortKind, ProjectedEvent, Receipt,
    Resource, Resources, RuleError, Terrain, VertexId, Viewer,
};
use serde_json::Value;
fn id(v: &Value) -> u8 {
    v.as_u64().unwrap().try_into().unwrap()
}
fn seat(s: u8) -> PlayerId {
    PlayerId::try_from(s).unwrap()
}
fn card(c: u8) -> CardId {
    CardId::try_from(c).unwrap()
}
fn edge(e: u8) -> EdgeId {
    EdgeId::try_from(e).unwrap()
}
fn vertex(v: u8) -> VertexId {
    VertexId::try_from(v).unwrap()
}
fn counts(v: &Value) -> [u8; 5] {
    serde_json::from_value(v.clone()).unwrap()
}
fn resources(v: &Value) -> Resources {
    Resources::try_from(counts(v)).unwrap()
}
fn resource(r: u8) -> Resource {
    [
        Resource::Brick,
        Resource::Wood,
        Resource::Wool,
        Resource::Wheat,
        Resource::Ore,
    ][usize::from(r)]
}
fn terrain(t: &str) -> Terrain {
    match t {
        "desert" => Terrain::Desert,
        "field" => Terrain::Field,
        "pasture" => Terrain::Pasture,
        "hills" => Terrain::Hills,
        "forest" => Terrain::Forest,
        "mountain" => Terrain::Mountain,
        _ => panic!("invalid frozen terrain"),
    }
}
fn act(e: &mut VerifiedGame, actor: u8, command: Command) -> Receipt {
    e.act(actor, command)
}
fn balances(e: &VerifiedGame, hands: &Value, bank: &Value) {
    let public = e.view(Viewer::Spectator).unwrap();
    assert_eq!(public.bank.counts(), &counts(bank));
    for p in &public.players {
        assert_eq!(
            e.view(Viewer::Seat(p.seat))
                .unwrap()
                .own_player
                .unwrap()
                .resources
                .counts(),
            &counts(&hands[p.seat.index()])
        );
        assert_eq!(
            p.resource_count,
            counts(&hands[p.seat.index()]).iter().sum::<u8>()
        );
    }
    for r in 0..5 {
        assert_eq!(
            u16::from(public.bank.counts()[r])
                + public
                    .players
                    .iter()
                    .map(|p| u16::from(counts(&hands[p.seat.index()])[r]))
                    .sum::<u16>(),
            19
        );
    }
}
fn viewers(n: u8) -> Vec<Viewer> {
    std::iter::once(Viewer::Spectator)
        .chain((0..n).map(|s| Viewer::Seat(seat(s))))
        .collect()
}
fn create(c: &Value) -> VerifiedGame {
    let n = id(&c["n"]);
    let seats = (0..n).collect::<Vec<_>>();
    let mode = c["mode"].as_str().unwrap();
    let config = if mode == "fixed" {
        GameConfig::fixed(&seats)
    } else {
        GameConfig::variable(&seats)
    }
    .unwrap();
    let mut seed = [0; 32];
    seed[..4].copy_from_slice(&(c["seed"].as_u64().unwrap() as u32).to_le_bytes());
    let (e, receipt) = Engine::create(config, seed).unwrap();
    let mut e = VerifiedGame::new(e);
    let snapshot: Value = serde_json::from_slice(&e.export_snapshot().unwrap()).unwrap();
    assert_eq!(snapshot["position"]["deck"], c["deck"]);
    assert_eq!(
        snapshot["random"]["wordPosition"],
        c["start_word"].as_u64().unwrap().to_string()
    );
    let v = e.view(Viewer::Spectator).unwrap();
    assert_eq!(v.starting_player, seat(id(&c["starter"])));
    assert_eq!(
        v.board.robber,
        HexId::try_from(id(&c["initial_robber"])).unwrap()
    );
    for (h, t) in c["terrain"].as_array().unwrap().iter().enumerate() {
        assert_eq!(v.board.terrain[h], terrain(t.as_str().unwrap()));
        assert_eq!(
            v.board.numbers[h],
            c["numbers"][h].as_u64().map(|x| x as u8)
        );
    }
    for (actual, expected) in v.board.ports.iter().zip(c["ports"].as_array().unwrap()) {
        assert_eq!(actual.edge, edge(id(&expected["edge"])));
        let kind = match expected["kind"].as_str().unwrap() {
            "generic" => PortKind::Generic,
            "brick" => PortKind::Special(Resource::Brick),
            "wood" => PortKind::Special(Resource::Wood),
            "wool" => PortKind::Special(Resource::Wool),
            "wheat" => PortKind::Special(Resource::Wheat),
            "ore" => PortKind::Special(Resource::Ore),
            _ => panic!("invalid frozen port"),
        };
        assert_eq!(actual.kind, kind);
    }
    let expected_starters = c["starter_rounds"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|round| round.as_array().unwrap().iter())
        .map(|roll| CreationEvent::StarterRolled {
            seat: seat(id(&roll[0])),
            dice: serde_json::from_value(roll[1].clone()).unwrap(),
        })
        .collect::<Vec<_>>();
    let actual_starters = receipt
        .events
        .iter()
        .filter(|event| matches!(event, CreationEvent::StarterRolled { .. }))
        .cloned()
        .collect::<Vec<_>>();
    assert_eq!(actual_starters, expected_starters);
    for viewer in viewers(n) {
        assert_eq!(
            e.project_creation_receipt(&receipt, viewer).unwrap().events,
            receipt.events
        );
    }
    if mode == "variable" {
        assert_eq!(v.turn, 0);
        assert!(v.board.roads.iter().all(Option::is_none));
        assert!(v.board.buildings.iter().all(Option::is_none));
        assert_eq!(v.bank.counts(), &[19; 5]);
        let steps = c["setup_records"].as_array().unwrap();
        for (i, step) in steps.iter().enumerate() {
            let actor = id(&step["actor"]);
            let round = id(&step["round"]);
            let vert = vertex(id(&step["vertex"]));
            let ed = edge(id(&step["edge"]));
            assert_eq!(
                e.view(Viewer::Spectator).unwrap().phase,
                Phase::SetupSettlement {
                    actor: seat(actor),
                    round
                }
            );
            assert!(e.legal_actions(seat(actor)).unwrap().choices.iter().any(|choice|matches!(choice,LegalAction::PlaceSetupSettlement{vertices} if vertices.contains(&vert))));
            let r = act(
                &mut e,
                actor,
                Command::PlaceSetupSettlement { vertex: vert },
            );
            let granted = r
                .events
                .iter()
                .filter_map(|event| match event {
                    Event::StartingResources { seat, resources } => Some((*seat, *resources)),
                    _ => None,
                })
                .collect::<Vec<_>>();
            assert!(granted.is_empty());
            balances(
                &e,
                &step["hands_after_settlement"],
                &step["bank_after_settlement"],
            );
            assert_eq!(
                e.view(Viewer::Spectator).unwrap().phase,
                Phase::SetupRoad {
                    actor: seat(actor),
                    round,
                    settlement: vert
                }
            );
            assert!(e.legal_actions(seat(actor)).unwrap().choices.iter().any(
                |choice| matches!(choice,LegalAction::PlaceSetupRoad{edges} if edges.contains(&ed))
            ));
            let r = act(&mut e, actor, Command::PlaceSetupRoad { edge: ed });
            let grants = r
                .events
                .iter()
                .filter_map(|event| match event {
                    Event::StartingResources { seat, resources } => Some((*seat, *resources)),
                    _ => None,
                })
                .collect::<Vec<_>>();
            if i + 1 == steps.len() {
                assert_eq!(
                    grants,
                    (0..n)
                        .map(|s| (seat(s), resources(&c["initial_hands"][usize::from(s)])))
                        .collect::<Vec<_>>()
                );
            } else {
                assert!(grants.is_empty());
            }
            balances(&e, &step["hands_after_road"], &step["bank_after_road"]);
            if i + 1 == steps.len() {
                assert_eq!(e.view(Viewer::Spectator).unwrap().phase, Phase::PreRoll);
                assert_eq!(e.view(Viewer::Spectator).unwrap().turn, 1);
            }
        }
    }
    balances(&e, &c["initial_hands"], &c["initial_bank"]);
    let v = e.view(Viewer::Spectator).unwrap();
    assert_eq!(
        v.board.roads.iter().filter(|owner| owner.is_some()).count(),
        2 * usize::from(n)
    );
    assert_eq!(
        v.board
            .buildings
            .iter()
            .filter(|building| building.is_some())
            .count(),
        2 * usize::from(n)
    );
    for (s, pairs) in c["presets"].as_array().unwrap().iter().enumerate() {
        for pair in pairs.as_array().unwrap() {
            let building = v.board.buildings[usize::from(id(&pair[0]))].unwrap();
            assert_eq!(building.owner, seat(s as u8));
            assert_eq!(building.kind, BuildingKind::Settlement);
            assert_eq!(
                v.board.roads[usize::from(id(&pair[1]))],
                Some(seat(s as u8))
            );
        }
        let p = &v.players[s];
        assert_eq!(p.public_score, 2);
        assert_eq!(p.longest_route_length, 1);
        assert_eq!(p.roads_remaining, 13);
        assert_eq!(p.settlements_remaining, 3);
        assert_eq!(p.cities_remaining, 4);
    }
    e
}
fn prefix(c: &Value, stop_before_terminal: bool) -> VerifiedGame {
    let n = id(&c["n"]);
    let mut e = create(c);
    let rows = c["records"].as_array().unwrap();
    let mut same_player_multi_hex = false;
    for (i, row) in rows.iter().enumerate() {
        let actor = id(&row["actor"]);
        let turn = row["turn"].as_u64().unwrap();
        let v = e.view(Viewer::Spectator).unwrap();
        assert_eq!(
            (v.turn, v.phase, v.active_player),
            (turn, Phase::PreRoll, seat(actor))
        );
        balances(&e, &row["pre_hands"], &row["pre_bank"]);
        let r = act(&mut e, actor, Command::RollDice);
        let dice: [u8; 2] = serde_json::from_value(row["dice"].clone()).unwrap();
        assert_eq!(
            r.events[0],
            Event::DiceRolled {
                seat: seat(actor),
                dice
            }
        );
        assert!(r.final_scores.is_none());
        if let Some(payout) = row.get("payout") {
            let expected = (0..n)
                .filter_map(|s| {
                    let resources = resources(&payout[usize::from(s)]);
                    (resources != Resources::default()).then_some(Event::Production {
                        seat: seat(s),
                        resources,
                    })
                })
                .collect::<Vec<_>>();
            assert_eq!(
                r.events
                    .iter()
                    .copied()
                    .filter(|event| matches!(event, Event::Production { .. }))
                    .collect::<Vec<_>>(),
                expected
            );
        }
        if c["mode"] == "variable"
            && dice.iter().sum::<u8>() == 5
            && row["payout"][1][2].as_u64().unwrap() > 0
        {
            // Red's V39/H13 and V9/H2 each produce wool: source total two on five.
            assert!(r.events.contains(&Event::Production {
                seat: seat(1),
                resources: Resources::try_from([0, 0, 2, 0, 0]).unwrap(),
            }));
            same_player_multi_hex = true;
        }
        for (s, bundle) in row["discard"].as_object().unwrap() {
            let s: u8 = s.parse().unwrap();
            let resources = resources(bundle);
            assert!(e.legal_actions(seat(s)).unwrap().choices.contains(
                &LegalAction::DiscardResources {
                    quantity: resources.counts().iter().sum()
                }
            ));
            let r = act(&mut e, s, Command::DiscardResources { resources });
            for viewer in viewers(n) {
                assert_eq!(
                    e.project_receipt(&r, viewer).unwrap().events[0],
                    ProjectedEvent::Discarded {
                        seat: seat(s),
                        count: resources.counts().iter().sum(),
                        resources: if viewer == Viewer::Seat(seat(s)) {
                            Some(resources)
                        } else {
                            None
                        }
                    }
                );
            }
        }
        if let Some(destination) = row.get("seven_move") {
            let r = act(
                &mut e,
                actor,
                Command::MoveRobber {
                    hex: HexId::try_from(id(destination)).unwrap(),
                },
            );
            assert!(
                !r.events
                    .iter()
                    .any(|event| matches!(event, Event::RobberTheft { .. }))
            );
        }
        balances(&e, &row["roll_hands"], &row["roll_bank"]);
        assert_eq!(e.view(Viewer::Spectator).unwrap().phase, Phase::Action);
        assert_eq!(
            row["actions"].as_array().unwrap().len(),
            row["action_states"].as_array().unwrap().len()
        );
        for (action, expected) in row["actions"]
            .as_array()
            .unwrap()
            .iter()
            .zip(row["action_states"].as_array().unwrap())
        {
            let kind = action[0].as_str().unwrap();
            if stop_before_terminal && kind == "roadbuilding" {
                return e;
            }
            let command = match kind {
                "buy" => Command::BuyDevelopmentCard,
                "trade" => Command::BankTrade {
                    give_resource: resource(id(&action[1])),
                    receive_resource: resource(id(&action[2])),
                },
                "city" => Command::UpgradeCity {
                    vertex: vertex(id(&action[1])),
                },
                "road" => Command::BuildRoad {
                    edge: edge(id(&action[1])),
                },
                "monopoly" => Command::PlayMonopoly {
                    card: card(id(&action[1])),
                    resource: resource(id(&action[2])),
                },
                "invention" => Command::PlayInvention {
                    card: card(id(&action[1])),
                    resources: resources(&action[2]),
                },
                "roadbuilding" => Command::PlayRoadBuilding {
                    card: card(id(&action[1])),
                },
                "freeRoad" => Command::PlaceFreeRoad {
                    edge: edge(id(&action[1])),
                },
                _ => panic!("invalid frozen action"),
            };
            if ["monopoly", "invention", "roadbuilding"].contains(&kind) {
                let owned = e
                    .view(Viewer::Seat(seat(actor)))
                    .unwrap()
                    .own_player
                    .unwrap();
                let held = owned
                    .development_cards
                    .iter()
                    .find(|c| c.id == card(id(&action[1])))
                    .unwrap();
                let expected_kind = match kind {
                    "monopoly" => DevelopmentKind::Monopoly,
                    "invention" => DevelopmentKind::Invention,
                    _ => DevelopmentKind::RoadBuilding,
                };
                assert_eq!(held.kind, expected_kind);
                assert_eq!(
                    held.acquired_turn,
                    expected["acquired_turn"].as_u64().unwrap()
                );
                assert!(held.acquired_on_earlier_turn);
            }
            let r = act(&mut e, actor, command);
            if let Some(payment) = expected.get("payment") {
                assert!(r.events.contains(&Event::ConstructionPaid {
                    seat: seat(actor),
                    resources: resources(payment)
                }));
            }
            if kind == "buy" {
                assert!(r.events.iter().any(|event|matches!(event,Event::DevelopmentPurchased{card:c,..} if *c==card(id(&action[1])))));
                for viewer in viewers(n) {
                    assert!(e.project_receipt(&r,viewer).unwrap().events.iter().any(|event|matches!(event,ProjectedEvent::DevelopmentPurchased{card:c,..} if *c==if viewer==Viewer::Seat(seat(actor)){Some(card(id(&action[1])))}else{None})));
                }
            }
            if kind == "monopoly" {
                let mut transfers = [0; 4];
                for (s, value) in expected["transfers"].as_array().unwrap().iter().enumerate() {
                    transfers[s] = id(value);
                }
                assert!(r.events.contains(&Event::MonopolyPlayed {
                    seat: seat(actor),
                    card: card(id(&action[1])),
                    resource: resource(id(&action[2])),
                    transfers
                }));
            }
            if kind == "invention" {
                assert!(r.events.contains(&Event::InventionPlayed {
                    seat: seat(actor),
                    card: card(id(&action[1])),
                    resources: resources(&action[2])
                }));
            }
            balances(&e, &expected["hands"], &expected["bank"]);
            assert_eq!(
                e.view(Viewer::Seat(seat(0)))
                    .unwrap()
                    .own_player
                    .unwrap()
                    .true_score,
                id(&expected["score"])
            );
            let own = e.view(Viewer::Seat(seat(0))).unwrap().own_player.unwrap();
            let held = expected["held_ids"]
                .as_array()
                .unwrap()
                .iter()
                .map(id)
                .collect::<Vec<_>>();
            assert_eq!(
                own.development_cards
                    .iter()
                    .map(|c| c.id.index() as u8)
                    .collect::<Vec<_>>(),
                held
            );
            if kind == "roadbuilding" {
                assert!(
                    e.validate(seat(0), &Command::PlaceFreeRoad { edge: edge(47) })
                        .is_ok()
                );
            }
            if kind == "freeRoad" {
                let scores: [u8; 4] = serde_json::from_value(c["final_scores"].clone()).unwrap();
                assert_eq!(r.final_scores, Some(scores));
                assert_eq!(
                    r.events[0],
                    Event::RoadPlaced {
                        seat: seat(0),
                        edge: edge(51)
                    }
                );
                assert_eq!(
                    r.events[1],
                    Event::LongestRouteChanged {
                        previous: None,
                        holder: Some(seat(0))
                    }
                );
                assert_eq!(
                    r.events.last(),
                    Some(&Event::GameFinished {
                        winner: seat(0),
                        score: 10
                    })
                );
            } else {
                assert!(r.final_scores.is_none());
            }
        }
        balances(&e, &row["hands"], &row["bank"]);
        assert_eq!(
            e.view(Viewer::Spectator).unwrap().board.robber,
            HexId::try_from(id(&row["robber"])).unwrap()
        );
        if i + 1 < rows.len() {
            act(&mut e, actor, Command::EndTurn);
        }
    }
    if c["mode"] == "variable" {
        assert!(same_player_multi_hex);
    }
    e
}
fn complete(c: &Value) {
    let n = id(&c["n"]);
    let mut e = prefix(c, false);
    let final_view = e.view(Viewer::Spectator).unwrap();
    let final_revision = final_view.revision;
    assert_eq!(
        final_view.phase,
        Phase::Finished {
            winner: seat(0),
            score: 10
        }
    );
    assert_eq!(final_view.players[0].longest_route_length, 5);
    assert_eq!(final_view.players[0].roads_remaining, 10);
    assert_eq!(final_view.board.roads[47], None);
    let mut revealed = [None; 5];
    for (i, c) in c["revealed_vp"].as_array().unwrap().iter().enumerate() {
        revealed[i] = Some(card(id(c)));
    }
    for viewer in viewers(n) {
        let v = e.view(viewer).unwrap();
        assert_eq!(v.revealed_victory_cards, revealed);
        assert_eq!(
            v.players.iter().map(|p| p.public_score).collect::<Vec<_>>(),
            std::iter::once(10)
                .chain(std::iter::repeat_n(2, usize::from(n - 1)))
                .collect::<Vec<_>>()
        );
    }
    let frozen = e.export_snapshot().unwrap();
    for actor in 0..n {
        assert!(e.legal_actions(seat(actor)).unwrap().choices.is_empty());
        for command in [
            Command::RollDice,
            Command::EndTurn,
            Command::BuyDevelopmentCard,
            Command::PlaceFreeRoad { edge: edge(47) },
        ] {
            assert_eq!(
                e.validate(seat(actor), &command).unwrap_err().code,
                RuleError::GameFinished
            );
            assert_eq!(
                e.execute(CommandEnvelope {
                    api_version: 1,
                    actor: seat(actor),
                    expected_revision: final_revision,
                    command
                })
                .unwrap_err()
                .code,
                RuleError::GameFinished
            );
            assert_eq!(e.export_snapshot().unwrap(), frozen);
        }
    }
    let mut control = prefix(c, true);
    assert_eq!(
        control
            .view(Viewer::Seat(seat(0)))
            .unwrap()
            .own_player
            .unwrap()
            .true_score,
        8
    );
    act(&mut control, 0, Command::EndTurn);
    let next_actor = id(&c["next_actor"]);
    let r = act(&mut control, next_actor, Command::RollDice);
    assert_eq!(
        r.events[0],
        Event::DiceRolled {
            seat: seat(next_actor),
            dice: serde_json::from_value(c["next_dice"].clone()).unwrap()
        }
    );
    let snapshot: Value = serde_json::from_slice(&e.export_snapshot().unwrap()).unwrap();
    assert_eq!(
        snapshot["random"]["wordPosition"],
        c["final_word"].as_u64().unwrap().to_string()
    );
    assert_eq!(final_revision, c["final_revision"].as_u64().unwrap());
    let snapshot: Value = serde_json::from_slice(&control.export_snapshot().unwrap()).unwrap();
    assert_eq!(
        snapshot["random"]["wordPosition"],
        c["next_word"].as_u64().unwrap().to_string()
    );
}
fn case(mode: &str, n: u8) -> Value {
    let cases: Value = serde_json::from_str(include_str!("../support/full-games.json")).unwrap();
    cases
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["mode"] == mode && c["n"] == n)
        .unwrap()
        .clone()
}
#[test]
fn full_game_fixed4_source_history_replays_and_resumes() {
    complete(&case("fixed", 4));
}
#[test]
fn full_game_variable3_source_history_replays_and_resumes() {
    complete(&case("variable", 3));
}
#[test]
fn full_game_variable4_source_history_replays_and_resumes() {
    complete(&case("variable", 4));
}
