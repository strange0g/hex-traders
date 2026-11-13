//! Shared checked domain values and immutable board topology.

mod board;
mod state;

pub(crate) use state::{Board, CardLocation, GameState, Position, Transition};
pub use state::{
    Building, BuildingKind, CardId, Command, DevelopmentKind, Event, LegalAction, Phase, Port,
    PortKind, ResumePhase, RobberOrigin, RuleError, Terrain, TradeOffer,
};

pub use board::BoardGraph;

/// A numeric identifier outside the fixed base-game graph.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GraphIdError {
    /// Hex identifiers must be below 19.
    Hex(u8),
    /// Vertex identifiers must be below 54.
    Vertex(u8),
    /// Edge identifiers must be below 72.
    Edge(u8),
}

impl std::fmt::Display for GraphIdError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hex(value) => write!(formatter, "invalid hex ID: {value}"),
            Self::Vertex(value) => write!(formatter, "invalid vertex ID: {value}"),
            Self::Edge(value) => write!(formatter, "invalid edge ID: {value}"),
        }
    }
}

impl std::error::Error for GraphIdError {}

macro_rules! graph_id {
    ($name:ident, $count:literal, $error:ident, $description:literal) => {
        #[doc = $description]
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(u8);

        impl $name {
            /// Number of elements of this kind in the base-game graph.
            pub const COUNT: usize = $count;

            /// The validated dense index, stable within schema version 1.
            pub fn index(self) -> usize {
                usize::from(self.0)
            }
        }

        impl TryFrom<u8> for $name {
            type Error = GraphIdError;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                if usize::from(value) < Self::COUNT {
                    Ok(Self(value))
                } else {
                    Err(GraphIdError::$error(value))
                }
            }
        }
    };
}

graph_id!(
    HexId,
    19,
    Hex,
    "A checked hex ID ordered by axial `(r, q)`."
);
graph_id!(
    VertexId,
    54,
    Vertex,
    "A checked vertex ID ordered by integer `(y, x)`."
);
graph_id!(
    EdgeId,
    72,
    Edge,
    "A checked edge ID ordered by its sorted endpoint pair."
);

/// Invalid native configuration or bounded domain input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModelError {
    /// The game needs exactly three or four seats.
    InvalidPlayerCount,
    /// Seat order must be a permutation of `0..player_count`.
    InvalidSeatOrder,
    /// This seat is outside the four possible base-game seats.
    InvalidSeat(u8),
    /// A resource count exceeds the base-game supply of 19.
    InvalidResources,
    /// A development-card identifier is outside the 25-card deck.
    InvalidCard(u8),
    /// Constructed data failed the initial game's complete invariants.
    InvalidInitialState,
}

impl std::fmt::Display for ModelError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPlayerCount => formatter.write_str("expected three or four players"),
            Self::InvalidSeatOrder => formatter.write_str("seat order is not a permutation"),
            Self::InvalidSeat(value) => write!(formatter, "invalid seat ID: {value}"),
            Self::InvalidResources => formatter.write_str("invalid resource counts"),
            Self::InvalidCard(value) => write!(formatter, "invalid card ID: {value}"),
            Self::InvalidInitialState => formatter.write_str("invalid initial game state"),
        }
    }
}

impl std::error::Error for ModelError {}

/// A checked seat index; configuration determines which seats exist in a game.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PlayerId(u8);

impl PlayerId {
    /// Dense index from zero through three.
    pub fn index(self) -> usize {
        usize::from(self.0)
    }
}

impl TryFrom<u8> for PlayerId {
    type Error = ModelError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < 4 {
            Ok(Self(value))
        } else {
            Err(ModelError::InvalidSeat(value))
        }
    }
}

/// Setup modes supported by the current creation path.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SetupMode {
    /// The source-verified beginner layout and preset pieces.
    Fixed,
    /// Shuffled terrain and source frame templates, followed by free placement.
    Variable,
}

/// Validated setup and seat order. No seed or host account metadata is stored.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameConfig {
    seat_order: Vec<PlayerId>,
    setup: SetupMode,
}

impl GameConfig {
    /// Configure variable setup with the same checked seat order as fixed setup.
    pub fn variable(seat_order: &[u8]) -> Result<Self, ModelError> {
        let mut config = Self::fixed(seat_order)?;
        config.setup = SetupMode::Variable;
        Ok(config)
    }

    /// Configure fixed setup with a permutation of seats `0..n`, for n=3 or 4.
    ///
    /// Returns a typed error for an invalid count, seat or permutation.
    pub fn fixed(seat_order: &[u8]) -> Result<Self, ModelError> {
        if !(3..=4).contains(&seat_order.len()) {
            return Err(ModelError::InvalidPlayerCount);
        }
        let mut seats = Vec::with_capacity(seat_order.len());
        for &value in seat_order {
            let seat = PlayerId::try_from(value)?;
            if seat.index() >= seat_order.len() || seats.contains(&seat) {
                return Err(ModelError::InvalidSeatOrder);
            }
            seats.push(seat);
        }
        Ok(Self {
            seat_order: seats,
            setup: SetupMode::Fixed,
        })
    }

    /// Borrow the supplied order, before rotation to the dice-selected starter.
    pub fn seat_order(&self) -> &[PlayerId] {
        &self.seat_order
    }

    /// The supported setup procedure.
    pub fn setup(&self) -> SetupMode {
        self.setup
    }
}

/// The five resource types in the edition's canonical order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Resource {
    Brick,
    Wood,
    Wool,
    Wheat,
    Ore,
}

impl Resource {
    pub(crate) fn index(self) -> usize {
        self as usize
    }
}

/// Checked resource counts in brick, wood, wool, wheat, ore order.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Resources([u8; 5]);

impl Resources {
    /// Borrow all five counts in canonical resource order.
    pub fn counts(&self) -> &[u8; 5] {
        &self.0
    }

    pub(crate) fn from_wide(counts: [u16; 5]) -> Result<Self, ModelError> {
        if counts.iter().any(|&count| count > 19) {
            return Err(ModelError::InvalidResources);
        }
        Ok(Self(counts.map(|count| {
            u8::try_from(count).expect("resource count checked against nineteen")
        })))
    }

    pub(crate) fn checked_add(&self, other: &Self) -> Result<Self, ModelError> {
        Self::from_wide(std::array::from_fn(|resource| {
            u16::from(self.0[resource]) + u16::from(other.0[resource])
        }))
    }

    pub(crate) fn checked_sub(&self, other: &Self) -> Result<Self, ModelError> {
        let mut counts = [0; 5];
        for (resource, count) in counts.iter_mut().enumerate() {
            *count = u16::from(self.0[resource])
                .checked_sub(u16::from(other.0[resource]))
                .ok_or(ModelError::InvalidResources)?;
        }
        Self::from_wide(counts)
    }

    pub(crate) fn total(&self) -> u16 {
        self.0.iter().map(|&count| u16::from(count)).sum()
    }
}

impl TryFrom<[u8; 5]> for Resources {
    type Error = ModelError;

    fn try_from(counts: [u8; 5]) -> Result<Self, Self::Error> {
        Self::from_wide(counts.map(u16::from))
    }
}

/// Facts recorded during creation; future hidden draws are never included.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CreationEvent {
    /// Source layout and supplied role/seat order were installed.
    GameCreated {
        setup: SetupMode,
        seat_order: Vec<PlayerId>,
    },
    /// A fixed preset settlement was installed.
    SettlementPlaced { seat: PlayerId, vertex: VertexId },
    /// A fixed preset road was installed.
    RoadPlaced { seat: PlayerId, edge: EdgeId },
    /// Resources from the public second settlement were allocated.
    StartingResources {
        seat: PlayerId,
        resources: Resources,
    },
    /// A starter-selection roll, separate from gameplay dice and production.
    StarterRolled { seat: PlayerId, dice: [u8; 2] },
    /// The selected player's first pre-roll turn began.
    TurnStarted { seat: PlayerId, turn: u64 },
    /// Variable setup awaits its first free settlement, before gameplay turn one.
    SetupStarted { phase: Phase },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_arithmetic_checks_wide_bounds_and_conserves_transfers() {
        let supply = Resources::try_from([19; 5]).unwrap();
        let empty = Resources::default();
        let bundle = Resources::try_from([3, 7, 0, 2, 19]).unwrap();
        let remainder = supply.checked_sub(&bundle).unwrap();
        assert_eq!(remainder.counts(), &[16, 12, 19, 17, 0]);
        assert_eq!(empty.checked_add(&bundle), Ok(bundle));
        assert_eq!(remainder.checked_add(&bundle), Ok(supply));
        assert_eq!(bundle.total(), 31);
        assert_eq!(supply.total(), 95);
        assert_eq!(
            empty.checked_sub(&bundle),
            Err(ModelError::InvalidResources)
        );
        assert_eq!(
            supply.checked_add(&bundle),
            Err(ModelError::InvalidResources)
        );
        for counts in [[20, 0, 0, 0, 0], [256, 0, 0, 0, 0], [u16::MAX; 5]] {
            assert_eq!(
                Resources::from_wide(counts),
                Err(ModelError::InvalidResources)
            );
        }
        assert_eq!(Resources::from_wide([19; 5]), Ok(supply));
        assert_eq!(supply.counts(), &[19; 5]);
        assert_eq!(empty.counts(), &[0; 5]);
    }

    #[test]
    fn variable_creation_configuration_checks_seats_with_the_same_bounds() {
        for order in [&[2, 0, 1][..], &[2, 0, 3, 1][..]] {
            let config = GameConfig::variable(order).unwrap();
            assert_eq!(config.setup(), SetupMode::Variable);
            assert_eq!(
                config.seat_order(),
                GameConfig::fixed(order).unwrap().seat_order()
            );
        }
        for order in [
            &[][..],
            &[0, 1],
            &[0, 1, 2, 3, 4],
            &[0, 0, 2],
            &[0, 1, 3],
            &[0, 1, 255],
        ] {
            assert_eq!(GameConfig::variable(order), GameConfig::fixed(order));
        }
    }

    #[test]
    fn fixed_creation_configuration_and_resources_accept_valid_inputs() {
        for order in [&[2, 0, 1][..], &[2, 0, 3, 1][..]] {
            let config = GameConfig::fixed(order).unwrap();
            assert_eq!(
                config
                    .seat_order()
                    .iter()
                    .map(|seat| seat.index())
                    .collect::<Vec<_>>(),
                order
                    .iter()
                    .map(|&seat| usize::from(seat))
                    .collect::<Vec<_>>()
            );
            assert_eq!(config.setup(), SetupMode::Fixed);
        }
        for value in 0..=19 {
            assert_eq!(
                Resources::try_from([value; 5]).unwrap().counts(),
                &[value; 5]
            );
        }
        assert_eq!(Resources::default().counts(), &[0; 5]);
        assert_eq!(
            Resources::try_from([19, 0, 1, 3, 7]).unwrap().counts(),
            &[19, 0, 1, 3, 7]
        );
    }

    #[test]
    fn fixed_creation_configuration_and_resources_reject_invalid_inputs() {
        for order in [&[][..], &[0, 1], &[0, 1, 2, 3, 4]] {
            assert_eq!(
                GameConfig::fixed(order),
                Err(ModelError::InvalidPlayerCount)
            );
        }
        for order in [&[0, 0, 2][..], &[0, 1, 3], &[0, 1, 2, 2]] {
            assert_eq!(GameConfig::fixed(order), Err(ModelError::InvalidSeatOrder));
        }
        assert_eq!(
            GameConfig::fixed(&[0, 1, 255]),
            Err(ModelError::InvalidSeat(255))
        );
        for value in 0..=u8::MAX {
            if value < 4 {
                assert_eq!(
                    PlayerId::try_from(value).unwrap().index(),
                    usize::from(value)
                );
            } else {
                assert_eq!(
                    PlayerId::try_from(value),
                    Err(ModelError::InvalidSeat(value))
                );
            }
            if value > 19 {
                for index in 0..5 {
                    let mut counts = [0; 5];
                    counts[index] = value;
                    assert_eq!(
                        Resources::try_from(counts),
                        Err(ModelError::InvalidResources)
                    );
                }
            }
        }
        let errors = [
            (
                ModelError::InvalidPlayerCount,
                "expected three or four players",
            ),
            (
                ModelError::InvalidSeatOrder,
                "seat order is not a permutation",
            ),
            (ModelError::InvalidSeat(255), "invalid seat ID: 255"),
            (ModelError::InvalidResources, "invalid resource counts"),
        ];
        for (error, expected) in errors {
            assert_eq!(error.to_string(), expected);
        }
    }

    #[test]
    fn fixed_creation_bounded_card_ids_reject_invalid_values() {
        for value in 0..=u8::MAX {
            if value < 25 {
                assert_eq!(CardId::try_from(value).unwrap().index(), usize::from(value));
            } else {
                assert_eq!(CardId::try_from(value), Err(ModelError::InvalidCard(value)));
            }
        }
        assert_eq!(
            ModelError::InvalidCard(25).to_string(),
            "invalid card ID: 25"
        );
        assert_eq!(
            ModelError::InvalidInitialState.to_string(),
            "invalid initial game state"
        );
    }
}
