# Spec: base-rules

Status: approved as the planning baseline on 9 November 2026. Depends only on
`game-model`. The user subsequently authorized autonomous implementation;
[tasks/todo.md](tasks/todo.md) records verified status.

## Objective

Implement the official 2025 English sixth-edition base game for 3–4 players as
reusable, deterministic calculations. This is the normative rules catalogue;
each numbered rule must have a conformance test. Turn sequencing and command
acceptance combine these rules in `turn-engine`.

Primary source: 2025 rulebook, CN3081, v6.250401 (original source URL privately preserved),
pages 3–12. Compatible clarifications come from the
official base-game FAQ (original source URL privately preserved), checked on
9 November 2026. Sixth-edition text takes precedence over older FAQ advice.
The FAQ contains extension-specific entries; those are outside this ruleset.
All rules below are written as engine requirements, not reproduced book text.

## Provider contract

Functions borrow `Board` and `Position` and return a calculation or typed
rejection. They do not choose the current actor, advance phases, draw random
values, send events, or commit a game. They supply:

- Setup and paid/free placement validation and candidate locations.
- Building costs, affordability, inventory availability, and port rates.
- Production claims and the bank-shortage allocation for a dice result.
- Discard obligations, legal robber destinations, eligible victims, and
  resource transfer calculations for already resolved random outcomes.
- Development-card eligibility/effects and resulting economy/board changes.
- Longest-road lengths, award ownership reconciliation, and score calculations.

Rule calculations may construct small value results. Do not create a generic
rule-object framework, scripting language, or separate runtime for each rule.
Phase-dependent validation is owned by the turn engine, which calls these
calculations once for an accepted action.

## Rules Catalogue

### Components and initial setup

| ID | Required behavior | Source |
|---|---|---|
| BR001 | Exactly 3 or 4 players; each has 15 roads, 5 settlements, and 4 cities. | p.3 |
| BR002 | Terrain multiset: 3 hills, 4 forests, 4 pastures, 4 fields, 3 mountains, 1 desert. Each non-desert terrain maps to its one resource. | pp.3,6 |
| BR003 | Number multiset: one 2 and one 12; two each of 3,4,5,6,8,9,10,11. Desert has no number. Supply starts with 19 cards of each resource and 25 development cards: 14 Knights, 2 Monopoly, 2 Road Building, 2 Invention, 5 VP. | p.3 |
| BR004 | Nine ports: four generic 3:1 and one 2:1 for each resource. Access requires the player's settlement or city at either endpoint of that port's coastal edge. | pp.3,7 |
| BR005 | Fixed setup uses the terrain, tokens, ports, and two settlement/road pairs shown on pp.4–5. In a 3-player fixed game omit the white preset role. Starting resources come only from the preset's designated second settlement. | pp.4–5 |
| BR006 | Variable setup shuffles terrain and the six printed sea-frame templates; retains each template's port spacing; assigns the A–R number sequence counterclockwise from a corner, spiraling inward and skipping desert. The robber begins on desert. Arbitrary independent shuffling of all number tokens is not this setup procedure. | p.11 |
| BR007 | Determine the starting player by two-dice comparison. Variable setup then places one settlement and adjacent road per player in order, followed by one settlement and adjacent road in reverse order. The last player places their two pairs consecutively across the round boundary. All setup placements are free. | pp.5,12 |
| BR008 | Every starting settlement obeys the distance rule. Its paired road must be adjacent to that newly placed settlement. The two starting settlements need not be connected. Give one resource per adjacent non-desert hex for the second settlement only; no production roll or city multiplier is involved. | p.12 |

The number sequence for BR006 is A–R:
`5, 2, 6, 3, 8, 10, 9, 12, 11, 4, 8, 10, 9, 4, 5, 6, 3, 11`.
Validate the sequence against the disc-letter fixture during implementation;
do not substitute a shuffle or silently add a balance/reroll rule.

Version 1 uses the top-right corner (HexId 2), permitted by the source's
any-corner rule. The counterclockwise spiral of hex IDs is
`2,1,0,3,7,12,16,17,18,15,11,6,5,4,8,13,14,10,9`.
Skip the desert without consuming a letter. This fixed starting corner is a
digital policy, not a claim that the book requires that one corner.

The fixed terrain/number layout, in `game-model`'s ascending `(r,q)` order, is:

```text
r=-2: desert/—, field/8, pasture/11
r=-1: hills/6, pasture/3, forest/4, mountain/9
r= 0: forest/10, field/5, hills/12, forest/11, pasture/5
r= 1: field/2, hills/9, mountain/4, forest/8
r= 2: mountain/6, field/3, pasture/10
```

For fixed setup, the source diagram's settlement vertices can be identified
unambiguously by the triples of adjacent hex IDs below. The source's road
orientation and coastal ports must also be captured and visually checked in
the fixture; hex triples alone do not specify a road or a port.

| Preset role | First settlement's hex triple | Second settlement's hex triple | Starting hand |
|---|---|---|---|
| Blue | 10,11,15 | 13,14,17 | brick, ore, wheat |
| Red | 12,13,16 | 1,2,5 | wheat, wool, wood |
| Orange | 5,6,10 | 3,4,8 | brick, wool, wheat |
| White, 4-player only | 14,15,18 | 8,9,13 | wheat, brick, brick |

The captured source IDs, cross-checked against the graph and the printed
illustrations, are first settlement/road then second settlement/road:
Blue `31/46, 40/57`; Red `39/55, 9/14`; Orange `19/21, 17/25`;
White `41/59, 29/42`. Fixed ports clockwise from the northwest are
`0 generic, 3 wool, 17 generic, 38 generic, 61 brick, 69 wood,
66 generic, 49 wheat, 18 ore`, where each ID is the coastal edge.

The six sea-frame pieces retain their printed port spacing when shuffled.
In clockwise order, the six five-edge coastal slots are
`[0,1,2,3,4]`, `[5,9,17,22,32]`, `[38,48,53,61,65]`,
`[71,70,69,68,67]`, `[66,62,54,49,39]`, and `[33,23,18,10,6]`.
Numbering a template by its printed protruding-tab label, its zero-based
port offsets and kinds are: `1: 0 generic, 3 wool`; `2: 2 generic`;
`3: 0 generic, 3 brick`; `4: 2 wood`; `5: 0 generic, 3 wheat`;
`6: 2 ore`. Fixed order `1,2,3,4,5,6` reproduces the diagram. These
IDs and capture notes are recorded in [the source fixture](tests/support/setup.json)
and [capture notes](tests/support/setup-source.md); images remain references,
not product assets. This fixture verifies source data; separate tests verify gameplay.

Preset roles are fixture identities, not account IDs. The caller maps seats
to Blue, Red, Orange, and optionally White for this setup. Any role can become
the starting player. Variable setup does not assign these preset positions.

### Turns, production, and a seven

| ID | Required behavior | Source |
|---|---|---|
| BR009 | Turns follow seat order. Each turn has production before action. Before rolling, the active player may play one eligible development card. In the action phase, trading, paid building, buying development cards, and playing an eligible card can occur in any order. | pp.6–7,9 |
| BR010 | Roll two independent uniform six-sided dice. A non-seven total produces on every matching unblocked hex for every player, regardless of whose turn it is. A settlement earns one resource per producing adjacent hex; a city earns two. Multiple buildings and matching hexes accumulate. | p.6 |
| BR011 | Compute total claims by resource before distributing. If stock covers all claims, pay everyone. If stock is short and two or more distinct players claim that resource, pay none of that resource. If exactly one player claims it, pay that player the remaining stock up to their claim. Other resource types are unaffected. | p.6; FAQ resource shortages |
| BR012 | A seven produces nothing. Every player with more than 7 resource cards discards floor(hand-total / 2), choosing which resource cards. Exactly 7 does not discard. Development cards do not count. Determine each obligation once, then complete all discards before robber movement. | pp.6,9; FAQ seven |
| BR013 | After a seven the active player must move the robber to a different land hex; desert is legal. The current hex is illegal even if the robber is briefly removed. Robber movement does not cost resources and does not change ownership or buildings. | p.6; FAQ robber |
| BR014 | Select at most one opponent with a building on the destination hex and attempt to take one uniformly random card from that opponent's resource hand. Multiple buildings owned by that opponent do not grant more steals. Never steal development cards. An adjacent opponent with no resource cards is still a selectable victim; the attempt transfers nothing. With no adjacent opponents, no steal occurs. | p.6; FAQ desert and empty victim |
| BR015 | A robber blocks that hex's production only. It does not block building, ports, trade, other hexes of the same resource, or that player's resource use. A Knight-triggered robber activation does not trigger discards. | pp.6,9; FAQ robber and Knight |

Do not choose a resource type uniformly for BR014: choose among actual cards,
weighted by the counts in the selected hand. For example, 3 wood and 1 ore
means a 3/4 wood probability, not a 1/2 probability.

### Building and trading

| Action | Brick | Wood | Wool | Wheat | Ore | Piece/deck requirement |
|---|---:|---:|---:|---:|---:|---|
| Road | 1 | 1 | 0 | 0 | 0 | A remaining road |
| Settlement | 1 | 1 | 1 | 1 | 0 | A remaining settlement |
| City upgrade | 0 | 0 | 0 | 2 | 3 | A remaining city and an owned settlement |
| Development-card purchase | 0 | 0 | 1 | 1 | 1 | At least one card left in the deck |

| ID | Required behavior | Source |
|---|---|---|
| BR016 | Paid construction/purchase returns the full cost to the bank. Insufficient resources, exhausted piece stock, or empty deck rejects the action without a partial payment. There is no per-turn purchase cap beyond legal placement and available resources/pieces. | pp.8–9 |
| BR017 | A new road occupies an empty edge and connects at an endpoint to an owned building or owned road. An opponent's building at that endpoint prevents continuation through it. Existing roads beyond a new blocking building remain owned and can be extended from their own unblocked endpoints. | p.8; FAQ roads |
| BR018 | A new settlement occupies an empty vertex, has no building at any adjacent vertex, and during normal play touches at least one owned road. Own and opponent buildings both enforce distance. Coast and port vertices are valid. No reservation mechanism exists. | p.8; FAQ settlements |
| BR019 | A city replaces only that player's existing settlement at the same vertex. Return that settlement piece to the player's stock. A city is worth two points total and produces two cards; do not count its replaced settlement as an additional building. | p.9 |
| BR020 | Buildings and roads cannot be voluntarily removed, moved, refunded, or converted back. Setup disconnection is the sole starting-placement exception; new paid settlements/roads must meet normal connectivity. Opponent roads do not by themselves stop a road touching a shared empty vertex. | pp.8–9; FAQ settlements/roads |
| BR021 | A player-to-player trade is immediate, bilateral, and exchanges positive resource bundles on both sides. The active player must be one party. Other players may propose/counteroffer, but cannot trade among themselves or with the bank on another player's turn. | p.7; FAQ triangular trade |
| BR022 | No gifts, development-card trades, credit, services, or secret exchanges. The same resource type must not occur on both sides of an exchange. Terms and completed exchanges are public. An offer is not a resource transfer; holdings are checked again at acceptance. | p.7; FAQ trade |
| BR023 | Bank trade converts 4 cards of one resource into 1 of a different resource. Use 3:1 with an owned generic port, or 2:1 for the resource named by an owned special port. Use the best eligible rate; rates do not stack. A city gives the same port rate as a settlement. | p.7 |
| BR024 | A new port building can immediately improve later bank trades in that action phase. The requested card must exist in the bank. One command performs one receive-card exchange; repeat it to trade multiple groups or resource types. | pp.7–8; compatible FAQ trade |

The engine does not adjudicate conversation, future promises, or strategy.
Actual exchanges must meet BR021–BR024. Trade-offer lifetime and acceptance
messages are specified as digital behavior in `engine-api`, not additional
tabletop mechanics.

### Development cards

| ID | Required behavior | Source |
|---|---|---|
| BR025 | Buy the top shuffled deck card, hidden from other players. Never return a bought/played card to the deck. Reject purchases when the deck is exhausted. Any number may be bought if affordable. | p.9 |
| BR026 | During the active player's own turn, play at most one non-VP development card. It must have been acquired on an earlier turn. It may be played before the dice or during the action phase, never during setup, discards, robber resolution, or another card's unfinished effect. A pre-roll play consumes the same per-turn allowance. | pp.6,9; FAQ development timing |
| BR027 | Knight: reveal the card, count it as played, and activate the robber with normal destination/victim rules but no discard. Played Knights count for Largest Army and remain public. | p.9; FAQ Knight |
| BR028 | Monopoly: choose exactly one resource; transfer every opponent's entire count of that type to the active player. The bank is not involved. Zero collected cards is a valid result. | p.9 |
| BR029 | Invention: choose two available bank resources, the same type or two types, and transfer them to the player. Do not grant a resource the bank lacks. If the entire bank has one card, take that one; if empty, reject activation without consuming the card or turn allowance. | p.9; approved engine policy RQ2 below for exhaustion |
| BR030 | Road Building: build two roads without resource payment, using normal connectivity, occupancy, and piece-stock rules. The second placement sees the first placement and may extend it. Placements need not be in the same network. No intervening unrelated action is allowed. Reject activation if no legal road/piece exists; after a placement, finish automatically if no further legal road/piece exists. A still-legal second road is mandatory unless victory ends play. | p.9; approved engine policy RQ1 below for exhaustion/forfeit |
| BR031 | Played progress cards leave the player's hand and cannot be played again. They remain outside the deck. VP cards are not ordinary playable effects and do not consume the non-VP allowance. | p.9 |
| BR032 | Hidden VP cards count toward the owner's true score. They may be revealed, including cards acquired this turn and multiple cards together, when the owner wins on their turn. Otherwise they stay hidden. No non-VP same-turn exception exists. | pp.9–10; FAQ VP |

Eligibility uses the acquisition turn of that specific card, not merely the
owner's count of that card type. Buying a Knight while holding an older Knight
must not prevent playing the old one or permit playing the new one.

### Longest Route, Largest Army, and victory

| ID | Required behavior | Source |
|---|---|---|
| BR033 | Settlement = 1 VP; city = 2 VP; each owned VP development card = 1 VP; Longest Route = 2 VP; Largest Army = 2 VP. Roads and unplayed Knights score nothing directly. | pp.2,8–10 |
| BR034 | Longest Route uses the longest continuous trail of owned road edges. Each edge may be counted once. A loop counts; branches cannot simply all be summed. Own buildings do not break continuity. An opponent building is a legal endpoint but traversal cannot continue through it. | p.8; FAQ Longest Road |
| BR035 | The road award requires length at least 5. An incumbent keeps it when still tied for the longest qualifying route. A strictly longer opponent takes it. If the incumbent ceases to qualify as a longest holder, assign it to a unique qualifying maximum or return it to the supply on a tie/no qualifying route. Recalculate after road placement and blocking settlement placement. | p.8; FAQ interrupted/tied road |
| BR036 | Largest Army requires at least 3 played Knights. The first qualifying player receives it; a challenger must have strictly more played Knights to take it. A tie leaves the current owner unchanged. Unplayed Knights never count. | p.9; FAQ army |
| BR037 | End immediately when the active player has at least 10 true points, including hidden VP cards. Check at turn start before any roll and after every score-changing step. Getting 10 points on somebody else's turn is not a win until that player's own turn starts. Another eligible active player can win before that happens. | p.10; FAQ victory |
| BR038 | Immediate victory supersedes unfinished choices: e.g. a first free road that grants 10 points ends the game without placing the second. Reveal the winner's VP cards and freeze gameplay. No final extra turn, extra building round, or continuing score collection. | p.10 immediate ending |

### Longest-road algorithm requirement

Use an exact edge-visited depth-first traversal of each player's owned-road
graph, with a small visited-edge bitset and maximum depth 15. A revisited vertex
is not automatically forbidden: loops must count correctly. Stop when entering
an opponent-occupied vertex, except that a trail can begin there and leave once.
Evaluate all needed starting endpoints so cycles and blocked endpoints are not
missed. Do not substitute shortest path, component size, or total owned roads.

Recompute a changed player's length after a road addition. A newly placed
settlement may affect opponents whose roads touch its vertex; update those
lengths and reconcile the award. City upgrades do not introduce a new blocker.
Cached lengths, if kept, have one calculator and tests against full recomputation.
Unrelated economy actions must not cause another longest-road search.

## Project Structure

The original illustrative file plan under `src/base_rules/` was: `mod.rs`, `setup.rs`, `building.rs`,
`production.rs`, `trade.rs`, `robber.rs`, `development.rs`, `scoring.rs`, and
`longest_road.rs`. Avoid splitting tiny helpers into standalone files.
Conformance tests live in `tests/rules_conformance.rs`; hand-built board
fixtures live in `tests/support/`. Every fixture identifies its source or the
independent expected calculation.

## Code Style

Use borrowed inputs, direct returns, explicit quantities, and one helper per
shared calculation. The intended score expression is straightforward:

```rust
pub fn score(parts: ScoreParts) -> u8 {
    parts.settlements
        + 2 * parts.cities
        + parts.victory_cards
        + 2 * u8::from(parts.has_longest_route)
        + 2 * u8::from(parts.has_largest_army)
}
```

Validated base-game quantities make this bounded; imported quantities are
validated first. Do not suppress overflow or validation errors to get a faster
benchmark. Other style and quality rules are inherited from the root specs.

## Commands

Focused local checks:

```bash
cargo test --locked base_rules_
cargo test --locked --test rules_conformance
cargo bench --locked --bench engine native/road_hard
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
```

Conformance test names use `base_rules_br001_...` through
`base_rules_br038_...`. One rule may need multiple scenarios.

## Testing Strategy

- Every BR ID has a directly asserted example. Rejectable rules also have an
  illegal-action case, including no partial payment or card consumption.
- Production covers settlement/city multipliers, multiple matching hexes,
  robber blocking, stock exact/short/zero, and one-versus-several claimants.
- Placement covers all boundary vertices, interrupted networks, opponent
  blockers, duplicate edges, distance, depleted pieces, and returned settlements.
- Trades cover all three rates, overlapping bundles, absent bank cards,
  inactive actors, no gifts/credit, and an immediately usable new port.
- Card tests cover old/new cards of the same kind, pre-roll play, a seven after
  a pre-roll Knight, no development-card theft/discard, and empty results.
- BR029 exhaustion tests cover zero/one/two available bank cards, two of one
  resource versus two types, insufficient selected stock, and an unchanged
  card/turn allowance on rejection. BR030 tests cover zero/one/two remaining
  road pieces, no legal site, a first road that leaves no legal second site,
  mandatory second placement when possible, and immediate first-road victory.
- Longest-road fixtures cover a chain, a branch, a six-edge loop, a loop with
  a tail, touching loops, own/opponent buildings, bypasses, length 4/5/15, and
  incumbent/no-incumbent ties. Expected paths are independent of production DFS.
- Victory covers a newly bought VP, award transfer on another player's turn,
  a win before rolling, and a win during a multi-step card effect.

## Boundaries

- Always: trace rule tests to the catalogue and official sources; reuse rule
  validation for execution and queries; preserve resource/piece conservation.
- Ask first: introduce a house rule, change the pinned edition, change an
  approved card policy, or replace exact
  road calculation with an approximation.
- Never: import `turn-engine`, `engine-api`, UI, storage, OS randomness, or a
  timer; enforce extension rules in this base game; use hidden information as
  a public error message.

## Success Criteria

- BR001–BR038 have passing traceable conformance cases.
- Shared validators give the same answer to command execution and candidates.
- Exact scoring and award transfer pass all graph/timing fixtures.
- Approved engine interpretations are distinguished from sourced official rules.

## Approved edge-case decisions

The main game rules are pinned. The consulted sixth-edition text and official
base-game FAQ do not explicitly settle these exhaustion/forfeit cases. On
9 November 2026 the user authorized choosing the best behavior for both cases.
The following engine policies are therefore approved, without claiming an
official clarification:

1. **RQ1, resolved — Road Building:** reject activation when no road piece or
   legal site exists, preserving the card and non-VP turn allowance. Otherwise
   play the card and place roads sequentially, up to two. After the first road,
   check victory, then current legal sites and piece stock. Finish immediately
   on victory or when a second road is impossible; otherwise require it.
   A player cannot voluntarily forfeit a legal second road. No lookahead search
   rejects a legal first placement merely because it leaves no second site.
2. **RQ2, resolved — Invention:** when the entire bank has one card left, take
   that card. When empty, reject activation, preserving the card and non-VP turn
   allowance. Otherwise require exactly two cards from available stock, allowing
   the same resource twice. Invalid resource choices reject atomically.

These policies complete the printed effects as far as supplies and legal
placements allow, prevent spending a card on an impossible effect, and avoid
an extra forfeiture command. Conformance tests must cover them alongside the
sourced rules. They require no configurable rule-variant mechanism.

## Open Questions

None for the card cases. The complete planning baseline is approved.
