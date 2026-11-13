//! Complete fixed-setup wins, frozen before product execution by independent source arithmetic.
use super::{seat, verified_game::VerifiedGame};
use hex_traders_engine::{
    CardId, Command, CommandEnvelope, EdgeId, Engine, Event, GameConfig, HexId, LegalAction, Phase,
    ProjectedEvent, Receipt, Resource, Resources, ResumePhase, RobberOrigin, RuleError, VertexId,
    Viewer,
};
use serde_json::Value;
fn id(v: &Value) -> u8 {
    v.as_u64().unwrap().try_into().unwrap()
}
fn counts(v: &Value) -> [u8; 5] {
    serde_json::from_value(v.clone()).unwrap()
}
fn resource(v: &Value) -> Resource {
    [
        Resource::Brick,
        Resource::Wood,
        Resource::Wool,
        Resource::Wheat,
        Resource::Ore,
    ][usize::from(id(v))]
}
fn card(id: u8) -> CardId {
    CardId::try_from(id).unwrap()
}
fn edge(id: u8) -> EdgeId {
    EdgeId::try_from(id).unwrap()
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
    }
    for resource in 0..5 {
        assert_eq!(
            u16::from(public.bank.counts()[resource])
                + public
                    .players
                    .iter()
                    .map(|p| u16::from(
                        e.view(Viewer::Seat(p.seat))
                            .unwrap()
                            .own_player
                            .unwrap()
                            .resources
                            .counts()[resource]
                    ))
                    .sum::<u16>(),
            19
        );
    }
}
fn case(mode: &str) -> Value {
    let data: Value =
        serde_json::from_str(include_str!("../support/victory-prefixes.json")).unwrap();
    data.as_array()
        .unwrap()
        .iter()
        .find(|c| c["mode"] == mode)
        .unwrap()
        .clone()
}
fn prefix(case: &Value, skip_winning_purchase: bool) -> (VerifiedGame, Option<Receipt>) {
    let mut seed = [0; 32];
    seed[..4].copy_from_slice(&(case["seed"].as_u64().unwrap() as u32).to_le_bytes());
    let (e, _) = Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), seed).unwrap();
    let mut e = VerifiedGame::new(e);
    assert_eq!(
        e.view(Viewer::Spectator).unwrap().starting_player,
        seat(id(&case["starter"]))
    );
    let rows = case["records"].as_array().unwrap();
    let mut final_receipt = None;
    for row in rows {
        let actor = id(&row["actor"]);
        let turn = row["turn"].as_u64().unwrap();
        let public = e.view(Viewer::Spectator).unwrap();
        assert_eq!(
            (public.turn, public.phase, public.active_player),
            (turn, Phase::PreRoll, seat(actor))
        );
        balances(&e, &row["pre_hands"], &row["pre_bank"]);
        if !row["preroll"].as_array().unwrap().is_empty() {
            let r = act(
                &mut e,
                actor,
                Command::PlayKnight {
                    card: card(id(&row["preroll"][0])),
                },
            );
            assert!(r.final_scores.is_none());
            assert_eq!(
                e.view(Viewer::Spectator).unwrap().phase,
                Phase::RobberMove {
                    origin: RobberOrigin::Knight,
                    continuation: ResumePhase::PreRoll
                }
            );
            assert_eq!(
                e.view(Viewer::Seat(seat(0)))
                    .unwrap()
                    .own_player
                    .unwrap()
                    .true_score,
                id(&row["preroll_score"])
            );
            act(
                &mut e,
                actor,
                Command::MoveRobber {
                    hex: HexId::try_from(id(&row["preroll"][1])).unwrap(),
                },
            );
            assert_eq!(e.view(Viewer::Spectator).unwrap().phase, Phase::PreRoll);
        }
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
            let expected = (0..3)
                .filter_map(|s| {
                    let resources = Resources::try_from(counts(&payout[s])).unwrap();
                    (resources != Resources::default()).then_some(Event::Production {
                        seat: seat(s as u8),
                        resources,
                    })
                })
                .collect::<Vec<_>>();
            let actual = r
                .events
                .iter()
                .copied()
                .filter(|e| matches!(e, Event::Production { .. }))
                .collect::<Vec<_>>();
            assert_eq!(actual, expected);
        }
        for (s, bundle) in row["discard"].as_object().unwrap() {
            let s: u8 = s.parse().unwrap();
            let resources = Resources::try_from(counts(bundle)).unwrap();
            assert!(e.legal_actions(seat(s)).unwrap().choices.contains(
                &LegalAction::DiscardResources {
                    quantity: resources.counts().iter().sum()
                }
            ));
            let r = act(&mut e, s, Command::DiscardResources { resources });
            for viewer in [
                Viewer::Spectator,
                Viewer::Seat(seat(0)),
                Viewer::Seat(seat(1)),
                Viewer::Seat(seat(2)),
            ] {
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
                    .any(|e| matches!(e, Event::RobberTheft { .. }))
            );
        }
        assert_eq!(e.view(Viewer::Spectator).unwrap().phase, Phase::Action);
        balances(&e, &row["roll_hands"], &row["roll_bank"]);
        let actions = row["actions"].as_array().unwrap();
        let states = row["action_states"].as_array().unwrap();
        assert_eq!(actions.len(), states.len());
        for (action, expected) in actions.iter().zip(states) {
            if skip_winning_purchase && expected["score"] == 10 {
                return (e, None);
            }
            let command = match action[0].as_str().unwrap() {
                "buy" => Command::BuyDevelopmentCard,
                "trade" => Command::BankTrade {
                    give_resource: resource(&action[1]),
                    receive_resource: resource(&action[2]),
                },
                "city" => Command::UpgradeCity {
                    vertex: VertexId::try_from(id(&action[1])).unwrap(),
                },
                "road" => Command::BuildRoad {
                    edge: edge(id(&action[1])),
                },
                other => panic!("unexpected frozen command {other}"),
            };
            let r = act(&mut e, actor, command);
            if action[0] == "buy" {
                if expected["score"] != 10 {
                    let own = e
                        .view(Viewer::Seat(seat(actor)))
                        .unwrap()
                        .own_player
                        .unwrap();
                    let bought = own
                        .development_cards
                        .iter()
                        .find(|c| c.id == card(id(&action[1])))
                        .unwrap();
                    assert_eq!(bought.acquired_turn, turn);
                    assert!(!bought.acquired_on_earlier_turn);
                }
                assert!(r.events.iter().any(|e|matches!(e,Event::DevelopmentPurchased{seat:s,card:c,..} if *s==seat(actor)&&*c==card(id(&action[1])))));
                for viewer in [
                    Viewer::Spectator,
                    Viewer::Seat(seat(0)),
                    Viewer::Seat(seat(1)),
                    Viewer::Seat(seat(2)),
                ] {
                    let purchase = e
                        .project_receipt(&r, viewer)
                        .unwrap()
                        .events
                        .into_iter()
                        .find(|e| matches!(e, ProjectedEvent::DevelopmentPurchased { .. }))
                        .unwrap();
                    assert!(
                        matches!(purchase,ProjectedEvent::DevelopmentPurchased{card:c,..} if c==if viewer==Viewer::Seat(seat(actor)){Some(card(id(&action[1])))}else{None})
                    );
                }
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
            if expected["score"] == 10 {
                assert_eq!(r.final_scores, Some([10, 2, 2, 0]));
                final_receipt = Some(r);
            } else {
                assert!(r.final_scores.is_none());
                assert_eq!(e.view(Viewer::Spectator).unwrap().phase, Phase::Action);
            }
        }
        balances(&e, &row["hands"], &row["bank"]);
        assert_eq!(
            e.view(Viewer::Spectator).unwrap().board.robber,
            HexId::try_from(id(&row["robber"])).unwrap()
        );
        if row != rows.last().unwrap() {
            let r = act(&mut e, actor, Command::EndTurn);
            assert!(r.final_scores.is_none());
        }
    }
    (e, final_receipt)
}
fn finished(e: &mut VerifiedGame, r: &Receipt, cards: [Option<CardId>; 5]) {
    assert_eq!(
        e.view(Viewer::Spectator).unwrap().phase,
        Phase::Finished {
            winner: seat(0),
            score: 10
        }
    );
    assert_eq!(
        r.events.last(),
        Some(&Event::GameFinished {
            winner: seat(0),
            score: 10
        })
    );
    assert_eq!(r.final_scores, Some([10, 2, 2, 0]));
    for viewer in [
        Viewer::Spectator,
        Viewer::Seat(seat(0)),
        Viewer::Seat(seat(1)),
        Viewer::Seat(seat(2)),
    ] {
        let v = e.view(viewer).unwrap();
        assert_eq!(
            v.players.iter().map(|p| p.public_score).collect::<Vec<_>>(),
            [10, 2, 2]
        );
        assert_eq!(v.revealed_victory_cards, cards);
        assert_eq!(
            e.project_receipt(r, viewer).unwrap().events.last(),
            Some(&ProjectedEvent::GameFinished {
                winner: seat(0),
                score: 10
            })
        );
    }
    let revision = e.view(Viewer::Spectator).unwrap().revision;
    let before = e.export_snapshot().unwrap();
    for actor in 0..3 {
        assert!(e.legal_actions(seat(actor)).unwrap().choices.is_empty());
        for command in [
            Command::RollDice,
            Command::EndTurn,
            Command::BuyDevelopmentCard,
            Command::PlayKnight { card: card(0) },
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
                    expected_revision: revision,
                    command
                })
                .unwrap_err()
                .code,
                RuleError::GameFinished
            );
            assert_eq!(e.export_snapshot().unwrap(), before);
        }
    }
}
#[test]
fn full_game_fixed3_victory_purchase_public_first_ten_and_new_vp_reveal() {
    let facts = case("purchase");
    let (mut e, r) = prefix(&facts, false);
    let r = r.unwrap();
    assert_eq!(r.events.len(), 4);
    assert_eq!(
        e.view(Viewer::Spectator)
            .unwrap()
            .development_deck_remaining,
        18
    );
    assert_eq!(e.view(Viewer::Spectator).unwrap().turn, 68);
    finished(
        &mut e,
        &r,
        [
            Some(card(21)),
            Some(card(22)),
            Some(card(23)),
            Some(card(24)),
            None,
        ],
    );
    let (mut control, _) = prefix(&facts, true);
    assert_eq!(
        control
            .view(Viewer::Seat(seat(0)))
            .unwrap()
            .own_player
            .unwrap()
            .true_score,
        9
    );
    act(&mut control, 0, Command::EndTurn);
    let r = act(&mut control, 1, Command::RollDice);
    assert_eq!(
        r.events[0],
        Event::DiceRolled {
            seat: seat(1),
            dice: [1, 5]
        }
    );
}
#[test]
fn full_game_fixed3_victory_third_knight_public_stops_before_robber_or_dice() {
    let facts = case("knight");
    let (mut e, r) = prefix(&facts, false);
    assert!(r.is_none());
    act(&mut e, 2, Command::EndTurn);
    assert_eq!(e.view(Viewer::Spectator).unwrap().turn, 71);
    assert_eq!(
        e.view(Viewer::Seat(seat(0)))
            .unwrap()
            .own_player
            .unwrap()
            .true_score,
        8
    );
    let own = e.view(Viewer::Seat(seat(0))).unwrap().own_player.unwrap();
    let held = own
        .development_cards
        .iter()
        .find(|c| c.id == card(0))
        .unwrap();
    assert_eq!(held.acquired_turn, 38);
    assert!(held.acquired_on_earlier_turn);
    let r = act(&mut e, 0, Command::PlayKnight { card: card(0) });
    assert_eq!(
        r.events[0],
        Event::KnightPlayed {
            seat: seat(0),
            card: card(0)
        }
    );
    assert_eq!(
        r.events[1],
        Event::LargestArmyChanged {
            previous: None,
            holder: Some(seat(0))
        }
    );
    assert_eq!(r.events.len(), 4);
    let last = facts["records"].as_array().unwrap().last().unwrap();
    balances(&e, &last["hands"], &last["bank"]);
    finished(
        &mut e,
        &r,
        [
            Some(card(21)),
            Some(card(22)),
            Some(card(23)),
            Some(card(24)),
            None,
        ],
    );
    let (mut control, _) = prefix(&facts, false);
    act(&mut control, 2, Command::EndTurn);
    let r = act(&mut control, 0, Command::RollDice);
    assert_eq!(
        r.events[0],
        Event::DiceRolled {
            seat: seat(0),
            dice: [4, 3]
        }
    );
}
#[test]
fn full_game_fixed3_victory_first_free_road_public_suppresses_a_still_legal_second() {
    let facts = case("freeRoad");
    let (mut e, r) = prefix(&facts, false);
    assert!(r.is_none());
    let before = e.view(Viewer::Spectator).unwrap();
    assert_eq!(
        (
            before.turn,
            before.players[0].longest_route_length,
            before.players[0].public_score
        ),
        (97, 3, 4)
    );
    assert_eq!(
        e.view(Viewer::Seat(seat(0)))
            .unwrap()
            .own_player
            .unwrap()
            .true_score,
        8
    );
    let own = e.view(Viewer::Seat(seat(0))).unwrap().own_player.unwrap();
    let held = own
        .development_cards
        .iter()
        .find(|c| c.id == card(14))
        .unwrap();
    assert_eq!(held.acquired_turn, 58);
    assert!(held.acquired_on_earlier_turn);
    let r = act(&mut e, 0, Command::PlayRoadBuilding { card: card(14) });
    assert_eq!(
        r.events[0],
        Event::RoadBuildingPlayed {
            seat: seat(0),
            card: card(14)
        }
    );
    assert!(
        e.validate(seat(0), &Command::PlaceFreeRoad { edge: edge(47) })
            .is_ok()
    );
    let r = act(&mut e, 0, Command::PlaceFreeRoad { edge: edge(51) });
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
    assert_eq!(r.events.len(), 4);
    let after = e.view(Viewer::Spectator).unwrap();
    assert_eq!(after.bank, before.bank);
    assert_eq!(after.last_dice, before.last_dice);
    assert_eq!(after.board.roads[47], None);
    assert_eq!(after.players[0].roads_remaining, 10);
    assert_eq!(after.players[0].longest_route_length, 5);
    assert_eq!(after.development_deck_remaining, 17);
    let last = facts["records"].as_array().unwrap().last().unwrap();
    balances(&e, &last["hands"], &last["bank"]);
    finished(
        &mut e,
        &r,
        [
            Some(card(20)),
            Some(card(21)),
            Some(card(22)),
            Some(card(23)),
            None,
        ],
    );
    let (mut control, _) = prefix(&facts, false);
    act(&mut control, 0, Command::EndTurn);
    let r = act(&mut control, 1, Command::RollDice);
    assert_eq!(
        r.events[0],
        Event::DiceRolled {
            seat: seat(1),
            dice: [4, 1]
        }
    );
}
