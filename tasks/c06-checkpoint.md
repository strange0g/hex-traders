# C06: Card ownership and resource effects

**PUBLIC DERIVATIVE — historical summary.** This records the 10 November 2026
checkpoint, not a new execution or verification of the release tree. Historical commit
IDs identify privately preserved source revisions; they are not public release-history
links. The original report, raw logs, oracle inputs and reviews remain privately
retained byte for byte under T35. T41 reconciled the original report's SHA-256
(`d94359b6f415da82ad999ac867ddbf8e5e6e4f2d5cf69bceda598f6fe4df2f45`) with the private
preservation inventory and checked the relevant archived evidence hashes. No runtime
engine checks were rerun for T41. Private machine paths, raw archive/index links and
original source URLs are omitted from this derivative; no raw log has been renamed as
fresh evidence. Previously excluded generated build trees and nonexistent tool-error
controls remain exclusions.

Rule provenance: [approved base rules](../SPEC-base-rules.md), BR016, BR025–BR026,
BR028–BR029, BR031–BR033 and BR037–BR038; approved policy RQ2. The approved 2025 English
sixth-edition baseline is document `CN3081`, revision `v6.250401`, pp.8–10, with the
compatible FAQ qualifications recorded in the catalogue. Requirements are paraphrased;
no publisher endorsement is claimed. Historical task IDs and completion context are
retained in [todo.md](todo.md).

Verified on 10 November 2026 on `codex/build-engine`; source/test HEAD is `0dcfbb5`. The
approved autonomous build continues under the existing plan.

## Delivered capability

Buying a development card pays one wool, wheat and ore, advances the existing shuffled
deck cursor once and records the specific owner's acquisition turn. Other players and
spectators see only buyer and deck/hand counts; the owner sees ID and kind. Old and new
cards of the same kind remain distinct. Held victory cards count immediately for the
owner's true score; a winning purchase reveals only the winner's VP cards and freezes
the game. Host-private receipts retain final scores; viewer projections preserve
original event facts and revisions.

Monopoly and Invention use the same individual-card eligibility check: active seat,
PreRoll or Action, an earlier-turn owned card of the matching kind, and no previous
non-VP card this turn. Rejections preserve the entire position, RNG and revision. A
successful play spends its canonical card, consumes the shared allowance and preserves
the phase. Turn start clears that allowance. Public views show spent progress kinds by
owner; old and new receipt projections retain the public effect without exposing another
unplayed card.

Monopoly transfers every opponent's chosen resource count, including zero, and leaves
the bank unchanged. Invention requires exactly two bank-available cards, including two
of one kind, when at least two remain; it takes the sole bank card when one remains; an
empty bank or invalid selection rejects without consumption. All changes use checked
bounded resource arithmetic. Legal-action queries obtain eligible owned IDs through the
execution validator and a single representative available Invention choice; they do not
enumerate all resource bundles. Neither card effect consumes randomness.

## Verified commits

| Slice | Commit |
|---|---|
| Purchase, private ownership, hidden VP and victory | `4f40b64` |
| Public purchase and receipt histories | `1aef169` |
| Shared eligibility, allowance and Monopoly | `e95286e` |
| Public Monopoly histories | `d505df6` |
| Invention and approved bank-exhaustion policy | `2bc629f` |
| Public Invention histories | `0dcfbb5` |

Every slice has RED/GREEN, appropriate full gates, independent review and retained
targeted mutation evidence in [todo.md](todo.md). Public fixture slices first fail on
missing module wiring; independently calculated public histories and mutations provide
their behavioral controls.

## Historical final standing gates

All commands passed on the final Phase 6 source/tests; the checkpoint report and status
alone add no executable changes. Original gate, advisory and focused logs remain
privately retained.

| Command | Actual result |
|---|---|
| `cargo fmt --all -- --check` | Passed |
| `cargo check --locked --all-targets` | Passed |
| `cargo clippy --locked --all-targets -- -D warnings` | Passed |
| `cargo build --locked --release` | Passed |
| `cargo test --locked --lib --tests` | 109 passed: 79 library, 18 public sequences, 5 model, 7 rule conformance; none ignored |
| `cargo test --locked --doc` | Five passed, including privacy compile-fail checks |
| `cargo doc --locked --no-deps` | Passed |
| `cargo llvm-cov --locked --lib --tests --ignore-filename-regex '(^\|/)(tests\|benches)/' --fail-under-lines 90` | 99.08% library lines, 3668/3702 |
| `cargo deny --locked check advisories` | Refreshed scan passed |

The advisory database requires access to its local lock. The authorized C06 scan passed;
the earlier read-only-lock failure remains in T17a evidence and is not presented as a
pass.

All final task filters ran nonzero passing tests:

| `cargo test --locked` suffix | Passed |
|---|---:|
| `--test game_sequences development_purchase_` | 2 |
| `base_rules_br025_` | 2 |
| `--lib privacy_purchase_` | 3 |
| `--test game_sequences monopoly_` | 1 |
| `base_rules_br026_` | 4 |
| `base_rules_br028_` | 2 |
| `--test game_sequences invention_` | 1 |
| `base_rules_br029_` | 3 |

## Independent evidence and review

A pinned rand/chacha oracle and independently checked source-layout production literals
drive the public T16 purchase, T17 Monopoly and T18 Invention histories. Their original
oracle sources/outputs remain privately retained. Tests embed their values, not private
paths. Monopoly histories cover three/four seats, PreRoll/Action, transfers of four/six
brick, original receipt privacy and following dice. Invention histories cover three/four
seats, both phases, two wool or brick plus ore, original receipts, bank/hands and
following dice. Separate constructed states cover exact-two, one-card and empty-bank
outcomes. A fresh reviewer independently compared 118,098 small-bank/selection cases
across both phases to an independent oracle.

Original T16a–T16b, T17a–T17b and T18a–T18b independent review reports remain privately
retained. Required test gaps were fixed before commit. Mutation controls caught
incorrect hidden purchase/view projection, a removed allowance guard, wrong spent-card
owner, exposed held-card kinds, incorrect Monopoly transfer, RQ2 and bank shortages, and
omitted Invention credits. These are targeted controls, not a mutation-score claim.
Source and logs remain privately retained; completed reviewer targets were removed. The
historical independent C06 checkpoint review verifies the commit/evidence mapping, final
counts and import/contract audit.

Manual import review confirms model → rules → turn engine → API dependency direction.
The existing prepared bank exchange and shared score/victory hooks remain intact;
bounded card locations and explicit RNG are unchanged by history length. Compared with
C05, approved specifications, constraints, dependencies, tool configuration and pinned
RNG/shuffle are unchanged. No new suppressions, ignored tests, unsafe code or executable
stubs were introduced.

## Limits at this historical checkpoint

T16–T18 are complete. Knight/robber continuation/Largest Army, Road Building, and full
victory scenarios remain T19–T21. Public histories here prove card purchase and resource
effects, not complete setup-to-winning games. Wire and snapshot/replay, final
conformance, measured performance/memory, Wasm/bindings and hosted CI remain later or
outside this release. Nothing was published.