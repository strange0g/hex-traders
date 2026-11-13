//! Engineered bounded Knight positions; public histories are a separate slice.
use super::*;
use crate::game_model::CardLocation;
use crate::{Building, BuildingKind, DevelopmentKind, GameConfig, ResumePhase, RobberOrigin};
fn seat(id: u8) -> PlayerId {
    PlayerId::try_from(id).unwrap()
}
fn card(id: u8) -> CardId {
    CardId::try_from(id).unwrap()
}
fn hex(id: u8) -> HexId {
    HexId::try_from(id).unwrap()
}
fn fixture() -> Engine {
    let mut seed = [0; 32];
    seed[..4].copy_from_slice(&511u32.to_le_bytes());
    let (mut engine, _) = Engine::create(GameConfig::fixed(&[0, 1, 2, 3]).unwrap(), seed).unwrap();
    let p = &mut engine.state.position;
    p.active_player = seat(0);
    p.turn = 2;
    p.phase = Phase::PreRoll;
    for (draw, id, owner, turn) in [
        (0, 8, 0, 1),
        (1, 9, 0, 1),
        (2, 10, 0, 1),
        (3, 11, 0, 2),
        (4, 12, 1, 1),
        (5, 20, 1, 1),
    ] {
        let index = p.deck.iter().position(|&value| value == card(id)).unwrap();
        p.deck.swap(draw, index);
        p.card_locations[id as usize] = CardLocation::Held {
            owner: seat(owner),
            acquired_turn: turn,
        };
    }
    p.draw_cursor = 6;
    engine
}
fn play(id: u8) -> Command {
    Command::PlayKnight { card: card(id) }
}
fn act(e: &mut Engine, actor: u8, command: Command) -> Receipt {
    let receipt = e
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(actor),
            expected_revision: e.state.revision,
            command,
        })
        .unwrap();
    let p = &e.state.position;
    for resource in 0..5 {
        assert_eq!(
            u16::from(p.bank.counts()[resource])
                + p.hands
                    .iter()
                    .map(|hand| u16::from(hand.counts()[resource]))
                    .sum::<u16>(),
            19
        );
    }
    for (index, id) in p.deck.iter().enumerate() {
        assert_eq!(
            p.card_locations[id.index()] == CardLocation::Undrawn,
            index >= usize::from(p.draw_cursor)
        );
    }
    receipt
}
fn reject(e: &mut Engine, actor: u8, command: Command, error: RuleError) {
    let before = e.state.position.clone();
    let random = e.state.random.clone();
    let revision = e.state.revision;
    assert_eq!(e.validate(seat(actor), &command).unwrap_err().code, error);
    assert_eq!(
        e.execute(CommandEnvelope {
            api_version: 1,
            actor: seat(actor),
            expected_revision: revision,
            command
        })
        .unwrap_err()
        .code,
        error
    );
    assert_eq!(e.state.position, before);
    assert_eq!(e.state.random, random);
    assert_eq!(e.state.revision, revision);
}
fn played(e: &mut Engine, id: u8, owner: u8) {
    if e.state.position.card_locations[id as usize] == CardLocation::Undrawn {
        let p = &mut e.state.position;
        let index = p.deck.iter().position(|&value| value == card(id)).unwrap();
        p.deck.swap(usize::from(p.draw_cursor), index);
        p.draw_cursor += 1;
    }
    e.state.position.card_locations[id as usize] =
        CardLocation::PlayedKnight { owner: seat(owner) };
}
#[test]
fn knight_owned_timing_queries_rejections_and_public_played_identity() {
    let mut e = fixture();
    assert!(
        e.legal_actions(seat(0))
            .unwrap()
            .choices
            .contains(&LegalAction::PlayKnight {
                cards: vec![card(8), card(9), card(10)]
            })
    );
    for id in [11, 12, 16, 20, 5] {
        reject(&mut e, 0, play(id), RuleError::CardNotPlayable);
    }
    reject(&mut e, 1, play(8), RuleError::WrongActor);
    for phase in [Phase::Action, Phase::PreRoll] {
        e.state.position.phase = phase;
        let before = e.state.position.clone();
        let random = e.state.random.clone();
        let receipt = act(&mut e, 0, play(8));
        assert_eq!(
            receipt.events,
            vec![
                Event::KnightPlayed {
                    seat: seat(0),
                    card: card(8)
                },
                Event::PhaseChanged {
                    phase: Phase::RobberMove {
                        origin: RobberOrigin::Knight,
                        continuation: if phase == Phase::PreRoll {
                            ResumePhase::PreRoll
                        } else {
                            ResumePhase::Action
                        }
                    }
                }
            ]
        );
        assert_eq!(
            e.state.position.card_locations[8],
            CardLocation::PlayedKnight { owner: seat(0) }
        );
        assert_eq!(e.state.position.played_knights(seat(0)), 1);
        assert!(e.state.position.played_non_vp);
        assert_eq!(e.state.position.hands, before.hands);
        assert_eq!(e.state.position.bank, before.bank);
        assert_eq!(e.state.random, random);
        for viewer in [
            Viewer::Spectator,
            Viewer::Seat(seat(0)),
            Viewer::Seat(seat(1)),
            Viewer::Seat(seat(2)),
            Viewer::Seat(seat(3)),
        ] {
            let view = e.view(viewer).unwrap();
            assert_eq!(view.players[0].played_knights, 1);
            assert!(view.players[0].spent_progress.is_empty());
            assert_eq!(view.largest_army_holder, None);
            assert_eq!(
                e.project_receipt(&receipt, viewer).unwrap().events[0],
                ProjectedEvent::KnightPlayed {
                    seat: seat(0),
                    card: card(8)
                }
            );
            assert_eq!(
                view.own_player.as_ref().map(|own| own.seat),
                match viewer {
                    Viewer::Seat(s) => Some(s),
                    Viewer::Spectator => None,
                }
            );
            if let Some(own) = view.own_player {
                assert!(!own.development_cards.iter().any(|c| c.id == card(8)));
            }
        }
        reject(&mut e, 0, play(9), RuleError::WrongPhase);
        e.state.position.phase = phase;
        reject(&mut e, 0, play(9), RuleError::CardNotPlayable);
        e.state.position = before;
    }
    let original = e.state.position.phase;
    for phase in [
        Phase::SetupSettlement {
            actor: seat(0),
            round: 1,
        },
        Phase::SetupRoad {
            actor: seat(0),
            round: 1,
            settlement: crate::VertexId::try_from(31).unwrap(),
        },
        Phase::Discard {
            obligations: [1, 0, 0, 0],
        },
        Phase::RobberMove {
            origin: RobberOrigin::Seven,
            continuation: ResumePhase::Action,
        },
        Phase::RobberVictim {
            origin: RobberOrigin::Seven,
            continuation: ResumePhase::Action,
            victims: [None, Some(seat(1)), None, None],
        },
    ] {
        e.state.position.phase = phase;
        reject(&mut e, 0, play(8), RuleError::WrongPhase);
    }
    e.state.position.phase = original;
    let before = e.state.position.clone();
    let random = e.state.random.clone();
    let revision = e.state.revision;
    assert_eq!(
        e.execute(CommandEnvelope {
            api_version: 1,
            actor: seat(0),
            expected_revision: revision + 1,
            command: play(8)
        })
        .unwrap_err()
        .code,
        RuleError::StaleRevision
    );
    assert_eq!(e.state.position, before);
    assert_eq!(e.state.random, random);
    assert_eq!(e.state.revision, revision);
    e.state.revision = u64::MAX;
    reject(&mut e, 0, play(8), RuleError::CounterExhausted);
}
#[test]
fn base_rules_br036_first_third_knight_ties_keep_incumbent_strict_higher_transfers() {
    let mut e = fixture();
    played(&mut e, 8, 0);
    played(&mut e, 9, 0);
    assert_eq!(e.state.position.played_knights(seat(0)), 2);
    let receipt = act(&mut e, 0, play(10));
    assert_eq!(e.state.position.largest_army_holder, Some(seat(0)));
    assert_eq!(
        receipt.events[1],
        Event::LargestArmyChanged {
            previous: None,
            holder: Some(seat(0))
        }
    );
    assert_eq!(
        crate::base_rules::current_score(&e.state.position, seat(0)),
        4
    );
    for id in [8, 9, 10] {
        played(&mut e, id, 1);
    }
    // Incumbent seat1 has three; seat0's newly acquired third ties before a fourth transfers.
    e.state.position.largest_army_holder = Some(seat(1));
    for id in [0, 1, 2, 3] {
        played(&mut e, id as u8, 0);
    }
    e.state.position.card_locations[2] = CardLocation::Held {
        owner: seat(0),
        acquired_turn: 1,
    };
    e.state.position.card_locations[3] = CardLocation::Held {
        owner: seat(0),
        acquired_turn: 1,
    };
    assert_eq!(
        crate::base_rules::reconcile_largest_army(&e.state.position, seat(0)),
        Some(seat(1))
    );
    e.state.position.phase = Phase::Action;
    e.state.position.played_non_vp = false;
    let tie = act(&mut e, 0, play(2));
    assert_eq!(e.state.position.largest_army_holder, Some(seat(1)));
    assert!(
        tie.events
            .iter()
            .all(|event| !matches!(event, Event::LargestArmyChanged { .. }))
    );
    e.state.position.phase = Phase::Action;
    e.state.position.played_non_vp = false;
    let receipt = act(&mut e, 0, play(3));
    assert_eq!(e.state.position.largest_army_holder, Some(seat(0)));
    assert_eq!(
        receipt.events[1],
        Event::LargestArmyChanged {
            previous: Some(seat(1)),
            holder: Some(seat(0))
        }
    );
}
#[test]
fn knight_zero_automatic_chosen_empty_victims_resume_both_origins_and_theft_privacy() {
    for start in [Phase::PreRoll, Phase::Action] {
        for destination in [7, 1, 5] {
            let mut e = fixture();
            e.state.position.phase = start;
            if destination == 1 {
                e.state.position.hands[1] = Resources::default();
                e.state.position.bank = Resources::try_from([15, 19, 18, 16, 18]).unwrap();
            }
            let play_receipt = act(&mut e, 0, play(8));
            let moved = act(
                &mut e,
                0,
                Command::MoveRobber {
                    hex: hex(destination),
                },
            );
            if destination == 5 {
                assert!(
                    matches!(e.state.position.phase,Phase::RobberVictim{origin:RobberOrigin::Knight,continuation,..} if continuation==if start==Phase::PreRoll{ResumePhase::PreRoll}else{ResumePhase::Action})
                );
                reject(
                    &mut e,
                    0,
                    Command::ChooseRobberVictim { victim: seat(0) },
                    RuleError::InvalidVictim,
                );
                let receipt = act(&mut e, 0, Command::ChooseRobberVictim { victim: seat(1) });
                for viewer in [
                    Viewer::Spectator,
                    Viewer::Seat(seat(0)),
                    Viewer::Seat(seat(1)),
                    Viewer::Seat(seat(2)),
                    Viewer::Seat(seat(3)),
                ] {
                    let theft = e.project_receipt(&receipt, viewer).unwrap().events[0];
                    assert!(
                        matches!(theft,ProjectedEvent::RobberTheft{count:1,resource,..} if resource.is_some()==matches!(viewer,Viewer::Seat(s) if s==seat(0)||s==seat(1)))
                    );
                    assert_eq!(
                        e.project_receipt(&play_receipt, viewer).unwrap().events[0],
                        ProjectedEvent::KnightPlayed {
                            seat: seat(0),
                            card: card(8)
                        }
                    );
                }
            } else if destination == 1 {
                assert!(matches!(
                    moved.events[1],
                    Event::RobberTheft { resource: None, .. }
                ));
            }
            assert_eq!(e.state.position.phase, start);
            reject(&mut e, 0, play(9), RuleError::CardNotPlayable);
        }
    }
}
#[test]
fn base_rules_br015_pre_roll_knight_then_seven_requires_new_obligations_and_second_move() {
    let mut e = fixture();
    e.state.random.set_word_pos(16);
    e.state.position.hands = [
        Resources::try_from([5, 3, 0, 0, 0]).unwrap(),
        Resources::default(),
        Resources::default(),
        Resources::default(),
    ];
    e.state.position.bank = Resources::try_from([14, 16, 19, 19, 19]).unwrap();
    act(&mut e, 0, play(8));
    assert!(matches!(
        e.state.position.phase,
        Phase::RobberMove {
            origin: RobberOrigin::Knight,
            ..
        }
    ));
    act(&mut e, 0, Command::MoveRobber { hex: hex(7) });
    assert_eq!(e.state.position.phase, Phase::PreRoll);
    let receipt = act(&mut e, 0, Command::RollDice);
    assert_eq!(
        receipt.events[0],
        Event::DiceRolled {
            seat: seat(0),
            dice: [6, 1]
        }
    );
    assert_eq!(
        e.state.position.phase,
        Phase::Discard {
            obligations: [4, 0, 0, 0]
        }
    );
    act(
        &mut e,
        0,
        Command::DiscardResources {
            resources: Resources::try_from([4, 0, 0, 0, 0]).unwrap(),
        },
    );
    assert_eq!(
        e.state.position.phase,
        Phase::RobberMove {
            origin: RobberOrigin::Seven,
            continuation: ResumePhase::Action
        }
    );
    act(&mut e, 0, Command::MoveRobber { hex: hex(0) });
    assert_eq!(e.state.position.phase, Phase::Action);
    reject(&mut e, 0, play(9), RuleError::CardNotPlayable);
}
#[test]
fn privacy_victory_third_knight_finishes_before_robber_and_reveals_only_winner() {
    let mut e = fixture();
    played(&mut e, 8, 0);
    played(&mut e, 9, 0);
    e.state.position.buildings = [None; 54];
    for (id, kind) in [
        (0, BuildingKind::City),
        (9, BuildingKind::City),
        (17, BuildingKind::City),
        (31, BuildingKind::Settlement),
        (40, BuildingKind::Settlement),
    ] {
        e.state.position.buildings[id] = Some(Building {
            owner: seat(0),
            kind,
        });
    }
    let random = e.state.random.clone();
    let robber = e.state.position.robber;
    let receipt = act(&mut e, 0, play(10));
    assert_eq!(
        receipt.events,
        vec![
            Event::KnightPlayed {
                seat: seat(0),
                card: card(10)
            },
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
    assert_eq!(receipt.final_scores, Some([10, 1, 0, 0]));
    assert_eq!(e.state.position.robber, robber);
    assert_eq!(e.state.random, random);
    assert_eq!(
        e.state.position.phase,
        Phase::Finished {
            winner: seat(0),
            score: 10
        }
    );
    for viewer in [
        Viewer::Spectator,
        Viewer::Seat(seat(0)),
        Viewer::Seat(seat(1)),
        Viewer::Seat(seat(2)),
        Viewer::Seat(seat(3)),
    ] {
        let view = e.view(viewer).unwrap();
        assert_eq!(view.players[1].public_score, 0);
        assert_eq!(view.revealed_victory_cards, [None; 5]);
        assert_eq!(view.largest_army_holder, Some(seat(0)));
    }
    reject(
        &mut e,
        0,
        Command::MoveRobber { hex: hex(7) },
        RuleError::GameFinished,
    );
    assert_eq!(card(20).kind(), DevelopmentKind::VictoryPoint);
}

#[test]
fn knight_creation_rejects_unearned_army_holder() {
    let (mut e, _) = Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), [0; 32]).unwrap();
    e.state.position.largest_army_holder = Some(seat(0));
    assert_eq!(
        crate::turn_engine::validate_created_game(&e.board, &e.state),
        Err(crate::ModelError::InvalidInitialState)
    );
}
