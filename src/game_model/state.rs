//! Bounded position facts, private cards and explicit random state.

use super::{
    BoardGraph, EdgeId, GameConfig, HexId, ModelError, PlayerId, Resource, Resources, VertexId,
};

/// Public terrain at a stable land-hex ID.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Terrain {
    /// Produces brick.
    Hills,
    /// Produces wood.
    Forest,
    /// Produces wool.
    Pasture,
    /// Produces wheat.
    Field,
    /// Produces ore.
    Mountain,
    /// Produces no resource and has no number token.
    Desert,
}

impl Terrain {
    pub(crate) fn resource(self) -> Option<Resource> {
        match self {
            Self::Hills => Some(Resource::Brick),
            Self::Forest => Some(Resource::Wood),
            Self::Pasture => Some(Resource::Wool),
            Self::Field => Some(Resource::Wheat),
            Self::Mountain => Some(Resource::Ore),
            Self::Desert => None,
        }
    }
}

/// The exchange rate offered at a public coastal port.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PortKind {
    /// Three of one resource for one other resource.
    Generic,
    /// Two of the named resource for one other resource.
    Special(Resource),
}

/// A coastal port; its accessible vertices are the edge endpoints.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Port {
    /// Stable coastal edge with the two accessible vertices.
    pub edge: EdgeId,
    /// Generic or resource-specific rate.
    pub kind: PortKind,
}

pub(crate) struct Board {
    pub graph: &'static BoardGraph,
    pub terrain: [Terrain; HexId::COUNT],
    pub numbers: [Option<u8>; HexId::COUNT],
    pub ports: [Port; 9],
    pub frame_permutation: [u8; 6],
}

/// Public building kind and its base-game point/production value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuildingKind {
    /// One point and one resource on production.
    Settlement,
    /// Two points and two resources on production.
    City,
}
impl BuildingKind {
    pub(crate) fn value(self) -> u8 {
        match self {
            Self::Settlement => 1,
            Self::City => 2,
        }
    }
}

/// Public settlement or city occupancy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Building {
    /// Settlement or city.
    pub kind: BuildingKind,
    /// Existing seat that owns this building.
    pub owner: PlayerId,
}

/// The five canonical base-game development-card kinds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DevelopmentKind {
    /// Activates the robber and counts toward Largest Army.
    Knight,
    /// Places up to two free roads.
    RoadBuilding,
    /// Takes one chosen resource type from opponents.
    Monopoly,
    /// Takes available resources from the bank.
    Invention,
    /// One hidden point, revealed when the owner wins.
    VictoryPoint,
}

/// Checked canonical identity; never expose another player's unplayed ID.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CardId(u8);

impl CardId {
    /// Stable index from zero through twenty-four.
    pub fn index(self) -> usize {
        usize::from(self.0)
    }

    /// Canonical kind; only the deck order is shuffled.
    pub fn kind(self) -> DevelopmentKind {
        match self.0 {
            0..=13 => DevelopmentKind::Knight,
            14..=15 => DevelopmentKind::RoadBuilding,
            16..=17 => DevelopmentKind::Monopoly,
            18..=19 => DevelopmentKind::Invention,
            _ => DevelopmentKind::VictoryPoint,
        }
    }
}

impl TryFrom<u8> for CardId {
    type Error = ModelError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < 25 {
            Ok(Self(value))
        } else {
            Err(ModelError::InvalidCard(value))
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CardLocation {
    Undrawn,
    Held { owner: PlayerId, acquired_turn: u64 },
    Spent { owner: PlayerId },
    PlayedKnight { owner: PlayerId },
    RevealedVictory { owner: PlayerId },
}

/// Origin of the currently supported pending robber effect.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RobberOrigin {
    /// An eligible Knight was revealed.
    Knight,
    /// A seven was rolled.
    Seven,
}
/// Stored ordinary-turn continuation after the pending robber effect.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResumePhase {
    /// Resume before rolling gameplay dice.
    PreRoll,
    /// Resume normal actions.
    Action,
}

/// Current public decision phase.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase {
    /// Free starting-settlement placement; round one precedes reverse-order round two.
    SetupSettlement {
        /// Existing seat required to place next.
        actor: PlayerId,
        /// Current setup round, initially one.
        round: u8,
    },
    /// Free road placement paired with the just-placed setup settlement.
    SetupRoad {
        /// Existing seat required to place next.
        actor: PlayerId,
        /// Current forward/reverse setup round.
        round: u8,
        /// The pending settlement whose incident road must be chosen.
        settlement: VertexId,
    },
    /// Mandatory sequential free roads; completion returns to the stored ordinary phase.
    FreeRoads {
        /// One or two roads remain, while a legal placement exists.
        remaining: u8,
        /// Ordinary phase to resume after the effect.
        continuation: ResumePhase,
    },
    /// Await one distinct adjacent opponent, even when its hand is empty.
    RobberVictim {
        origin: RobberOrigin,
        continuation: ResumePhase,
        victims: [Option<PlayerId>; 4],
    },
    /// The active player has not rolled gameplay dice.
    PreRoll,
    /// Completed game; no gameplay commands are accepted.
    Finished { winner: PlayerId, score: u8 },
    /// Ordinary post-roll actions are available.
    Action,
    /// Fixed outstanding resource quantities by seat; any obligated seat may act.
    Discard { obligations: [u8; 4] },
    /// Await the active seat's robber destination after all obligations.
    RobberMove {
        origin: RobberOrigin,
        continuation: ResumePhase,
    },
}

/// One public bilateral offer, expressed from the proposer's perspective.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TradeOffer {
    /// Revision at which this offer was created.
    pub id: u64,
    /// Ordinary turn in which this offer was created.
    pub turn: u64,
    /// Seat offering the give bundle.
    pub proposer: PlayerId,
    /// Seat invited to exchange the receive bundle.
    pub recipient: PlayerId,
    /// Positive resources offered by the proposer.
    pub give: Resources,
    /// Positive, disjoint resources requested from the recipient.
    pub receive: Resources,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Position {
    pub config: GameConfig,
    pub roads: [Option<PlayerId>; EdgeId::COUNT],
    pub buildings: [Option<Building>; VertexId::COUNT],
    pub road_lengths: [u8; 4],
    pub longest_route_holder: Option<PlayerId>,
    pub largest_army_holder: Option<PlayerId>,
    pub second_settlements: [Option<VertexId>; 4],
    pub robber: HexId,
    pub bank: Resources,
    pub hands: [Resources; 4],
    pub offer: Option<TradeOffer>,
    pub deck: [CardId; 25],
    pub draw_cursor: u8,
    pub card_locations: [CardLocation; 25],
    pub played_non_vp: bool,
    pub final_scores: Option<[u8; 4]>,
    pub starting_player: PlayerId,
    pub active_player: PlayerId,
    pub turn: u64,
    pub phase: Phase,
    pub last_dice: Option<[u8; 2]>,
}

impl Position {
    pub(crate) fn played_knights(&self, owner: PlayerId) -> u8 {
        self.card_locations
            .iter()
            .filter(|&&location| location == CardLocation::PlayedKnight { owner })
            .count() as u8
    }

    pub(crate) fn held_cards(&self, owner: PlayerId) -> impl Iterator<Item = (CardId, u64)> + '_ {
        self.card_locations.iter().enumerate().filter_map(
            move |(index, location)| match *location {
                CardLocation::Held {
                    owner: holder,
                    acquired_turn,
                } if holder == owner => Some((
                    CardId::try_from(index as u8).expect("bounded card ID"),
                    acquired_turn,
                )),
                _ => None,
            },
        )
    }

    pub(crate) fn revealed_victory_cards(&self) -> [Option<CardId>; 5] {
        let mut cards = [None; 5];
        let mut next = 0;
        for (index, location) in self.card_locations.iter().enumerate() {
            if matches!(location, CardLocation::RevealedVictory { .. }) {
                cards[next] = Some(CardId::try_from(index as u8).expect("bounded card ID"));
                next += 1;
            }
        }
        cards
    }
}

pub(crate) struct GameState {
    pub position: Position,
    pub random: rand_chacha::ChaCha12Rng,
    pub revision: u64,
}

/// Currently supported player intents; no generated result is supplied by a caller.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Command {
    /// Propose an explicit bilateral resource exchange without reserving cards.
    ProposeTrade {
        recipient: PlayerId,
        give: Resources,
        receive: Resources,
    },
    /// Accept the current offer as its recipient and exchange both bundles.
    AcceptTrade { offer_id: u64 },
    /// Reject the current offer as its recipient.
    RejectTrade { offer_id: u64 },
    /// Cancel the current offer as its proposer.
    CancelTrade { offer_id: u64 },
    /// Spend an eligible Road Building and begin mandatory sequential free placements.
    PlayRoadBuilding { card: CardId },
    /// Place the next connected free road during Road Building.
    PlaceFreeRoad { edge: EdgeId },
    /// Reveal an eligible Knight and begin the robber unless Army points win immediately.
    PlayKnight { card: CardId },
    /// Pay wool/wheat/ore for the next already shuffled hidden card.
    BuyDevelopmentCard,
    /// Spend an eligible Monopoly and collect one resource from every opponent.
    PlayMonopoly { card: CardId, resource: Resource },
    /// Spend an eligible Invention to take the chosen available bank resources.
    PlayInvention { card: CardId, resources: Resources },
    /// Exchange one resource group for one different bank card at the best owned rate.
    BankTrade {
        give_resource: Resource,
        receive_resource: Resource,
    },
    /// Pay two wheat/three ore to replace an owned settlement with a city.
    UpgradeCity { vertex: VertexId },
    /// Pay brick/wood/wool/wheat for a distance-valid owned-road settlement.
    BuildSettlement { vertex: VertexId },
    /// Pay one brick/wood for a legal connected road during ordinary actions.
    BuildRoad { edge: EdgeId },
    /// Move to a different land hex and resolve zero/one victims automatically.
    MoveRobber { hex: HexId },
    /// Select one listed opponent for a weighted resource-card theft attempt.
    ChooseRobberVictim { victim: PlayerId },
    /// Choose the exact resources for an outstanding discard.
    DiscardResources { resources: Resources },
    /// Finish the active seat's fully resolved ordinary turn.
    EndTurn,
    /// Roll two engine-generated dice before ordinary actions.
    RollDice,
    /// Place a free starting settlement at an empty distance-valid vertex.
    PlaceSetupSettlement { vertex: VertexId },
    /// Place the free road incident to the pending settlement.
    PlaceSetupRoad { edge: EdgeId },
}

/// Stable player-safe rejection reasons for implemented native operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleError {
    /// No undrawn development card remains.
    EmptyDeck,
    /// The supplied card cannot be played by this actor now.
    CardNotPlayable,
    /// Resource kinds on both sides must differ.
    InvalidTrade,
    /// An unrelated inactive seat cannot replace an existing offer.
    OfferBusy,
    /// The specified offer is absent or has been replaced.
    OfferNotFound,
    /// The current offer cannot be funded; details remain private.
    OfferUnavailable,
    /// The requested resource is unavailable in the bank.
    BankShortage,
    /// The selected target is not the actor's settlement.
    NotOwned,
    /// The actor cannot pay the fixed resource cost.
    InsufficientResources,
    /// The actor has used all pieces of the required kind.
    NoPieces,
    /// A supplied choice is invalid, such as leaving the robber on its current hex.
    InvalidInput,
    /// The selected seat is not a listed adjacent opponent.
    InvalidVictim,
    /// The envelope's API version is unsupported.
    UnsupportedApiVersion(u32),
    /// This seat does not exist in the configured game.
    InvalidSeat(u8),
    /// The envelope refers to a different revision.
    StaleRevision,
    /// This game has already ended.
    GameFinished,
    /// An existing seat is not the required actor.
    WrongActor,
    /// This command is unavailable in the current phase.
    WrongPhase,
    /// A checked revision or turn counter cannot advance.
    CounterExhausted,
    /// The chosen board slot is occupied.
    Occupied,
    /// A building occupies a vertex adjacent to the chosen settlement.
    DistanceRule,
    /// Chosen discard has the wrong quantity or exceeds own holdings.
    InvalidDiscard,
    /// The road has no permitted connection.
    Disconnected,
}

impl std::fmt::Display for RuleError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyDeck => formatter.write_str("empty deck"),
            Self::CardNotPlayable => formatter.write_str("card not playable"),
            Self::InvalidTrade => formatter.write_str("invalid trade"),
            Self::OfferBusy => formatter.write_str("offer busy"),
            Self::OfferNotFound => formatter.write_str("offer not found"),
            Self::OfferUnavailable => formatter.write_str("offer unavailable"),
            Self::BankShortage => formatter.write_str("bank shortage"),
            Self::NotOwned => formatter.write_str("not owned"),
            Self::InsufficientResources => formatter.write_str("insufficient resources"),
            Self::NoPieces => formatter.write_str("no pieces"),
            Self::InvalidInput => formatter.write_str("invalid input"),
            Self::InvalidVictim => formatter.write_str("invalid victim"),
            Self::UnsupportedApiVersion(version) => {
                write!(formatter, "unsupported API version: {version}")
            }
            Self::InvalidSeat(seat) => write!(formatter, "invalid seat ID: {seat}"),
            Self::StaleRevision => formatter.write_str("stale revision"),
            Self::GameFinished => formatter.write_str("game finished"),
            Self::WrongActor => formatter.write_str("wrong actor"),
            Self::WrongPhase => formatter.write_str("wrong phase"),
            Self::CounterExhausted => formatter.write_str("counter exhausted"),
            Self::Occupied => formatter.write_str("occupied"),
            Self::DistanceRule => formatter.write_str("distance rule"),
            Self::InvalidDiscard => formatter.write_str("invalid discard"),
            Self::Disconnected => formatter.write_str("disconnected"),
        }
    }
}

impl std::error::Error for RuleError {}

/// Facts in a host-private ordinary receipt; project before sending to players.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Event {
    /// A public offer was created at this revision.
    TradeOfferCreated { offer: TradeOffer },
    /// Both public offer bundles were exchanged atomically.
    TradeCompleted { offer: TradeOffer },
    /// A public offer was closed, without disclosing private affordability.
    TradeOfferClosed { offer_id: u64 },
    /// Public progress identity spent before mandatory free placements.
    RoadBuildingPlayed { seat: PlayerId, card: CardId },
    /// Public revealed Knight identity, counted before any robber decision.
    KnightPlayed { seat: PlayerId, card: CardId },
    /// Public first acquisition or strict-higher transfer of Largest Army.
    LargestArmyChanged {
        previous: Option<PlayerId>,
        holder: Option<PlayerId>,
    },
    /// Host-private identity from this purchase, with public buyer/deck count.
    DevelopmentPurchased {
        seat: PlayerId,
        card: CardId,
        remaining: u8,
    },
    /// Public Monopoly identity, resource and amounts taken by seat ID.
    MonopolyPlayed {
        seat: PlayerId,
        card: CardId,
        resource: Resource,
        transfers: [u8; 4],
    },
    /// Public Invention identity and resources taken from the bank.
    InventionPlayed {
        seat: PlayerId,
        card: CardId,
        resources: Resources,
    },
    /// Winner's VP identities become public immediately before game completion.
    VictoryPointsRevealed {
        winner: PlayerId,
        cards: [Option<CardId>; 5],
    },
    /// Public completed exchange for one received card at the engine-selected rate.
    BankTraded {
        seat: PlayerId,
        give_resource: Resource,
        receive_resource: Resource,
        rate: u8,
    },
    /// The owner's settlement was replaced by one city at the same vertex.
    CityUpgraded { seat: PlayerId, vertex: VertexId },
    /// The public fixed construction or card-purchase cost returned to the bank.
    ConstructionPaid {
        seat: PlayerId,
        resources: Resources,
    },
    /// Public award reconciliation; `None` means the award is in supply.
    LongestRouteChanged {
        previous: Option<PlayerId>,
        holder: Option<PlayerId>,
    },
    /// Public destination chosen by the active seat.
    RobberMoved { seat: PlayerId, hex: HexId },
    /// Host-private resource kind; no resource denotes an empty-handed attempt.
    RobberTheft {
        thief: PlayerId,
        victim: PlayerId,
        resource: Option<Resource>,
    },
    /// Host-private discarded types; public projection exposes count only.
    Discarded {
        seat: PlayerId,
        resources: Resources,
    },
    /// Immediate owner-turn victory; no further decisions follow.
    GameFinished { winner: PlayerId, score: u8 },
    /// Two resolved public gameplay dice.
    DiceRolled { seat: PlayerId, dice: [u8; 2] },
    /// Public allocation from matching unblocked hexes.
    Production {
        seat: PlayerId,
        resources: Resources,
    },
    /// The accepted free road was installed.
    RoadPlaced { seat: PlayerId, edge: EdgeId },
    /// Public allocation from the designated second settlement.
    StartingResources {
        seat: PlayerId,
        resources: Resources,
    },
    /// The first ordinary pre-roll turn began.
    TurnStarted { seat: PlayerId, turn: u64 },
    /// The accepted settlement was installed.
    SettlementPlaced { seat: PlayerId, vertex: VertexId },
    /// The next public decision phase was entered.
    PhaseChanged { phase: Phase },
}

/// A supported legal action and its bounded current choices.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LegalAction {
    /// Seats to whom a proposal may be made; bundles are validated separately.
    ProposeTrade { recipients: Vec<PlayerId> },
    /// Accept the current offer as its recipient.
    AcceptTrade { offer_id: u64 },
    /// Reject the current offer as its recipient.
    RejectTrade { offer_id: u64 },
    /// Cancel the current offer as its proposer.
    CancelTrade { offer_id: u64 },
    /// Eligible earlier-turn Road Building identities with at least one legal road.
    PlayRoadBuilding { cards: Vec<CardId> },
    /// Mandatory free road candidates on the updated position.
    PlaceFreeRoad { edges: Vec<EdgeId> },
    /// Eligible owned earlier-turn Knight identities, in canonical order.
    PlayKnight { cards: Vec<CardId> },
    /// The active seat can afford one available hidden development card.
    BuyDevelopmentCard,
    /// Eligible owned Monopoly IDs in canonical order.
    PlayMonopoly { cards: Vec<CardId> },
    /// Eligible owned Invention IDs; validate one specific bank choice separately.
    PlayInvention { cards: Vec<CardId> },
    /// Affordable available one-card exchanges, in resource-kind order.
    BankTrade { pairs: Vec<(Resource, Resource)> },
    /// All owned settlements currently affordable with a remaining city piece.
    UpgradeCity { vertices: Vec<VertexId> },
    /// All currently affordable distance-valid owned-road vertices.
    BuildSettlement { vertices: Vec<VertexId> },
    /// All currently affordable connected road edges, in stable ID order.
    BuildRoad { edges: Vec<EdgeId> },
    /// All different land hexes in stable ID order.
    MoveRobber { hexes: Vec<HexId> },
    /// Distinct adjacent opponents in seat-ID order, including empty hands.
    ChooseRobberVictim { victims: Vec<PlayerId> },
    /// Quantity required from this seat; resource bundles are not enumerated.
    DiscardResources { quantity: u8 },
    /// Finish the current ordinary turn.
    EndTurn,
    /// The active seat may roll without a caller-supplied result.
    RollDice,
    /// All empty edges incident to the pending settlement, in stable ID order.
    PlaceSetupRoad { edges: Vec<EdgeId> },
    /// All currently legal free starting-settlement vertices, in stable ID order.
    PlaceSetupSettlement { vertices: Vec<VertexId> },
}

pub(crate) struct Transition {
    pub position: Position,
    pub revision: u64,
    pub random: Option<rand_chacha::ChaCha12Rng>,
    pub events: Vec<Event>,
}
