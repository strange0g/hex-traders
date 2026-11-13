# Spec: game-model

Status: approved as the planning baseline on 9 November 2026. Module boundaries
and Rust are approved in [CAPABILITY-MAP.md](CAPABILITY-MAP.md). The user
subsequently authorized autonomous implementation; [tasks/todo.md](tasks/todo.md)
records verified status.

## Objective

Represent the complete 3–4-player base game without a UI or a transport. Give
`base-rules`, `turn-engine`, and `engine-api` one shared vocabulary and one
authoritative representation of the position. Model invalid states explicitly
enough to reject them at creation and snapshot-import boundaries.

## Provider contract

| Type / value | Contract |
|---|---|
| `PlayerId` | Stable seat index, 0–3; only seats below the configured player count exist. Turn order is the supplied seat order, rotated to the starting player. |
| `HexId`, `VertexId`, `EdgeId` | Distinct newtypes with checked construction, dense numeric indices, and deterministic identity within schema version 1. |
| `Resource` | Exactly `Brick`, `Wood`, `Wool`, `Wheat`, `Ore`. Wire names follow the 2025 edition. |
| `Resources` | Five integer counts in that order; one resource type never exceeds the base-game total of 19. Arithmetic uses a wider intermediate and checks bounds. |
| `Terrain` | Hills, forest, pasture, field, mountain, desert; resource mapping is data in one place. |
| `Building` | Owner and settlement/city kind, attached to a vertex. An unoccupied vertex is explicit. |
| `Port` | A coastal edge with its two accessible vertices and either a generic 3:1 rate or a named 2:1 resource rate. |
| `DevelopmentKind` | Knight, Road Building, Monopoly, Invention, Victory Point. |
| `CardId` | Unique identity among the 25 development cards; private except when needed to identify a viewer's own card or a played card. |
| `Board` | Immutable topology, terrain, numbers, ports, and source layout identity after creation. |
| `Position` | Current mutable game facts represented as a value; does not contain a renderer, clock, network connection, or match history. |
| `RandomState` | Explicit clonable ChaCha12 generator state, separate from the position's board facts and from public player views. |
| `GameState` | Position, random state, and monotonic revision. The board is supplied separately by read-only reference to calculations. |
| `Phase` | Tagged phase with its required actor(s), decision payload, and continuation; not a collection of independent Boolean flags. |
| `Command` | Tagged player intent with only its required inputs. Actor and expected revision are in the host envelope. |
| `RuleError` | Stable machine-readable rejection reason; detailed private facts are not included in player-facing errors. |
| `Event` | Typed facts about an accepted transition; full events are host-private until projected for a viewer. |
| `Transition` | Next position and revision, optional changed RNG state, and ordered events; the facade commits them together only on success. |

These are Rust types, not a second JSON-shaped model inside the core. Wire
representations belong to `engine-api`. The same command types and phase
definitions are used by execution and legal-action queries.

### Board identity and graph

- The island has 19 hexes, 54 vertices, and 72 edges. Coastal vertices remain
  legal building sites even though fewer than three land hexes touch them.
- Use axial coordinates with radius 2: `max(abs(q), abs(r), abs(q + r)) <= 2`.
  Assign hex IDs by ascending `(r, q)`. Rows contain 3, 4, 5, 4, and 3 hexes.
- For deterministic integer corner identity, a hex center is `(2q + r, 3r)`.
  Its clockwise corners, starting at the top, are `(0,-2)`, `(1,-1)`, `(1,1)`,
  `(0,2)`, `(-1,1)`, `(-1,-1)` relative to that center.
- Deduplicate corners and assign vertex IDs by ascending `(y, x)`. Assign edge
  IDs by sorting their ordered endpoint pairs `(min_vertex, max_vertex)`.
  These are logical geometry coordinates; no pixels, screen sizes, or graphics
  library types enter the model.
- Precompute hex-to-vertices, hex-to-edges, vertex-to-hexes,
  vertex-to-vertices, vertex-to-edges, and edge-to-endpoints once. Neighbor lists
  have deterministic order, and every incidence is reciprocal.
- Store immutable topology once and borrow it. Per-match terrain/number/port
  layouts may differ; do not share one match's layout with another by accident.
- No floating-point equality or string parsing is used to find neighbors.

### Position contents

The position contains these facts, whether stored directly or represented by a
small equivalent tagged structure:

| Area | Required contents |
|---|---|
| Participants | Player count, seat order, selected setup mode, starting seat. Display names and account identities belong to the host. |
| Board occupancy | Owner of each of 72 road slots; owner/kind of each of 54 building slots; robber hex. |
| Economy | Five bank counts; five resource counts per player. |
| Development cards | Shuffled deck order, draw cursor, location/owner of each card, kind, acquisition turn for cards in hand, played Knights and spent progress cards. |
| Turn | Active player, monotonic turn number, phase, whether a non-VP development card has been played this turn, last dice result when relevant. |
| Setup | Placement round, current actor, last placed settlement awaiting its adjacent road, second settlement identity per player. |
| Pending decisions | Outstanding discard quantities; robber continuation and eligible victims; remaining free roads and continuation. Only payloads appropriate to the phase exist. |
| Trading | At most one current bilateral offer, its ID, proposer, recipient, give/receive bundles, and originating turn. No escrow. |
| Awards | Current Longest Route and Largest Army holders; authoritative road-length results if retained as derived cached values. |
| Completion | Winner and final scores when finished; no further accepted gameplay commands. |

Piece stocks follow 15 roads, 5 settlements, and 4 cities per player. Remaining
stocks may be derived from occupancy or retained as verified cached counts; do
not maintain two independent authoritative inventories. The same rule applies
to scores and played Knight counts. Keep cards and hands in bounded structures;
resource cards are counts, not heap objects with individual identities.

For compact development-card identity, schema 1 starts with IDs 0–13 as Knights,
14–15 as Road Building, 16–17 as Monopoly, 18–19 as Invention, and 20–24 as VP.
Only the deck permutation is shuffled. Exposing another player's unplayed
`CardId` would reveal its kind, so IDs must be removed from their public view.

### Invariants

1. Bank plus all hands equals 19 for each resource; counts are integers and
   nonnegative. Resources are never created, destroyed, or traded on credit.
2. Roads on the board plus remaining road pieces equals 15 per player.
   Settlements on the board plus remaining settlement pieces equals 5; cities
   on the board plus remaining city pieces equals 4. Upgrading returns one
   settlement piece and uses one city piece.
3. Every development card has exactly one location. The deck, hands, played
   cards, and revealed VP cards conserve the original 25-card distribution.
4. Occupied vertices never share an edge. Every road and building has an
   existing owner; all referenced graph IDs exist.
5. There is exactly one robber, on an existing land hex, including desert.
6. The phase and its payload agree. Setup has no paid construction, normal
   turns have one active seat, and pending choices refer only to eligible seats.
7. An acquired development card cannot have an acquisition turn in the future.
   A card in a hand cannot also be in the deck or in a played pile.
8. Award holders, cached lengths, inventories, and scores agree with the rules.
9. A finished position has an eligible winner and no unresolved continuation.
10. A rejected command leaves position, random state, revision, and offer intact.

Full invariant validation runs at trusted-state construction, snapshot import,
and in tests. Normal internal helpers trust validated types; they do not rescan
the entire state at each function boundary. Gameplay-specific legality belongs
to `base-rules` and `turn-engine` rather than to domain constructors.

## Project Structure

The original illustrative file plan under `src/game_model/` was: `mod.rs`, `ids.rs`, `board.rs`,
`resources.rs`, `state.rs`, `command.rs`, and `event.rs`. Split only as size and
responsibility justify. Tests of private helpers are colocated; graph and
invariant integration tests use `tests/model_invariants.rs`.

## Code Style

Use descriptive `snake_case` functions, PascalCase enums/newtypes, exhaustive
matches, private fields for validated domain values, and standard Rust results.
An example of the intended style:

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VertexId(u8);

impl TryFrom<u8> for VertexId {
    type Error = RuleError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < 54 {
            Ok(Self(value))
        } else {
            Err(RuleError::InvalidVertex)
        }
    }
}
```

The illustration is a specification example, not saved implementation. Avoid
generic ID registries, inheritance-like traits for every value, or dynamic
maps for fixed base-game dimensions.

## Commands

Focused local checks:

```bash
cargo check --locked --all-targets
cargo test --locked game_model_
cargo test --locked --test model_invariants
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
```

Tests for this contract use the `game_model_` name prefix. The full-project
commands and dependencies are in [SPEC.md](SPEC.md).

## Testing Strategy

- Verify all graph counts, stable ID fixtures, reciprocal adjacency, coastal
  membership, and absence of duplicate edges/corners.
- Test numeric bounds, invalid seat/graph/card IDs, and arithmetic underflow
  and overflow with failures that do not change a state.
- Assert every invariant on states reached through legal command sequences.
  Property tests shrink failures and preserve the failing seed/sequence.
- Test that cloning a current position never clones the board topology or
  complete command history implicitly. Measure position size/copy cost rather
  than imposing an unmeasured byte target.
- Use direct expected fixtures, not an expected graph rebuilt by the same
  implementation under test.

## Boundaries

- Always: validate external IDs and imported state; keep the board immutable;
  keep representation changes compatible with the declared snapshot version;
  apply [CONSTRAINTS.md](CONSTRAINTS.md).
- Ask first: change base-game quantities, published IDs, snapshot meaning, or
  the four-module dependency direction.
- Never: store UI objects, account credentials, networking, wall-clock time,
  or an unbounded action log in the position; use `unsafe` to avoid checks.

## Success Criteria

- All 19/54/72 graph elements have stable identities and complete adjacency.
- Every legal engine state satisfies the conservation and phase invariants.
- Invalid imported states fail with a typed error before becoming playable.
- All dependent modules use these shared types rather than parallel models.

## Open Questions

None about this module's responsibilities. Exact struct packing remains an
implementation choice constrained by invariants and measured performance.
