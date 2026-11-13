//! Complete original-receipt allowlists and paired private-state checks.
use super::*;
use crate::game_model::CardLocation;
use crate::{GameConfig, LegalAction, RuleError};
fn seat(id: u8) -> PlayerId {
    PlayerId::try_from(id).unwrap()
}
fn card(id: u8) -> CardId {
    CardId::try_from(id).unwrap()
}
fn res(v: [u8; 5]) -> Resources {
    Resources::try_from(v).unwrap()
}
fn game() -> Engine {
    Engine::create(GameConfig::fixed(&[0, 1, 2, 3]).unwrap(), [8; 32])
        .unwrap()
        .0
}
fn sample() -> Receipt {
    let offer = TradeOffer {
        id: 2,
        turn: 3,
        proposer: seat(0),
        recipient: seat(1),
        give: res([1, 0, 0, 0, 0]),
        receive: res([0, 1, 0, 0, 0]),
    };
    Receipt {
        api_version: 1,
        rules_version: "hex-traders-base-v1",
        random_algorithm: "chacha12-rand010-v1",
        actor: seat(0),
        command: Command::DiscardResources {
            resources: res([0, 0, 2, 0, 1]),
        },
        previous_revision: 6,
        revision: 7,
        final_scores: Some([10, 9, 7, 6]),
        events: vec![
            Event::TradeOfferCreated { offer },
            Event::TradeCompleted { offer },
            Event::TradeOfferClosed { offer_id: 2 },
            Event::RoadBuildingPlayed {
                seat: seat(0),
                card: card(14),
            },
            Event::KnightPlayed {
                seat: seat(1),
                card: card(1),
            },
            Event::LargestArmyChanged {
                previous: None,
                holder: Some(seat(1)),
            },
            Event::DevelopmentPurchased {
                seat: seat(1),
                card: card(24),
                remaining: 9,
            },
            Event::MonopolyPlayed {
                seat: seat(0),
                card: card(16),
                resource: Resource::Ore,
                transfers: [0, 0, 1, 0],
            },
            Event::InventionPlayed {
                seat: seat(2),
                card: card(18),
                resources: res([0, 0, 2, 0, 0]),
            },
            Event::VictoryPointsRevealed {
                winner: seat(0),
                cards: [Some(card(20)), None, None, None, None],
            },
            Event::BankTraded {
                seat: seat(0),
                give_resource: Resource::Brick,
                receive_resource: Resource::Wood,
                rate: 4,
            },
            Event::CityUpgraded {
                seat: seat(0),
                vertex: VertexId::try_from(31).unwrap(),
            },
            Event::ConstructionPaid {
                seat: seat(0),
                resources: res([1, 1, 0, 0, 0]),
            },
            Event::LongestRouteChanged {
                previous: None,
                holder: Some(seat(0)),
            },
            Event::RobberMoved {
                seat: seat(0),
                hex: HexId::try_from(1).unwrap(),
            },
            Event::RobberTheft {
                thief: seat(0),
                victim: seat(2),
                resource: Some(Resource::Ore),
            },
            Event::Discarded {
                seat: seat(1),
                resources: res([0, 0, 2, 0, 1]),
            },
            Event::GameFinished {
                winner: seat(0),
                score: 10,
            },
            Event::DiceRolled {
                seat: seat(0),
                dice: [2, 3],
            },
            Event::Production {
                seat: seat(0),
                resources: res([1, 0, 0, 0, 0]),
            },
            Event::RoadPlaced {
                seat: seat(0),
                edge: EdgeId::try_from(1).unwrap(),
            },
            Event::StartingResources {
                seat: seat(0),
                resources: res([1, 0, 1, 0, 0]),
            },
            Event::TurnStarted {
                seat: seat(0),
                turn: 3,
            },
            Event::SettlementPlaced {
                seat: seat(0),
                vertex: VertexId::try_from(0).unwrap(),
            },
            Event::PhaseChanged {
                phase: Phase::Action,
            },
        ],
    }
}
#[test]
fn wire_projected_events_keep_every_variant_and_exact_private_allowlists() {
    let e = game();
    let receipt = sample();
    let expected = [
        ("tradeOfferCreated", vec!["offer", "type"]),
        ("tradeCompleted", vec!["offer", "type"]),
        ("tradeOfferClosed", vec!["offerId", "type"]),
        ("roadBuildingPlayed", vec!["card", "seat", "type"]),
        ("knightPlayed", vec!["card", "seat", "type"]),
        ("largestArmyChanged", vec!["holder", "previous", "type"]),
        (
            "developmentPurchased",
            vec!["card", "remaining", "seat", "type"],
        ),
        (
            "monopolyPlayed",
            vec!["card", "resource", "seat", "transfers", "type"],
        ),
        ("inventionPlayed", vec!["card", "resources", "seat", "type"]),
        ("victoryPointsRevealed", vec!["cards", "type", "winner"]),
        (
            "bankTraded",
            vec!["giveResource", "rate", "receiveResource", "seat", "type"],
        ),
        ("cityUpgraded", vec!["seat", "type", "vertex"]),
        ("constructionPaid", vec!["resources", "seat", "type"]),
        ("longestRouteChanged", vec!["holder", "previous", "type"]),
        ("robberMoved", vec!["hex", "seat", "type"]),
        (
            "robberTheft",
            vec!["count", "resource", "thief", "type", "victim"],
        ),
        ("discarded", vec!["count", "resources", "seat", "type"]),
        ("gameFinished", vec!["score", "type", "winner"]),
        ("diceRolled", vec!["dice", "seat", "type"]),
        ("production", vec!["resources", "seat", "type"]),
        ("roadPlaced", vec!["edge", "seat", "type"]),
        ("startingResources", vec!["resources", "seat", "type"]),
        ("turnStarted", vec!["seat", "turn", "type"]),
        ("settlementPlaced", vec!["seat", "type", "vertex"]),
        ("phaseChanged", vec!["phase", "type"]),
    ];
    for viewer in [
        Viewer::Spectator,
        Viewer::Seat(seat(0)),
        Viewer::Seat(seat(1)),
        Viewer::Seat(seat(2)),
        Viewer::Seat(seat(3)),
    ] {
        let projected = e.project_receipt(&receipt, viewer).unwrap();
        let bytes = projected.to_json().unwrap();
        let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(value["revision"], "7");
        let events = value["events"].as_array().unwrap();
        assert_eq!(events.len(), 25);
        for (event, (tag, keys)) in events.iter().zip(&expected) {
            assert_eq!(event["type"], *tag);
            assert_eq!(
                event
                    .as_object()
                    .unwrap()
                    .keys()
                    .map(String::as_str)
                    .collect::<Vec<_>>(),
                *keys
            );
        }
        assert_eq!(events[6]["card"].is_null(), viewer != Viewer::Seat(seat(1)));
        assert_eq!(
            events[15]["resource"].is_null(),
            ![Viewer::Seat(seat(0)), Viewer::Seat(seat(2))].contains(&viewer)
        );
        assert_eq!(
            events[16]["resources"].is_null(),
            viewer != Viewer::Seat(seat(1))
        );
        assert_eq!(events[20]["edge"], 1);
        assert_eq!(events[23]["vertex"], 0);
        assert_eq!(projected.to_json().unwrap(), bytes);
    }
}
fn expected(viewer: Viewer) -> Vec<ProjectedEvent> {
    let offer = TradeOffer {
        id: 2,
        turn: 3,
        proposer: seat(0),
        recipient: seat(1),
        give: res([1, 0, 0, 0, 0]),
        receive: res([0, 1, 0, 0, 0]),
    };
    vec![
        ProjectedEvent::TradeOfferCreated { offer },
        ProjectedEvent::TradeCompleted { offer },
        ProjectedEvent::TradeOfferClosed { offer_id: 2 },
        ProjectedEvent::RoadBuildingPlayed {
            seat: seat(0),
            card: card(14),
        },
        ProjectedEvent::KnightPlayed {
            seat: seat(1),
            card: card(1),
        },
        ProjectedEvent::LargestArmyChanged {
            previous: None,
            holder: Some(seat(1)),
        },
        ProjectedEvent::DevelopmentPurchased {
            seat: seat(1),
            card: if viewer == Viewer::Seat(seat(1)) {
                Some(card(24))
            } else {
                None
            },
            remaining: 9,
        },
        ProjectedEvent::MonopolyPlayed {
            seat: seat(0),
            card: card(16),
            resource: Resource::Ore,
            transfers: [0, 0, 1, 0],
        },
        ProjectedEvent::InventionPlayed {
            seat: seat(2),
            card: card(18),
            resources: res([0, 0, 2, 0, 0]),
        },
        ProjectedEvent::VictoryPointsRevealed {
            winner: seat(0),
            cards: [Some(card(20)), None, None, None, None],
        },
        ProjectedEvent::BankTraded {
            seat: seat(0),
            give_resource: Resource::Brick,
            receive_resource: Resource::Wood,
            rate: 4,
        },
        ProjectedEvent::CityUpgraded {
            seat: seat(0),
            vertex: VertexId::try_from(31).unwrap(),
        },
        ProjectedEvent::ConstructionPaid {
            seat: seat(0),
            resources: res([1, 1, 0, 0, 0]),
        },
        ProjectedEvent::LongestRouteChanged {
            previous: None,
            holder: Some(seat(0)),
        },
        ProjectedEvent::RobberMoved {
            seat: seat(0),
            hex: HexId::try_from(1).unwrap(),
        },
        ProjectedEvent::RobberTheft {
            thief: seat(0),
            victim: seat(2),
            count: 1,
            resource: if viewer == Viewer::Seat(seat(0)) || viewer == Viewer::Seat(seat(2)) {
                Some(Resource::Ore)
            } else {
                None
            },
        },
        ProjectedEvent::Discarded {
            seat: seat(1),
            count: 3,
            resources: if viewer == Viewer::Seat(seat(1)) {
                Some(res([0, 0, 2, 0, 1]))
            } else {
                None
            },
        },
        ProjectedEvent::GameFinished {
            winner: seat(0),
            score: 10,
        },
        ProjectedEvent::DiceRolled {
            seat: seat(0),
            dice: [2, 3],
        },
        ProjectedEvent::Production {
            seat: seat(0),
            resources: res([1, 0, 0, 0, 0]),
        },
        ProjectedEvent::RoadPlaced {
            seat: seat(0),
            edge: EdgeId::try_from(1).unwrap(),
        },
        ProjectedEvent::StartingResources {
            seat: seat(0),
            resources: res([1, 0, 1, 0, 0]),
        },
        ProjectedEvent::TurnStarted {
            seat: seat(0),
            turn: 3,
        },
        ProjectedEvent::SettlementPlaced {
            seat: seat(0),
            vertex: VertexId::try_from(0).unwrap(),
        },
        ProjectedEvent::PhaseChanged {
            phase: Phase::Action,
        },
    ]
}
#[test]
fn privacy_native_every_event_uses_original_facts_and_the_explicit_viewer_allowlist() {
    let mut e = game();
    let receipt = sample();
    let random = e.state.random.clone();
    let position = e.state.position.clone();
    for viewer in std::iter::once(Viewer::Spectator).chain((0..4).map(|id| Viewer::Seat(seat(id))))
    {
        assert_eq!(
            e.project_receipt(&receipt, viewer).unwrap(),
            ProjectedReceipt {
                revision: 7,
                events: expected(viewer)
            }
        );
    }
    assert_eq!(e.state.position, position);
    assert_eq!(e.state.random, random);
    assert_eq!(e.state.revision, 0);
    e.state.revision = 99;
    for viewer in std::iter::once(Viewer::Spectator).chain((0..4).map(|id| Viewer::Seat(seat(id))))
    {
        assert_eq!(
            e.project_receipt(&receipt, viewer).unwrap(),
            ProjectedReceipt {
                revision: 7,
                events: expected(viewer)
            }
        );
    }
    let mut changed = receipt.clone();
    changed.events[6] = Event::DevelopmentPurchased {
        seat: seat(1),
        card: card(23),
        remaining: 9,
    };
    changed.events[15] = Event::RobberTheft {
        thief: seat(0),
        victim: seat(2),
        resource: Some(Resource::Brick),
    };
    changed.events[16] = Event::Discarded {
        seat: seat(1),
        resources: res([0, 1, 0, 0, 2]),
    };
    for viewer in [Viewer::Spectator, Viewer::Seat(seat(3))] {
        assert_eq!(
            e.project_receipt(&receipt, viewer),
            e.project_receipt(&changed, viewer)
        );
    }
    for id in [0, 1, 2] {
        assert_ne!(
            e.project_receipt(&receipt, Viewer::Seat(seat(id))),
            e.project_receipt(&changed, Viewer::Seat(seat(id)))
        );
    }
}
#[test]
fn privacy_native_paired_private_decks_hands_rng_and_vp_leave_unauthorized_views_and_queries_equal()
{
    let mut a = game();
    let mut b = game();
    for e in [&mut a, &mut b] {
        let p = &mut e.state.position;
        p.active_player = seat(0);
        p.phase = Phase::Action;
        p.turn = 2;
        for (draw, id) in [8, 20].into_iter().enumerate() {
            let index = p.deck.iter().position(|&c| c == card(id)).unwrap();
            p.deck.swap(draw, index);
        }
        p.draw_cursor = 2;
        p.card_locations[8] = CardLocation::Held {
            owner: seat(1),
            acquired_turn: 1,
        };
        p.card_locations[20] = CardLocation::Held {
            owner: seat(2),
            acquired_turn: 1,
        };
    }
    b.state.position.hands.swap(1, 2);
    b.state.position.card_locations[8] = CardLocation::Held {
        owner: seat(2),
        acquired_turn: 1,
    };
    b.state.position.card_locations[20] = CardLocation::Held {
        owner: seat(1),
        acquired_turn: 1,
    };
    b.state.position.deck[2..].reverse();
    b.state.random.set_word_pos(100);
    for viewer in [
        Viewer::Spectator,
        Viewer::Seat(seat(0)),
        Viewer::Seat(seat(3)),
    ] {
        assert_eq!(
            format!("{:?}", a.view(viewer).unwrap()),
            format!("{:?}", b.view(viewer).unwrap())
        );
    }
    for id in [1, 2] {
        assert_ne!(
            a.view(Viewer::Seat(seat(id))).unwrap().own_player,
            b.view(Viewer::Seat(seat(id))).unwrap().own_player
        );
    }
    assert_eq!(
        a.legal_actions(seat(0)).unwrap(),
        b.legal_actions(seat(0)).unwrap()
    );
    let foreign = Command::PlayKnight { card: card(8) };
    assert_eq!(
        a.validate(seat(0), &foreign).unwrap_err().code,
        RuleError::CardNotPlayable
    );
    assert_eq!(a.validate(seat(0), &foreign), b.validate(seat(0), &foreign));
    assert!(
        !a.legal_actions(seat(0)).unwrap().choices.iter().any(
            |item| matches!(item,LegalAction::PlayKnight { cards } if cards.contains(&card(8)))
        )
    );
}
