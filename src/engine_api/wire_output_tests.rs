use super::*;
use crate::{Command, CommandEnvelope, GameConfig, RuleError, Viewer};
fn seat(v: u8) -> PlayerId {
    PlayerId::try_from(v).unwrap()
}
fn card(v: u8) -> CardId {
    CardId::try_from(v).unwrap()
}
fn edge(v: u8) -> EdgeId {
    EdgeId::try_from(v).unwrap()
}
fn vertex(v: u8) -> VertexId {
    VertexId::try_from(v).unwrap()
}
fn json<T: ?Sized>(v: &T) -> serde_json::Value
where
    for<'a> Wire<'a, T>: Serialize,
{
    serde_json::from_slice(&encode(&Wire(v)).unwrap()).unwrap()
}
#[test]
fn wire_all_phases_and_choices_have_exact_tags_fields_and_counters() {
    let phases = [
        (
            Phase::SetupSettlement {
                actor: seat(3),
                round: 2,
            },
            serde_json::json!({"type":"setupSettlement","actor":3,"round":2}),
        ),
        (
            Phase::SetupRoad {
                actor: seat(3),
                round: 2,
                settlement: vertex(53),
            },
            serde_json::json!({"type":"setupRoad","actor":3,"round":2,"settlement":53}),
        ),
        (Phase::PreRoll, serde_json::json!({"type":"preRoll"})),
        (Phase::Action, serde_json::json!({"type":"action"})),
        (
            Phase::Discard {
                obligations: [0, 8, 0, 4],
            },
            serde_json::json!({"type":"discard","obligations":[0,8,0,4]}),
        ),
        (
            Phase::RobberMove {
                origin: RobberOrigin::Seven,
                continuation: ResumePhase::Action,
            },
            serde_json::json!({"type":"robberMove","origin":"seven","continuation":"action"}),
        ),
        (
            Phase::RobberVictim {
                origin: RobberOrigin::Knight,
                continuation: ResumePhase::PreRoll,
                victims: [None, Some(seat(1)), None, Some(seat(3))],
            },
            serde_json::json!({"type":"robberVictim","origin":"knight","continuation":"preRoll","victims":[null,1,null,3]}),
        ),
        (
            Phase::FreeRoads {
                remaining: 1,
                continuation: ResumePhase::PreRoll,
            },
            serde_json::json!({"type":"freeRoads","remaining":1,"continuation":"preRoll"}),
        ),
        (
            Phase::Finished {
                winner: seat(3),
                score: 12,
            },
            serde_json::json!({"type":"finished","winner":3,"score":12}),
        ),
    ];
    for (phase, expected) in phases {
        assert_eq!(json::<Phase>(&phase), expected);
    }
    let choices = [
        (
            LegalAction::ProposeTrade {
                recipients: vec![seat(1), seat(3)],
            },
            serde_json::json!({"type":"proposeTrade","recipients":[1,3]}),
        ),
        (
            LegalAction::AcceptTrade { offer_id: u64::MAX },
            serde_json::json!({"type":"acceptTrade","offerId":"18446744073709551615"}),
        ),
        (
            LegalAction::RejectTrade { offer_id: 0 },
            serde_json::json!({"type":"rejectTrade","offerId":"0"}),
        ),
        (
            LegalAction::CancelTrade { offer_id: 12 },
            serde_json::json!({"type":"cancelTrade","offerId":"12"}),
        ),
        (
            LegalAction::PlayRoadBuilding {
                cards: vec![card(14), card(15)],
            },
            serde_json::json!({"type":"playRoadBuilding","cards":[14,15]}),
        ),
        (
            LegalAction::PlaceFreeRoad {
                edges: vec![edge(71)],
            },
            serde_json::json!({"type":"placeFreeRoad","edges":[71]}),
        ),
        (
            LegalAction::PlayKnight {
                cards: vec![card(13)],
            },
            serde_json::json!({"type":"playKnight","cards":[13]}),
        ),
        (
            LegalAction::BuyDevelopmentCard,
            serde_json::json!({"type":"buyDevelopmentCard"}),
        ),
        (
            LegalAction::PlayMonopoly {
                cards: vec![card(16)],
            },
            serde_json::json!({"type":"playMonopoly","cards":[16]}),
        ),
        (
            LegalAction::PlayInvention {
                cards: vec![card(18)],
            },
            serde_json::json!({"type":"playInvention","cards":[18]}),
        ),
        (
            LegalAction::BankTrade {
                pairs: vec![(Resource::Brick, Resource::Ore)],
            },
            serde_json::json!({"type":"bankTrade","pairs":[["brick","ore"]]}),
        ),
        (
            LegalAction::UpgradeCity {
                vertices: vec![vertex(53)],
            },
            serde_json::json!({"type":"upgradeCity","vertices":[53]}),
        ),
        (
            LegalAction::BuildSettlement {
                vertices: vec![vertex(0)],
            },
            serde_json::json!({"type":"buildSettlement","vertices":[0]}),
        ),
        (
            LegalAction::BuildRoad {
                edges: vec![edge(0)],
            },
            serde_json::json!({"type":"buildRoad","edges":[0]}),
        ),
        (
            LegalAction::MoveRobber {
                hexes: vec![HexId::try_from(18).unwrap()],
            },
            serde_json::json!({"type":"moveRobber","hexes":[18]}),
        ),
        (
            LegalAction::ChooseRobberVictim {
                victims: vec![seat(3)],
            },
            serde_json::json!({"type":"chooseRobberVictim","victims":[3]}),
        ),
        (
            LegalAction::DiscardResources { quantity: 47 },
            serde_json::json!({"type":"discardResources","quantity":47}),
        ),
        (LegalAction::EndTurn, serde_json::json!({"type":"endTurn"})),
        (
            LegalAction::RollDice,
            serde_json::json!({"type":"rollDice"}),
        ),
        (
            LegalAction::PlaceSetupRoad {
                edges: vec![edge(71)],
            },
            serde_json::json!({"type":"placeSetupRoad","edges":[71]}),
        ),
        (
            LegalAction::PlaceSetupSettlement {
                vertices: vec![vertex(53)],
            },
            serde_json::json!({"type":"placeSetupSettlement","vertices":[53]}),
        ),
    ];
    for (choice, expected) in choices {
        assert_eq!(json::<LegalAction>(&choice), expected);
    }
    let actions = LegalActions {
        revision: u64::MAX,
        choices: vec![LegalAction::RollDice],
    };
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&actions.to_json().unwrap()).unwrap(),
        serde_json::json!({"revision":"18446744073709551615","choices":[{"type":"rollDice"}]})
    );
}
#[test]
fn wire_views_creation_and_errors_are_explicit_bounded_allowlists() {
    for (id, kind, name) in [
        (0, DevelopmentKind::Knight, "knight"),
        (14, DevelopmentKind::RoadBuilding, "roadBuilding"),
        (16, DevelopmentKind::Monopoly, "monopoly"),
        (18, DevelopmentKind::Invention, "invention"),
        (24, DevelopmentKind::VictoryPoint, "victoryPoint"),
    ] {
        let v = DevelopmentCardView {
            id: card(id),
            kind,
            acquired_turn: 1,
            acquired_on_earlier_turn: true,
        };
        assert_eq!(json::<DevelopmentCardView>(&v)["kind"], name);
    }
    let (mut engine, creation) =
        super::super::Engine::create(GameConfig::fixed(&[0, 1, 2, 3]).unwrap(), [8; 32]).unwrap();
    let projected = engine
        .project_creation_receipt(&creation, Viewer::Spectator)
        .unwrap();
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&projected.to_json().unwrap()).unwrap()["events"].as_array().unwrap().len(),creation.events.len());
    let own = engine.view(Viewer::Seat(seat(0))).unwrap();
    let value = json::<GameView>(&own);
    assert_eq!(
        value["ownPlayer"]["resources"],
        serde_json::json!({"brick":1,"wood":0,"wool":0,"wheat":1,"ore":1})
    );
    assert_eq!(
        value["board"]["graph"]["hexVertices"][0],
        serde_json::json!([0, 4, 8, 12, 7, 3])
    );
    assert_eq!(
        value["board"]["graph"]["edgeVertices"][71],
        serde_json::json!([50, 53])
    );
    assert_eq!(
        value["players"][0]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        vec![
            "citiesRemaining",
            "developmentCardCount",
            "longestRouteLength",
            "playedKnights",
            "publicScore",
            "resourceCount",
            "roadsRemaining",
            "seat",
            "settlementsRemaining",
            "spentProgress"
        ]
    );
    let card_view = DevelopmentCardView {
        id: card(24),
        kind: DevelopmentKind::VictoryPoint,
        acquired_turn: u64::MAX,
        acquired_on_earlier_turn: false,
    };
    assert_eq!(
        json::<DevelopmentCardView>(&card_view),
        serde_json::json!({"id":24,"kind":"victoryPoint","acquiredTurn":"18446744073709551615","acquiredOnEarlierTurn":false})
    );
    let roll = engine
        .execute(CommandEnvelope {
            api_version: 1,
            actor: own.active_player,
            expected_revision: 0,
            command: Command::RollDice,
        })
        .unwrap();
    assert_eq!(
        engine
            .project_receipt(&roll, Viewer::Spectator)
            .unwrap()
            .to_json()
            .unwrap(),
        engine
            .project_receipt(&roll, Viewer::Seat(seat(3)))
            .unwrap()
            .to_json()
            .unwrap()
    );
    let errors = [
        (RuleError::EmptyDeck, "emptyDeck"),
        (RuleError::CardNotPlayable, "cardNotPlayable"),
        (RuleError::InvalidTrade, "invalidTrade"),
        (RuleError::OfferBusy, "offerBusy"),
        (RuleError::OfferNotFound, "offerNotFound"),
        (RuleError::OfferUnavailable, "offerUnavailable"),
        (RuleError::BankShortage, "bankShortage"),
        (RuleError::NotOwned, "notOwned"),
        (RuleError::InsufficientResources, "insufficientResources"),
        (RuleError::NoPieces, "noPieces"),
        (RuleError::InvalidInput, "invalidInput"),
        (RuleError::InvalidVictim, "invalidVictim"),
        (
            RuleError::UnsupportedApiVersion(999),
            "unsupportedApiVersion",
        ),
        (RuleError::InvalidSeat(255), "invalidSeat"),
        (RuleError::StaleRevision, "staleRevision"),
        (RuleError::GameFinished, "gameFinished"),
        (RuleError::WrongActor, "wrongActor"),
        (RuleError::WrongPhase, "wrongPhase"),
        (RuleError::CounterExhausted, "counterExhausted"),
        (RuleError::Occupied, "occupied"),
        (RuleError::DistanceRule, "distanceRule"),
        (RuleError::InvalidDiscard, "invalidDiscard"),
        (RuleError::Disconnected, "disconnected"),
    ];
    for (code, name) in errors {
        let error = Rejection {
            code,
            current_revision: u64::MAX,
        };
        assert_eq!(
            json::<Rejection>(&error),
            serde_json::json!({"code":name,"currentRevision":"18446744073709551615"})
        );
    }
    let error = engine.execute_json(b"privateSentinel").unwrap_err();
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&error.to_json().unwrap()).unwrap(),
        serde_json::json!({"code":"invalidInput","currentRevision":"1"})
    );
    let (variable, created) =
        super::super::Engine::create(GameConfig::variable(&[0, 1, 2]).unwrap(), [0; 32]).unwrap();
    let value = serde_json::from_slice::<serde_json::Value>(
        &variable
            .project_creation_receipt(&created, Viewer::Spectator)
            .unwrap()
            .to_json()
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        value["events"].as_array().unwrap().last().unwrap()["type"],
        "setupStarted"
    );
    assert!(super::super::wire::parse(&own.to_json().unwrap()).is_ok());
}

#[test]
fn wire_creation_events_and_all_output_counters_match_literal_contracts() {
    use crate::Resources;
    let events = vec![
        CreationEvent::GameCreated {
            setup: SetupMode::Variable,
            seat_order: vec![seat(2), seat(0), seat(1)],
        },
        CreationEvent::SettlementPlaced {
            seat: seat(3),
            vertex: vertex(53),
        },
        CreationEvent::RoadPlaced {
            seat: seat(3),
            edge: edge(71),
        },
        CreationEvent::StartingResources {
            seat: seat(3),
            resources: Resources::try_from([1, 2, 3, 4, 5]).unwrap(),
        },
        CreationEvent::StarterRolled {
            seat: seat(3),
            dice: [6, 1],
        },
        CreationEvent::TurnStarted {
            seat: seat(3),
            turn: u64::MAX,
        },
        CreationEvent::SetupStarted {
            phase: Phase::SetupRoad {
                actor: seat(3),
                round: 2,
                settlement: vertex(53),
            },
        },
    ];
    let expected = serde_json::json!([
        {"type":"gameCreated","setup":"variable","seatOrder":[2,0,1]},
        {"type":"settlementPlaced","seat":3,"vertex":53},
        {"type":"roadPlaced","seat":3,"edge":71},
        {"type":"startingResources","seat":3,"resources":{"brick":1,"wood":2,"wool":3,"wheat":4,"ore":5}},
        {"type":"starterRolled","seat":3,"dice":[6,1]},
        {"type":"turnStarted","seat":3,"turn":"18446744073709551615"},
        {"type":"setupStarted","phase":{"type":"setupRoad","actor":3,"round":2,"settlement":53}}
    ]);
    assert_eq!(json::<Vec<CreationEvent>>(&events), expected);
    let (engine, _) =
        super::super::Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), [0; 32]).unwrap();
    for (counter, decimal) in [(0, "0"), (u64::MAX, "18446744073709551615")] {
        let mut view = engine.view(Viewer::Spectator).unwrap();
        view.revision = counter;
        view.turn = counter;
        let value = json::<GameView>(&view);
        assert_eq!(value["revision"], decimal);
        assert_eq!(value["turn"], decimal);
        let offer = TradeOffer {
            id: counter,
            turn: counter,
            proposer: seat(0),
            recipient: seat(1),
            give: Resources::try_from([1, 0, 0, 0, 0]).unwrap(),
            receive: Resources::try_from([0, 1, 0, 0, 0]).unwrap(),
        };
        assert_eq!(
            json::<TradeOffer>(&offer),
            serde_json::json!({"id":decimal,"turn":decimal,"proposer":0,"recipient":1,"give":{"brick":1,"wood":0,"wool":0,"wheat":0,"ore":0},"receive":{"brick":0,"wood":1,"wool":0,"wheat":0,"ore":0}})
        );
        let receipt = ProjectedReceipt {
            revision: counter,
            events: vec![
                ProjectedEvent::TurnStarted {
                    seat: seat(0),
                    turn: counter,
                },
                ProjectedEvent::TradeOfferClosed { offer_id: counter },
            ],
        };
        assert_eq!(
            json::<ProjectedReceipt>(&receipt),
            serde_json::json!({"revision":decimal,"events":[{"type":"turnStarted","seat":0,"turn":decimal},{"type":"tradeOfferClosed","offerId":decimal}]})
        );
        let creation = ProjectedCreationReceipt {
            revision: counter,
            events: vec![CreationEvent::TurnStarted {
                seat: seat(0),
                turn: counter,
            }],
        };
        assert_eq!(
            json::<ProjectedCreationReceipt>(&creation),
            serde_json::json!({"revision":decimal,"events":[{"type":"turnStarted","seat":0,"turn":decimal}]})
        );
    }
}
