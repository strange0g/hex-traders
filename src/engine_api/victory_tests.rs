//! Score components and all-command terminal rejection through the native facade.
use super::*;
use crate::game_model::CardLocation;
use crate::{Building, BuildingKind, DevelopmentKind, GameConfig};
fn seat(id: u8) -> PlayerId {
    PlayerId::try_from(id).unwrap()
}
fn card(id: u8) -> CardId {
    CardId::try_from(id).unwrap()
}
fn fixture() -> Engine {
    let (mut e, _) = Engine::create(GameConfig::fixed(&[0, 1, 2, 3]).unwrap(), [0; 32]).unwrap();
    let p = &mut e.state.position;
    p.buildings = [None; 54];
    p.roads = [None; 72];
    for (v, kind) in [
        (0, BuildingKind::City),
        (9, BuildingKind::City),
        (17, BuildingKind::City),
        (31, BuildingKind::Settlement),
        (40, BuildingKind::Settlement),
    ] {
        p.buildings[v] = Some(Building {
            owner: seat(0),
            kind,
        });
    }
    for id in [0, 6, 10, 18, 14, 25, 46, 57, 23] {
        p.roads[id] = Some(seat(0));
    }
    p.road_lengths = [5, 0, 0, 0];
    p.longest_route_holder = Some(seat(0));
    for (draw, id) in [0u8, 1, 2, 20, 21].into_iter().enumerate() {
        let index = p.deck.iter().position(|&c| c == card(id)).unwrap();
        p.deck.swap(draw, index);
        p.card_locations[usize::from(id)] = if id < 3 {
            CardLocation::Held {
                owner: seat(0),
                acquired_turn: 1,
            }
        } else {
            CardLocation::Held {
                owner: seat(if id == 20 { 0 } else { 1 }),
                acquired_turn: 1,
            }
        };
    }
    p.draw_cursor = 5;
    p.largest_army_holder = None;
    p.active_player = seat(3);
    p.phase = Phase::Action;
    p.turn = 4;
    p.hands = [Resources::default(); 4];
    p.bank = Resources::try_from([19; 5]).unwrap();
    e
}
#[test]
fn base_rules_br033_true_public_components_and_revealed_vp_are_counted_once() {
    let mut e = fixture();
    let p = &mut e.state.position;
    assert_eq!(crate::base_rules::public_score(p, seat(0)), 10);
    assert_eq!(crate::base_rules::current_score(p, seat(0)), 11);
    for id in 0..3 {
        p.card_locations[id] = CardLocation::PlayedKnight { owner: seat(0) };
    }
    p.largest_army_holder = Some(seat(0));
    // Three cities and two settlements=8, Route=2, Army=2, one hidden VP=1.
    assert_eq!(crate::base_rules::public_score(p, seat(0)), 12);
    assert_eq!(crate::base_rules::current_score(p, seat(0)), 13);
    assert_eq!(crate::base_rules::public_score(p, seat(1)), 0);
    assert_eq!(crate::base_rules::current_score(p, seat(1)), 1);
    p.longest_route_holder = None;
    assert_eq!(crate::base_rules::current_score(p, seat(0)), 11);
    p.largest_army_holder = None;
    assert_eq!(crate::base_rules::current_score(p, seat(0)), 9);
    p.card_locations[20] = CardLocation::RevealedVictory { owner: seat(0) };
    assert_eq!(crate::base_rules::public_score(p, seat(0)), 9);
    assert_eq!(crate::base_rules::current_score(p, seat(0)), 9);
    // Held Knights add no direct points.
    p.card_locations[0] = CardLocation::Held {
        owner: seat(0),
        acquired_turn: 1,
    };
    assert_eq!(crate::base_rules::current_score(p, seat(0)), 9);
}
fn commands() -> [Command; 21] {
    [
        Command::ProposeTrade {
            recipient: seat(1),
            give: Resources::try_from([1, 0, 0, 0, 0]).unwrap(),
            receive: Resources::try_from([0, 1, 0, 0, 0]).unwrap(),
        },
        Command::AcceptTrade { offer_id: 1 },
        Command::RejectTrade { offer_id: 1 },
        Command::CancelTrade { offer_id: 1 },
        Command::PlaceSetupSettlement {
            vertex: VertexId::try_from(0).unwrap(),
        },
        Command::PlaceSetupRoad {
            edge: EdgeId::try_from(0).unwrap(),
        },
        Command::RollDice,
        Command::DiscardResources {
            resources: Resources::default(),
        },
        Command::MoveRobber {
            hex: HexId::try_from(1).unwrap(),
        },
        Command::ChooseRobberVictim { victim: seat(1) },
        Command::PlaceFreeRoad {
            edge: EdgeId::try_from(1).unwrap(),
        },
        Command::BuildRoad {
            edge: EdgeId::try_from(1).unwrap(),
        },
        Command::BuildSettlement {
            vertex: VertexId::try_from(1).unwrap(),
        },
        Command::UpgradeCity {
            vertex: VertexId::try_from(31).unwrap(),
        },
        Command::BuyDevelopmentCard,
        Command::PlayKnight { card: card(3) },
        Command::PlayRoadBuilding { card: card(14) },
        Command::PlayMonopoly {
            card: card(16),
            resource: Resource::Brick,
        },
        Command::PlayInvention {
            card: card(18),
            resources: Resources::try_from([2, 0, 0, 0, 0]).unwrap(),
        },
        Command::BankTrade {
            give_resource: Resource::Brick,
            receive_resource: Resource::Wood,
        },
        Command::EndTurn,
    ]
}
#[test]
fn privacy_victory_above_ten_turn_start_reveals_winner_only_and_rejects_every_command() {
    let mut e = fixture();
    let rng = e.state.random.clone();
    let r = e
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(3),
            expected_revision: 0,
            command: Command::EndTurn,
        })
        .unwrap();
    assert_eq!(
        r.events,
        vec![
            Event::TurnStarted {
                seat: seat(0),
                turn: 5
            },
            Event::VictoryPointsRevealed {
                winner: seat(0),
                cards: [Some(card(20)), None, None, None, None]
            },
            Event::GameFinished {
                winner: seat(0),
                score: 11
            }
        ]
    );
    assert_eq!(r.final_scores, Some([11, 1, 0, 0]));
    assert_eq!(e.state.random, rng);
    assert_eq!(e.state.position.last_dice, None);
    assert_eq!(
        e.state.position.phase,
        Phase::Finished {
            winner: seat(0),
            score: 11
        }
    );
    let before = e.state.position.clone();
    for viewer in std::iter::once(Viewer::Spectator).chain((0..4).map(|id| Viewer::Seat(seat(id))))
    {
        let v = e.view(viewer).unwrap();
        assert_eq!(v.players[0].public_score, 11);
        assert_eq!(v.players[1].public_score, 0);
        assert_eq!(
            v.revealed_victory_cards,
            [Some(card(20)), None, None, None, None]
        );
        assert_eq!(
            e.project_receipt(&r, viewer).unwrap().events[1],
            ProjectedEvent::VictoryPointsRevealed {
                winner: seat(0),
                cards: v.revealed_victory_cards
            }
        );
        if let Some(own) = v.own_player {
            assert_eq!(
                own.true_score,
                if own.seat == seat(0) {
                    11
                } else if own.seat == seat(1) {
                    1
                } else {
                    0
                }
            );
            if own.seat == seat(1) {
                assert_eq!(own.development_cards[0].kind, DevelopmentKind::VictoryPoint);
            }
        }
    }
    for actor in 0..4 {
        assert!(e.legal_actions(seat(actor)).unwrap().choices.is_empty());
        for command in commands() {
            assert_eq!(
                e.validate(seat(actor), &command).unwrap_err().code,
                RuleError::GameFinished
            );
            assert_eq!(
                e.execute(CommandEnvelope {
                    api_version: 1,
                    actor: seat(actor),
                    expected_revision: 1,
                    command
                })
                .unwrap_err()
                .code,
                RuleError::GameFinished
            );
            assert_eq!(e.state.position, before);
            assert_eq!(e.state.revision, 1);
            assert_eq!(e.state.random, rng);
        }
    }
    assert_eq!(
        e.state.position.card_locations[21],
        CardLocation::Held {
            owner: seat(1),
            acquired_turn: 1
        }
    );
}
