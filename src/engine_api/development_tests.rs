//! Exact purchase identities are canonical pinned-shuffle literals, never type counts.
use super::*;
use crate::game_model::CardLocation;
use crate::{Building, BuildingKind, CardId, DevelopmentKind, GameConfig};
fn seat(id: u8) -> PlayerId {
    PlayerId::try_from(id).unwrap()
}
fn card(id: u8) -> CardId {
    CardId::try_from(id).unwrap()
}
fn fixture() -> Engine {
    let mut seed = [0; 32];
    seed[0] = 21;
    let (mut engine, _) = Engine::create(GameConfig::fixed(&[0, 1, 2, 3]).unwrap(), seed).unwrap();
    engine.state.position.active_player = seat(0);
    engine.state.position.phase = Phase::Action;
    fund(&mut engine, [0, 0, 4, 4, 4]);
    engine
}
fn fund(engine: &mut Engine, counts: [u8; 5]) {
    engine.state.position.hands = [Resources::default(); 4];
    engine.state.position.hands[0] = Resources::try_from(counts).unwrap();
    engine.state.position.bank = Resources::try_from(counts.map(|count| 19 - count)).unwrap();
}
fn buy(engine: &mut Engine) -> Receipt {
    engine
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(0),
            expected_revision: engine.state.revision,
            command: Command::BuyDevelopmentCard,
        })
        .unwrap()
}
fn reject(engine: &mut Engine, actor: PlayerId, command: Command, error: RuleError) {
    let before = engine.state.position.clone();
    let random = engine.state.random.clone();
    let revision = engine.state.revision;
    assert_eq!(engine.validate(actor, &command).unwrap_err().code, error);
    assert_eq!(
        engine
            .execute(CommandEnvelope {
                api_version: 1,
                actor,
                expected_revision: revision,
                command
            })
            .unwrap_err()
            .code,
        error
    );
    assert_eq!(engine.state.position, before);
    assert_eq!(engine.state.random, random);
    assert_eq!(engine.state.revision, revision);
}
fn conservation(engine: &Engine) {
    let position = &engine.state.position;
    let mut ids = position.deck.map(|id| id.index());
    ids.sort();
    assert_eq!(ids, std::array::from_fn::<_, 25, _>(|id| id));
    for (index, id) in position.deck.iter().enumerate() {
        let location = position.card_locations[id.index()];
        if index >= usize::from(position.draw_cursor) {
            assert_eq!(location, CardLocation::Undrawn);
        } else {
            match location {
                CardLocation::Held {
                    owner,
                    acquired_turn,
                } => {
                    assert!(position.config.seat_order().contains(&owner));
                    assert!(acquired_turn <= position.turn);
                }
                CardLocation::RevealedVictory { owner } => {
                    assert_eq!(id.kind(), DevelopmentKind::VictoryPoint);
                    assert!(
                        matches!(position.phase, Phase::Finished { winner, .. } if winner == owner)
                    );
                }
                CardLocation::PlayedKnight { owner } => {
                    assert_eq!(id.kind(), DevelopmentKind::Knight);
                    assert!(position.config.seat_order().contains(&owner));
                }
                CardLocation::Spent { owner } => {
                    assert!(matches!(
                        id.kind(),
                        DevelopmentKind::Monopoly
                            | DevelopmentKind::Invention
                            | DevelopmentKind::RoadBuilding
                    ));
                    assert!(position.config.seat_order().contains(&owner));
                }
                CardLocation::Undrawn => panic!("drawn card has no owner"),
            }
        }
    }
    for index in 0..5 {
        assert_eq!(
            u16::from(position.bank.counts()[index])
                + position
                    .hands
                    .iter()
                    .map(|hand| u16::from(hand.counts()[index]))
                    .sum::<u16>(),
            19
        );
    }
}
#[test]
fn base_rules_br025_development_purchase_exact_draw_cost_cursor_no_rng_and_timing() {
    let mut engine = fixture();
    let random = engine.state.random.clone();
    let expected = [23, 8, 16, 17];
    for (index, id) in expected.into_iter().enumerate() {
        if index == 3 {
            engine.state.position.turn = 4;
        }
        assert!(
            engine
                .legal_actions(seat(0))
                .unwrap()
                .choices
                .contains(&LegalAction::BuyDevelopmentCard)
        );
        crate::base_rules::reset_route_searches();
        let receipt = buy(&mut engine);
        assert_eq!(receipt.previous_revision, index as u64);
        assert_eq!(receipt.revision, index as u64 + 1);
        assert_eq!(receipt.final_scores, None);
        assert_eq!(
            receipt.events,
            vec![
                Event::ConstructionPaid {
                    seat: seat(0),
                    resources: Resources::try_from([0, 0, 1, 1, 1]).unwrap()
                },
                Event::DevelopmentPurchased {
                    seat: seat(0),
                    card: card(id),
                    remaining: 24 - index as u8
                },
            ]
        );
        assert_eq!(engine.state.position.draw_cursor, index as u8 + 1);
        assert_eq!(
            engine.state.position.card_locations[usize::from(id)],
            CardLocation::Held {
                owner: seat(0),
                acquired_turn: if index == 3 { 4 } else { 1 }
            }
        );
        assert_eq!(
            engine.state.position.hands[0].counts(),
            &[0, 0, 3 - index as u8, 3 - index as u8, 3 - index as u8]
        );
        assert_eq!(
            engine.state.position.bank.counts(),
            &[19, 19, 16 + index as u8, 16 + index as u8, 16 + index as u8]
        );
        assert_eq!(engine.state.random, random);
        assert_eq!(crate::base_rules::route_searches(), 0);
        conservation(&engine);
    }
    let own = engine
        .view(Viewer::Seat(seat(0)))
        .unwrap()
        .own_player
        .unwrap();
    assert_eq!(own.true_score, 3);
    let monopolies = own
        .development_cards
        .iter()
        .filter(|card| card.kind == DevelopmentKind::Monopoly)
        .collect::<Vec<_>>();
    assert_eq!(monopolies.len(), 2);
    assert_eq!(
        (
            monopolies[0].id,
            monopolies[0].acquired_turn,
            monopolies[0].acquired_on_earlier_turn
        ),
        (card(16), 1, true)
    );
    assert_eq!(
        (
            monopolies[1].id,
            monopolies[1].acquired_turn,
            monopolies[1].acquired_on_earlier_turn
        ),
        (card(17), 4, false)
    );
    assert!(
        !engine
            .legal_actions(seat(0))
            .unwrap()
            .choices
            .contains(&LegalAction::BuyDevelopmentCard)
    );
    reject(
        &mut engine,
        seat(0),
        Command::BuyDevelopmentCard,
        RuleError::InsufficientResources,
    );
}
#[test]
fn development_purchase_rejections_preserve_every_private_fact_and_precedence() {
    for index in [2, 3, 4] {
        let mut engine = fixture();
        let mut counts = [0, 0, 1, 1, 1];
        counts[index] = 0;
        fund(&mut engine, counts);
        reject(
            &mut engine,
            seat(0),
            Command::BuyDevelopmentCard,
            RuleError::InsufficientResources,
        );
    }
    for phase in [
        Phase::PreRoll,
        Phase::SetupSettlement {
            actor: seat(0),
            round: 1,
        },
        Phase::SetupRoad {
            actor: seat(0),
            round: 1,
            settlement: VertexId::try_from(31).unwrap(),
        },
        Phase::Discard {
            obligations: [1, 0, 0, 0],
        },
        Phase::RobberMove {
            origin: crate::RobberOrigin::Seven,
            continuation: crate::ResumePhase::Action,
        },
        Phase::RobberVictim {
            origin: crate::RobberOrigin::Seven,
            continuation: crate::ResumePhase::Action,
            victims: [Some(seat(1)), Some(seat(2)), None, None],
        },
    ] {
        let mut engine = fixture();
        engine.state.position.phase = phase;
        reject(
            &mut engine,
            seat(0),
            Command::BuyDevelopmentCard,
            RuleError::WrongPhase,
        );
        reject(
            &mut engine,
            seat(1),
            Command::BuyDevelopmentCard,
            RuleError::WrongActor,
        );
    }
    let mut engine = fixture();
    reject(
        &mut engine,
        seat(1),
        Command::BuyDevelopmentCard,
        RuleError::WrongActor,
    );
    engine.state.revision = u64::MAX;
    reject(
        &mut engine,
        seat(0),
        Command::BuyDevelopmentCard,
        RuleError::CounterExhausted,
    );
    let before = engine.state.position.clone();
    let random = engine.state.random.clone();
    for (api_version, actor, revision, error) in [
        (2, seat(0), 0, RuleError::UnsupportedApiVersion(2)),
        (1, seat(0), 0, RuleError::StaleRevision),
    ] {
        assert_eq!(
            engine
                .execute(CommandEnvelope {
                    api_version,
                    actor,
                    expected_revision: revision,
                    command: Command::BuyDevelopmentCard
                })
                .unwrap_err()
                .code,
            error
        );
        assert_eq!(engine.state.position, before);
        assert_eq!(engine.state.random, random);
    }
    let (mut three, _) = Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), [0; 32]).unwrap();
    reject(
        &mut three,
        seat(3),
        Command::BuyDevelopmentCard,
        RuleError::InvalidSeat(3),
    );
}
#[test]
fn base_rules_br025_development_purchase_deck_exhaustion_conserves_all_twenty_five() {
    let mut engine = fixture();
    let random = engine.state.random.clone();
    let deck = [
        23, 8, 16, 17, 10, 7, 9, 18, 1, 6, 22, 15, 2, 24, 13, 20, 3, 11, 4, 12, 19, 21, 5, 0, 14,
    ];
    for (index, id) in deck.into_iter().enumerate() {
        fund(&mut engine, [0, 0, 1, 1, 1]);
        let receipt = buy(&mut engine);
        assert_eq!(
            receipt.events[1],
            Event::DevelopmentPurchased {
                seat: seat(0),
                card: card(id),
                remaining: 24 - index as u8
            }
        );
        conservation(&engine);
    }
    assert_eq!(engine.state.position.draw_cursor, 25);
    assert_eq!(engine.state.random, random);
    fund(&mut engine, [0, 0, 1, 1, 1]);
    reject(
        &mut engine,
        seat(0),
        Command::BuyDevelopmentCard,
        RuleError::EmptyDeck,
    );
    assert!(
        !engine
            .legal_actions(seat(0))
            .unwrap()
            .choices
            .contains(&LegalAction::BuyDevelopmentCard)
    );
}
#[test]
fn privacy_purchase_owner_only_identity_score_and_original_receipts() {
    let mut engine = fixture();
    let first = buy(&mut engine);
    let second = buy(&mut engine);
    let random = engine.state.random.clone();
    let before = engine.state.position.clone();
    for viewer in [
        Viewer::Spectator,
        Viewer::Seat(seat(0)),
        Viewer::Seat(seat(1)),
        Viewer::Seat(seat(2)),
        Viewer::Seat(seat(3)),
    ] {
        let view = engine.view(viewer).unwrap();
        assert_eq!(view.players[0].development_card_count, 2);
        assert_eq!(view.players[0].public_score, 2);
        assert_eq!(view.development_deck_remaining, 23);
        assert_eq!(view.revealed_victory_cards, [None; 5]);
        if let Some(own) = view.own_player {
            assert_eq!(
                own.development_cards.len(),
                if viewer == Viewer::Seat(seat(0)) {
                    2
                } else {
                    0
                }
            );
            assert_eq!(
                own.true_score,
                if viewer == Viewer::Seat(seat(0)) {
                    3
                } else {
                    2
                }
            );
        }
        for (receipt, id, remaining) in [(&first, 23, 24), (&second, 8, 23)] {
            let projected = engine.project_receipt(receipt, viewer).unwrap();
            assert_eq!(projected.revision, receipt.revision);
            assert_eq!(
                projected.events[1],
                ProjectedEvent::DevelopmentPurchased {
                    seat: seat(0),
                    remaining,
                    card: if viewer == Viewer::Seat(seat(0)) {
                        Some(card(id))
                    } else {
                        None
                    }
                }
            );
        }
    }
    // Swap hidden kinds without changing any public count, score, bank or RNG fact.
    engine.state.position.deck.swap(0, 1);
    engine.state.position.card_locations.swap(23, 8);
    let public_before = engine.view(Viewer::Spectator).unwrap();
    engine.state.position.deck.swap(0, 2);
    engine.state.position.card_locations.swap(16, 8);
    assert_eq!(
        format!("{:?}", engine.view(Viewer::Spectator).unwrap()),
        format!("{public_before:?}")
    );
    assert_eq!(engine.state.random, random);
    engine.state.position = before;
}
fn nine_points(engine: &mut Engine) {
    engine.state.position.buildings = [None; 54];
    for (id, kind) in [
        (31, BuildingKind::City),
        (40, BuildingKind::City),
        (42, BuildingKind::City),
        (48, BuildingKind::Settlement),
    ] {
        engine.state.position.buildings[id] = Some(Building {
            owner: seat(0),
            kind,
        });
    }
    engine.state.position.roads = [None; 72];
    for id in [53, 47, 46, 45, 44, 51, 57, 63] {
        engine.state.position.roads[id] = Some(seat(0));
    }
    engine.state.position.road_lengths = [8, 0, 0, 0];
    engine.state.position.longest_route_holder = Some(seat(0));
    assert_eq!(
        crate::base_rules::current_score(&engine.state.position, seat(0)),
        9
    );
}
#[test]
fn privacy_purchase_new_vp_finishes_at_ten_reveals_winner_only_and_freezes() {
    let mut engine = fixture();
    nine_points(&mut engine);
    // Loser's VP and first Knight are dealt; next pinned ID16 is moved with ID23.
    engine.state.position.deck.swap(0, 2);
    for id in [16, 8] {
        engine.state.position.card_locations[id] = CardLocation::Held {
            owner: seat(1),
            acquired_turn: 1,
        };
    }
    engine.state.position.draw_cursor = 2;
    // Keep a losing hidden VP as well, preserving unique locations/prefix.
    engine.state.position.deck.swap(0, 10);
    engine.state.position.card_locations[16] = CardLocation::Undrawn;
    engine.state.position.card_locations[22] = CardLocation::Held {
        owner: seat(1),
        acquired_turn: 1,
    };
    let random = engine.state.random.clone();
    let receipt = buy(&mut engine);
    assert_eq!(
        engine.state.position.phase,
        Phase::Finished {
            winner: seat(0),
            score: 10
        }
    );
    assert_eq!(engine.state.position.final_scores, Some([10, 1, 0, 0]));
    assert_eq!(receipt.final_scores, Some([10, 1, 0, 0]));
    assert_eq!(
        receipt.events[2],
        Event::VictoryPointsRevealed {
            winner: seat(0),
            cards: [Some(card(23)), None, None, None, None]
        }
    );
    assert_eq!(
        receipt.events.last(),
        Some(&Event::GameFinished {
            winner: seat(0),
            score: 10
        })
    );
    assert_eq!(engine.state.random, random);
    conservation(&engine);
    for viewer in [
        Viewer::Spectator,
        Viewer::Seat(seat(0)),
        Viewer::Seat(seat(1)),
        Viewer::Seat(seat(2)),
        Viewer::Seat(seat(3)),
    ] {
        let view = engine.view(viewer).unwrap();
        assert_eq!(view.players[0].public_score, 10);
        assert_eq!(view.players[1].public_score, 0);
        assert_eq!(
            view.revealed_victory_cards,
            [Some(card(23)), None, None, None, None]
        );
        assert_eq!(
            engine.project_receipt(&receipt, viewer).unwrap().events[2],
            ProjectedEvent::VictoryPointsRevealed {
                winner: seat(0),
                cards: view.revealed_victory_cards
            }
        );
        if let Some(own) = view.own_player
            && own.seat == seat(1)
        {
            assert_eq!(own.true_score, 1);
            assert_eq!(own.development_cards.len(), 2);
        }
    }
    for actor in 0..4 {
        assert!(
            engine
                .legal_actions(seat(actor))
                .unwrap()
                .choices
                .is_empty()
        );
        reject(
            &mut engine,
            seat(actor),
            Command::BuyDevelopmentCard,
            RuleError::GameFinished,
        );
        reject(
            &mut engine,
            seat(actor),
            Command::RollDice,
            RuleError::GameFinished,
        );
        reject(
            &mut engine,
            seat(actor),
            Command::EndTurn,
            RuleError::GameFinished,
        );
    }
}
#[test]
fn privacy_purchase_multiple_hidden_vps_wait_for_own_turn_and_reveal_together() {
    let mut engine = fixture();
    nine_points(&mut engine);
    engine.state.position.buildings[48] = None; // Three cities + road award = eight.
    // Two winner VPs bought earlier and one losing VP; all locations match the prefix.
    for (index, id) in [20_u8, 23, 22].into_iter().enumerate() {
        let found = engine
            .state
            .position
            .deck
            .iter()
            .position(|&item| item == card(id))
            .unwrap();
        engine.state.position.deck.swap(index, found);
        engine.state.position.card_locations[usize::from(id)] = CardLocation::Held {
            owner: if id == 22 { seat(1) } else { seat(0) },
            acquired_turn: 1,
        };
    }
    engine.state.position.draw_cursor = 3;
    engine.state.position.active_player = seat(3);
    assert_eq!(
        engine.view(Viewer::Spectator).unwrap().players[0].public_score,
        8
    );
    assert_eq!(
        engine
            .view(Viewer::Seat(seat(0)))
            .unwrap()
            .own_player
            .unwrap()
            .true_score,
        10
    );
    let random = engine.state.random.clone();
    let receipt = engine
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(3),
            expected_revision: 0,
            command: Command::EndTurn,
        })
        .unwrap();
    assert_eq!(
        receipt.events,
        vec![
            Event::TurnStarted {
                seat: seat(0),
                turn: 2
            },
            Event::VictoryPointsRevealed {
                winner: seat(0),
                cards: [Some(card(20)), Some(card(23)), None, None, None]
            },
            Event::GameFinished {
                winner: seat(0),
                score: 10
            },
        ]
    );
    assert_eq!(receipt.final_scores, Some([10, 1, 0, 0]));
    assert_eq!(engine.state.position.final_scores, receipt.final_scores);
    assert_eq!(engine.state.random, random);
    conservation(&engine);
    assert_eq!(engine.state.position.held_cards(seat(0)).count(), 0);
    assert_eq!(engine.state.position.held_cards(seat(1)).count(), 1);
    for viewer in [
        Viewer::Spectator,
        Viewer::Seat(seat(0)),
        Viewer::Seat(seat(1)),
        Viewer::Seat(seat(2)),
        Viewer::Seat(seat(3)),
    ] {
        let projected = engine.project_receipt(&receipt, viewer).unwrap();
        assert_eq!(
            projected.events.last(),
            Some(&ProjectedEvent::GameFinished {
                winner: seat(0),
                score: 10
            })
        );
        assert_eq!(engine.view(viewer).unwrap().players[0].public_score, 10);
        assert_eq!(engine.view(viewer).unwrap().players[1].public_score, 0);
    }
    for command in [
        Command::BuyDevelopmentCard,
        Command::EndTurn,
        Command::RollDice,
        Command::BankTrade {
            give_resource: Resource::Wool,
            receive_resource: Resource::Ore,
        },
        Command::UpgradeCity {
            vertex: VertexId::try_from(31).unwrap(),
        },
        Command::BuildSettlement {
            vertex: VertexId::try_from(48).unwrap(),
        },
        Command::BuildRoad {
            edge: EdgeId::try_from(0).unwrap(),
        },
        Command::MoveRobber {
            hex: HexId::try_from(1).unwrap(),
        },
        Command::ChooseRobberVictim { victim: seat(1) },
        Command::DiscardResources {
            resources: Resources::default(),
        },
        Command::PlaceSetupSettlement {
            vertex: VertexId::try_from(0).unwrap(),
        },
        Command::PlaceSetupRoad {
            edge: EdgeId::try_from(0).unwrap(),
        },
    ] {
        reject(&mut engine, seat(0), command, RuleError::GameFinished);
    }
}
