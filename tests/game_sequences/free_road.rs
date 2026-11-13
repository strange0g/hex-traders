//! Frozen source-production and independent rand/chacha prefixes for both seat counts.
use super::seat;
use hex_traders_engine::{
    CardId, Command, CommandEnvelope, DevelopmentKind, EdgeId, Engine, Event, GameConfig, HexId,
    LegalAction, Phase, ProjectedEvent, Receipt, ResumePhase, RobberOrigin, RuleError, Viewer,
};
use serde_json::Value;
fn edge(id: u8) -> EdgeId {
    EdgeId::try_from(id).unwrap()
}
fn card(id: u8) -> CardId {
    CardId::try_from(id).unwrap()
}
fn counts(v: &Value) -> [u8; 5] {
    serde_json::from_value(v.clone()).unwrap()
}
fn act(e: &mut Engine, actor: u8, command: Command) -> Receipt {
    let revision = e.view(Viewer::Spectator).unwrap().revision;
    assert!(e.validate(seat(actor), &command).is_ok());
    let r = e
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(actor),
            expected_revision: revision,
            command,
        })
        .unwrap();
    assert_eq!((r.previous_revision, r.revision), (revision, revision + 1));
    let public = e.view(Viewer::Spectator).unwrap();
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
    r
}
fn reject(e: &mut Engine, command: Command, error: RuleError) {
    let before = format!("{:?}", e.view(Viewer::Spectator).unwrap());
    let own = e.view(Viewer::Seat(seat(0))).unwrap().own_player;
    let revision = e.view(Viewer::Spectator).unwrap().revision;
    assert_eq!(e.validate(seat(0), &command).unwrap_err().code, error);
    assert_eq!(
        e.execute(CommandEnvelope {
            api_version: 1,
            actor: seat(0),
            expected_revision: revision,
            command
        })
        .unwrap_err()
        .code,
        error
    );
    assert_eq!(format!("{:?}", e.view(Viewer::Spectator).unwrap()), before);
    assert_eq!(e.view(Viewer::Seat(seat(0))).unwrap().own_player, own);
}
fn balances(e: &Engine, facts: &Value) {
    let public = e.view(Viewer::Spectator).unwrap();
    assert_eq!(public.bank.counts(), &counts(&facts["bank"]));
    for p in &public.players {
        assert_eq!(
            e.view(Viewer::Seat(p.seat))
                .unwrap()
                .own_player
                .unwrap()
                .resources
                .counts(),
            &counts(&facts["hands"][p.seat.index()])
        );
    }
}
#[test]
fn free_roads_public_sequential_pre_roll_and_next_dice() {
    let cases: Value =
        serde_json::from_str(include_str!("../support/free-road-prefixes.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let n = case["n"].as_u64().unwrap() as u8;
        let seed_number = case["seed"].as_u64().unwrap() as u32;
        let mut seed = [0; 32];
        seed[..4].copy_from_slice(&seed_number.to_le_bytes());
        let order = (0..n).collect::<Vec<_>>();
        let (mut e, _) = Engine::create(GameConfig::fixed(&order).unwrap(), seed).unwrap();
        let mut purchase = None;
        for facts in case["turn_facts"].as_array().unwrap() {
            let actor = facts["actor"].as_u64().unwrap() as u8;
            let r = act(&mut e, actor, Command::RollDice);
            let dice: [u8; 2] = serde_json::from_value(facts["dice"].clone()).unwrap();
            assert_eq!(
                r.events[0],
                Event::DiceRolled {
                    seat: seat(actor),
                    dice
                }
            );
            if facts["turn"] == 1 {
                let r = act(&mut e, 0, Command::BuyDevelopmentCard);
                assert_eq!(
                    r.events[1],
                    Event::DevelopmentPurchased {
                        seat: seat(0),
                        card: card(14),
                        remaining: 24
                    }
                );
                purchase = Some(r);
            }
            balances(&e, facts);
            act(&mut e, actor, Command::EndTurn);
        }
        let public = e.view(Viewer::Spectator).unwrap();
        assert_eq!(public.turn, u64::from(n) + 1);
        assert_eq!(public.phase, Phase::PreRoll);
        let final_facts = case["turn_facts"].as_array().unwrap().last().unwrap();
        let r = act(&mut e, 0, Command::PlayRoadBuilding { card: card(14) });
        let pending = Phase::FreeRoads {
            remaining: 2,
            continuation: ResumePhase::PreRoll,
        };
        assert_eq!(
            r.events,
            vec![
                Event::RoadBuildingPlayed {
                    seat: seat(0),
                    card: card(14)
                },
                Event::PhaseChanged { phase: pending }
            ]
        );
        reject(
            &mut e,
            Command::PlaceFreeRoad { edge: edge(44) },
            RuleError::Disconnected,
        );
        let r = act(&mut e, 0, Command::PlaceFreeRoad { edge: edge(45) });
        assert_eq!(
            r.events[0],
            Event::RoadPlaced {
                seat: seat(0),
                edge: edge(45)
            }
        );
        assert_eq!(
            e.view(Viewer::Spectator).unwrap().phase,
            Phase::FreeRoads {
                remaining: 1,
                continuation: ResumePhase::PreRoll
            }
        );
        assert_eq!(
            e.view(Viewer::Spectator).unwrap().players[0].longest_route_length,
            2
        );
        let choices = e.legal_actions(seat(0)).unwrap().choices;
        let [LegalAction::PlaceFreeRoad { edges }] = choices.as_slice() else {
            panic!("only mandatory free placement")
        };
        assert!(edges.contains(&edge(44)));
        for id in 0..72 {
            assert_eq!(
                edges.contains(&edge(id)),
                e.validate(seat(0), &Command::PlaceFreeRoad { edge: edge(id) })
                    .is_ok()
            );
        }
        reject(&mut e, Command::EndTurn, RuleError::WrongPhase);
        reject(
            &mut e,
            Command::BuildRoad { edge: edge(44) },
            RuleError::WrongPhase,
        );
        let r = act(&mut e, 0, Command::PlaceFreeRoad { edge: edge(44) });
        assert_eq!(
            r.events,
            vec![
                Event::RoadPlaced {
                    seat: seat(0),
                    edge: edge(44)
                },
                Event::PhaseChanged {
                    phase: Phase::PreRoll
                }
            ]
        );
        balances(&e, final_facts);
        let public = e.view(Viewer::Spectator).unwrap();
        assert_eq!(public.players[0].longest_route_length, 3);
        assert_eq!(public.longest_route_holder, None);
        assert_eq!(public.players[0].public_score, 2);
        assert_eq!(public.players[0].roads_remaining, 11);
        assert_eq!(
            public.players[0].spent_progress,
            vec![DevelopmentKind::RoadBuilding]
        );
        for viewer in
            std::iter::once(Viewer::Spectator).chain((0..n).map(|id| Viewer::Seat(seat(id))))
        {
            let original = e
                .project_receipt(purchase.as_ref().unwrap(), viewer)
                .unwrap();
            assert_eq!(
                original.events[1],
                ProjectedEvent::DevelopmentPurchased {
                    seat: seat(0),
                    card: if viewer == Viewer::Seat(seat(0)) {
                        Some(card(14))
                    } else {
                        None
                    },
                    remaining: 24
                }
            );
            assert_eq!(
                e.project_receipt(&r, viewer).unwrap().events,
                vec![
                    ProjectedEvent::RoadPlaced {
                        seat: seat(0),
                        edge: edge(44)
                    },
                    ProjectedEvent::PhaseChanged {
                        phase: Phase::PreRoll
                    }
                ]
            );
        }
        reject(
            &mut e,
            Command::PlayRoadBuilding { card: card(14) },
            RuleError::CardNotPlayable,
        );
        let dice: [u8; 2] = serde_json::from_value(case["next_dice"].clone()).unwrap();
        let r = act(&mut e, 0, Command::RollDice);
        assert_eq!(
            r.events[0],
            Event::DiceRolled {
                seat: seat(0),
                dice
            }
        );
        if n == 3 {
            assert_eq!(
                e.view(Viewer::Spectator).unwrap().phase,
                Phase::RobberMove {
                    origin: RobberOrigin::Seven,
                    continuation: ResumePhase::Action
                }
            );
            act(
                &mut e,
                0,
                Command::MoveRobber {
                    hex: HexId::try_from(7).unwrap(),
                },
            );
        }
        assert_eq!(e.view(Viewer::Spectator).unwrap().phase, Phase::Action);
    }
}
