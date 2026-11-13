# C05: Construction and maritime economy

**PUBLIC DERIVATIVE — historical summary.** This records the 10 November 2026
checkpoint, not a new execution or verification of the release tree. Historical commit
IDs identify privately preserved source revisions; they are not public release-history
links. The original report, raw logs, oracle inputs and reviews remain privately
retained byte for byte under T35. T41 reconciled the original report's SHA-256
(`46ad4b93e4e33df8583d13bea670c4e678760495b20826a83d744cff40032857`) with the private
preservation inventory and checked the relevant archived evidence hashes. No runtime
engine checks were rerun for T41. Private machine paths, raw archive/index links and
original source URLs are omitted from this derivative; no raw log has been renamed as
fresh evidence. Previously excluded generated build trees and nonexistent tool-error
controls remain exclusions.

Rule provenance: [approved base rules](../SPEC-base-rules.md), BR004, BR016–BR020,
BR023–BR024, BR033–BR035 and BR037–BR038. The approved 2025 English sixth-edition
baseline is document `CN3081`, revision `v6.250401`, pp.3, 7–10, with the compatible FAQ
qualifications recorded in the catalogue. Requirements are paraphrased; no publisher
endorsement is claimed. Historical task IDs and completion context are retained in
[todo.md](todo.md).

Verified on 10 November 2026 on `codex/build-engine`. The approved autonomous build
continues without a change to rules, privacy, quality requirements, dependencies or
publishing authorization. The verified source/test commit is `c39bc91`; later checkpoint
documentation changes no behavior.

## Delivered capability

Paid settlements use one shared distance/owned-road/piece/cost validator for execution
and candidates. Checked brick/wood/wool/wheat payment, placement, changed-opponent route
refresh, award reconciliation and active-player victory commit in one revision with no
RNG draw. The builder's own route is unchanged. Only a non-builder with at least two
incident roads can be interrupted. A base vertex has degree at most three and the
builder needs an own road, so two opponents merely touching it cannot both have
through-routes there. Exact lengths, award-to-supply/transfer and active/off-turn timing
are verified.

City upgrades replace only an owned settlement, pay two wheat and three ore, return its
settlement piece and consume a city piece. Occupancy remains the sole stock
representation; a subsequent paid settlement can use the returned piece. The city is one
two-point building and produces two cards. Its existing blocker does not change, so the
route cache and award need no search. The shared active-score hook freezes a winning
upgrade immediately.

Bank trades calculate the best owned port rate: 2:1 for its named resource, 3:1 with a
generic port, otherwise 4:1. Both endpoints and settlements/cities grant the same
access; ports do not stack and foreign buildings grant nothing. One pure checked
BankExchange prepares rate/hand/bank from the current state. The shared internal
validator returns it only for BankTrade; native validation and queries discard prepared
success, while evaluation explicitly consumes it once. There is no persistent exchange
cache, second actor/phase validation path, new RNG draw or route search. Same-kind,
unavailable-bank and unaffordable exchanges reject atomically. New port settlements and
city upgrades apply their rate immediately. Public receipts project their original
placement/payment/ exchange/award/victory facts, including when projected after later
commands.

## Verified commits

| Slice | Commit |
|---|---|
| Atomic settlement, route blocking and score timing | `9969722` |
| Public settlement/coastal/blocking histories | `e4337c7` |
| City replacement, stock return and victory | `5c6a9ff` |
| Public upgrade and double production | `bdebff7` |
| Prepared bank exchange, rates and candidates | `b1b6c30` |
| Public 4:1/3:1/2:1 histories | `c39bc91` |

Every slice has RED/GREEN, relevant full gates, independent review and targeted mutation
evidence retained in [todo.md](todo.md). Public fixture slices first fail on missing
module wiring; their independent source/RNG expectations and behavior mutations provide
the corresponding behavioral evidence.

## Historical final standing gates

All commands exited zero against the final Phase 5 source/tests. Original logs are
privately retained. Later status/report documentation changes no behavior.

| Command | Actual result |
|---|---|
| `cargo fmt --all -- --check` | Passed |
| `cargo check --locked --all-targets` | Passed |
| `cargo clippy --locked --all-targets -- -D warnings` | Passed |
| `cargo build --locked --release` | Passed |
| `cargo test --locked --lib --tests` | 90 passed: 64 library, 14 public sequences, 5 model, 7 rule conformance; none ignored |
| `cargo test --locked --doc` | Five passed, including privacy compile-fail checks |
| `cargo doc --locked --no-deps` | Passed |
| `cargo llvm-cov --locked --lib --tests --ignore-filename-regex '(^\|/)(tests\|benches)/' --fail-under-lines 90` | 99.02% library lines, 3342/3375 |
| `cargo deny --locked check advisories` | Authorized refreshed scan passed with `advisories ok` |

The advisory database needs an outside-workspace lock/refresh; the authorized C05
invocation passed. The ordinary T13 invocation failed on a read-only lock and is not
presented as a pass. All dependencies remain unchanged.

Every final focused filter runs nonzero passing tests:

| `cargo test --locked` suffix | Passed |
|---|---:|
| `--test game_sequences settlement_` | 2 |
| `base_rules_br018_` | 1 |
| `base_rules_br035_` | 2 |
| `--test game_sequences city_` | 1 |
| `base_rules_br019_` | 1 |
| `--test game_sequences bank_trade_` | 3 |
| `base_rules_br023_` | 1 |
| `base_rules_br024_` | 1 |

## Independent inputs and review

The frozen graph fixture supplies the literal 72 endpoints. A separate subset-DP oracle
checks the existing 15 route fixtures plus engineered blocking/transfer/
off-turn/two-opponent inputs, without importing the product graph/calculator. Its report
and literal inputs remain privately retained; product tests embed the relevant exact
road/building/length/score inputs and validate their distance and caches. A public
Orange settlement at V30, reached via E29/E36 from V19, interrupts Blue's actual
five-edge trail and returns the award at length three.

Standalone pinned rand/chacha sources and source-only production inputs for T13–T15
remain privately retained. No tracked test reads those private files. Public tests
contain literal seeds/dice/hands/bank/graph witnesses: seed29573 builds coastal
brick-port V42; seed36673 proves the blocking settlement; seed8433 upgrades V40 then
produces two ore; seed50456 independently proves normal 4:1 and immediately improved
2:1; seed27147 builds generic-port V3 and uses 3:1. All candidate lists/25 resource
pairs use the ordinary validator. Exact next dice prove nonrandom actions and queries
preserve the stream.

Fresh Sol/high reviews cover settlement blockers/award/victory and city stock/ winning
upgrade. Fresh Terra/high reviews independently check the public graph, source
production, RNG, candidate and payment arithmetic, and the bank pipeline. Original
T13a–T13b, T14a–T14b and T15a–T15b reports remain privately retained. Required findings
are resolved before commits. Controls caught an incorrect blocking-road threshold,
missing victory checks, reduced city ore cost, one-card city production, inverted port
ownership and a 2→3 special-port rate. Original/mutant sources/logs remain privately
retained; completed generated review targets were removed. These are targeted controls,
not a mutation-score claim. Final full gates were run by the implementing worker; review
reports state which tests/checks each reviewer independently repeated.

Manual import/constraint audit confirms model→rules→turn-engine→API direction, one
shared validator/cost/calculator, bounded position arrays, borrowed topology, separate
explicit RNG and caller-owned history. Diff against C04 leaves specs, constraints,
Cargo/dependencies and gate configuration unchanged. No reverse imports,
UI/transport/storage/timer/global RNG, suppression, ignored/deleted assertions or
executable stubs were introduced. New public sequence families are focused modules; the
integration root adds only small declarations.

## Limits at this historical checkpoint

T13–T15 are complete and C05 evidence is presented under the existing autonomous
authorization. T16 onward remains pending. All cards still reside in the deck, so public
and true score currently agree; T16 must extend the same score/finish helpers for owned
hidden VP cards, and T19 adds Largest Army. The active/off-turn threshold positions are
valid bounded engineered inputs. Complete accepted setup-to-ten-point histories,
including four game/setup combinations, remain T21/T29; these tests do not claim those
histories. Current public proof covers construction, blocking, returned stock, score
increase, city production and all three bank rates.

Strict JSON, snapshots/replay, complete-game properties/conformance, measured
performance/memory, Wasm/bindings and hosted CI remain later/out-of-scope work. Nothing
was published or deployed. No approved rule or quality threshold changed.