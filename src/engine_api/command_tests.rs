use super::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GameConfig;
    #[test]
    fn setup_settlement_native_path_is_atomic_and_projects_public_facts() {
        use crate::{
            Command, CommandEnvelope, Event, LegalAction, Phase, ProjectedEvent, RuleError, Viewer,
        };
        let mut seed = [0; 32];
        seed[0] = 12;
        let (mut engine, _) =
            Engine::create(GameConfig::variable(&[2, 0, 1]).unwrap(), seed).unwrap();
        let actor = PlayerId::try_from(0).unwrap();
        let vertex = crate::VertexId::try_from(0).unwrap();
        let command = Command::PlaceSetupSettlement { vertex };
        let actions = engine.legal_actions(actor).unwrap();
        assert_eq!(actions.revision, 0);
        assert_eq!(
            actions.choices,
            vec![LegalAction::PlaceSetupSettlement {
                vertices: (0..54)
                    .map(|id| crate::VertexId::try_from(id).unwrap())
                    .collect()
            }]
        );
        assert_eq!(engine.validate(actor, &command), Ok(()));
        let position = engine.state.random.get_word_pos();
        let receipt = engine
            .execute(CommandEnvelope {
                api_version: 1,
                actor,
                expected_revision: 0,
                command,
            })
            .unwrap();
        let phase = Phase::SetupRoad {
            actor,
            round: 1,
            settlement: vertex,
        };
        assert_eq!((receipt.previous_revision, receipt.revision), (0, 1));
        assert_eq!(receipt.api_version, 1);
        assert_eq!(receipt.rules_version, "hex-traders-base-v1");
        assert_eq!(receipt.random_algorithm, "chacha12-rand010-v1");
        assert_eq!(receipt.actor, actor);
        assert_eq!(receipt.command, command);
        assert_eq!(
            receipt.events,
            vec![
                Event::SettlementPlaced {
                    seat: actor,
                    vertex
                },
                Event::PhaseChanged { phase }
            ]
        );
        assert_eq!(engine.state.position.phase, phase);
        assert_eq!(engine.state.position.buildings[0].unwrap().owner, actor);
        assert_eq!(engine.state.position.bank.counts(), &[19; 5]);
        assert!(
            engine
                .state
                .position
                .hands
                .iter()
                .all(|hand| hand.counts() == &[0; 5])
        );
        assert_eq!(engine.state.position.turn, 0);
        assert_eq!(engine.state.random.get_word_pos(), position);
        assert_eq!(
            engine.legal_actions(actor).unwrap().choices,
            vec![LegalAction::PlaceSetupRoad {
                edges: vec![
                    crate::EdgeId::try_from(0).unwrap(),
                    crate::EdgeId::try_from(1).unwrap()
                ]
            }]
        );
        assert_eq!(
            engine.validate(actor, &command).unwrap_err().code,
            RuleError::WrongPhase
        );
        for viewer in [
            Viewer::Spectator,
            Viewer::Seat(actor),
            Viewer::Seat(PlayerId::try_from(1).unwrap()),
            Viewer::Seat(PlayerId::try_from(2).unwrap()),
        ] {
            let projected = engine.project_receipt(&receipt, viewer).unwrap();
            assert_eq!(projected.revision, 1);
            assert_eq!(
                projected.events,
                vec![
                    ProjectedEvent::SettlementPlaced {
                        seat: actor,
                        vertex
                    },
                    ProjectedEvent::PhaseChanged { phase }
                ]
            );
        }
        engine.state.revision = 7;
        engine.state.position.phase = Phase::PreRoll;
        let old = engine.project_receipt(&receipt, Viewer::Spectator).unwrap();
        assert_eq!(old.revision, 1);
        assert_eq!(
            old.events,
            vec![
                ProjectedEvent::SettlementPlaced {
                    seat: actor,
                    vertex
                },
                ProjectedEvent::PhaseChanged { phase }
            ]
        );
        assert_eq!(
            engine
                .project_receipt(&receipt, Viewer::Seat(PlayerId::try_from(3).unwrap()))
                .unwrap_err()
                .code,
            RuleError::InvalidSeat(3)
        );
    }

    #[test]
    fn setup_settlement_rejections_preserve_all_private_state() {
        use crate::{Command, CommandEnvelope, Phase, RuleError};
        use rand::Rng;
        let (mut engine, _) =
            Engine::create(GameConfig::variable(&[0, 1, 2]).unwrap(), [0; 32]).unwrap();
        let actor = engine.state.position.active_player;
        let other = PlayerId::try_from(((actor.index() + 1) % 3) as u8).unwrap();
        let command = Command::PlaceSetupSettlement {
            vertex: crate::VertexId::try_from(0).unwrap(),
        };
        for (version, submitted_actor, revision, error) in [
            (
                2,
                PlayerId::try_from(3).unwrap(),
                9,
                RuleError::UnsupportedApiVersion(2),
            ),
            (
                1,
                PlayerId::try_from(3).unwrap(),
                9,
                RuleError::InvalidSeat(3),
            ),
            (1, other, 9, RuleError::StaleRevision),
            (1, other, 0, RuleError::WrongActor),
        ] {
            let before = engine.state.position.clone();
            let rng_position = engine.state.random.get_word_pos();
            let mut next = engine.state.random.clone();
            let expected_next = next.next_u64();
            let rejection = engine
                .execute(CommandEnvelope {
                    api_version: version,
                    actor: submitted_actor,
                    expected_revision: revision,
                    command,
                })
                .unwrap_err();
            assert_eq!(rejection.code, error);
            assert_eq!(rejection.current_revision, 0);
            assert_eq!(engine.state.position, before);
            assert_eq!(engine.state.revision, 0);
            assert_eq!(engine.state.random.get_word_pos(), rng_position);
            assert_eq!(engine.state.random.clone().next_u64(), expected_next);
        }
        engine
            .execute(CommandEnvelope {
                api_version: 1,
                actor,
                expected_revision: 0,
                command,
            })
            .unwrap();
        // Engineered next-settlement fixture isolates distance/occupancy before roads exist.
        engine.state.position.phase = Phase::SetupSettlement {
            actor: other,
            round: 1,
        };
        engine.state.position.active_player = other;
        let rng_position = engine.state.random.get_word_pos();
        // Independent incidence: V0's only neighbors are V3 and V4.
        assert_eq!(
            engine.legal_actions(other).unwrap().choices,
            vec![crate::LegalAction::PlaceSetupSettlement {
                vertices: (0..54)
                    .filter(|id| ![0, 3, 4].contains(id))
                    .map(|id| crate::VertexId::try_from(id).unwrap())
                    .collect()
            }]
        );
        assert_eq!(engine.state.random.get_word_pos(), rng_position);
        for (vertex, error) in [(0, RuleError::Occupied), (4, RuleError::DistanceRule)] {
            let before = engine.state.position.clone();
            let candidate = Command::PlaceSetupSettlement {
                vertex: crate::VertexId::try_from(vertex).unwrap(),
            };
            assert_eq!(engine.validate(other, &candidate).unwrap_err().code, error);
            assert_eq!(
                engine
                    .execute(CommandEnvelope {
                        api_version: 1,
                        actor: other,
                        expected_revision: 1,
                        command: candidate
                    })
                    .unwrap_err()
                    .code,
                error
            );
            assert_eq!(engine.state.position, before);
            assert_eq!(engine.state.revision, 1);
        }
        engine.state.revision = u64::MAX;
        let before = engine.state.position.clone();
        let candidate = Command::PlaceSetupSettlement {
            vertex: crate::VertexId::try_from(21).unwrap(),
        };
        assert_eq!(
            engine
                .execute(CommandEnvelope {
                    api_version: 1,
                    actor: other,
                    expected_revision: u64::MAX,
                    command: candidate
                })
                .unwrap_err()
                .code,
            RuleError::CounterExhausted
        );
        assert_eq!(engine.state.position, before);
        assert_eq!(engine.state.revision, u64::MAX);
    }
    #[test]
    fn setup_road_pairing_order_completion_and_atomicity() {
        use crate::{
            Command, CommandEnvelope, EdgeId, Event, LegalAction, Phase, RuleError, VertexId,
        };
        use rand::Rng;
        let mut seed = [0; 32];
        seed[0] = 12;
        let (mut engine, _) =
            Engine::create(GameConfig::variable(&[2, 0, 1]).unwrap(), seed).unwrap();
        let rng = engine.state.random.clone();
        // Literal snake trace; public integration proof follows in T07c.
        let pairs = [
            (0, 0, 1),
            (1, 21, 23),
            (2, 51, 66),
            (2, 14, 16),
            (1, 35, 51),
            (0, 1, 2),
        ];
        for (pair, (seat, vertex, edge)) in pairs.into_iter().enumerate() {
            let actor = PlayerId::try_from(seat).unwrap();
            let vertex = VertexId::try_from(vertex).unwrap();
            let edge = EdgeId::try_from(edge).unwrap();
            let revision = (pair * 2) as u64;
            assert_eq!(
                engine.state.position.phase,
                Phase::SetupSettlement {
                    actor,
                    round: if pair < 3 { 1 } else { 2 }
                }
            );
            engine
                .execute(CommandEnvelope {
                    api_version: 1,
                    actor,
                    expected_revision: revision,
                    command: Command::PlaceSetupSettlement { vertex },
                })
                .unwrap();
            assert_eq!(
                engine.state.position.second_settlements[actor.index()],
                if pair < 3 { None } else { Some(vertex) }
            );
            let actions = engine.legal_actions(actor).unwrap();
            assert!(
                matches!(&actions.choices[..], [LegalAction::PlaceSetupRoad {edges}] if edges.contains(&edge))
            );
            if pair == 0 {
                assert_eq!(
                    actions.choices,
                    vec![LegalAction::PlaceSetupRoad {
                        edges: vec![EdgeId::try_from(0).unwrap(), edge]
                    }]
                );
                let before = engine.state.position.clone();
                assert_eq!(
                    engine
                        .execute(CommandEnvelope {
                            api_version: 1,
                            actor,
                            expected_revision: 1,
                            command: Command::PlaceSetupRoad {
                                edge: EdgeId::try_from(2).unwrap()
                            }
                        })
                        .unwrap_err()
                        .code,
                    RuleError::Disconnected
                );
                assert_eq!(engine.state.position, before);
                assert_eq!(engine.state.revision, 1);
            }
            if pair == 5 {
                let before = engine.state.position.clone();
                assert_eq!(
                    engine
                        .execute(CommandEnvelope {
                            api_version: 1,
                            actor,
                            expected_revision: 11,
                            command: Command::PlaceSetupRoad {
                                edge: EdgeId::try_from(1).unwrap()
                            }
                        })
                        .unwrap_err()
                        .code,
                    RuleError::Occupied
                );
                assert_eq!(engine.state.position, before);
                assert_eq!(engine.state.revision, 11);
            }
            assert!(
                engine
                    .state
                    .position
                    .hands
                    .iter()
                    .all(|hand| hand.counts() == &[0; 5])
            );
            assert_eq!(engine.state.position.bank.counts(), &[19; 5]);
            let command = Command::PlaceSetupRoad { edge };
            assert_eq!(engine.validate(actor, &command), Ok(()));
            let receipt = engine
                .execute(CommandEnvelope {
                    api_version: 1,
                    actor,
                    expected_revision: revision + 1,
                    command,
                })
                .unwrap();
            assert_eq!(
                receipt.events.first(),
                Some(&Event::RoadPlaced { seat: actor, edge })
            );
            assert_eq!(engine.state.revision, revision + 2);
            assert_eq!(engine.state.random.get_word_pos(), rng.get_word_pos());
            assert_eq!(
                engine.state.random.clone().next_u64(),
                rng.clone().next_u64()
            );
            assert_eq!(
                engine
                    .execute(CommandEnvelope {
                        api_version: 1,
                        actor,
                        expected_revision: revision + 1,
                        command
                    })
                    .unwrap_err()
                    .code,
                RuleError::StaleRevision
            );
            if pair == 5 {
                assert_eq!(engine.state.position.phase, Phase::PreRoll);
                assert_eq!(engine.state.position.turn, 1);
                assert_eq!(
                    receipt
                        .events
                        .iter()
                        .filter(|event| matches!(event, Event::StartingResources { .. }))
                        .count(),
                    3
                );
                assert_eq!(
                    receipt.events.last(),
                    Some(&Event::PhaseChanged {
                        phase: Phase::PreRoll
                    })
                );
                let expected = vec![
                    ProjectedEvent::RoadPlaced { seat: actor, edge },
                    ProjectedEvent::StartingResources {
                        seat: PlayerId::try_from(2).unwrap(),
                        resources: Resources::try_from([0, 0, 1, 1, 0]).unwrap(),
                    },
                    ProjectedEvent::StartingResources {
                        seat: actor,
                        resources: Resources::try_from([0, 0, 1, 0, 0]).unwrap(),
                    },
                    ProjectedEvent::StartingResources {
                        seat: PlayerId::try_from(1).unwrap(),
                        resources: Resources::try_from([0, 1, 0, 2, 0]).unwrap(),
                    },
                    ProjectedEvent::TurnStarted {
                        seat: actor,
                        turn: 1,
                    },
                    ProjectedEvent::PhaseChanged {
                        phase: Phase::PreRoll,
                    },
                ];
                engine.state.position.phase = Phase::SetupSettlement { actor, round: 1 };
                engine.state.revision = 99;
                for viewer in [
                    Viewer::Spectator,
                    Viewer::Seat(actor),
                    Viewer::Seat(PlayerId::try_from(1).unwrap()),
                    Viewer::Seat(PlayerId::try_from(2).unwrap()),
                ] {
                    let projected = engine.project_receipt(&receipt, viewer).unwrap();
                    assert_eq!(projected.revision, 12);
                    assert_eq!(projected.events, expected);
                }
                engine.state.position.phase = Phase::PreRoll;
                engine.state.revision = 12;
            } else {
                assert_eq!(engine.state.position.turn, 0);
            }
        }
        assert_eq!(
            engine.state.position.hands.map(|hand| *hand.counts()),
            [[0, 0, 1, 0, 0], [0, 1, 0, 2, 0], [0, 0, 1, 1, 0], [0; 5]]
        );
        assert_eq!(engine.state.position.bank.counts(), &[19, 18, 17, 16, 19]);
        assert_eq!(
            engine
                .view(Viewer::Spectator)
                .unwrap()
                .players
                .iter()
                .map(|player| player.longest_route_length)
                .collect::<Vec<_>>(),
            vec![1, 2, 1]
        );
    }
}

#[cfg(test)]
mod dice_tests {
    use super::*;
    use crate::{Building, BuildingKind, GameConfig, HexId, ResumePhase, RobberOrigin};
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha12Rng;
    fn fixture(byte: u8) -> Engine {
        let (mut engine, _) =
            Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), [0; 32]).unwrap();
        let mut seed = [0; 32];
        seed[0] = byte;
        engine.state.random = ChaCha12Rng::from_seed(seed);
        engine.state.position.roads = [None; 72];
        engine.state.position.road_lengths = [0; 4];
        engine.state.position.buildings = [None; 54];
        engine.state.position.bank = Resources::try_from([19; 5]).unwrap();
        engine.state.position.hands = [Resources::default(); 4];
        engine
    }
    #[test]
    fn turn_dice_production_atomicity_and_public_facts() {
        // Independent pinned-dependency oracle: raw seed6 gives [4,4], word position2.
        let mut engine = fixture(6);
        for (id, owner, kind) in [
            (1, 0, BuildingKind::City),
            (8, 0, BuildingKind::Settlement),
            (9, 1, BuildingKind::Settlement),
            (42, 2, BuildingKind::City),
        ] {
            engine.state.position.buildings[id] = Some(Building {
                owner: PlayerId::try_from(owner).unwrap(),
                kind,
            });
        }
        let actor = engine.state.position.active_player;
        let other = PlayerId::try_from(0).unwrap();
        assert_ne!(actor, other);
        let before = engine.state.position.clone();
        let rng = engine.state.random.clone();
        assert_eq!(
            engine.legal_actions(actor).unwrap().choices,
            vec![LegalAction::RollDice]
        );
        assert!(engine.legal_actions(other).unwrap().choices.is_empty());
        assert_eq!(engine.validate(actor, &Command::RollDice), Ok(()));
        for (seat, revision, error) in [
            (other, 0, RuleError::WrongActor),
            (actor, 1, RuleError::StaleRevision),
        ] {
            assert_eq!(
                engine
                    .execute(CommandEnvelope {
                        api_version: 1,
                        actor: seat,
                        expected_revision: revision,
                        command: Command::RollDice
                    })
                    .unwrap_err()
                    .code,
                error
            );
            assert_eq!(engine.state.position, before);
            assert_eq!(engine.state.revision, 0);
            assert_eq!(
                engine.state.random.clone().next_u64(),
                rng.clone().next_u64()
            );
        }
        let receipt = engine
            .execute(CommandEnvelope {
                api_version: 1,
                actor,
                expected_revision: 0,
                command: Command::RollDice,
            })
            .unwrap();
        assert_eq!(engine.state.position.last_dice, Some([4, 4]));
        assert_eq!(engine.state.random.get_word_pos(), 2);
        assert_eq!(engine.state.position.phase, Phase::Action);
        assert_eq!(
            engine.state.position.hands.map(|hand| *hand.counts()),
            [[0, 0, 0, 3, 0], [0, 0, 0, 1, 0], [0, 2, 0, 0, 0], [0; 5]]
        );
        assert_eq!(engine.state.position.bank.counts(), &[19, 17, 19, 15, 19]);
        let expected = vec![
            ProjectedEvent::DiceRolled {
                seat: actor,
                dice: [4, 4],
            },
            ProjectedEvent::Production {
                seat: PlayerId::try_from(0).unwrap(),
                resources: Resources::try_from([0, 0, 0, 3, 0]).unwrap(),
            },
            ProjectedEvent::Production {
                seat: PlayerId::try_from(1).unwrap(),
                resources: Resources::try_from([0, 0, 0, 1, 0]).unwrap(),
            },
            ProjectedEvent::Production {
                seat: actor,
                resources: Resources::try_from([0, 2, 0, 0, 0]).unwrap(),
            },
            ProjectedEvent::PhaseChanged {
                phase: Phase::Action,
            },
        ];
        for viewer in [Viewer::Spectator, Viewer::Seat(actor), Viewer::Seat(other)] {
            assert_eq!(
                engine.project_receipt(&receipt, viewer).unwrap().events,
                expected
            );
        }
        let after = engine.state.position.clone();
        let rng = engine.state.random.clone();
        assert_eq!(
            engine
                .execute(CommandEnvelope {
                    api_version: 1,
                    actor,
                    expected_revision: 1,
                    command: Command::RollDice
                })
                .unwrap_err()
                .code,
            RuleError::WrongPhase
        );
        assert_eq!(engine.state.position, after);
        assert_eq!(
            engine.state.random.clone().next_u64(),
            rng.clone().next_u64()
        );
        engine.state.position.phase = Phase::PreRoll;
        engine.state.revision = u64::MAX;
        assert_eq!(
            engine
                .execute(CommandEnvelope {
                    api_version: 1,
                    actor,
                    expected_revision: u64::MAX,
                    command: Command::RollDice
                })
                .unwrap_err()
                .code,
            RuleError::CounterExhausted
        );
        assert_eq!(
            engine.state.random.clone().next_u64(),
            rng.clone().next_u64()
        );
    }
    #[test]
    fn turn_seven_fixes_obligations_without_production() {
        // Independent raw seed5 [2,5] and seed8 [6,1] both give seven.
        for (seed, hands, obligations) in [
            (
                5,
                [[8, 0, 0, 0, 0], [7, 0, 0, 0, 0], [4, 5, 0, 0, 0], [0; 5]],
                [4, 0, 4, 0],
            ),
            (8, [[0; 5]; 4], [0; 4]),
        ] {
            let mut engine = fixture(seed);
            engine.state.position.hands = hands.map(|counts| Resources::try_from(counts).unwrap());
            engine.state.position.bank = Resources::try_from(std::array::from_fn(|resource| {
                19 - hands.iter().map(|hand| hand[resource]).sum::<u8>()
            }))
            .unwrap();
            let before = engine.state.position.hands;
            let bank = engine.state.position.bank;
            let actor = engine.state.position.active_player;
            let receipt = engine
                .execute(CommandEnvelope {
                    api_version: 1,
                    actor,
                    expected_revision: 0,
                    command: Command::RollDice,
                })
                .unwrap();
            let phase = if obligations == [0; 4] {
                Phase::RobberMove {
                    origin: RobberOrigin::Seven,
                    continuation: ResumePhase::Action,
                }
            } else {
                Phase::Discard { obligations }
            };
            assert_eq!(engine.state.position.phase, phase);
            assert_eq!(engine.state.position.hands, before);
            assert_eq!(engine.state.position.bank, bank);
            assert_eq!(receipt.events.len(), 2);
            assert!(matches!(receipt.events[0], Event::DiceRolled { .. }));
            assert_eq!(
                engine.legal_actions(actor).unwrap().choices,
                if obligations == [0; 4] {
                    vec![LegalAction::MoveRobber {
                        hexes: (1..19).map(|id| HexId::try_from(id).unwrap()).collect(),
                    }]
                } else if obligations[actor.index()] == 0 {
                    vec![]
                } else {
                    vec![LegalAction::DiscardResources {
                        quantity: obligations[actor.index()],
                    }]
                }
            );
            assert_eq!(engine.state.random.get_word_pos(), 2);
            assert_eq!(engine.state.position.robber, HexId::try_from(0).unwrap());
        }
    }
}

#[cfg(test)]
mod end_turn_tests {
    use super::*;
    use crate::{Building, BuildingKind, GameConfig};
    use rand::Rng;
    #[test]
    fn turn_end_rotation_checked_counters_and_immediate_next_owner_win() {
        let (mut engine, _) =
            Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), [0; 32]).unwrap();
        let actor = PlayerId::try_from(2).unwrap();
        let next = PlayerId::try_from(0).unwrap();
        let command = Command::EndTurn;
        let before = engine.state.position.clone();
        let rng = engine.state.random.clone();
        assert_eq!(
            engine.validate(actor, &command).unwrap_err().code,
            RuleError::WrongPhase
        );
        assert_eq!(engine.state.position, before);
        assert_eq!(
            engine.state.random.clone().next_u64(),
            rng.clone().next_u64()
        );
        engine.state.position.phase = Phase::Action;
        engine.state.position.last_dice = Some([4, 4]);
        assert_eq!(
            engine.legal_actions(actor).unwrap().choices,
            vec![
                LegalAction::ProposeTrade {
                    recipients: vec![
                        PlayerId::try_from(0).unwrap(),
                        PlayerId::try_from(1).unwrap()
                    ]
                },
                LegalAction::EndTurn
            ]
        );
        let receipt = engine
            .execute(CommandEnvelope {
                api_version: 1,
                actor,
                expected_revision: 0,
                command,
            })
            .unwrap();
        assert_eq!(
            (
                engine.state.position.active_player,
                engine.state.position.turn,
                engine.state.position.phase,
                engine.state.position.last_dice
            ),
            (next, 2, Phase::PreRoll, None)
        );
        assert_eq!(
            receipt.events,
            vec![
                Event::TurnStarted {
                    seat: next,
                    turn: 2
                },
                Event::PhaseChanged {
                    phase: Phase::PreRoll
                }
            ]
        );
        assert_eq!(
            engine.state.random.clone().next_u64(),
            rng.clone().next_u64()
        );
        engine.state.position.active_player = actor;
        engine.state.position.phase = Phase::Action;
        engine.state.position.turn = u64::MAX;
        let before = engine.state.position.clone();
        assert_eq!(
            engine
                .execute(CommandEnvelope {
                    api_version: 1,
                    actor,
                    expected_revision: 1,
                    command
                })
                .unwrap_err()
                .code,
            RuleError::CounterExhausted
        );
        assert_eq!(engine.state.position, before);
        assert_eq!(engine.state.revision, 1);
        engine.state.position.turn = 1;
        // Valid stocks and distance: four cities plus two settlements for next owner.
        engine.state.position.buildings = [None; 54];
        engine.state.position.roads = [None; 72];
        engine.state.position.road_lengths = [0; 4];
        for (vertex, kind) in [
            (0, BuildingKind::City),
            (1, BuildingKind::Settlement),
            (14, BuildingKind::City),
            (21, BuildingKind::City),
            (35, BuildingKind::City),
            (51, BuildingKind::Settlement),
        ] {
            engine.state.position.buildings[vertex] = Some(Building { owner: next, kind });
        }
        for (owner, vertices) in [(1, [26, 53]), (2, [12, 45])] {
            for vertex in vertices {
                engine.state.position.buildings[vertex] = Some(Building {
                    owner: PlayerId::try_from(owner).unwrap(),
                    kind: BuildingKind::Settlement,
                });
            }
        }
        for (owner, edges) in [
            (0, &[1, 2, 16, 23, 51, 66][..]),
            (1, &[38, 71]),
            (2, &[19, 64]),
        ] {
            for &edge in edges {
                engine.state.position.roads[edge] = Some(PlayerId::try_from(owner).unwrap());
            }
        }
        let receipt = engine
            .execute(CommandEnvelope {
                api_version: 1,
                actor,
                expected_revision: 1,
                command,
            })
            .unwrap();
        assert_eq!(
            engine.state.position.phase,
            Phase::Finished {
                winner: next,
                score: 10
            }
        );
        assert_eq!(
            receipt.events,
            vec![
                Event::TurnStarted {
                    seat: next,
                    turn: 2
                },
                Event::GameFinished {
                    winner: next,
                    score: 10
                }
            ]
        );
        let expected = vec![
            ProjectedEvent::TurnStarted {
                seat: next,
                turn: 2,
            },
            ProjectedEvent::GameFinished {
                winner: next,
                score: 10,
            },
        ];
        for viewer in [Viewer::Spectator, Viewer::Seat(next), Viewer::Seat(actor)] {
            assert_eq!(
                engine.project_receipt(&receipt, viewer).unwrap().events,
                expected
            );
        }
        assert!(engine.legal_actions(next).unwrap().choices.is_empty());
        let before = engine.state.position.clone();
        assert_eq!(
            engine
                .execute(CommandEnvelope {
                    api_version: 1,
                    actor,
                    expected_revision: 2,
                    command
                })
                .unwrap_err()
                .code,
            RuleError::GameFinished
        );
        assert_eq!(engine.state.position, before);
        assert_eq!(engine.state.revision, 2);
        assert_eq!(
            engine.state.random.clone().next_u64(),
            rng.clone().next_u64()
        );
    }
    #[test]
    fn turn_end_rotates_non_numeric_four_seat_order() {
        let (mut engine, _) =
            Engine::create(GameConfig::fixed(&[2, 0, 3, 1]).unwrap(), [0; 32]).unwrap();
        engine.state.position.active_player = PlayerId::try_from(2).unwrap();
        for (revision, next) in [0, 3, 1, 2].into_iter().enumerate() {
            engine.state.position.phase = Phase::Action;
            let actor = engine.state.position.active_player;
            engine
                .execute(CommandEnvelope {
                    api_version: 1,
                    actor,
                    expected_revision: revision as u64,
                    command: Command::EndTurn,
                })
                .unwrap();
            assert_eq!(
                engine.state.position.active_player,
                PlayerId::try_from(next).unwrap()
            );
            assert_eq!(engine.state.position.turn, revision as u64 + 2);
        }
    }
}

#[cfg(test)]
mod discard_tests {
    use super::*;
    use crate::{GameConfig, ResumePhase, RobberOrigin};
    use rand::Rng;
    #[test]
    fn privacy_discard_atomicity_exact_choice_and_original_allowlist() {
        let (mut engine, _) =
            Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), [0; 32]).unwrap();
        engine.state.position.phase = Phase::Discard {
            obligations: [7, 7, 7, 0],
        };
        engine.state.position.hands = [[2, 5, 3, 2, 2], [1, 2, 2, 5, 5], [6, 2, 2, 4, 1], [0; 5]]
            .map(|counts| Resources::try_from(counts).unwrap());
        engine.state.position.bank = Resources::try_from([10, 10, 12, 8, 11]).unwrap();
        engine.state.position.last_dice = Some([6, 1]);
        use crate::game_model::CardLocation;
        engine.state.position.turn = 2;
        for (draw, id, location) in [
            (
                0,
                0,
                CardLocation::Held {
                    owner: PlayerId::try_from(1).unwrap(),
                    acquired_turn: 1,
                },
            ),
            (
                1,
                1,
                CardLocation::PlayedKnight {
                    owner: PlayerId::try_from(0).unwrap(),
                },
            ),
            (
                2,
                14,
                CardLocation::Spent {
                    owner: PlayerId::try_from(0).unwrap(),
                },
            ),
            (
                3,
                20,
                CardLocation::Held {
                    owner: PlayerId::try_from(1).unwrap(),
                    acquired_turn: 1,
                },
            ),
        ] {
            let card = crate::CardId::try_from(id).unwrap();
            let index = engine
                .state
                .position
                .deck
                .iter()
                .position(|&c| c == card)
                .unwrap();
            engine.state.position.deck.swap(draw, index);
            engine.state.position.card_locations[id as usize] = location;
        }
        engine.state.position.draw_cursor = 4;
        let development_locations = engine.state.position.card_locations;
        let actor = PlayerId::try_from(1).unwrap();
        let resources = Resources::try_from([0, 0, 0, 5, 2]).unwrap();
        let command = Command::DiscardResources { resources };
        let rng = engine.state.random.clone();
        assert_eq!(
            engine.legal_actions(actor).unwrap().choices,
            vec![LegalAction::DiscardResources { quantity: 7 }]
        );
        for (choice, error) in [
            ([0, 0, 0, 5, 1], RuleError::InvalidDiscard),
            ([7, 0, 0, 0, 0], RuleError::InvalidDiscard),
        ] {
            let before = engine.state.position.clone();
            let bad = Command::DiscardResources {
                resources: Resources::try_from(choice).unwrap(),
            };
            assert_eq!(engine.validate(actor, &bad).unwrap_err().code, error);
            assert_eq!(
                engine
                    .execute(CommandEnvelope {
                        api_version: 1,
                        actor,
                        expected_revision: 0,
                        command: bad
                    })
                    .unwrap_err()
                    .code,
                error
            );
            assert_eq!(engine.state.position, before);
            assert_eq!(engine.state.revision, 0);
            assert_eq!(
                engine.state.random.clone().next_u64(),
                rng.clone().next_u64()
            );
        }
        let receipt = engine
            .execute(CommandEnvelope {
                api_version: 1,
                actor,
                expected_revision: 0,
                command,
            })
            .unwrap();
        assert_eq!(engine.state.position.card_locations, development_locations);
        assert_eq!(
            engine.state.position.phase,
            Phase::Discard {
                obligations: [7, 0, 7, 0]
            }
        );
        assert_eq!(engine.state.position.hands[1].counts(), &[1, 2, 2, 0, 3]);
        assert_eq!(engine.state.position.bank.counts(), &[10, 10, 12, 13, 13]);
        assert!(engine.legal_actions(actor).unwrap().choices.is_empty());
        assert_eq!(
            engine.validate(actor, &command).unwrap_err().code,
            RuleError::WrongActor
        );
        assert_eq!(
            engine.state.random.clone().next_u64(),
            rng.clone().next_u64()
        );
        let phase = Phase::Discard {
            obligations: [7, 0, 7, 0],
        };
        let expected_public = vec![
            ProjectedEvent::Discarded {
                seat: actor,
                count: 7,
                resources: None,
            },
            ProjectedEvent::PhaseChanged { phase },
        ];
        let mut different = receipt.clone();
        different.events[0] = Event::Discarded {
            seat: actor,
            resources: Resources::try_from([1, 2, 2, 2, 0]).unwrap(),
        };
        engine.state.position.phase = Phase::RobberMove {
            origin: RobberOrigin::Seven,
            continuation: ResumePhase::Action,
        };
        engine.state.revision = 99;
        for viewer in [
            Viewer::Spectator,
            Viewer::Seat(PlayerId::try_from(0).unwrap()),
            Viewer::Seat(PlayerId::try_from(2).unwrap()),
        ] {
            let projected = engine.project_receipt(&receipt, viewer).unwrap();
            assert_eq!(projected.revision, 1);
            assert_eq!(projected.events, expected_public);
            assert_eq!(
                projected,
                engine.project_receipt(&different, viewer).unwrap()
            );
        }
        assert_eq!(
            engine
                .project_receipt(&receipt, Viewer::Seat(actor))
                .unwrap()
                .events,
            vec![
                ProjectedEvent::Discarded {
                    seat: actor,
                    count: 7,
                    resources: Some(resources)
                },
                ProjectedEvent::PhaseChanged { phase }
            ]
        );
        assert_ne!(
            engine.project_receipt(&receipt, Viewer::Seat(actor)),
            engine.project_receipt(&different, Viewer::Seat(actor))
        );
    }
}
