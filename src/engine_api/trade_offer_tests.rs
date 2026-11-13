//! Negotiation actors, public terms, private hands and offer lifetime.
use super::*;
use crate::game_model::CardLocation;
use crate::{GameConfig, TradeOffer};

fn seat(id: u8) -> PlayerId {
    PlayerId::try_from(id).unwrap()
}
fn card(id: u8) -> CardId {
    CardId::try_from(id).unwrap()
}
fn res(counts: [u8; 5]) -> Resources {
    Resources::try_from(counts).unwrap()
}
fn propose(recipient: u8, give: [u8; 5], receive: [u8; 5]) -> Command {
    Command::ProposeTrade {
        recipient: seat(recipient),
        give: res(give),
        receive: res(receive),
    }
}
fn fixture() -> Engine {
    let (mut engine, _) =
        Engine::create(GameConfig::fixed(&[0, 1, 2, 3]).unwrap(), [5; 32]).unwrap();
    let p = &mut engine.state.position;
    p.active_player = seat(0);
    p.phase = Phase::Action;
    p.turn = 2;
    p.hands = [
        res([3, 2, 1, 1, 1]),
        res([1, 0, 0, 2, 1]),
        res([0, 2, 2, 1, 0]),
        res([1, 1, 0, 0, 0]),
    ];
    p.bank = res([14, 14, 16, 15, 17]);
    for (draw, id) in [16, 14, 0, 18].into_iter().enumerate() {
        let index = p
            .deck
            .iter()
            .position(|&candidate| candidate == card(id))
            .unwrap();
        p.deck.swap(draw, index);
        p.card_locations[id as usize] = CardLocation::Held {
            owner: seat(0),
            acquired_turn: 1,
        };
    }
    p.draw_cursor = 4;
    engine
}
fn act(e: &mut Engine, actor: u8, command: Command) -> Receipt {
    e.execute(CommandEnvelope {
        api_version: 1,
        actor: seat(actor),
        expected_revision: e.state.revision,
        command,
    })
    .unwrap()
}
fn reject(e: &mut Engine, actor: u8, command: Command, code: RuleError) {
    let before = e.state.position.clone();
    let random = e.state.random.clone();
    let revision = e.state.revision;
    assert_eq!(e.validate(seat(actor), &command).unwrap_err().code, code);
    assert_eq!(
        e.execute(CommandEnvelope {
            api_version: 1,
            actor: seat(actor),
            expected_revision: revision,
            command
        })
        .unwrap_err()
        .code,
        code
    );
    assert_eq!(e.state.position, before);
    assert_eq!(e.state.random, random);
    assert_eq!(e.state.revision, revision);
}
fn terms() -> Command {
    propose(1, [2, 0, 0, 0, 0], [0, 0, 2, 0, 0])
}
fn expected_offer(
    id: u64,
    proposer: u8,
    recipient: u8,
    give: [u8; 5],
    receive: [u8; 5],
) -> TradeOffer {
    TradeOffer {
        id,
        turn: 2,
        proposer: seat(proposer),
        recipient: seat(recipient),
        give: res(give),
        receive: res(receive),
    }
}

#[test]
fn base_rules_br021_proposal_counteroffer_and_authorized_close_are_atomic_public_facts() {
    let mut e = fixture();
    let position = e.state.position.clone();
    let random = e.state.random.clone();
    // Recipient has no wool; proposal checks only the proposer and does not escrow.
    assert!(e.validate(seat(0), &terms()).is_ok());
    let offer = expected_offer(1, 0, 1, [2, 0, 0, 0, 0], [0, 0, 2, 0, 0]);
    let first = act(&mut e, 0, terms());
    assert_eq!(first.events, vec![Event::TradeOfferCreated { offer }]);
    assert_eq!(e.state.position.hands, position.hands);
    assert_eq!(e.state.position.bank, position.bank);
    assert_eq!(e.state.random, random);
    assert_eq!(e.state.position.offer, Some(offer));
    for viewer in [
        Viewer::Spectator,
        Viewer::Seat(seat(0)),
        Viewer::Seat(seat(1)),
        Viewer::Seat(seat(2)),
        Viewer::Seat(seat(3)),
    ] {
        assert_eq!(e.view(viewer).unwrap().offer, Some(offer));
        assert_eq!(
            e.project_receipt(&first, viewer).unwrap(),
            ProjectedReceipt {
                revision: 1,
                events: vec![ProjectedEvent::TradeOfferCreated { offer }]
            }
        );
    }
    assert!(
        e.legal_actions(seat(0))
            .unwrap()
            .choices
            .contains(&LegalAction::ProposeTrade {
                recipients: vec![seat(1), seat(2), seat(3)]
            })
    );
    assert!(
        e.legal_actions(seat(1))
            .unwrap()
            .choices
            .contains(&LegalAction::ProposeTrade {
                recipients: vec![seat(0)]
            })
    );
    assert!(
        !e.legal_actions(seat(2))
            .unwrap()
            .choices
            .iter()
            .any(|choice| matches!(choice, LegalAction::ProposeTrade { .. }))
    );
    assert!(
        e.legal_actions(seat(1))
            .unwrap()
            .choices
            .contains(&LegalAction::RejectTrade { offer_id: 1 })
    );
    assert!(
        e.legal_actions(seat(0))
            .unwrap()
            .choices
            .contains(&LegalAction::CancelTrade { offer_id: 1 })
    );
    reject(
        &mut e,
        2,
        propose(0, [0, 0, 1, 0, 0], [1, 0, 0, 0, 0]),
        RuleError::OfferBusy,
    );
    reject(
        &mut e,
        0,
        Command::RejectTrade { offer_id: 1 },
        RuleError::WrongActor,
    );
    reject(
        &mut e,
        1,
        Command::CancelTrade { offer_id: 1 },
        RuleError::WrongActor,
    );
    let counter = propose(0, [0, 0, 0, 1, 0], [1, 0, 0, 0, 0]);
    let replacement = expected_offer(2, 1, 0, [0, 0, 0, 1, 0], [1, 0, 0, 0, 0]);
    let second = act(&mut e, 1, counter);
    assert_eq!(
        second.events,
        vec![
            Event::TradeOfferClosed { offer_id: 1 },
            Event::TradeOfferCreated { offer: replacement }
        ]
    );
    assert_eq!(e.state.position.offer, Some(replacement));
    reject(
        &mut e,
        0,
        Command::RejectTrade { offer_id: 1 },
        RuleError::OfferNotFound,
    );
    let third = act(&mut e, 0, Command::RejectTrade { offer_id: 2 });
    assert_eq!(third.events, vec![Event::TradeOfferClosed { offer_id: 2 }]);
    assert_eq!(e.state.position.offer, None);
    assert_eq!(
        e.project_receipt(&first, Viewer::Spectator).unwrap(),
        ProjectedReceipt {
            revision: 1,
            events: vec![ProjectedEvent::TradeOfferCreated { offer }]
        }
    );
    assert_eq!(e.state.position.hands, position.hands);
    assert_eq!(e.state.random, random);
    let fourth = act(&mut e, 1, propose(0, [0, 0, 0, 1, 0], [1, 0, 0, 0, 0]));
    assert_eq!(fourth.revision, 4);
    assert_eq!(
        act(&mut e, 1, Command::CancelTrade { offer_id: 4 }).events,
        vec![Event::TradeOfferClosed { offer_id: 4 }]
    );
}

#[test]
fn base_rules_br021_invalid_actors_terms_and_revisions_reject_without_any_state_change() {
    let mut e = fixture();
    for cmd in [
        propose(0, [1, 0, 0, 0, 0], [0, 1, 0, 0, 0]),
        propose(1, [0; 5], [0, 1, 0, 0, 0]),
        propose(1, [1, 0, 0, 0, 0], [0; 5]),
        propose(1, [1, 0, 0, 0, 0], [1, 1, 0, 0, 0]),
    ] {
        reject(&mut e, 0, cmd, RuleError::InvalidTrade);
    }
    reject(
        &mut e,
        0,
        propose(1, [4, 0, 0, 0, 0], [0, 1, 0, 0, 0]),
        RuleError::InsufficientResources,
    );
    reject(
        &mut e,
        1,
        propose(2, [1, 0, 0, 0, 0], [0, 1, 0, 0, 0]),
        RuleError::WrongActor,
    );
    reject(
        &mut e,
        0,
        Command::RejectTrade { offer_id: 1 },
        RuleError::OfferNotFound,
    );
    e.state.position.phase = Phase::PreRoll;
    reject(&mut e, 0, terms(), RuleError::WrongPhase);
    e.state.position.phase = Phase::Action;
    let before = e.state.position.clone();
    let random = e.state.random.clone();
    let revision = e.state.revision;
    assert_eq!(
        e.execute(CommandEnvelope {
            api_version: 1,
            actor: seat(0),
            expected_revision: revision + 1,
            command: terms()
        })
        .unwrap_err()
        .code,
        RuleError::StaleRevision
    );
    assert_eq!(e.state.position, before);
    assert_eq!(e.state.random, random);
    assert_eq!(e.state.revision, revision);
    e.state.revision = u64::MAX;
    reject(&mut e, 0, terms(), RuleError::CounterExhausted);
    let mut three = fixture();
    three.state.position.config = GameConfig::fixed(&[0, 1, 2]).unwrap();
    reject(
        &mut three,
        0,
        propose(3, [1, 0, 0, 0, 0], [0, 1, 0, 0, 0]),
        RuleError::InvalidSeat(3),
    );
}

#[test]
fn privacy_trade_proposal_validation_and_candidates_do_not_reveal_counterpart_hands() {
    let mut a = fixture();
    let mut b = fixture();
    b.state.position.hands[1] = res([0, 0, 1, 2, 1]);
    b.state.position.hands[2] = res([1, 2, 1, 1, 0]);
    let spectator_a = format!("{:?}", a.view(Viewer::Spectator).unwrap());
    assert_eq!(
        spectator_a,
        format!("{:?}", b.view(Viewer::Spectator).unwrap())
    );
    assert_eq!(a.validate(seat(0), &terms()), b.validate(seat(0), &terms()));
    assert_eq!(
        a.legal_actions(seat(0)).unwrap(),
        b.legal_actions(seat(0)).unwrap()
    );
    let receipt_a = act(&mut a, 0, terms());
    let receipt_b = act(&mut b, 0, terms());
    assert_eq!(
        a.project_receipt(&receipt_a, Viewer::Spectator).unwrap(),
        b.project_receipt(&receipt_b, Viewer::Spectator).unwrap()
    );
    assert_eq!(
        a.legal_actions(seat(0)).unwrap(),
        b.legal_actions(seat(0)).unwrap()
    );
    assert_ne!(
        a.view(Viewer::Seat(seat(1))).unwrap().own_player,
        b.view(Viewer::Seat(seat(1))).unwrap().own_player
    );
}

#[test]
fn offer_lifetime_hand_changes_zero_monopoly_and_turn_end() {
    let mut e = fixture();
    let before = e.state.position.hands;
    act(&mut e, 0, terms());
    let receipt = act(
        &mut e,
        0,
        Command::PlayInvention {
            card: card(18),
            resources: res([0, 0, 2, 0, 0]),
        },
    );
    assert_eq!(
        receipt.events,
        vec![
            Event::InventionPlayed {
                seat: seat(0),
                card: card(18),
                resources: res([0, 0, 2, 0, 0])
            },
            Event::TradeOfferClosed { offer_id: 1 }
        ]
    );
    assert_eq!(e.state.position.offer, None);
    assert_ne!(e.state.position.hands[0], before[0]);
    let mut zero = fixture();
    zero.state.position.hands = [
        res([3, 2, 1, 1, 1]),
        Resources::default(),
        Resources::default(),
        Resources::default(),
    ];
    zero.state.position.bank = res([16, 17, 18, 18, 18]);
    act(&mut zero, 0, propose(1, [1, 0, 0, 0, 0], [0, 0, 0, 0, 1]));
    let prior = zero.state.position.hands;
    let random = zero.state.random.clone();
    let effect = act(
        &mut zero,
        0,
        Command::PlayMonopoly {
            card: card(16),
            resource: Resource::Ore,
        },
    );
    assert_eq!(
        effect.events,
        vec![Event::MonopolyPlayed {
            seat: seat(0),
            card: card(16),
            resource: Resource::Ore,
            transfers: [0; 4]
        }]
    );
    assert_eq!(zero.state.position.hands, prior);
    assert_eq!(zero.state.random, random);
    assert!(zero.state.position.offer.is_some());
    let end = act(&mut zero, 0, Command::EndTurn);
    assert!(matches!(
        end.events.as_slice(),
        [
            Event::TradeOfferClosed { offer_id: 1 },
            Event::TurnStarted { .. },
            Event::PhaseChanged { .. }
        ]
    ));
    assert_eq!(zero.state.position.offer, None);
}

#[test]
fn offer_lifetime_knight_and_road_effect_close_before_award_or_phase_change() {
    let mut road = fixture();
    act(&mut road, 0, terms());
    let r = act(&mut road, 0, Command::PlayRoadBuilding { card: card(14) });
    assert!(matches!(
        r.events.as_slice(),
        [
            Event::RoadBuildingPlayed { .. },
            Event::TradeOfferClosed { offer_id: 1 },
            Event::PhaseChanged { .. }
        ]
    ));
    assert_eq!(road.state.position.offer, None);
    let mut knight = fixture();
    for (draw, id) in [(4, 1), (5, 2)] {
        let index = knight
            .state
            .position
            .deck
            .iter()
            .position(|&c| c == card(id))
            .unwrap();
        knight.state.position.deck.swap(draw, index);
        knight.state.position.card_locations[id as usize] =
            CardLocation::PlayedKnight { owner: seat(0) };
    }
    knight.state.position.draw_cursor = 6;
    act(&mut knight, 0, terms());
    let r = act(&mut knight, 0, Command::PlayKnight { card: card(0) });
    assert!(matches!(
        r.events.as_slice(),
        [
            Event::KnightPlayed { .. },
            Event::TradeOfferClosed { offer_id: 1 },
            Event::LargestArmyChanged { .. },
            Event::PhaseChanged { .. }
        ]
    ));
    assert_eq!(knight.state.position.offer, None);
}

#[test]
fn offer_lifetime_paid_actions_and_invalid_commands_keep_exact_order_and_rng() {
    for kind in 0..3 {
        let mut e = fixture();
        e.state.position.hands[0] = res([4, 2, 1, 2, 3]);
        e.state.position.bank = res([13, 14, 16, 14, 15]);
        act(&mut e, 0, terms());
        let random = e.state.random.clone();
        reject(
            &mut e,
            0,
            Command::BuildRoad {
                edge: EdgeId::try_from(71).unwrap(),
            },
            RuleError::Disconnected,
        );
        assert!(e.state.position.offer.is_some());
        let command = match kind {
            0 => Command::BankTrade {
                give_resource: Resource::Brick,
                receive_resource: Resource::Ore,
            },
            1 => Command::BuyDevelopmentCard,
            _ => {
                let edge = e
                    .legal_actions(seat(0))
                    .unwrap()
                    .choices
                    .into_iter()
                    .find_map(|choice| match choice {
                        LegalAction::BuildRoad { edges } => edges.first().copied(),
                        _ => None,
                    })
                    .unwrap();
                Command::BuildRoad { edge }
            }
        };
        let receipt = act(&mut e, 0, command);
        assert_eq!(e.state.position.offer, None);
        assert_eq!(e.state.random, random);
        let close = receipt
            .events
            .iter()
            .position(|event| matches!(event, Event::TradeOfferClosed { offer_id: 1 }))
            .unwrap();
        assert!(close > 0);
        assert!(receipt.events[..close].iter().all(|event| !matches!(
            event,
            Event::LongestRouteChanged { .. }
                | Event::LargestArmyChanged { .. }
                | Event::GameFinished { .. }
                | Event::PhaseChanged { .. }
        )));
        assert!(
            !receipt.events[close + 1..]
                .iter()
                .any(|event| matches!(event, Event::TradeOfferClosed { .. }))
        );
    }
}

#[test]
fn offer_lifetime_winning_knight_closes_before_award_and_game_finished() {
    use crate::{Building, BuildingKind};
    let mut e = fixture();
    e.state.position.buildings = [None; 54];
    e.state.position.roads = [None; 72];
    e.state.position.road_lengths = [0; 4];
    e.state.position.longest_route_holder = None;
    for id in [0, 9, 17, 31] {
        e.state.position.buildings[id] = Some(Building {
            owner: seat(0),
            kind: BuildingKind::City,
        });
    }
    for (draw, id) in [(4, 1), (5, 2)] {
        let index = e
            .state
            .position
            .deck
            .iter()
            .position(|&c| c == card(id))
            .unwrap();
        e.state.position.deck.swap(draw, index);
        e.state.position.card_locations[id as usize] =
            CardLocation::PlayedKnight { owner: seat(0) };
    }
    e.state.position.draw_cursor = 6;
    act(&mut e, 0, terms());
    let random = e.state.random.clone();
    let receipt = act(&mut e, 0, Command::PlayKnight { card: card(0) });
    assert_eq!(
        receipt.events,
        vec![
            Event::KnightPlayed {
                seat: seat(0),
                card: card(0)
            },
            Event::TradeOfferClosed { offer_id: 1 },
            Event::LargestArmyChanged {
                previous: None,
                holder: Some(seat(0))
            },
            Event::GameFinished {
                winner: seat(0),
                score: 10
            }
        ]
    );
    assert_eq!(e.state.position.offer, None);
    assert_eq!(
        e.state.position.phase,
        Phase::Finished {
            winner: seat(0),
            score: 10
        }
    );
    assert_eq!(e.state.random, random);
}

#[test]
fn privacy_trade_input_actor_and_phase_error_precedence_is_atomic() {
    let (mut e, _) = Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), [5; 32]).unwrap();
    let invalid = propose(3, [1, 0, 0, 0, 0], [0, 1, 0, 0, 0]);
    let before = e.state.position.clone();
    let random = e.state.random.clone();
    let revision = e.state.revision;
    assert_eq!(
        e.validate(seat(0), &invalid).unwrap_err().code,
        RuleError::InvalidSeat(3)
    );
    assert_eq!(
        e.execute(CommandEnvelope {
            api_version: 1,
            actor: seat(0),
            expected_revision: revision + 1,
            command: invalid
        })
        .unwrap_err()
        .code,
        RuleError::InvalidSeat(3)
    );
    assert_eq!(e.state.position, before);
    assert_eq!(e.state.random, random);
    assert_eq!(e.state.revision, revision);
    e.state.position.phase = Phase::Finished {
        winner: seat(0),
        score: 10,
    };
    reject(&mut e, 0, invalid, RuleError::InvalidSeat(3));
    let mut e = fixture();
    e.state.position.phase = Phase::PreRoll;
    reject(
        &mut e,
        0,
        propose(0, [1, 0, 0, 0, 0], [0, 1, 0, 0, 0]),
        RuleError::WrongPhase,
    );
    reject(
        &mut e,
        1,
        propose(1, [1, 0, 0, 0, 0], [0, 1, 0, 0, 0]),
        RuleError::WrongActor,
    );
}

#[test]
fn offer_lifetime_nonzero_monopoly_closes_after_public_transfer() {
    let mut e = fixture();
    let bank = e.state.position.bank;
    let random = e.state.random.clone();
    act(&mut e, 0, terms());
    let r = act(
        &mut e,
        0,
        Command::PlayMonopoly {
            card: card(16),
            resource: Resource::Brick,
        },
    );
    assert_eq!(
        r.events,
        vec![
            Event::MonopolyPlayed {
                seat: seat(0),
                card: card(16),
                resource: Resource::Brick,
                transfers: [0, 1, 0, 1]
            },
            Event::TradeOfferClosed { offer_id: 1 }
        ]
    );
    assert_eq!(
        e.state.position.hands.map(|hand| *hand.counts()),
        [
            [5, 2, 1, 1, 1],
            [0, 0, 0, 2, 1],
            [0, 2, 2, 1, 0],
            [0, 1, 0, 0, 0]
        ]
    );
    assert_eq!(e.state.position.offer, None);
    assert_eq!(e.state.position.bank, bank);
    assert_eq!(e.state.random, random);
}

#[test]
fn offer_lifetime_winning_paid_road_closes_before_route_award_and_finish() {
    use crate::{Building, BuildingKind};
    let mut e = fixture();
    e.state.position.buildings = [None; 54];
    e.state.position.roads = [None; 72];
    for (vertex, kind) in [
        (0, BuildingKind::City),
        (9, BuildingKind::City),
        (17, BuildingKind::City),
        (31, BuildingKind::Settlement),
        (40, BuildingKind::Settlement),
    ] {
        e.state.position.buildings[vertex] = Some(Building {
            owner: seat(0),
            kind,
        });
    }
    for id in [0, 6, 10, 18, 14, 25, 46, 57] {
        e.state.position.roads[id] = Some(seat(0));
    }
    e.state.position.road_lengths = [4, 0, 0, 0];
    e.state.position.longest_route_holder = None;
    act(&mut e, 0, terms());
    let random = e.state.random.clone();
    let r = act(
        &mut e,
        0,
        Command::BuildRoad {
            edge: EdgeId::try_from(23).unwrap(),
        },
    );
    assert_eq!(
        r.events,
        vec![
            Event::ConstructionPaid {
                seat: seat(0),
                resources: res([1, 1, 0, 0, 0])
            },
            Event::RoadPlaced {
                seat: seat(0),
                edge: EdgeId::try_from(23).unwrap()
            },
            Event::TradeOfferClosed { offer_id: 1 },
            Event::LongestRouteChanged {
                previous: None,
                holder: Some(seat(0))
            },
            Event::GameFinished {
                winner: seat(0),
                score: 10
            }
        ]
    );
    assert_eq!(e.state.position.offer, None);
    assert_eq!(e.state.random, random);
}
