//! Public acceptance and effect lifetime, with C06's independent source/RNG fixtures.
use super::seat;
use hex_traders_engine::{
    CardId, Command, CommandEnvelope, Engine, Event, GameConfig, Phase, ProjectedEvent,
    ProjectedReceipt, Receipt, Resource, Resources, RuleError, TradeOffer, Viewer,
};
fn res(v: [u8; 5]) -> Resources {
    Resources::try_from(v).unwrap()
}
fn propose(recipient: u8, give: [u8; 5], receive: [u8; 5]) -> Command {
    Command::ProposeTrade {
        recipient: seat(recipient),
        give: res(give),
        receive: res(receive),
    }
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
    r
}
fn hands(e: &Engine, expected: &[[u8; 5]]) {
    for (id, v) in expected.iter().enumerate() {
        assert_eq!(
            e.view(Viewer::Seat(seat(id as u8)))
                .unwrap()
                .own_player
                .unwrap()
                .resources
                .counts(),
            v
        );
    }
}
fn project(e: &Engine, r: &Receipt, seats: &[u8], events: Vec<ProjectedEvent>) {
    for viewer in
        std::iter::once(Viewer::Spectator).chain(seats.iter().map(|&id| Viewer::Seat(seat(id))))
    {
        assert_eq!(
            e.project_receipt(r, viewer).unwrap(),
            ProjectedReceipt {
                revision: r.revision,
                events: events.clone()
            }
        );
    }
}
fn project_purchase(e: &Engine, r: &Receipt, seats: &[u8], buyer: u8, card_id: u8) {
    for viewer in
        std::iter::once(Viewer::Spectator).chain(seats.iter().map(|&id| Viewer::Seat(seat(id))))
    {
        assert_eq!(
            e.project_receipt(r, viewer).unwrap(),
            ProjectedReceipt {
                revision: r.revision,
                events: vec![
                    ProjectedEvent::ConstructionPaid {
                        seat: seat(buyer),
                        resources: res([0, 0, 1, 1, 1])
                    },
                    ProjectedEvent::DevelopmentPurchased {
                        seat: seat(buyer),
                        card: if viewer == Viewer::Seat(seat(buyer)) {
                            Some(CardId::try_from(card_id).unwrap())
                        } else {
                            None
                        },
                        remaining: 24
                    },
                ],
            }
        );
    }
}
fn setup(
    seats: &[u8],
    first: u8,
    rolls: &[(u8, [u8; 2])],
    buyer: u8,
    card_id: u8,
    last_dice: [u8; 2],
) -> (Engine, Receipt) {
    let mut seed = [0; 32];
    seed[0] = first;
    let (mut e, _) = Engine::create(GameConfig::fixed(seats).unwrap(), seed).unwrap();
    let mut purchase = None;
    for &(actor, dice) in rolls {
        assert_eq!(
            act(&mut e, actor, Command::RollDice).events[0],
            Event::DiceRolled {
                seat: seat(actor),
                dice
            }
        );
        if actor == buyer {
            let r = act(&mut e, actor, Command::BuyDevelopmentCard);
            assert!(r.events.contains(&Event::DevelopmentPurchased {
                seat: seat(actor),
                card: CardId::try_from(card_id).unwrap(),
                remaining: 24
            }));
            purchase = Some(r);
        }
        act(&mut e, actor, Command::EndTurn);
    }
    assert_eq!(
        act(&mut e, buyer, Command::RollDice).events[0],
        Event::DiceRolled {
            seat: seat(buyer),
            dice: last_dice
        }
    );
    assert_eq!(e.view(Viewer::Spectator).unwrap().phase, Phase::Action);
    (e, purchase.unwrap())
}
struct Fixture<'a> {
    seats: &'a [u8],
    rolls: &'a [(u8, [u8; 2])],
    active: u8,
    recipient: u8,
    receive: [u8; 5],
    before: &'a [[u8; 5]],
    after: &'a [[u8; 5]],
    bank: [u8; 5],
    dice: [u8; 2],
    next: [u8; 2],
}
#[test]
fn trade_accept_public_both_orientations_project_exact_exchange_and_preserve_dice() {
    let fixtures = [
        Fixture {
            seats: &[0, 1, 2],
            rolls: &[(0, [2, 4]), (1, [1, 4]), (2, [1, 4]), (0, [2, 1])],
            active: 1,
            recipient: 0,
            receive: [0, 0, 0, 0, 1],
            before: &[[1, 1, 2, 2, 1], [0, 1, 0, 1, 0], [2, 0, 2, 3, 0]],
            after: &[[1, 2, 2, 2, 0], [0, 0, 0, 1, 1], [2, 0, 2, 3, 0]],
            bank: [16, 17, 15, 13, 18],
            dice: [2, 6],
            next: [5, 6],
        },
        Fixture {
            seats: &[0, 1, 2, 3],
            rolls: &[(0, [1, 4]), (1, [1, 4]), (2, [2, 1]), (3, [2, 6])],
            active: 0,
            recipient: 1,
            receive: [0, 0, 0, 1, 0],
            before: &[
                [1, 2, 1, 1, 0],
                [0, 1, 2, 2, 0],
                [1, 1, 2, 3, 0],
                [2, 1, 0, 3, 0],
            ],
            after: &[
                [1, 1, 1, 2, 0],
                [0, 2, 2, 1, 0],
                [1, 1, 2, 3, 0],
                [2, 1, 0, 3, 0],
            ],
            bank: [15, 14, 14, 10, 19],
            dice: [5, 6],
            next: [5, 1],
        },
    ];
    let wood = [0, 1, 0, 0, 0];
    for fixture in fixtures {
        for active_proposes in [true, false] {
            let (mut e, purchase) = setup(
                fixture.seats,
                8,
                fixture.rolls,
                fixture.active,
                18,
                fixture.dice,
            );
            hands(&e, fixture.before);
            assert_eq!(
                e.view(Viewer::Spectator).unwrap().bank.counts(),
                &fixture.bank
            );
            let (proposer, recipient, give, receive) = if active_proposes {
                (fixture.active, fixture.recipient, wood, fixture.receive)
            } else {
                (fixture.recipient, fixture.active, fixture.receive, wood)
            };
            let created = act(&mut e, proposer, propose(recipient, give, receive));
            let offer = TradeOffer {
                id: created.revision,
                turn: 5,
                proposer: seat(proposer),
                recipient: seat(recipient),
                give: res(give),
                receive: res(receive),
            };
            assert_eq!(created.events, vec![Event::TradeOfferCreated { offer }]);
            let accepted = act(
                &mut e,
                recipient,
                Command::AcceptTrade { offer_id: offer.id },
            );
            assert_eq!(
                accepted.events,
                vec![
                    Event::TradeCompleted { offer },
                    Event::TradeOfferClosed { offer_id: offer.id }
                ]
            );
            hands(&e, fixture.after);
            assert_eq!(
                e.view(Viewer::Spectator).unwrap().bank.counts(),
                &fixture.bank
            );
            assert_eq!(e.view(Viewer::Spectator).unwrap().offer, None);
            assert_eq!(
                e.execute(CommandEnvelope {
                    api_version: 1,
                    actor: seat(recipient),
                    expected_revision: created.revision,
                    command: Command::AcceptTrade { offer_id: offer.id }
                })
                .unwrap_err()
                .code,
                RuleError::StaleRevision
            );
            project(
                &e,
                &created,
                fixture.seats,
                vec![ProjectedEvent::TradeOfferCreated { offer }],
            );
            project(
                &e,
                &accepted,
                fixture.seats,
                vec![
                    ProjectedEvent::TradeCompleted { offer },
                    ProjectedEvent::TradeOfferClosed { offer_id: offer.id },
                ],
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
                    dice: fixture.next
                }
            );
            project(
                &e,
                &accepted,
                fixture.seats,
                vec![
                    ProjectedEvent::TradeCompleted { offer },
                    ProjectedEvent::TradeOfferClosed { offer_id: offer.id },
                ],
            );
            project_purchase(&e, &purchase, fixture.seats, fixture.active, 18);
        }
    }
}
#[test]
fn offer_lifetime_public_monopoly_changed_and_zero_outcomes() {
    let seats = &[0, 1, 2];
    let rolls = &[(0, [1, 5]), (1, [5, 4]), (2, [3, 2]), (0, [6, 5])];
    for zero in [false, true] {
        let (mut e, purchase) = setup(seats, 5, rolls, 1, 16, [3, 2]);
        hands(&e, &[[2, 1, 2, 1, 1], [1, 1, 1, 0, 0], [2, 1, 1, 3, 1]]);
        let bank = [14, 16, 15, 15, 17];
        assert_eq!(e.view(Viewer::Spectator).unwrap().bank.counts(), &bank);
        if zero {
            let first = act(&mut e, 1, propose(0, [0, 1, 0, 0, 0], [0, 0, 0, 0, 1]));
            act(
                &mut e,
                0,
                Command::AcceptTrade {
                    offer_id: first.revision,
                },
            );
            hands(&e, &[[2, 2, 2, 1, 0], [1, 0, 1, 0, 1], [2, 1, 1, 3, 1]]);
            let second = act(&mut e, 1, propose(2, [1, 0, 0, 0, 0], [0, 0, 0, 0, 1]));
            act(
                &mut e,
                2,
                Command::AcceptTrade {
                    offer_id: second.revision,
                },
            );
            hands(&e, &[[2, 2, 2, 1, 0], [0, 0, 1, 0, 2], [3, 1, 1, 3, 0]]);
        }
        let open = if zero {
            act(&mut e, 1, propose(0, [0, 0, 0, 0, 1], [0, 1, 0, 0, 0]))
        } else {
            act(&mut e, 1, propose(0, [0, 1, 0, 0, 0], [0, 0, 0, 0, 1]))
        };
        let offer = e.view(Viewer::Spectator).unwrap().offer.unwrap();
        assert_eq!(offer.id, open.revision);
        let resource = if zero { Resource::Ore } else { Resource::Brick };
        let transfers = if zero { [0; 4] } else { [2, 0, 2, 0] };
        let r = act(
            &mut e,
            1,
            Command::PlayMonopoly {
                card: CardId::try_from(16).unwrap(),
                resource,
            },
        );
        let played = Event::MonopolyPlayed {
            seat: seat(1),
            card: CardId::try_from(16).unwrap(),
            resource,
            transfers,
        };
        let mut projected = vec![ProjectedEvent::MonopolyPlayed {
            seat: seat(1),
            card: CardId::try_from(16).unwrap(),
            resource,
            transfers,
        }];
        if !zero {
            projected.push(ProjectedEvent::TradeOfferClosed { offer_id: offer.id });
        }
        project(&e, &r, seats, projected.clone());
        assert_eq!(e.view(Viewer::Spectator).unwrap().phase, Phase::Action);
        if zero {
            assert_eq!(r.events, vec![played]);
            assert_eq!(e.view(Viewer::Spectator).unwrap().offer, Some(offer));
            hands(&e, &[[2, 2, 2, 1, 0], [0, 0, 1, 0, 2], [3, 1, 1, 3, 0]]);
        } else {
            assert_eq!(
                r.events,
                vec![played, Event::TradeOfferClosed { offer_id: offer.id }]
            );
            assert_eq!(e.view(Viewer::Spectator).unwrap().offer, None);
            hands(&e, &[[0, 1, 2, 1, 1], [5, 1, 1, 0, 0], [0, 1, 1, 3, 1]]);
        }
        assert_eq!(e.view(Viewer::Spectator).unwrap().bank.counts(), &bank);
        let end = act(&mut e, 1, Command::EndTurn);
        if zero {
            assert!(
                matches!(end.events.first(),Some(Event::TradeOfferClosed { offer_id }) if *offer_id==offer.id)
            );
        }
        assert_eq!(
            act(&mut e, 2, Command::RollDice).events[0],
            Event::DiceRolled {
                seat: seat(2),
                dice: [2, 5]
            }
        );
        project(&e, &r, seats, projected);
        project_purchase(&e, &purchase, seats, 1, 16);
    }
}
#[test]
fn offer_lifetime_public_invention_closes_even_when_proposed_payment_remains_affordable() {
    let seats = &[0, 1, 2];
    let rolls = &[(0, [2, 4]), (1, [1, 4]), (2, [1, 4]), (0, [2, 1])];
    let (mut e, purchase) = setup(seats, 8, rolls, 1, 18, [2, 6]);
    let created = act(&mut e, 1, propose(0, [0, 1, 0, 0, 0], [0, 0, 0, 0, 1]));
    let r = act(
        &mut e,
        1,
        Command::PlayInvention {
            card: CardId::try_from(18).unwrap(),
            resources: res([0, 0, 2, 0, 0]),
        },
    );
    assert_eq!(
        r.events,
        vec![
            Event::InventionPlayed {
                seat: seat(1),
                card: CardId::try_from(18).unwrap(),
                resources: res([0, 0, 2, 0, 0])
            },
            Event::TradeOfferClosed {
                offer_id: created.revision
            }
        ]
    );
    let projected = vec![
        ProjectedEvent::InventionPlayed {
            seat: seat(1),
            card: CardId::try_from(18).unwrap(),
            resources: res([0, 0, 2, 0, 0]),
        },
        ProjectedEvent::TradeOfferClosed {
            offer_id: created.revision,
        },
    ];
    project(&e, &r, seats, projected.clone());
    hands(&e, &[[1, 1, 2, 2, 1], [0, 1, 2, 1, 0], [2, 0, 2, 3, 0]]);
    assert_eq!(
        e.view(Viewer::Spectator).unwrap().bank.counts(),
        &[16, 17, 13, 13, 18]
    );
    assert_eq!(e.view(Viewer::Spectator).unwrap().offer, None);
    act(&mut e, 1, Command::EndTurn);
    assert_eq!(
        act(&mut e, 2, Command::RollDice).events[0],
        Event::DiceRolled {
            seat: seat(2),
            dice: [5, 6]
        }
    );
    project(&e, &r, seats, projected);
    project_purchase(&e, &purchase, seats, 1, 18);
}
