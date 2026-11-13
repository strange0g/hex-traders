//! Explicit acceptance, checked exchange and recipient-scoped privacy.
use super::*;
use crate::GameConfig;
fn seat(id: u8) -> PlayerId {
    PlayerId::try_from(id).unwrap()
}
fn res(counts: [u8; 5]) -> Resources {
    Resources::try_from(counts).unwrap()
}
fn fixture() -> Engine {
    let (mut e, _) = Engine::create(GameConfig::fixed(&[0, 1, 2, 3]).unwrap(), [5; 32]).unwrap();
    e.state.position.active_player = seat(0);
    e.state.position.phase = Phase::Action;
    e.state.position.turn = 2;
    e.state.position.hands = [
        res([3, 2, 1, 1, 1]),
        res([1, 0, 0, 2, 1]),
        res([0, 2, 2, 1, 0]),
        res([1, 1, 0, 0, 0]),
    ];
    e.state.position.bank = res([14, 14, 16, 15, 17]);
    e
}
fn offer(recipient: u8, give: [u8; 5], receive: [u8; 5]) -> Command {
    Command::ProposeTrade {
        recipient: seat(recipient),
        give: res(give),
        receive: res(receive),
    }
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
#[test]
fn base_rules_br022_trade_accept_both_actor_orientations_conserves_and_projects_original_facts() {
    for active_proposes in [true, false] {
        let mut e = fixture();
        let random = e.state.random.clone();
        let bank = e.state.position.bank;
        let (proposer, recipient, command) = if active_proposes {
            (0, 1, offer(1, [2, 0, 0, 0, 0], [0, 0, 0, 1, 0]))
        } else {
            (1, 0, offer(0, [0, 0, 0, 1, 0], [2, 0, 0, 0, 0]))
        };
        act(&mut e, proposer, command);
        let original = e.state.position.offer.unwrap();
        let accept = Command::AcceptTrade {
            offer_id: original.id,
        };
        assert!(e.validate(seat(recipient), &accept).is_ok());
        assert!(e.legal_actions(seat(recipient)).unwrap().choices.contains(
            &LegalAction::AcceptTrade {
                offer_id: original.id
            }
        ));
        assert!(!e.legal_actions(seat(proposer)).unwrap().choices.contains(
            &LegalAction::AcceptTrade {
                offer_id: original.id
            }
        ));
        let receipt = act(&mut e, recipient, accept);
        assert_eq!((receipt.previous_revision, receipt.revision), (1, 2));
        assert_eq!(
            receipt.events,
            vec![
                Event::TradeCompleted { offer: original },
                Event::TradeOfferClosed {
                    offer_id: original.id
                }
            ]
        );
        assert_eq!(
            e.state.position.hands.map(|hand| *hand.counts()),
            [
                [1, 2, 1, 2, 1],
                [3, 0, 0, 1, 1],
                [0, 2, 2, 1, 0],
                [1, 1, 0, 0, 0]
            ]
        );
        assert_eq!(e.state.position.bank, bank);
        assert_eq!(e.state.random, random);
        assert_eq!(e.state.position.offer, None);
        assert_eq!(e.state.position.phase, Phase::Action);
        for resource in 0..5 {
            assert_eq!(
                u16::from(bank.counts()[resource])
                    + e.state
                        .position
                        .hands
                        .iter()
                        .map(|hand| u16::from(hand.counts()[resource]))
                        .sum::<u16>(),
                19
            );
        }
        for viewer in [
            Viewer::Spectator,
            Viewer::Seat(seat(0)),
            Viewer::Seat(seat(1)),
            Viewer::Seat(seat(2)),
            Viewer::Seat(seat(3)),
        ] {
            assert_eq!(
                e.project_receipt(&receipt, viewer).unwrap(),
                ProjectedReceipt {
                    revision: 2,
                    events: vec![
                        ProjectedEvent::TradeCompleted { offer: original },
                        ProjectedEvent::TradeOfferClosed {
                            offer_id: original.id
                        }
                    ]
                }
            );
        }
        act(&mut e, 0, Command::EndTurn);
        assert_eq!(
            e.project_receipt(&receipt, Viewer::Spectator).unwrap(),
            ProjectedReceipt {
                revision: 2,
                events: vec![
                    ProjectedEvent::TradeCompleted { offer: original },
                    ProjectedEvent::TradeOfferClosed {
                        offer_id: original.id
                    }
                ]
            }
        );
    }
}
#[test]
fn base_rules_br022_trade_accept_rejection_and_duplicate_response_never_transfer_twice() {
    let mut e = fixture();
    act(&mut e, 0, offer(1, [2, 0, 0, 0, 0], [0, 0, 0, 1, 0]));
    reject(
        &mut e,
        0,
        Command::AcceptTrade { offer_id: 1 },
        RuleError::WrongActor,
    );
    reject(
        &mut e,
        0,
        Command::AcceptTrade { offer_id: 2 },
        RuleError::WrongActor,
    );
    reject(
        &mut e,
        2,
        Command::AcceptTrade { offer_id: 1 },
        RuleError::WrongActor,
    );
    reject(
        &mut e,
        1,
        Command::AcceptTrade { offer_id: 2 },
        RuleError::OfferNotFound,
    );
    let before = e.state.position.clone();
    let random = e.state.random.clone();
    let revision = e.state.revision;
    assert_eq!(
        e.execute(CommandEnvelope {
            api_version: 1,
            actor: seat(1),
            expected_revision: revision - 1,
            command: Command::AcceptTrade { offer_id: 1 }
        })
        .unwrap_err()
        .code,
        RuleError::StaleRevision
    );
    assert_eq!(e.state.position, before);
    assert_eq!(e.state.random, random);
    assert_eq!(e.state.revision, revision);
    act(&mut e, 1, Command::AcceptTrade { offer_id: 1 });
    let before = e.state.position.clone();
    let random = e.state.random.clone();
    let revision = e.state.revision;
    assert_eq!(
        e.execute(CommandEnvelope {
            api_version: 1,
            actor: seat(1),
            expected_revision: 1,
            command: Command::AcceptTrade { offer_id: 1 }
        })
        .unwrap_err()
        .code,
        RuleError::StaleRevision
    );
    assert_eq!(e.state.position, before);
    assert_eq!(e.state.random, random);
    assert_eq!(e.state.revision, revision);
    reject(
        &mut e,
        1,
        Command::AcceptTrade { offer_id: 1 },
        RuleError::OfferNotFound,
    );
    let mut replaced = fixture();
    act(&mut replaced, 0, offer(1, [2, 0, 0, 0, 0], [0, 0, 0, 1, 0]));
    act(&mut replaced, 0, offer(1, [1, 0, 0, 0, 0], [0, 0, 0, 1, 0]));
    reject(
        &mut replaced,
        1,
        Command::AcceptTrade { offer_id: 1 },
        RuleError::OfferNotFound,
    );
}
#[test]
fn privacy_trade_accept_checks_only_own_hand_in_queries_and_privately_rechecks_both_in_execution() {
    let mut lacking = fixture();
    act(&mut lacking, 0, offer(1, [2, 0, 0, 0, 0], [0, 0, 2, 0, 0]));
    reject(
        &mut lacking,
        1,
        Command::AcceptTrade { offer_id: 1 },
        RuleError::OfferUnavailable,
    );
    assert!(
        !lacking
            .legal_actions(seat(1))
            .unwrap()
            .choices
            .contains(&LegalAction::AcceptTrade { offer_id: 1 })
    );
    let mut good = fixture();
    let mut malformed = fixture();
    for e in [&mut good, &mut malformed] {
        act(e, 0, offer(1, [2, 0, 0, 0, 0], [0, 0, 0, 1, 0]));
    }
    // An intentionally impossible retained offer: only private cards change, totals stay public-equivalent.
    malformed.state.position.hands[0] = res([1, 2, 3, 1, 1]);
    malformed.state.position.hands[2] = res([2, 2, 0, 1, 0]);
    assert_eq!(
        format!("{:?}", good.view(Viewer::Spectator).unwrap()),
        format!("{:?}", malformed.view(Viewer::Spectator).unwrap())
    );
    let command = Command::AcceptTrade { offer_id: 1 };
    assert_eq!(
        good.validate(seat(1), &command),
        malformed.validate(seat(1), &command)
    );
    assert_eq!(
        good.legal_actions(seat(1)).unwrap(),
        malformed.legal_actions(seat(1)).unwrap()
    );
    let before = malformed.state.position.clone();
    let random = malformed.state.random.clone();
    let revision = malformed.state.revision;
    assert_eq!(
        malformed
            .execute(CommandEnvelope {
                api_version: 1,
                actor: seat(1),
                expected_revision: revision,
                command
            })
            .unwrap_err()
            .code,
        RuleError::OfferUnavailable
    );
    assert_eq!(malformed.state.position, before);
    assert_eq!(malformed.state.random, random);
    assert_eq!(malformed.state.revision, revision);
    let accepted = act(&mut good, 1, command);
    assert_eq!(accepted.revision, 2);
}

#[test]
fn base_rules_br022_stale_offer_turn_wrong_phase_and_counter_limit_are_atomic() {
    let mut e = fixture();
    act(&mut e, 0, offer(1, [2, 0, 0, 0, 0], [0, 0, 0, 1, 0]));
    e.state.position.phase = Phase::PreRoll;
    reject(
        &mut e,
        1,
        Command::AcceptTrade { offer_id: 1 },
        RuleError::WrongPhase,
    );
    e.state.position.phase = Phase::Action;
    e.state.position.turn += 1;
    reject(
        &mut e,
        1,
        Command::AcceptTrade { offer_id: 1 },
        RuleError::OfferNotFound,
    );
    assert!(
        !e.legal_actions(seat(1))
            .unwrap()
            .choices
            .contains(&LegalAction::AcceptTrade { offer_id: 1 })
    );
    e.state.position.turn -= 1;
    e.state.revision = u64::MAX;
    reject(
        &mut e,
        1,
        Command::AcceptTrade { offer_id: 1 },
        RuleError::CounterExhausted,
    );
    assert!(
        !e.legal_actions(seat(1))
            .unwrap()
            .choices
            .contains(&LegalAction::AcceptTrade { offer_id: 1 })
    );
}

#[test]
fn base_rules_br022_accept_rechecks_public_offer_terms_without_consuming_it() {
    for (give, receive) in [
        ([0, 0, 0, 0, 0], [0, 0, 0, 1, 0]),
        ([1, 0, 0, 0, 0], [1, 0, 0, 0, 0]),
    ] {
        let mut e = fixture();
        act(&mut e, 0, offer(1, [2, 0, 0, 0, 0], [0, 0, 0, 1, 0]));
        let current = e.state.position.offer.as_mut().unwrap();
        current.give = res(give);
        current.receive = res(receive);
        reject(
            &mut e,
            1,
            Command::AcceptTrade { offer_id: 1 },
            RuleError::OfferUnavailable,
        );
        assert!(
            !e.legal_actions(seat(1))
                .unwrap()
                .choices
                .contains(&LegalAction::AcceptTrade { offer_id: 1 })
        );
    }
}
