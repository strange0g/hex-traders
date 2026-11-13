# Spec: engine-api

Status: approved as the planning baseline on 9 November 2026. Depends on the
other three modules; none imports this module. This is the only public library
entry point. The user subsequently authorized autonomous implementation;
[tasks/todo.md](tasks/todo.md) records verified status.

## Objective

Let a consuming program create and play a complete game, display legal choices
and viewer-safe state, save/resume it, and verify a replay. The UI supplies
intent and renders results; it does not calculate costs, score, eligibility,
or turn progression. Native callers use Rust values and direct calls. A host
adapter may serialize these contracts for another language or UI.

## Public provider contract

The facade owns immutable board data and the current `GameState`. Conceptual
operations are:

| Operation | Inputs | Output / behavior |
|---|---|---|
| `Engine::create` | Validated `GameConfig`, host-private `[u8; 32]` seed | Engine and creation receipt, or typed creation error. |
| `Engine::execute` | `CommandEnvelope` | Accepted receipt after committing one transition, or rejection with no change. |
| `Engine::view` | Host-selected `Viewer` | A view tagged with the current revision. |
| `Engine::legal_actions` | Seat selected by host | Current available action kinds and bounded choice lists for that seat. |
| `Engine::validate` | Seat and one typed command | Legality verdict using the same validators as execution; no RNG use or mutation. |
| `Engine::project_receipt` | Private receipt and host-selected viewer | Ordered viewer-safe events at that receipt's revision. |
| `Engine::export_snapshot` | No player input | Complete host-private snapshot of this engine. |
| `Engine::from_snapshot` | Host-private snapshot bytes | A new fully validated engine, or import error. |
| `verify_replay` | Initial snapshot, accepted records | Reconstructed engine or first mismatch with record index. |

Read methods borrow the engine. `execute` requires exclusive mutable access,
so two commands cannot commit concurrently through one facade. The host
serializes access if it shares a game across callers; no async runtime, mutex,
or networking service is required inside the engine. Do not make a view or
serialize JSON on every native command unless the caller asks for it.

The host derives the acting seat and viewer from its own trusted context. The
engine verifies seat existence and game eligibility; it does not authenticate
accounts. A seed, omniscient receipt, snapshot, or raw engine memory is trusted
host data. A local application that owns all engine memory can inspect it;
viewer projection alone cannot protect secrets from that application's owner.

### Configuration and wire conventions

- `GameConfig` has `seatOrder`, a permutation of seats `0..n` with `n` equal to
  3 or 4, and `setup`, exactly `fixed` or `variable`. No duplicate player count
  field is necessary. Fixed roles map to that order as Blue, Red, Orange, White;
  White is absent for three seats. The starting seat is determined by dice.
- Display names, colors chosen by a UI, account IDs, and match IDs are host
  metadata. They do not change graph IDs or rule eligibility.
- Native IDs use distinct checked types. Wire hex, vertex, edge, seat, and card
  IDs are bounded JSON integers. Resources are objects with exactly five keys:
  `brick`, `wood`, `wool`, `wheat`, `ore`; counts are integers from 0 through 19.
- Rust functions use `snake_case`; JSON tags and fields use `camelCase`.
  Unknown command tags/fields, duplicate keys, wrong shapes, floats in integer
  fields, and out-of-range IDs/counts are rejected. All fields listed below
  are required unless stated otherwise.
- Revision and turn counters use checked `u64` internally and canonical decimal
  strings on the wire. Offer IDs are the decimal revision at offer creation.
  Reject signs, whitespace, leading zeros except `"0"`, and overflow.
- `apiVersion = 1`, `schemaVersion = 1`,
  `rulesVersion = "hex-traders-base-v1"`, and
  `randomAlgorithm = "chacha12-rand010-v1"` identify independent contracts.
  Version 1 includes the approved RQ1/RQ2 card policies in SPEC-base-rules.md.

### Command envelope

```json
{
  "apiVersion": 1,
  "actor": 0,
  "expectedRevision": "12",
  "command": { "type": "buildRoad", "edge": 25 }
}
```

`actor` is inserted or checked by the host; a client-supplied seat is not proof
of identity. The revision must exactly match the current game. Every accepted
command increments it once, including negotiation. A stale rejection contains
the current public revision and prompts a fresh view.

Commands are not automatically retried: the facade has no persistent request-ID
store. Resubmitting an accepted envelope produces a stale-revision rejection,
never a second effect. If a response is lost, refresh state; do not blindly
replace its expected revision and retry. A transport needing recoverable
receipts owns request deduplication outside the rules library.

### Complete command catalogue

All commands below are variants of one shared `Command` type. `card` identifies
an owned card of the stated kind. Card eligibility follows BR025–BR032.

| Wire tag | Payload besides `type` | Permitted phase / actor |
|---|---|---|
| `placeSetupSettlement` | `vertex` | SetupSettlement; current setup seat. |
| `placeSetupRoad` | `edge` | SetupRoad; same seat, adjacent to its pending settlement. |
| `rollDice` | None | PreRoll; active seat. Dice are generated by the engine. |
| `discardResources` | `resources` | Discard; a seat with an outstanding obligation, exact required total. |
| `moveRobber` | `hex` | RobberMove; active seat, a different hex. |
| `chooseRobberVictim` | `victim` | RobberVictim; active seat, one listed opponent. |
| `placeFreeRoad` | `edge` | FreeRoads; active seat. A legal second placement is mandatory; impossible continuation ends automatically under RQ1. |
| `buildRoad` | `edge` | Action; active seat, pays normal cost. |
| `buildSettlement` | `vertex` | Action; active seat, pays normal cost. |
| `upgradeCity` | `vertex` | Action; active seat, replaces an owned settlement. |
| `buyDevelopmentCard` | None | Action; active seat. Engine selects the next hidden deck card. |
| `playKnight` | `card` | PreRoll or Action; active seat. Begins robber effect unless victory occurs. |
| `playRoadBuilding` | `card` | PreRoll or Action; active seat. Begins free placements; rejects without consumption if no legal road/piece exists. |
| `playMonopoly` | `card`, `resource` | PreRoll or Action; active seat. Choice and complete transfer are atomic. |
| `playInvention` | `card`, `resources` | PreRoll or Action; active seat. Require two bank-available cards, or the sole remaining bank card; reject empty-bank activation without consumption. |
| `bankTrade` | `giveResource`, `receiveResource` | Action; active seat. Engine calculates the best rate and exchanges for one card. |
| `proposeTrade` | `recipient`, `give`, `receive` | Action; both parties exist and differ, and active seat is one party. Bundles are from proposer's perspective. |
| `acceptTrade` | `offerId` | Action; current offer's recipient only. |
| `rejectTrade` | `offerId` | Action; current offer's recipient only. |
| `cancelTrade` | `offerId` | Action; current offer's proposer only. |
| `endTurn` | None | Action; active seat, no unfinished mandatory effect. |

No command supplies score, dice, a random draw, bank balances, winner, or next
phase. There is no manual VP play: immediate victory checks reveal the winner's
VP cards. No free-road forfeiture command exists under the approved RQ1 policy.

### Trade-offer behavior

These are approved digital negotiation policies, not new tabletop rules:

1. Keep at most one outstanding bilateral offer. Give and receive are both
   positive bundles with disjoint resource kinds. Validate the proposer's own
   give bundle now; do not expose the recipient's hand by validating their
   holdings in somebody else's proposal query. No escrow or transfer occurs.
2. A new proposal replaces the current offer if the proposer is the active
   seat or already participates in that offer. A different inactive seat gets
   `offerBusy`. Replacement emits closure then creation events in that order.
   Counteroffers reverse perspective and make their author the new proposer.
3. All seats and spectators see the same offer ID, parties, and terms. The
   recipient explicitly accepts or rejects. The proposer may cancel. An active
   player who received an offer can reject it before selecting another party.
4. Acceptance rechecks phase, turn, ID, revision, both current holdings, and
   both bundles, then transfers everything atomically. Insufficient holdings
   cannot produce a partial exchange. Failure leaves the offer intact and
   returns a generic `offerUnavailable` without the other player's counts.
5. Close an offer on successful acceptance/rejection/cancellation/replacement,
   on any resource-hand change to either party, on leaving Action for a card
   effect, on turn ending, or on victory. Hand-change closure is unconditional,
   so its reason does not reveal whether a hidden hand could fund the offer.
   Rejected commands never close it. A new port building does not reserve cards.
6. Offers have no time limit, promise, side payment, or future delivery. Every
   accepted negotiation command advances revision. Refresh an outdated view
   before replying to an offer; an old offer ID cannot identify its replacement.

### Queries and required choices

`legal_actions` returns action kinds, eligible owned card IDs, and the applicable
bounded choices: setup/build/free-road edges or vertices, robber destinations,
victims, the caller's discard quantity, available bank-trade resource pairs,
and offer-response availability. Results are revision-tagged and deterministic.
Separate cheap queries from a full view; do not enumerate every resource
bundle, discard combination, trade, or Invention choice on every update.
`validate` checks a specific fully supplied choice without executing it.

Phase views identify active/setup seat, outstanding discard seats/quantities,
remaining free roads, robber origin and continuation, and who must act next.
Pending victim IDs are public because adjacency is public. Legal card IDs and
hand-specific choices belong only to their owner. Spectators have no seat-specific
legal-action query. Queries neither mutate state nor consume randomness.

Candidate queries call shared predicates, not a second interpretation of a
rule. A candidate can become stale; only execution against current revision
commits it. A validation verdict is not a reservation or a prepared transaction.

### Viewer-safe state

`Viewer` is either `Seat(PlayerId)` or `Spectator`. Construct a view from an
explicit allowlist; do not serialize the private state and remove a few fields.

| Data | All viewers | Owning seat only / trusted host |
|---|---|---|
| Board | Stable IDs, incidence, terrain, numbers, ports, roads/buildings, robber. | None additional needed by a UI. |
| Turn | Revision, setup mode/order, phase and public obligations, active seat, turn, last gameplay dice, winner. | Private full snapshot stays host-only. |
| Resources | Each seat's total hand count; bank availability by resource. | Exact own five-resource hand; full hands host-only. |
| Development | Hand count, deck remaining count, played Knights and spent progress kinds. | Own unplayed card ID/kind/acquisition eligibility. No opponent unplayed IDs, kinds, purchase-card identity, or deck order. |
| Scoring | Buildings, award holders, road lengths, played Knight counts, public score; winner's revealed VP cards and winning score after completion. | Own true score including hidden VP; other seats' hidden scores and final score breakdowns host-only. |
| Negotiation | Current offer and its public terms. | Affordability-derived choices only for the relevant seat. |
| Randomness | Resolved public dice and starting-seat rolls. | Seed, RNG position, future draws always host-only. |

The bank's five counts are physically countable shared supply. Making them
explicit is the approved digital information policy. Opponent resource types
can be inferred from public history; views still must not reveal their private
discard or theft types. A UI is free to retain its already received public facts.

### Events, results, and errors

An accepted private receipt contains versions, actor, accepted command, previous
and new revisions, and ordered typed events. It is not automatically a player
response. A viewer response contains only new revision and projected events;
the UI can request a view when necessary. Event categories are:

| Event fact | Projection |
|---|---|
| Setup/layout, starter rolls, turn/phase changes | Public, with bounded graph and phase payloads. |
| Production and starting resources | Public allocations by resource; determined by public board. |
| Dice, road/settlement/city placement, paid costs | Public. |
| Discard | Public seat and total count; types only to discarding seat. |
| Robber move/victim/theft | Public hex, parties, transferred count (0 or 1); resource type only to thief and victim. |
| Development purchase | Public buyer and remaining deck count; card identity/kind only to buyer. |
| Development play | Public card kind and effect, including Monopoly resource/transfers and Invention choices. |
| Offer creation/closure and completed trade | Public parties, ID, terms/exchange; closure reason from the generic policy above. |
| Award/victory | Public previous/new holder, winner, winning score and revealed winner VP cards. |

Do not echo a raw command envelope in public events: discards and owned card
IDs contain private input. Project receipts using their original facts, not a
later position. Events within a receipt use deterministic ordering: direct
action, automatic economy/effect results, offer closure, award changes, then
phase/turn/victory. No event after `gameFinished` asks for further decisions.

Rejections use one error shape: `code`, `currentRevision`, optional public
`field`, and optional public `ruleId`. Text is diagnostic, not something a UI
must parse. Native errors are typed variants. Stable code families cover:

- Shape/version/range: `invalidInput`, `unsupportedApiVersion`, `invalidSeat`,
  `invalidHex`, `invalidVertex`, `invalidEdge`, `invalidCard`.
- Context: `staleRevision`, `gameFinished`, `wrongActor`, `wrongPhase`,
  `counterExhausted`.
- Rules: `occupied`, `distanceRule`, `disconnected`, `notOwned`,
  `insufficientResources`, `noPieces`, `emptyDeck`, `cardNotPlayable`,
  `cardAlreadyPlayedThisTurn`, `invalidDiscard`, `invalidVictim`, `bankShortage`,
  `invalidTrade`, `offerBusy`, `offerNotFound`, `offerUnavailable`.
- Persistence/replay: `unsupportedSchemaVersion`, `incompatibleRulesVersion`,
  `incompatibleRandomAlgorithm`, `invalidSnapshot`, `replayMismatch`.

Error precedence is deterministic: external shape/version/ranges, revision,
finished state, actor/phase, then action rules. Missing or foreign card IDs use
a generic `cardNotPlayable`; never reveal who owns that card or its kind.
Seat-scoped validation cannot query another player's affordability. Corrupt
host-private imports may produce detailed host-only diagnostics.

### Snapshot and replay formats

Snapshot schema 1 is a JSON envelope containing schema/rules/RNG versions,
validated config and layout identity, full immutable board layout, position,
revision, and private RNG seed plus decimal word position. Topology is rebuilt
from stable graph IDs rather than trusted imported adjacency. Stream is zero.
The position includes all pending continuations and any outstanding offer.
Seed bytes are exactly 64 lowercase hexadecimal characters. Counters follow
the decimal convention above; RNG position is checked below `2^68`.

Import rejects unknown versions, duplicate/unknown fields, bad distributions,
bad owners/IDs, impossible phase payloads, inconsistent stocks/locations/awards,
and other model invariants before activation. Derived values are recomputed
and compared rather than trusting imported scores or caches. These checks
establish a valid state, not proof that a malicious host played honestly.
Fixed layouts must equal the pinned fixture. Variable layouts must identify
their source-frame permutation and match its port positions plus the pinned
number spiral for the supplied terrain; arbitrary valid-count port/token
layouts are not accepted as this setup mode.

Bound snapshot input to 64 KiB before parsing and structural depth to 16; fixed
arrays have exact base-game lengths. Keep strings/enums bounded. A valid output
must round-trip within those limits. Import constructs a separate engine, so
failure cannot overwrite an existing one. The library returns bytes/values;
it performs no file writes, encryption, database operations, or autosaving.
The host owns atomic storage and secret access. Byte-stable export within
schema 1 uses declared field order and no unordered maps.

Replay format 1 contains an initial snapshot plus accepted envelopes in order,
their previous/new revisions, and the expected ordered private events. The
host can retain snapshots for checkpoints; current state contains no log.
Verification executes each command through the ordinary engine, checks exact
revision progression and event equality, and returns the first rejection or
mismatch. Compare final snapshots in golden fixtures; an expected final
snapshot may also accompany an exported replay. No cryptographic proof or
event-sourced transition system is required.

Rejected commands are absent from accepted replay history. Resuming a saved
position does not need old history. Full replay contains hidden inputs and the
seed and stays host-private. Player-visible activity history is made from
projected receipts; it is not sufficient to reconstruct the hidden engine.
Replay input can be streamed record by record with the host limiting length;
each record has a 64 KiB maximum and depth 16. No unbounded log parser is needed.

Native and eventual Wasm consumers share the same version contracts. Incompatible
snapshots/replays reject explicitly; version 1 promises no speculative migration
framework. A concrete format change must define migration or an explicit break
with tests before release. An added enum variant can break exhaustive Rust
matches, so additive wire changes alone are not a native compatibility promise.

## Project Structure

The original illustrative file plan under `src/engine_api/` was: `mod.rs`, `engine.rs`, `view.rs`, `wire.rs`,
`snapshot.rs`, `replay.rs`, and `error.rs` where justified. Re-export public
types from `src/lib.rs`; keep raw position internals and private receipts out
of accidental serialization. Integration tests: `tests/api_contract.rs`,
`tests/privacy.rs`, `tests/snapshots.rs`, `tests/replay.rs`.

## Code Style

Use typed inputs/results, small exhaustive projections, and explicit conversion
at the external boundary. Example of projection intent:

```rust
let private_hand = match viewer {
    Viewer::Seat(seat) if seat == player => Some(position.hand(player)),
    _ => None,
};
```

Return an owned bounded view when requested; do not expose a mutable reference
to internal state. Avoid a generic transport framework, per-command services,
REST endpoints, or handwritten JSON parsing. Follow the root style/constraints.

## Commands

Focused local checks:

```bash
cargo test --locked --test api_contract
cargo test --locked --test privacy
cargo test --locked --test snapshots
cargo test --locked --test replay
cargo bench --locked --bench engine view
cargo bench --locked --bench engine snapshot
cargo doc --locked --no-deps
```

## Testing Strategy

- Contract fixtures cover every command tag, response/error shape, counter
  precision, wrong versions, unknown/duplicate fields, malformed and huge inputs.
- Execute full setup-to-victory games using only public contracts. Compare legal
  candidates with acceptance at the same revision. Test two competing commands,
  lost-response resubmission, stale offers, replacement, and atomic acceptance.
- Privacy tests compare each seat and spectator across every view/event/error,
  including another player's purchase, private discard, theft, hidden VP, and
  later projection of an older receipt. Keep private data out by construction.
- Round-trip every phase, shuffle/deck cursor, acquired-card turn, random word
  position, offer, award tie, and finished game. Mutate snapshots to violate
  bounds/invariants and assert rejection without losing an existing engine.
- Replay all accepted commands, including negotiation/discards, from initial
  state and midgame snapshots. Compare revisions, events, final state, and next
  random output; altered commands or events identify the first divergence.

## Boundaries

- Always: validate at trust boundaries, derive viewer/actor from host context,
  reuse rule validation, preserve atomicity and private data, version contracts.
- Ask first: break a public command/view/snapshot contract, change information
  or negotiation policy, or add a transport/storage/binding dependency without
  a concrete consuming requirement.
- Never: serialize raw state to a player, publish seed/private replays, trust
  client scores/dice, use errors to expose another hand, or write storage here.

## Success Criteria

- Any consumer can complete both setup modes and every mandatory decision
  through this contract without its own rule implementation.
- Only authorized host-selected viewer data leaves the facade; full private
  receipts and snapshots are visibly separate operations.
- Save/resume and replay preserve state, revision, continuation, and RNG results.
- All accepted/rejected commands and offer interactions have stable semantics.

## Open Questions

RQ1/RQ2 are resolved in [SPEC-base-rules.md](SPEC-base-rules.md). Trade,
bank-visibility, and starter-tie policies are approved with the planning baseline.
