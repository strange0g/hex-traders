#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

mod base_rules;
mod engine_api;
mod game_model;
mod turn_engine;

pub use engine_api::{
    BoardView, BoundaryError, CommandEnvelope, CreationReceipt, DevelopmentCardView, Engine,
    GameView, LegalActions, OwnPlayerView, ProjectedCreationReceipt, ProjectedEvent,
    ProjectedReceipt, PublicPlayerView, Receipt, Rejection, ReplayError, ReplayVerifier, Viewer,
    decode_command_envelope, encode_command_envelope,
};
pub use game_model::CreationEvent;
pub use game_model::{BoardGraph, EdgeId, GraphIdError, HexId, VertexId};
pub use game_model::{
    Building, BuildingKind, CardId, DevelopmentKind, Phase, Port, PortKind, Resource, Resources,
    ResumePhase, RobberOrigin, Terrain, TradeOffer,
};
pub use game_model::{GameConfig, ModelError, PlayerId, SetupMode};

pub use game_model::{Command, Event, LegalAction, RuleError};
