//! Explicit native allowlists for current state and creation facts.

use super::{CreationReceipt, Engine};
use crate::game_model::{
    BoardGraph, Building, BuildingKind, CardId, CardLocation, CreationEvent, DevelopmentKind,
    EdgeId, HexId, ModelError, Phase, PlayerId, Port, Resources, SetupMode, Terrain, TradeOffer,
    VertexId,
};

/// Audience selected by the trusted host; the engine checks seat existence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Viewer {
    /// Include only this existing seat's private hand and true score.
    Seat(PlayerId),
    /// Include public facts only.
    Spectator,
}

/// Owned public layout and occupancy, borrowing only the immutable topology.
#[derive(Clone, Debug)]
pub struct BoardView {
    /// Shared stable graph and all public incidence queries.
    pub graph: &'static BoardGraph,
    /// Terrain indexed by hex ID.
    pub terrain: [Terrain; HexId::COUNT],
    /// Production numbers indexed by hex ID; desert has no number.
    pub numbers: [Option<u8>; HexId::COUNT],
    /// Nine coastal ports with public exchange rates.
    pub ports: [Port; 9],
    /// Road ownership indexed by edge ID.
    pub roads: [Option<PlayerId>; EdgeId::COUNT],
    /// Settlement/city occupancy indexed by vertex ID.
    pub buildings: [Option<Building>; VertexId::COUNT],
    /// Current robber location.
    pub robber: HexId,
}

/// Public holdings and scores for one existing seat.
///
/// ```compile_fail
/// use hex_traders_engine::{Engine, GameConfig, Viewer};
/// let (engine, _) = Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), [0; 32]).unwrap();
/// let view = engine.view(Viewer::Spectator).unwrap();
/// let opponent_hand = view.players[0].resources;
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicPlayerView {
    /// Existing seat ID.
    pub seat: PlayerId,
    /// Total resource cards, without resource types.
    pub resource_count: u8,
    /// Road pieces still in supply.
    pub roads_remaining: u8,
    /// Settlement pieces still in supply.
    pub settlements_remaining: u8,
    /// City pieces still in supply.
    pub cities_remaining: u8,
    /// Unplayed development-card count without identities or kinds.
    pub development_card_count: u8,
    /// Public spent progress-card kinds, in canonical card order.
    pub spent_progress: Vec<DevelopmentKind>,
    /// Revealed Knights counted toward Largest Army.
    pub played_knights: u8,
    /// Visible building and award points; initially two in fixed setup.
    pub public_score: u8,
    /// Exact current longest trail, cached after relevant road changes.
    pub longest_route_length: u8,
}

/// One owned, unplayed card with its individual acquisition timing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DevelopmentCardView {
    /// Private canonical identity for this owner's choices.
    pub id: CardId,
    /// Kind of this particular card.
    pub kind: DevelopmentKind,
    /// Ordinary turn in which this card was purchased.
    pub acquired_turn: u64,
    /// Whether its purchase predates this turn; other play restrictions still apply.
    pub acquired_on_earlier_turn: bool,
}

/// Facts available only to the selected owning seat.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnPlayerView {
    /// Seat whose private facts these are.
    pub seat: PlayerId,
    /// Exact five-resource hand.
    pub resources: Resources,
    /// Complete score, including immediately acquired hidden VP cards.
    pub true_score: u8,
    /// Own unplayed IDs/kinds/timing, in canonical ID order.
    pub development_cards: Vec<DevelopmentCardView>,
}

/// Viewer-safe current state, allocated only when explicitly requested.
///
/// The view contains current public facts and the selected owner's private hand.
/// No private deck order, seed, random state or opponent exact hands appear.
/// All mutable arrays are owned copies; the shared graph is read-only.
///
/// ```
/// use hex_traders_engine::{Engine, GameConfig, PlayerId, Viewer};
/// let (engine, receipt) = Engine::create(GameConfig::fixed(&[0, 1, 2])?, [0; 32])?;
/// let view = engine.view(Viewer::Seat(PlayerId::try_from(0)?))?;
/// assert_eq!(view.own_player.unwrap().resources.counts().iter().sum::<u8>(), 3);
/// let public = engine.project_creation_receipt(&receipt, Viewer::Spectator)?;
/// assert_eq!(public.revision, view.revision);
/// # Ok::<(), hex_traders_engine::ModelError>(())
/// ```
///
/// ```compile_fail
/// use hex_traders_engine::{Engine, GameConfig, Viewer};
/// let (engine, _) = Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), [0; 32]).unwrap();
/// let view = engine.view(Viewer::Spectator).unwrap();
/// let private_deck = view.deck;
/// ```
#[derive(Clone, Debug)]
pub struct GameView {
    /// Revision at projection time.
    pub revision: u64,
    /// Public board layout and occupancy.
    pub board: BoardView,
    /// Selected setup procedure.
    pub setup: SetupMode,
    /// Supplied seat order, also determining the fixed preset roles.
    pub seat_order: Vec<PlayerId>,
    /// Supplied seat order rotated to the starting player.
    pub turn_order: Vec<PlayerId>,
    /// Seat selected by starter rolls.
    pub starting_player: PlayerId,
    /// Seat whose turn is active.
    pub active_player: PlayerId,
    /// Current turn counter.
    pub turn: u64,
    /// Current public decision phase.
    pub phase: Phase,
    /// Last gameplay dice; starter rolls do not populate this field.
    pub last_dice: Option<[u8; 2]>,
    /// Current road award owner, or supply when nobody uniquely qualifies.
    pub longest_route_holder: Option<PlayerId>,
    /// Largest Army holder; qualifying ties retain the incumbent.
    pub largest_army_holder: Option<PlayerId>,
    /// Public bank supply by resource.
    pub bank: Resources,
    /// Current public bilateral proposal, if any.
    pub offer: Option<TradeOffer>,
    /// Undrawn development-card count.
    pub development_deck_remaining: u8,
    /// Only the winner's VP identities after completion; absent during play.
    pub revealed_victory_cards: [Option<CardId>; 5],
    /// Public facts in supplied seat order.
    pub players: Vec<PublicPlayerView>,
    /// Exact own hand and score, absent for spectators.
    pub own_player: Option<OwnPlayerView>,
}

/// Public creation response, distinct from the host's versioned receipt.
///
/// Creation records every tied starter round under the approved reroll policy,
/// so its event count is not subject to the gameplay-command transition bound.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectedCreationReceipt {
    /// The original receipt's revision, never a later engine revision.
    pub revision: u64,
    /// Ordered public setup allocations and resolved starter/turn facts.
    pub events: Vec<CreationEvent>,
}

impl Engine {
    /// Project public current state and the selected seat's own private facts.
    ///
    /// Returns `InvalidSeat` if the selected seat is absent from this game.
    /// This borrows the engine and never consumes randomness or changes state.
    pub fn view(&self, viewer: Viewer) -> Result<GameView, ModelError> {
        self.validate_viewer(viewer)?;
        let position = &self.state.position;
        let players: Vec<_> = position
            .config
            .seat_order()
            .iter()
            .map(|&seat| {
                let roads = position
                    .roads
                    .iter()
                    .filter(|&&owner| owner == Some(seat))
                    .count() as u8;
                let settlements = position
                    .buildings
                    .iter()
                    .flatten()
                    .filter(|building| {
                        building.owner == seat && building.kind == BuildingKind::Settlement
                    })
                    .count() as u8;
                let cities = position
                    .buildings
                    .iter()
                    .flatten()
                    .filter(|building| {
                        building.owner == seat && building.kind == BuildingKind::City
                    })
                    .count() as u8;
                PublicPlayerView {
                    seat,
                    resource_count: u8::try_from(position.hands[seat.index()].total())
                        .expect("at most ninety-five resource cards"),
                    roads_remaining: 15 - roads,
                    settlements_remaining: 5 - settlements,
                    cities_remaining: 4 - cities,
                    development_card_count: position.held_cards(seat).count() as u8,
                    played_knights: position.played_knights(seat),
                    spent_progress: position
                        .card_locations
                        .iter()
                        .enumerate()
                        .filter(|(_, location)| matches!(location, CardLocation::Spent { owner } if *owner == seat))
                        .map(|(id, _)| CardId::try_from(id as u8).expect("bounded card ID").kind())
                        .collect(),
                    public_score: crate::base_rules::public_score(position, seat),
                    longest_route_length: position.road_lengths[seat.index()],
                }
            })
            .collect();
        let own_player = match viewer {
            Viewer::Seat(seat) => Some(OwnPlayerView {
                seat,
                resources: position.hands[seat.index()],
                true_score: crate::base_rules::current_score(position, seat),
                development_cards: position
                    .held_cards(seat)
                    .map(|(id, acquired_turn)| DevelopmentCardView {
                        id,
                        kind: id.kind(),
                        acquired_turn,
                        acquired_on_earlier_turn: acquired_turn < position.turn,
                    })
                    .collect(),
            }),
            Viewer::Spectator => None,
        };
        let mut turn_order = position.config.seat_order().to_vec();
        let start = turn_order
            .iter()
            .position(|&seat| seat == position.starting_player)
            .expect("validated starter");
        turn_order.rotate_left(start);
        Ok(GameView {
            revision: self.state.revision,
            board: BoardView {
                graph: self.board.graph,
                terrain: self.board.terrain,
                numbers: self.board.numbers,
                ports: self.board.ports,
                roads: position.roads,
                buildings: position.buildings,
                robber: position.robber,
            },
            setup: position.config.setup(),
            seat_order: position.config.seat_order().to_vec(),
            turn_order,
            starting_player: position.starting_player,
            active_player: position.active_player,
            turn: position.turn,
            phase: position.phase,
            last_dice: position.last_dice,
            longest_route_holder: position.longest_route_holder,
            largest_army_holder: position.largest_army_holder,
            bank: position.bank,
            offer: position.offer,
            development_deck_remaining: 25 - position.draw_cursor,
            revealed_victory_cards: position.revealed_victory_cards(),
            players,
            own_player,
        })
    }

    /// Project creation facts for a viewer whose seat exists in this engine.
    ///
    /// This operation handles only `CreationReceipt`; use `project_receipt`
    /// for ordinary accepted commands.
    /// All `CreationEvent` variants are public, including starting allocations
    /// determined by the board. It copies the receipt's original facts and
    /// revision without consulting the current position to reconstruct events.
    pub fn project_creation_receipt(
        &self,
        receipt: &CreationReceipt,
        viewer: Viewer,
    ) -> Result<ProjectedCreationReceipt, ModelError> {
        self.validate_viewer(viewer)?;
        Ok(ProjectedCreationReceipt {
            revision: receipt.revision,
            events: receipt.events.clone(),
        })
    }

    fn validate_viewer(&self, viewer: Viewer) -> Result<(), ModelError> {
        if let Viewer::Seat(seat) = viewer
            && !self.state.position.config.seat_order().contains(&seat)
        {
            return Err(ModelError::InvalidSeat(seat.index() as u8));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BuildingKind, Engine, GameConfig};
    #[test]
    fn turn_city_views_share_occupancy_score_and_stocks() {
        let (mut engine, _) =
            Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), [0; 32]).unwrap();
        // Rare city fixture: a replaced settlement releases one settlement piece.
        engine.state.position.buildings[31].as_mut().unwrap().kind = BuildingKind::City;
        let owner = PlayerId::try_from(0).unwrap();
        for viewer in [
            Viewer::Spectator,
            Viewer::Seat(owner),
            Viewer::Seat(PlayerId::try_from(1).unwrap()),
        ] {
            let view = engine.view(viewer).unwrap();
            let public = view
                .players
                .iter()
                .find(|player| player.seat == owner)
                .unwrap();
            assert_eq!(
                (
                    public.settlements_remaining,
                    public.cities_remaining,
                    public.public_score
                ),
                (4, 3, 3)
            );
            assert_eq!(view.board.buildings[31].unwrap().kind, BuildingKind::City);
            if viewer == Viewer::Seat(owner) {
                assert_eq!(view.own_player.unwrap().true_score, 3);
            }
        }
    }
}

#[cfg(test)]
mod route_tests {
    use super::*;
    use crate::{Command, CommandEnvelope, GameConfig};
    #[test]
    fn longest_route_views_and_unrelated_commands_only_read_cached_lengths() {
        let (mut engine, _) =
            Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), [0; 32]).unwrap();
        crate::base_rules::reset_route_searches();
        for viewer in [
            Viewer::Spectator,
            Viewer::Seat(engine.state.position.active_player),
        ] {
            assert_eq!(
                engine
                    .view(viewer)
                    .unwrap()
                    .players
                    .iter()
                    .map(|player| player.longest_route_length)
                    .collect::<Vec<_>>(),
                vec![1, 1, 1]
            );
        }
        let actor = engine.state.position.active_player;
        engine.legal_actions(actor).unwrap();
        engine
            .execute(CommandEnvelope {
                api_version: 1,
                actor,
                expected_revision: 0,
                command: Command::RollDice,
            })
            .unwrap();
        engine
            .execute(CommandEnvelope {
                api_version: 1,
                actor,
                expected_revision: 1,
                command: Command::EndTurn,
            })
            .unwrap();
        engine.view(Viewer::Spectator).unwrap();
        assert_eq!(crate::base_rules::route_searches(), 0);
        let (mut setup, _) =
            Engine::create(GameConfig::variable(&[0, 1, 2]).unwrap(), [0; 32]).unwrap();
        let actor = setup.state.position.active_player;
        crate::base_rules::reset_route_searches();
        setup
            .execute(CommandEnvelope {
                api_version: 1,
                actor,
                expected_revision: 0,
                command: Command::PlaceSetupSettlement {
                    vertex: VertexId::try_from(0).unwrap(),
                },
            })
            .unwrap();
        assert_eq!(crate::base_rules::route_searches(), 0);
        setup
            .execute(CommandEnvelope {
                api_version: 1,
                actor,
                expected_revision: 1,
                command: Command::PlaceSetupRoad {
                    edge: EdgeId::try_from(1).unwrap(),
                },
            })
            .unwrap();
        assert_eq!(crate::base_rules::route_searches(), 1);
        assert_eq!(setup.state.position.road_lengths[actor.index()], 1);
        setup.view(Viewer::Spectator).unwrap();
        assert_eq!(crate::base_rules::route_searches(), 1);
    }
    #[test]
    fn longest_route_creation_rejects_inconsistent_cached_lengths() {
        let (mut engine, _) =
            Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), [0; 32]).unwrap();
        engine.state.position.road_lengths[0] = 2;
        assert_eq!(
            crate::turn_engine::validate_created_game(&engine.board, &engine.state),
            Err(ModelError::InvalidInitialState)
        );
        let (mut engine, _) =
            Engine::create(GameConfig::variable(&[0, 1, 2]).unwrap(), [0; 32]).unwrap();
        engine.state.position.road_lengths[3] = 1;
        assert_eq!(
            crate::turn_engine::validate_created_game(&engine.board, &engine.state),
            Err(ModelError::InvalidInitialState)
        );
    }
}

#[cfg(test)]
mod award_tests {
    use super::*;
    use crate::{Building, Event, GameConfig, ProjectedEvent};
    #[test]
    fn longest_route_award_view_score_and_blocking_update_use_shared_helpers() {
        let (mut engine, _) =
            Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), [0; 32]).unwrap();
        let owner = PlayerId::try_from(0).unwrap();
        let opponent = PlayerId::try_from(1).unwrap();
        assert_eq!(
            engine.view(Viewer::Spectator).unwrap().longest_route_holder,
            None
        );
        // Valid bounded engineered position: own V0 connects the independent five-edge trail.
        engine.state.position.buildings[0] = Some(Building {
            owner,
            kind: BuildingKind::Settlement,
        });
        for id in [0, 6, 10, 18, 23] {
            engine.state.position.roads[id] = Some(owner);
        }
        let mut events = vec![];
        crate::turn_engine::refresh_routes(
            &engine.board,
            &mut engine.state.position,
            &[owner],
            &mut events,
        );
        assert_eq!(
            events,
            vec![Event::LongestRouteChanged {
                previous: None,
                holder: Some(owner)
            }]
        );
        assert_eq!(engine.state.position.road_lengths, [5, 1, 1, 0]);
        for viewer in [
            Viewer::Spectator,
            Viewer::Seat(owner),
            Viewer::Seat(opponent),
        ] {
            let view = engine.view(viewer).unwrap();
            assert_eq!(view.longest_route_holder, Some(owner));
            assert_eq!(view.players[0].public_score, 5);
            assert_eq!(view.players[0].roads_remaining, 8);
            if viewer == Viewer::Seat(owner) {
                assert_eq!(view.own_player.unwrap().true_score, 5);
            }
        }
        // A blocker splits the trail into two and three; loss returns the award to supply.
        engine.state.position.roads[11] = Some(opponent);
        engine.state.position.buildings[7] = Some(Building {
            owner: opponent,
            kind: BuildingKind::Settlement,
        });
        crate::base_rules::reset_route_searches();
        events.clear();
        crate::turn_engine::refresh_routes(
            &engine.board,
            &mut engine.state.position,
            &[owner],
            &mut events,
        );
        assert_eq!(crate::base_rules::route_searches(), 1);
        assert_eq!(engine.state.position.road_lengths, [3, 1, 1, 0]);
        assert_eq!(
            events,
            vec![Event::LongestRouteChanged {
                previous: Some(owner),
                holder: None
            }]
        );
        assert_eq!(
            engine.view(Viewer::Spectator).unwrap().players[0].public_score,
            3
        );
        let receipt = super::super::Receipt {
            api_version: 1,
            rules_version: "hex-traders-base-v1",
            random_algorithm: "chacha12-rand010-v1",
            actor: opponent,
            command: crate::Command::EndTurn,
            previous_revision: 10,
            revision: 11,
            events,
            final_scores: None,
        };
        for viewer in [
            Viewer::Spectator,
            Viewer::Seat(owner),
            Viewer::Seat(opponent),
        ] {
            assert_eq!(
                engine.project_receipt(&receipt, viewer).unwrap().events,
                vec![ProjectedEvent::LongestRouteChanged {
                    previous: Some(owner),
                    holder: None
                }]
            );
        }
        assert_eq!(crate::base_rules::route_searches(), 1);
    }
    #[test]
    fn longest_route_creation_rejects_an_unearned_initial_holder() {
        let (mut engine, _) =
            Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), [0; 32]).unwrap();
        engine.state.position.longest_route_holder = Some(PlayerId::try_from(0).unwrap());
        assert_eq!(
            crate::turn_engine::validate_created_game(&engine.board, &engine.state),
            Err(ModelError::InvalidInitialState)
        );
    }
}
