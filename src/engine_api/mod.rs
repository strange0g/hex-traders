//! Native facade and explicitly host-owned creation receipts.

mod command;
mod receipt;
mod replay;
mod snapshot;
mod view;
mod wire;
mod wire_output;
pub use command::{CommandEnvelope, LegalActions, Rejection};
pub use receipt::{ProjectedEvent, ProjectedReceipt, Receipt};
pub use replay::{ReplayError, ReplayVerifier};
pub use wire::{BoundaryError, decode_command_envelope, encode_command_envelope};

pub use view::{
    BoardView, DevelopmentCardView, GameView, OwnPlayerView, ProjectedCreationReceipt,
    PublicPlayerView, Viewer,
};

use crate::{
    game_model::{Board, CreationEvent, GameConfig, GameState, ModelError},
    turn_engine::{create_game, validate_created_game},
};

/// A host-owned game. The complete position and RNG stay sealed; views use safe allowlists.
///
/// ```compile_fail
/// use hex_traders_engine::{Engine, GameConfig};
/// let (engine, _) = Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), [0; 32]).unwrap();
/// let private_state = engine.state;
/// ```
pub struct Engine {
    board: Board,
    state: GameState,
}

/// Creation facts for the trusted host, separate from a viewer response.
///
/// This receipt contains resolved setup facts, never the seed or shuffled deck.
/// Every starter roll is retained, so tied rounds can grow this receipt under
/// the approved reroll-until-one policy. No history is stored in the position.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreationReceipt {
    /// Native API contract version.
    pub api_version: u32,
    /// Rules edition and approved digital policy version.
    pub rules_version: &'static str,
    /// Pinned random generator and unbiased sampling algorithm version.
    pub random_algorithm: &'static str,
    /// Initial revision, always zero.
    pub revision: u64,
    /// Ordered setup facts, resolved starter rolls and first required decision.
    pub events: Vec<CreationEvent>,
}

impl Engine {
    /// Create a fixed or variable game from checked seats and a private host seed.
    ///
    /// The host must choose an unpredictable seed and keep it private. The
    /// variable terrain/frame shuffles precede the shuffled deck and starter rolls.
    /// Tied leaders reroll in seat order. Variable creation awaits free placement;
    /// fixed creation starts gameplay turn one before rolling.
    /// No engine is returned if construction invariants fail.
    ///
    /// ```
    /// use hex_traders_engine::{Engine, GameConfig};
    /// // Deterministic fixture seed; real hosts generate a private unpredictable seed.
    /// let (_, receipt) = Engine::create(GameConfig::fixed(&[2, 0, 1])?, [0; 32])?;
    /// assert_eq!(receipt.revision, 0);
    /// # Ok::<(), hex_traders_engine::ModelError>(())
    /// ```
    pub fn create(
        config: GameConfig,
        seed: [u8; 32],
    ) -> Result<(Self, CreationReceipt), ModelError> {
        let created = create_game(config, seed);
        let engine = Self {
            board: created.board,
            state: created.state,
        };
        validate_created_game(&engine.board, &engine.state)?;
        let receipt = CreationReceipt {
            api_version: 1,
            rules_version: "hex-traders-base-v1",
            random_algorithm: "chacha12-rand010-v1",
            revision: engine.state.revision,
            events: created.events,
        };
        Ok((engine, receipt))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_model::{CreationEvent, GameConfig, PlayerId, SetupMode};

    #[test]
    fn variable_creation_facade_views_and_projection_expose_the_initial_setup_decision() {
        use crate::{Phase, Terrain};
        let mut layouts = Vec::new();
        for (first_byte, order) in [(12, &[2, 0, 1][..]), (17, &[2, 0, 3, 1][..])] {
            let mut seed = [0; 32];
            seed[0] = first_byte;
            let (engine, receipt) =
                Engine::create(GameConfig::variable(order).unwrap(), seed).unwrap();
            let before = engine.state.random.get_word_pos();
            let phase = Phase::SetupSettlement {
                actor: PlayerId::try_from(0).unwrap(),
                round: 1,
            };
            assert_eq!(
                receipt.events.last(),
                Some(&CreationEvent::SetupStarted { phase })
            );
            let public = engine.view(Viewer::Spectator).unwrap();
            assert_eq!(public.phase, phase);
            assert_eq!(public.setup, SetupMode::Variable);
            assert_eq!(
                (public.revision, public.turn, public.last_dice),
                (0, 0, None)
            );
            assert_eq!(public.active_player.index(), 0);
            assert_eq!(public.starting_player, public.active_player);
            let expected_order = if order.len() == 3 {
                vec![0, 1, 2]
            } else {
                vec![0, 3, 1, 2]
            };
            assert_eq!(
                public
                    .turn_order
                    .iter()
                    .map(|seat| seat.index())
                    .collect::<Vec<_>>(),
                expected_order
            );
            assert_eq!(public.bank.counts(), &[19; 5]);
            assert_eq!(public.development_deck_remaining, 25);
            assert!(public.own_player.is_none());
            assert!(public.board.roads.iter().all(Option::is_none));
            assert!(public.board.buildings.iter().all(Option::is_none));
            assert_eq!(
                public.board.terrain[0],
                if first_byte == 12 {
                    Terrain::Hills
                } else {
                    Terrain::Mountain
                }
            );
            for player in &public.players {
                assert_eq!(
                    (
                        player.roads_remaining,
                        player.settlements_remaining,
                        player.cities_remaining
                    ),
                    (15, 5, 4)
                );
                assert_eq!(
                    (
                        player.resource_count,
                        player.development_card_count,
                        player.public_score,
                        player.longest_route_length
                    ),
                    (0, 0, 0, 0)
                );
                let viewer = Viewer::Seat(player.seat);
                let own = engine.view(viewer).unwrap().own_player.unwrap();
                assert_eq!(own.seat, player.seat);
                assert_eq!(own.resources.counts(), &[0; 5]);
                assert_eq!(own.true_score, 0);
                assert_eq!(
                    engine
                        .project_creation_receipt(&receipt, viewer)
                        .unwrap()
                        .events,
                    receipt.events
                );
            }
            assert_eq!(
                engine
                    .project_creation_receipt(&receipt, Viewer::Spectator)
                    .unwrap()
                    .events,
                receipt.events
            );
            if order.len() == 3 {
                let invalid = Viewer::Seat(PlayerId::try_from(3).unwrap());
                assert!(matches!(
                    engine.view(invalid),
                    Err(ModelError::InvalidSeat(3))
                ));
                assert_eq!(
                    engine.project_creation_receipt(&receipt, invalid),
                    Err(ModelError::InvalidSeat(3))
                );
            }
            assert_eq!(engine.state.random.get_word_pos(), before);
            layouts.push(public.board.terrain);
        }
        assert_ne!(layouts[0], layouts[1]);
    }

    #[test]
    fn fixed_creation_facade_receipt_records_public_setup_and_starter_facts() {
        let config = GameConfig::fixed(&[2, 0, 1]).unwrap();
        let mut seed = [0; 32];
        seed[0] = 6;
        let (engine, receipt) = Engine::create(config.clone(), seed).unwrap();
        assert_eq!(receipt.api_version, 1);
        assert_eq!(receipt.rules_version, "hex-traders-base-v1");
        assert_eq!(receipt.random_algorithm, "chacha12-rand010-v1");
        assert_eq!(receipt.revision, 0);
        assert_eq!(
            receipt.events.first(),
            Some(&CreationEvent::GameCreated {
                setup: SetupMode::Fixed,
                seat_order: config.seat_order().to_vec()
            })
        );
        assert_eq!(
            receipt.events.last(),
            Some(&CreationEvent::TurnStarted {
                seat: PlayerId::try_from(2).unwrap(),
                turn: 1
            })
        );
        assert_eq!(receipt.events.len(), 22);
        // Literal source placements, in deterministic numeric-ID order per seat.
        let placements = [
            (2, [31, 40], [46, 57], [1, 0, 0, 1, 1]),
            (0, [9, 39], [14, 55], [0, 1, 1, 1, 0]),
            (1, [17, 19], [21, 25], [1, 0, 1, 1, 0]),
        ];
        for (role, (seat, vertices, edges, hand)) in placements.into_iter().enumerate() {
            let seat = PlayerId::try_from(seat).unwrap();
            assert_eq!(
                &receipt.events[1 + role * 4..5 + role * 4],
                &[
                    CreationEvent::SettlementPlaced {
                        seat,
                        vertex: crate::VertexId::try_from(vertices[0]).unwrap()
                    },
                    CreationEvent::SettlementPlaced {
                        seat,
                        vertex: crate::VertexId::try_from(vertices[1]).unwrap()
                    },
                    CreationEvent::RoadPlaced {
                        seat,
                        edge: crate::EdgeId::try_from(edges[0]).unwrap()
                    },
                    CreationEvent::RoadPlaced {
                        seat,
                        edge: crate::EdgeId::try_from(edges[1]).unwrap()
                    },
                ]
            );
            assert_eq!(
                receipt.events[13 + role],
                CreationEvent::StartingResources {
                    seat,
                    resources: crate::Resources::try_from(hand).unwrap()
                }
            );
        }
        let (second, same_receipt) = Engine::create(config, seed).unwrap();
        assert_eq!(receipt, same_receipt);
        assert_eq!(engine.state.position.deck, second.state.position.deck);
        assert_eq!(
            engine.state.random.get_word_pos(),
            second.state.random.get_word_pos()
        );
        assert!(std::ptr::eq(engine.board.graph, second.board.graph));
        assert!(!std::ptr::eq(&engine.board.terrain, &second.board.terrain));
        let position_copy = engine.state.position.clone();
        assert_eq!(position_copy.roads, engine.state.position.roads);
        assert_eq!(position_copy.deck, engine.state.position.deck);
    }

    #[test]
    fn privacy_initial_allow_only_public_facts_and_the_own_hand() {
        use crate::{Phase, Viewer};
        for order in [&[2, 0, 1][..], &[2, 0, 3, 1][..]] {
            let (engine, receipt) =
                Engine::create(GameConfig::fixed(order).unwrap(), [0; 32]).unwrap();
            let public = engine.view(Viewer::Spectator).unwrap();
            assert_eq!(public.revision, 0);
            assert_eq!(public.setup, SetupMode::Fixed);
            assert_eq!(public.seat_order, engine.state.position.config.seat_order());
            assert_eq!(public.phase, Phase::PreRoll);
            assert_eq!(public.turn, 1);
            assert_eq!(public.last_dice, None);
            assert_eq!(public.active_player, engine.state.position.active_player);
            assert_eq!(public.starting_player, public.active_player);
            assert_eq!(public.turn_order.first(), Some(&public.starting_player));
            assert_eq!(public.turn_order.len(), order.len());
            assert_eq!(public.players.len(), order.len());
            assert_eq!(public.bank, engine.state.position.bank);
            assert_eq!(public.development_deck_remaining, 25);
            assert_eq!(public.own_player, None);
            assert_eq!(
                engine
                    .project_creation_receipt(&receipt, Viewer::Spectator)
                    .unwrap()
                    .events,
                receipt.events
            );
            assert!(std::ptr::eq(public.board.graph, engine.board.graph));
            assert_eq!(public.board.terrain, engine.board.terrain);
            assert_eq!(public.board.numbers, engine.board.numbers);
            assert_eq!(public.board.ports, engine.board.ports);
            assert_eq!(public.board.roads, engine.state.position.roads);
            assert_eq!(public.board.buildings, engine.state.position.buildings);
            assert_eq!(public.board.robber, engine.state.position.robber);
            for &seat in engine.state.position.config.seat_order() {
                let view = engine.view(Viewer::Seat(seat)).unwrap();
                assert_eq!(
                    engine
                        .project_creation_receipt(&receipt, Viewer::Seat(seat))
                        .unwrap()
                        .events,
                    receipt.events
                );
                assert_eq!(view.players, public.players);
                let mut board_copy = view.board.clone();
                board_copy.roads[0] = Some(seat);
                board_copy.numbers[0] = Some(2);
                assert_eq!(engine.state.position.roads, public.board.roads);
                assert_eq!(engine.board.numbers, public.board.numbers);
                let own = view.own_player.unwrap();
                assert_eq!(own.seat, seat);
                assert_eq!(own.resources, engine.state.position.hands[seat.index()]);
                assert_eq!(own.true_score, 2);
                let player = view
                    .players
                    .iter()
                    .find(|player| player.seat == seat)
                    .unwrap();
                assert_eq!(player.resource_count, 3);
                assert_eq!(
                    (
                        player.roads_remaining,
                        player.settlements_remaining,
                        player.cities_remaining
                    ),
                    (13, 3, 4)
                );
                assert_eq!(player.development_card_count, 0);
                assert_eq!(player.public_score, 2);
                assert_eq!(player.longest_route_length, 1);
            }
            if order.len() == 3 {
                assert_eq!(
                    engine
                        .view(Viewer::Seat(PlayerId::try_from(3).unwrap()))
                        .unwrap_err(),
                    ModelError::InvalidSeat(3)
                );
            }
        }
    }

    #[test]
    fn privacy_initial_project_the_original_creation_receipt_for_every_viewer() {
        use crate::Viewer;
        let mut first_seed = [0; 32];
        first_seed[0] = 6;
        let mut second_seed = [0; 32];
        second_seed[0] = 21;
        let (original, receipt) =
            Engine::create(GameConfig::fixed(&[2, 0, 1]).unwrap(), first_seed).unwrap();
        let (mut different, _) =
            Engine::create(GameConfig::fixed(&[1, 2, 0]).unwrap(), second_seed).unwrap();
        // Synthetic later host fixture: projection still uses receipt revision zero.
        different.state.revision = 7;
        assert_ne!(
            original.state.position.starting_player,
            different.state.position.starting_player
        );
        for viewer in [
            Viewer::Spectator,
            Viewer::Seat(PlayerId::try_from(0).unwrap()),
            Viewer::Seat(PlayerId::try_from(1).unwrap()),
            Viewer::Seat(PlayerId::try_from(2).unwrap()),
        ] {
            let projected = different
                .project_creation_receipt(&receipt, viewer)
                .unwrap();
            assert_eq!(projected.revision, receipt.revision);
            assert_eq!(projected.events, receipt.events);
        }
        assert_eq!(
            different
                .project_creation_receipt(&receipt, Viewer::Seat(PlayerId::try_from(3).unwrap())),
            Err(ModelError::InvalidSeat(3))
        );
    }

    #[test]
    fn privacy_initial_hide_private_variations_and_never_consume_randomness() {
        use crate::Viewer;
        use rand::Rng;

        let (engine, receipt) =
            Engine::create(GameConfig::fixed(&[2, 0, 1]).unwrap(), [0; 32]).unwrap();
        let (mut altered, _) =
            Engine::create(GameConfig::fixed(&[2, 0, 1]).unwrap(), [0; 32]).unwrap();
        // Synthetic private variants keep every public hand total and resource conservation.
        altered.state.position.hands.swap(0, 2);
        altered.state.position.deck.reverse();
        altered.state.random.set_word_pos(100);
        let public = format!("{:?}", engine.view(Viewer::Spectator).unwrap());
        assert_eq!(
            format!("{:?}", altered.view(Viewer::Spectator).unwrap()),
            public
        );
        let uninvolved = Viewer::Seat(PlayerId::try_from(1).unwrap());
        assert_eq!(
            format!("{:?}", altered.view(uninvolved).unwrap()),
            format!("{:?}", engine.view(uninvolved).unwrap())
        );
        let changed = Viewer::Seat(PlayerId::try_from(0).unwrap());
        assert_ne!(
            altered.view(changed).unwrap().own_player,
            engine.view(changed).unwrap().own_player
        );

        let seed = engine.state.random.get_seed();
        let stream = engine.state.random.get_stream();
        let word_position = engine.state.random.get_word_pos();
        let mut next = engine.state.random.clone();
        let expected_next = next.next_u64();
        for viewer in [
            Viewer::Spectator,
            Viewer::Seat(PlayerId::try_from(0).unwrap()),
            Viewer::Seat(PlayerId::try_from(1).unwrap()),
            Viewer::Seat(PlayerId::try_from(2).unwrap()),
            Viewer::Seat(PlayerId::try_from(3).unwrap()),
        ] {
            let view = engine.view(viewer);
            let projected = engine.project_creation_receipt(&receipt, viewer);
            assert_eq!(view.is_ok(), projected.is_ok());
        }
        assert_eq!(engine.state.random.get_seed(), seed);
        assert_eq!(engine.state.random.get_stream(), stream);
        assert_eq!(engine.state.random.get_word_pos(), word_position);
        let mut next_after = engine.state.random.clone();
        assert_eq!(next_after.next_u64(), expected_next);
        assert_eq!(
            format!("{:?}", engine.view(Viewer::Spectator).unwrap()),
            public
        );
    }
}
