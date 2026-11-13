//! Seeded public negotiation histories using independent C06 source/RNG fixtures.
use super::seat;
use hex_traders_engine::{
    CardId, Command, CommandEnvelope, Engine, Event, GameConfig, LegalAction, Phase,
    ProjectedEvent, ProjectedReceipt, Receipt, Resources, RuleError, TradeOffer, Viewer,
};
fn res(counts: [u8; 5]) -> Resources {
    Resources::try_from(counts).unwrap()
}
fn act(e: &mut Engine, actor: u8, command: Command) -> Receipt {
    let revision = e.view(Viewer::Spectator).unwrap().revision;
    assert!(e.validate(seat(actor), &command).is_ok());
    let receipt = e
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(actor),
            expected_revision: revision,
            command,
        })
        .unwrap();
    assert_eq!(
        (receipt.previous_revision, receipt.revision),
        (revision, revision + 1)
    );
    receipt
}
fn propose(recipient: u8, give: [u8; 5], receive: [u8; 5]) -> Command {
    Command::ProposeTrade {
        recipient: seat(recipient),
        give: res(give),
        receive: res(receive),
    }
}
fn hands(e: &Engine, expected: &[[u8; 5]]) {
    for (id, counts) in expected.iter().enumerate() {
        assert_eq!(
            e.view(Viewer::Seat(seat(id as u8)))
                .unwrap()
                .own_player
                .unwrap()
                .resources
                .counts(),
            counts
        );
    }
}
fn project(e: &Engine, receipt: &Receipt, seats: &[u8], events: Vec<ProjectedEvent>) {
    for viewer in
        std::iter::once(Viewer::Spectator).chain(seats.iter().map(|&id| Viewer::Seat(seat(id))))
    {
        assert_eq!(
            e.project_receipt(receipt, viewer).unwrap(),
            ProjectedReceipt {
                revision: receipt.revision,
                events: events.clone()
            }
        );
    }
}
fn offer_views(e: &Engine, seats: &[u8], expected: Option<TradeOffer>) {
    for viewer in
        std::iter::once(Viewer::Spectator).chain(seats.iter().map(|&id| Viewer::Seat(seat(id))))
    {
        assert_eq!(e.view(viewer).unwrap().offer, expected);
    }
}
struct Fixture<'a> {
    seats: &'a [u8],
    rolls: &'a [(u8, [u8; 2])],
    active: u8,
    recipient: u8,
    other: u8,
    received: [u8; 5],
    before: &'a [[u8; 5]],
    bank: [u8; 5],
    play_dice: [u8; 2],
    following: [u8; 2],
}
#[test]
fn trade_offer_public_active_inactive_counteroffer_and_close_preserve_hands_and_rng() {
    let fixtures = [
        Fixture {
            seats: &[0, 1, 2],
            rolls: &[(0, [2, 4]), (1, [1, 4]), (2, [1, 4]), (0, [2, 1])],
            active: 1,
            recipient: 0,
            other: 2,
            received: [0, 0, 0, 0, 1],
            before: &[[1, 1, 2, 2, 1], [0, 1, 0, 1, 0], [2, 0, 2, 3, 0]],
            bank: [16, 17, 15, 13, 18],
            play_dice: [2, 6],
            following: [5, 6],
        },
        Fixture {
            seats: &[0, 1, 2, 3],
            rolls: &[(0, [1, 4]), (1, [1, 4]), (2, [2, 1]), (3, [2, 6])],
            active: 0,
            recipient: 1,
            other: 2,
            received: [0, 0, 0, 1, 0],
            before: &[
                [1, 2, 1, 1, 0],
                [0, 1, 2, 2, 0],
                [1, 1, 2, 3, 0],
                [2, 1, 0, 3, 0],
            ],
            bank: [15, 14, 14, 10, 19],
            play_dice: [5, 6],
            following: [5, 1],
        },
    ];
    for fixture in fixtures {
        let mut seed = [0; 32];
        seed[0] = 8;
        let (mut e, _) = Engine::create(GameConfig::fixed(fixture.seats).unwrap(), seed).unwrap();
        for &(actor, dice) in fixture.rolls {
            assert_eq!(
                act(&mut e, actor, Command::RollDice).events[0],
                Event::DiceRolled {
                    seat: seat(actor),
                    dice
                }
            );
            if actor == fixture.active {
                let purchase = act(&mut e, actor, Command::BuyDevelopmentCard);
                assert!(purchase.events.contains(&Event::DevelopmentPurchased {
                    seat: seat(actor),
                    card: CardId::try_from(18).unwrap(),
                    remaining: 24
                }));
            }
            act(&mut e, actor, Command::EndTurn);
        }
        assert_eq!(
            act(&mut e, fixture.active, Command::RollDice).events[0],
            Event::DiceRolled {
                seat: seat(fixture.active),
                dice: fixture.play_dice
            }
        );
        let before = e.view(Viewer::Spectator).unwrap();
        assert_eq!(
            (before.active_player, before.phase),
            (seat(fixture.active), Phase::Action)
        );
        assert_eq!(before.bank.counts(), &fixture.bank);
        hands(&e, fixture.before);
        let wood = [0, 1, 0, 0, 0];
        let first = act(
            &mut e,
            fixture.active,
            propose(fixture.recipient, wood, fixture.received),
        );
        let first_offer = TradeOffer {
            id: first.revision,
            turn: 5,
            proposer: seat(fixture.active),
            recipient: seat(fixture.recipient),
            give: res(wood),
            receive: res(fixture.received),
        };
        assert_eq!(
            first.events,
            vec![Event::TradeOfferCreated { offer: first_offer }]
        );
        offer_views(&e, fixture.seats, Some(first_offer));
        project(
            &e,
            &first,
            fixture.seats,
            vec![ProjectedEvent::TradeOfferCreated { offer: first_offer }],
        );
        assert!(
            e.legal_actions(seat(fixture.recipient))
                .unwrap()
                .choices
                .contains(&LegalAction::RejectTrade {
                    offer_id: first.revision
                })
        );
        let revision = e.view(Viewer::Spectator).unwrap().revision;
        assert_eq!(
            e.execute(CommandEnvelope {
                api_version: 1,
                actor: seat(fixture.other),
                expected_revision: revision,
                command: propose(fixture.active, [1, 0, 0, 0, 0], wood)
            })
            .unwrap_err()
            .code,
            RuleError::OfferBusy
        );
        assert_eq!(e.view(Viewer::Spectator).unwrap().revision, revision);
        hands(&e, fixture.before);
        let counter = act(
            &mut e,
            fixture.recipient,
            propose(fixture.active, fixture.received, wood),
        );
        let counter_offer = TradeOffer {
            id: counter.revision,
            turn: 5,
            proposer: seat(fixture.recipient),
            recipient: seat(fixture.active),
            give: res(fixture.received),
            receive: res(wood),
        };
        assert_eq!(
            counter.events,
            vec![
                Event::TradeOfferClosed {
                    offer_id: first.revision
                },
                Event::TradeOfferCreated {
                    offer: counter_offer
                }
            ]
        );
        offer_views(&e, fixture.seats, Some(counter_offer));
        project(
            &e,
            &counter,
            fixture.seats,
            vec![
                ProjectedEvent::TradeOfferClosed {
                    offer_id: first.revision,
                },
                ProjectedEvent::TradeOfferCreated {
                    offer: counter_offer,
                },
            ],
        );
        assert_eq!(
            e.validate(
                seat(fixture.active),
                &Command::RejectTrade {
                    offer_id: first.revision
                }
            )
            .unwrap_err()
            .code,
            RuleError::OfferNotFound
        );
        let rejected = act(
            &mut e,
            fixture.active,
            Command::RejectTrade {
                offer_id: counter.revision,
            },
        );
        assert_eq!(
            rejected.events,
            vec![Event::TradeOfferClosed {
                offer_id: counter.revision
            }]
        );
        let inactive = act(
            &mut e,
            fixture.recipient,
            propose(fixture.active, fixture.received, wood),
        );
        let inactive_offer = TradeOffer {
            id: inactive.revision,
            ..counter_offer
        };
        assert_eq!(
            inactive.events,
            vec![Event::TradeOfferCreated {
                offer: inactive_offer
            }]
        );
        offer_views(&e, fixture.seats, Some(inactive_offer));
        assert!(
            e.legal_actions(seat(fixture.recipient))
                .unwrap()
                .choices
                .contains(&LegalAction::CancelTrade {
                    offer_id: inactive.revision
                })
        );
        let cancelled = act(
            &mut e,
            fixture.recipient,
            Command::CancelTrade {
                offer_id: inactive.revision,
            },
        );
        assert_eq!(
            cancelled.events,
            vec![Event::TradeOfferClosed {
                offer_id: inactive.revision
            }]
        );
        let source = act(
            &mut e,
            fixture.active,
            propose(fixture.recipient, wood, fixture.received),
        );
        let source_offer = TradeOffer {
            id: source.revision,
            ..first_offer
        };
        assert_eq!(
            source.events,
            vec![Event::TradeOfferCreated {
                offer: source_offer
            }]
        );
        let replaced = act(
            &mut e,
            fixture.active,
            propose(fixture.other, wood, [1, 0, 0, 0, 0]),
        );
        let replaced_offer = TradeOffer {
            id: replaced.revision,
            turn: 5,
            proposer: seat(fixture.active),
            recipient: seat(fixture.other),
            give: res(wood),
            receive: res([1, 0, 0, 0, 0]),
        };
        assert_eq!(
            replaced.events,
            vec![
                Event::TradeOfferClosed {
                    offer_id: source.revision
                },
                Event::TradeOfferCreated {
                    offer: replaced_offer
                }
            ]
        );
        offer_views(&e, fixture.seats, Some(replaced_offer));
        let replacement_rejected = act(
            &mut e,
            fixture.other,
            Command::RejectTrade {
                offer_id: replaced.revision,
            },
        );
        assert_eq!(
            replacement_rejected.events,
            vec![Event::TradeOfferClosed {
                offer_id: replaced.revision
            }]
        );
        offer_views(&e, fixture.seats, None);
        hands(&e, fixture.before);
        assert_eq!(
            e.view(Viewer::Spectator).unwrap().bank.counts(),
            &fixture.bank
        );
        project(
            &e,
            &first,
            fixture.seats,
            vec![ProjectedEvent::TradeOfferCreated { offer: first_offer }],
        );
        project(
            &e,
            &counter,
            fixture.seats,
            vec![
                ProjectedEvent::TradeOfferClosed {
                    offer_id: first.revision,
                },
                ProjectedEvent::TradeOfferCreated {
                    offer: counter_offer,
                },
            ],
        );
        project(
            &e,
            &rejected,
            fixture.seats,
            vec![ProjectedEvent::TradeOfferClosed {
                offer_id: counter.revision,
            }],
        );
        project(
            &e,
            &inactive,
            fixture.seats,
            vec![ProjectedEvent::TradeOfferCreated {
                offer: inactive_offer,
            }],
        );
        project(
            &e,
            &cancelled,
            fixture.seats,
            vec![ProjectedEvent::TradeOfferClosed {
                offer_id: inactive.revision,
            }],
        );
        project(
            &e,
            &source,
            fixture.seats,
            vec![ProjectedEvent::TradeOfferCreated {
                offer: source_offer,
            }],
        );
        project(
            &e,
            &replaced,
            fixture.seats,
            vec![
                ProjectedEvent::TradeOfferClosed {
                    offer_id: source.revision,
                },
                ProjectedEvent::TradeOfferCreated {
                    offer: replaced_offer,
                },
            ],
        );
        project(
            &e,
            &replacement_rejected,
            fixture.seats,
            vec![ProjectedEvent::TradeOfferClosed {
                offer_id: replaced.revision,
            }],
        );
        let next = fixture.seats[(fixture
            .seats
            .iter()
            .position(|&id| id == fixture.active)
            .unwrap()
            + 1)
            % fixture.seats.len()];
        act(&mut e, fixture.active, Command::EndTurn);
        assert_eq!(
            act(&mut e, next, Command::RollDice).events[0],
            Event::DiceRolled {
                seat: seat(next),
                dice: fixture.following
            }
        );
    }
}
