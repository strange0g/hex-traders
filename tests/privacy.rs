//! Reachable native privacy and original-receipt checks for every viewer.
use hex_traders_engine::{
    CardId, Command, CommandEnvelope, DevelopmentKind, Engine, Event, GameConfig, LegalAction,
    PlayerId, ProjectedEvent, ProjectedReceipt, Resources, RuleError, Viewer,
};
fn seat(id: u8) -> PlayerId {
    PlayerId::try_from(id).unwrap()
}
fn res(v: [u8; 5]) -> Resources {
    Resources::try_from(v).unwrap()
}
fn act(e: &mut Engine, actor: u8, command: Command) -> hex_traders_engine::Receipt {
    let revision = e.view(Viewer::Spectator).unwrap().revision;
    e.execute(CommandEnvelope {
        api_version: 1,
        actor: seat(actor),
        expected_revision: revision,
        command,
    })
    .unwrap()
}
#[test]
fn privacy_public_purchase_trade_and_old_receipts_have_explicit_per_viewer_facts() {
    let mut seed = [0; 32];
    seed[0] = 8;
    let (mut e, _) = Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), seed).unwrap();
    assert_eq!(e.view(Viewer::Spectator).unwrap().active_player, seat(0));
    assert_eq!(
        act(&mut e, 0, Command::RollDice).events[0],
        Event::DiceRolled {
            seat: seat(0),
            dice: [2, 4]
        }
    );
    act(&mut e, 0, Command::EndTurn);
    assert_eq!(
        act(&mut e, 1, Command::RollDice).events[0],
        Event::DiceRolled {
            seat: seat(1),
            dice: [1, 4]
        }
    );
    let purchase = act(&mut e, 1, Command::BuyDevelopmentCard);
    let card = CardId::try_from(18).unwrap();
    assert!(purchase.events.contains(&Event::DevelopmentPurchased {
        seat: seat(1),
        card,
        remaining: 24
    }));
    let proposal = act(
        &mut e,
        1,
        Command::ProposeTrade {
            recipient: seat(0),
            give: res([0, 1, 0, 0, 0]),
            receive: res([0, 0, 0, 0, 1]),
        },
    );
    let offer = e.view(Viewer::Spectator).unwrap().offer.unwrap();
    assert_eq!(offer.id, proposal.revision);
    assert!(
        e.legal_actions(seat(0))
            .unwrap()
            .choices
            .contains(&LegalAction::AcceptTrade { offer_id: offer.id })
    );
    let shared = e.view(Viewer::Spectator).unwrap();
    let public_json: serde_json::Value =
        serde_json::from_slice(&shared.to_json().unwrap()).unwrap();
    assert!(public_json["ownPlayer"].is_null());
    assert!(public_json.get("seed").is_none());
    assert!(public_json.get("deck").is_none());
    assert_eq!(
        public_json
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        vec![
            "activePlayer",
            "bank",
            "board",
            "developmentDeckRemaining",
            "largestArmyHolder",
            "lastDice",
            "longestRouteHolder",
            "offer",
            "ownPlayer",
            "phase",
            "players",
            "revealedVictoryCards",
            "revision",
            "seatOrder",
            "setup",
            "startingPlayer",
            "turn",
            "turnOrder"
        ]
    );
    assert_eq!(public_json["offer"]["id"], offer.id.to_string());
    assert_eq!(public_json["board"]["roads"].as_array().unwrap().len(), 72);
    assert_eq!(
        public_json["board"]["buildings"].as_array().unwrap().len(),
        54
    );
    assert!(shared.own_player.is_none());
    for id in 0..3 {
        let view = e.view(Viewer::Seat(seat(id))).unwrap();
        let wire: serde_json::Value = serde_json::from_slice(&view.to_json().unwrap()).unwrap();
        assert_eq!(wire["players"], public_json["players"]);
        assert_eq!(wire["offer"], public_json["offer"]);
        assert_eq!(wire["ownPlayer"]["seat"], id);
        if id == 1 {
            assert_eq!(wire["ownPlayer"]["developmentCards"][0]["id"], 18);
            assert_eq!(
                wire["ownPlayer"]["developmentCards"][0]["kind"],
                "invention"
            );
        }
        assert_eq!(view.players, shared.players);
        assert_eq!(view.bank, shared.bank);
        assert_eq!(view.offer, Some(offer));
        let own = view.own_player.unwrap();
        assert_eq!(own.seat, seat(id));
        if id == 1 {
            assert_eq!(own.development_cards.len(), 1);
            assert_eq!(
                (own.development_cards[0].id, own.development_cards[0].kind),
                (card, DevelopmentKind::Invention)
            );
        } else {
            assert!(own.development_cards.is_empty());
        }
    }
    let accepted = act(&mut e, 0, Command::AcceptTrade { offer_id: offer.id });
    assert_eq!(
        accepted.events,
        vec![
            Event::TradeCompleted { offer },
            Event::TradeOfferClosed { offer_id: offer.id }
        ]
    );
    for viewer in [
        Viewer::Spectator,
        Viewer::Seat(seat(0)),
        Viewer::Seat(seat(1)),
        Viewer::Seat(seat(2)),
    ] {
        let wire: serde_json::Value = serde_json::from_slice(
            &e.project_receipt(&purchase, viewer)
                .unwrap()
                .to_json()
                .unwrap(),
        )
        .unwrap();
        assert_eq!(wire["revision"], purchase.revision.to_string());
        assert_eq!(
            wire["events"][1]["card"],
            if viewer == Viewer::Seat(seat(1)) {
                serde_json::json!(18)
            } else {
                serde_json::Value::Null
            }
        );
        assert!(wire.get("command").is_none());
        assert!(wire.get("finalScores").is_none());
        assert_eq!(
            e.project_receipt(&purchase, viewer).unwrap(),
            ProjectedReceipt {
                revision: purchase.revision,
                events: vec![
                    ProjectedEvent::ConstructionPaid {
                        seat: seat(1),
                        resources: res([0, 0, 1, 1, 1])
                    },
                    ProjectedEvent::DevelopmentPurchased {
                        seat: seat(1),
                        card: if viewer == Viewer::Seat(seat(1)) {
                            Some(card)
                        } else {
                            None
                        },
                        remaining: 24
                    }
                ]
            }
        );
        assert_eq!(
            e.project_receipt(&accepted, viewer).unwrap(),
            ProjectedReceipt {
                revision: accepted.revision,
                events: vec![
                    ProjectedEvent::TradeCompleted { offer },
                    ProjectedEvent::TradeOfferClosed { offer_id: offer.id }
                ]
            }
        );
    }
    let before = e.view(Viewer::Spectator).unwrap().revision;
    assert_eq!(
        e.execute(CommandEnvelope {
            api_version: 1,
            actor: seat(0),
            expected_revision: proposal.revision,
            command: Command::AcceptTrade { offer_id: offer.id }
        })
        .unwrap_err()
        .code,
        RuleError::StaleRevision
    );
    assert_eq!(e.view(Viewer::Spectator).unwrap().revision, before);
    act(&mut e, 1, Command::EndTurn);
    assert_eq!(
        e.project_receipt(&accepted, Viewer::Spectator)
            .unwrap()
            .revision,
        accepted.revision
    );
}
