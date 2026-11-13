//! Native candidate agreement, full rejection atomicity and counter endpoints.
use super::*;
use crate::game_model::CardLocation;
use crate::{GameConfig, ResumePhase, RobberOrigin};
fn seat(id: u8) -> PlayerId {
    PlayerId::try_from(id).unwrap()
}
fn card(id: u8) -> CardId {
    CardId::try_from(id).unwrap()
}
fn res(v: [u8; 5]) -> Resources {
    Resources::try_from(v).unwrap()
}
fn fixture() -> Engine {
    let (mut e, _) = Engine::create(GameConfig::fixed(&[0, 1, 2, 3]).unwrap(), [9; 32]).unwrap();
    let p = &mut e.state.position;
    p.active_player = seat(0);
    p.turn = 2;
    p.phase = Phase::Action;
    p.hands = [
        res([6; 5]),
        res([0, 1, 0, 0, 0]),
        Resources::default(),
        Resources::default(),
    ];
    p.bank = res([13, 12, 13, 13, 13]);
    for (draw, id) in [0, 14, 16, 18].into_iter().enumerate() {
        let index = p.deck.iter().position(|&c| c == card(id)).unwrap();
        p.deck.swap(draw, index);
        p.card_locations[id as usize] = CardLocation::Held {
            owner: seat(0),
            acquired_turn: 1,
        };
    }
    p.draw_cursor = 4;
    e
}
fn duplicate(e: &Engine) -> Engine {
    let (mut copy, _) = Engine::create(e.state.position.config.clone(), [9; 32]).unwrap();
    copy.state.position = e.state.position.clone();
    copy.state.random = e.state.random.clone();
    copy.state.revision = e.state.revision;
    copy
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
fn commands(e: &Engine, actor: PlayerId) -> Vec<Command> {
    let mut result = Vec::new();
    for choice in e.legal_actions(actor).unwrap().choices {
        match choice {
            LegalAction::ProposeTrade { recipients } => {
                let index = e.state.position.hands[actor.index()]
                    .counts()
                    .iter()
                    .position(|&count| count > 0)
                    .unwrap();
                let mut give = [0; 5];
                give[index] = 1;
                let mut receive = [0; 5];
                receive[(index + 1) % 5] = 1;
                result.extend(
                    recipients
                        .into_iter()
                        .map(|recipient| Command::ProposeTrade {
                            recipient,
                            give: res(give),
                            receive: res(receive),
                        }),
                );
            }
            LegalAction::AcceptTrade { offer_id } => result.push(Command::AcceptTrade { offer_id }),
            LegalAction::RejectTrade { offer_id } => result.push(Command::RejectTrade { offer_id }),
            LegalAction::CancelTrade { offer_id } => result.push(Command::CancelTrade { offer_id }),
            LegalAction::PlayRoadBuilding { cards } => result.extend(
                cards
                    .into_iter()
                    .map(|card| Command::PlayRoadBuilding { card }),
            ),
            LegalAction::PlaceFreeRoad { edges } => result.extend(
                edges
                    .into_iter()
                    .map(|edge| Command::PlaceFreeRoad { edge }),
            ),
            LegalAction::PlayKnight { cards } => {
                result.extend(cards.into_iter().map(|card| Command::PlayKnight { card }))
            }
            LegalAction::BuyDevelopmentCard => result.push(Command::BuyDevelopmentCard),
            LegalAction::PlayMonopoly { cards } => {
                result.extend(cards.into_iter().map(|card| Command::PlayMonopoly {
                    card,
                    resource: Resource::Brick,
                }))
            }
            LegalAction::PlayInvention { cards } => {
                result.extend(cards.into_iter().map(|card| Command::PlayInvention {
                    card,
                    resources: res([2, 0, 0, 0, 0]),
                }))
            }
            LegalAction::BankTrade { pairs } => {
                result.extend(pairs.into_iter().map(|(give_resource, receive_resource)| {
                    Command::BankTrade {
                        give_resource,
                        receive_resource,
                    }
                }))
            }
            LegalAction::UpgradeCity { vertices } => result.extend(
                vertices
                    .into_iter()
                    .map(|vertex| Command::UpgradeCity { vertex }),
            ),
            LegalAction::BuildSettlement { vertices } => result.extend(
                vertices
                    .into_iter()
                    .map(|vertex| Command::BuildSettlement { vertex }),
            ),
            LegalAction::BuildRoad { edges } => {
                result.extend(edges.into_iter().map(|edge| Command::BuildRoad { edge }))
            }
            LegalAction::MoveRobber { hexes } => {
                result.extend(hexes.into_iter().map(|hex| Command::MoveRobber { hex }))
            }
            LegalAction::ChooseRobberVictim { victims } => result.extend(
                victims
                    .into_iter()
                    .map(|victim| Command::ChooseRobberVictim { victim }),
            ),
            LegalAction::DiscardResources { quantity } => {
                assert_eq!(quantity, 4);
                result.push(Command::DiscardResources {
                    resources: res([4, 0, 0, 0, 0]),
                });
            }
            LegalAction::EndTurn => result.push(Command::EndTurn),
            LegalAction::RollDice => result.push(Command::RollDice),
            LegalAction::PlaceSetupRoad { edges } => result.extend(
                edges
                    .into_iter()
                    .map(|edge| Command::PlaceSetupRoad { edge }),
            ),
            LegalAction::PlaceSetupSettlement { vertices } => result.extend(
                vertices
                    .into_iter()
                    .map(|vertex| Command::PlaceSetupSettlement { vertex }),
            ),
        }
    }
    result
}
fn audit(
    e: &Engine,
    actor: PlayerId,
    kinds: &mut std::collections::HashSet<std::mem::Discriminant<Command>>,
) {
    let position = e.state.position.clone();
    let random = e.state.random.clone();
    let revision = e.state.revision;
    let actions = commands(e, actor);
    assert!(!actions.is_empty());
    kinds.extend(actions.iter().map(std::mem::discriminant));
    for command in &actions {
        assert!(e.validate(actor, command).is_ok(), "{command:?}");
        let mut copy = duplicate(e);
        let r = copy
            .execute(CommandEnvelope {
                api_version: 1,
                actor,
                expected_revision: revision,
                command: *command,
            })
            .unwrap();
        assert_eq!(r.revision, revision + 1);
    }
    assert_eq!(e.state.position, position);
    assert_eq!(e.state.random, random);
    assert_eq!(e.state.revision, revision);
    let mut exhausted = duplicate(e);
    exhausted.state.revision = u64::MAX;
    let position = exhausted.state.position.clone();
    let random = exhausted.state.random.clone();
    assert!(exhausted.legal_actions(actor).unwrap().choices.is_empty());
    for command in actions {
        assert_eq!(
            exhausted.validate(actor, &command).unwrap_err().code,
            RuleError::CounterExhausted,
            "{command:?}"
        );
        assert_eq!(
            exhausted
                .execute(CommandEnvelope {
                    api_version: 1,
                    actor,
                    expected_revision: u64::MAX,
                    command
                })
                .unwrap_err()
                .code,
            RuleError::CounterExhausted
        );
        assert_eq!(exhausted.state.position, position);
        assert_eq!(exhausted.state.random, random);
        assert_eq!(exhausted.state.revision, u64::MAX);
    }
}
#[test]
fn native_contract_every_candidate_executes_at_revision_and_disappears_at_revision_exhaustion() {
    let mut kinds = std::collections::HashSet::new();
    let mut e = fixture();
    audit(&e, seat(0), &mut kinds);
    act(
        &mut e,
        0,
        Command::ProposeTrade {
            recipient: seat(1),
            give: res([1, 0, 0, 0, 0]),
            receive: res([0, 1, 0, 0, 0]),
        },
    );
    audit(&e, seat(0), &mut kinds);
    audit(&e, seat(1), &mut kinds);
    let mut preroll = fixture();
    preroll.state.position.phase = Phase::PreRoll;
    audit(&preroll, seat(0), &mut kinds);
    act(
        &mut preroll,
        0,
        Command::PlayRoadBuilding { card: card(14) },
    );
    audit(&preroll, seat(0), &mut kinds);
    let mut robber = fixture();
    robber.state.position.card_locations[0] = CardLocation::PlayedKnight { owner: seat(0) };
    robber.state.position.played_non_vp = true;
    robber.state.position.phase = Phase::RobberMove {
        origin: RobberOrigin::Knight,
        continuation: ResumePhase::Action,
    };
    audit(&robber, seat(0), &mut kinds);
    let (hex, victims) = (0..19)
        .map(|id| HexId::try_from(id).unwrap())
        .map(|hex| {
            (
                hex,
                crate::base_rules::robber_victims(
                    robber.board.graph,
                    &robber.state.position.buildings,
                    seat(0),
                    hex,
                ),
            )
        })
        .find(|(_, victims)| victims.iter().flatten().count() > 1)
        .unwrap();
    robber.state.position.robber = hex;
    robber.state.position.phase = Phase::RobberVictim {
        origin: RobberOrigin::Knight,
        continuation: ResumePhase::Action,
        victims,
    };
    audit(&robber, seat(0), &mut kinds);
    let mut discard = fixture();
    discard.state.position.hands = [
        res([8, 0, 0, 0, 0]),
        res([0, 1, 0, 0, 0]),
        Resources::default(),
        Resources::default(),
    ];
    discard.state.position.bank = res([11, 18, 19, 19, 19]);
    discard.state.position.last_dice = Some([1, 6]);
    discard.state.position.phase = Phase::Discard {
        obligations: [4, 0, 0, 0],
    };
    audit(&discard, seat(0), &mut kinds);
    let (mut setup, _) =
        Engine::create(GameConfig::variable(&[0, 1, 2, 3]).unwrap(), [9; 32]).unwrap();
    let actor = match setup.state.position.phase {
        Phase::SetupSettlement { actor, .. } => actor,
        _ => panic!("variable setup phase"),
    };
    audit(&setup, actor, &mut kinds);
    let first = commands(&setup, actor).into_iter().next().unwrap();
    act(&mut setup, actor.index() as u8, first);
    audit(&setup, actor, &mut kinds);
    // V12-E12-V8-E7-V4 gives a distance-valid paid settlement candidate.
    let mut settlement = fixture();
    settlement.state.position.buildings = [None; 54];
    settlement.state.position.buildings[12] = Some(crate::Building {
        owner: seat(0),
        kind: crate::BuildingKind::Settlement,
    });
    settlement.state.position.roads = [None; 72];
    settlement.state.position.roads[7] = Some(seat(0));
    settlement.state.position.roads[12] = Some(seat(0));
    settlement.state.position.road_lengths = [2, 0, 0, 0];
    audit(&settlement, seat(0), &mut kinds);
    assert_eq!(
        kinds.len(),
        21,
        "every current command variant must be exercised"
    );
}
#[test]
fn native_contract_turn_counter_limit_removes_only_end_turn_and_preserves_offer_and_rng() {
    let mut e = fixture();
    act(
        &mut e,
        0,
        Command::ProposeTrade {
            recipient: seat(1),
            give: res([1, 0, 0, 0, 0]),
            receive: res([0, 1, 0, 0, 0]),
        },
    );
    e.state.position.turn = u64::MAX;
    e.state.position.offer.as_mut().unwrap().turn = u64::MAX;
    let before = e.state.position.clone();
    let random = e.state.random.clone();
    let revision = e.state.revision;
    assert_eq!(
        e.validate(seat(0), &Command::EndTurn).unwrap_err().code,
        RuleError::CounterExhausted
    );
    assert!(
        !e.legal_actions(seat(0))
            .unwrap()
            .choices
            .contains(&LegalAction::EndTurn)
    );
    assert!(
        e.legal_actions(seat(1))
            .unwrap()
            .choices
            .contains(&LegalAction::AcceptTrade { offer_id: 1 })
    );
    assert_eq!(
        e.execute(CommandEnvelope {
            api_version: 1,
            actor: seat(0),
            expected_revision: revision,
            command: Command::EndTurn
        })
        .unwrap_err()
        .code,
        RuleError::CounterExhausted
    );
    assert_eq!(e.state.position, before);
    assert_eq!(e.state.random, random);
    assert_eq!(e.state.revision, revision);
    let mut boundary = fixture();
    boundary.state.position.turn = u64::MAX - 1;
    let random = boundary.state.random.clone();
    let r = act(&mut boundary, 0, Command::EndTurn);
    assert_eq!(boundary.state.position.turn, u64::MAX);
    assert_eq!(r.revision, 1);
    assert_eq!(boundary.state.random, random);
}
