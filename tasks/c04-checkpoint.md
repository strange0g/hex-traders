# C04: Robber and exact road rules

**PUBLIC DERIVATIVE — historical summary.** This records the 10 November 2026
checkpoint, not a new execution or verification of the release tree. Historical commit
IDs identify privately preserved source revisions; they are not public release-history
links. The original report, raw logs, oracle inputs and reviews remain privately
retained byte for byte under T35. T41 reconciled the original report's SHA-256
(`8082739dde172602d49811baeb28d77fc96ebad047f2d368ce049b3741b6a0e4`) with the private
preservation inventory and checked the relevant archived evidence hashes. No runtime
engine checks were rerun for T41. Private machine paths, raw archive/index links and
original source URLs are omitted from this derivative; no raw log has been renamed as
fresh evidence. Previously excluded generated build trees and nonexistent tool-error
controls remain exclusions.

Rule provenance: [approved base rules](../SPEC-base-rules.md), BR013–BR017, BR033–BR035
and BR037–BR038. The approved 2025 English sixth-edition baseline is document `CN3081`,
revision `v6.250401`, pp.6, 8–10, with the compatible FAQ qualifications recorded in the
catalogue. Requirements are paraphrased; no publisher endorsement is claimed. Historical
task IDs and completion context are retained in [todo.md](todo.md).

Verified on 10 November 2026 on `codex/build-engine`, through source/test commit
`65d2d23`. The baseline and autonomous build authorization were approved on 9 November;
no rules, quality requirements or publishing authorization changed.

## Delivered capability

Both robber commands resolve Seven→Action movement and victim decisions. All other
hexes, including desert, are legal; distinct adjacent opponents include empty hands.
Zero/one/multiple-victim paths resume the stored phase. Nonempty theft samples a real
card index in fixed resource order; empty theft consumes no RNG. Projection gives the
resource type only to thief/victim, retaining public party/count facts. Rejected
commands preserve position/RNG/revision.

The sole exact edge-visited route calculator supports branches, loops, shared edges,
legal repeated vertices, opponent blockers and all 15 road pieces. Real creation
validates four bounded cached lengths; views and unrelated commands read them without
searching. Setup/paid road placement refreshes only the owner. Incumbent-aware
reconciliation handles the threshold, ties, takeover and return to supply. Shared
scoring gives two award points and no direct road points.

Paid roads use one placement/cost validator for execution and candidates. Either
unblocked owned connection suffices; an opponent road at an empty vertex does not block
it. Checked brick/wood payment, placement, exact route refresh, award and the existing
active-score victory hook form one atomic revision, with no RNG draw. The engineered
valid 8→10 fixture proves immediate finish and freeze. The public fixed-game sequence
independently proves three payments and the 2→3→5 route award, exact legal candidates,
public event order and next dice.

## Verified commits

| Slice | Commit |
|---|---|
| Mechanical command test extraction | `7a1aec3` |
| Live robber movement/theft and private receipts | `b2fb146` |
| Public robber traces and next dice | `ccbf2ba` |
| Exact route calculator and cached consumers | `9e602a5` |
| Incumbent awards and shared scores | `65cd051` |
| Atomic paid road transition and victory | `d678439` |
| Public road/candidate/award sequence | `65d2d23` |

Each slice has independently reviewed acceptance and standing evidence in
[todo.md](todo.md). T12's new public family is a focused module; the integration root
adds only its three-line declaration.

## Historical final standing gates

All final commands below exited zero against the source/tests committed in `65d2d23`.
Evidence/status documentation was the only later change.

| Command | Actual result |
|---|---|
| `cargo fmt --all -- --check` | Passed |
| `cargo check --locked --all-targets` | Passed |
| `cargo clippy --locked --all-targets -- -D warnings` | Passed |
| `cargo build --locked --release` | Passed |
| `cargo test --locked --lib --tests` | 69 passed: 49 library, 8 public sequences, 5 model invariants, 7 rules conformance; none ignored |
| `cargo test --locked --doc` | 5 passed, including privacy compile-fail examples |
| `cargo doc --locked --no-deps` | Passed |
| `cargo llvm-cov --locked --lib --tests --ignore-filename-regex '(^\|/)(tests\|benches)/' --fail-under-lines 90` | 99.00% library line coverage, 3061/3092 |
| `cargo deny --locked check advisories` | Authorized refreshed scan passed with `advisories ok` |

The ordinary sandbox advisory invocation first failed on a read-only advisory lock path;
it supplies no pass. The authorized invocation could refresh/lock the database and
exited zero. The lockfile, dependencies and check configuration remain unchanged.
Original final logs, including the refreshed advisory result, are retained privately;
the tables preserve their historical verdicts.

Every final focused filter ran nonzero passing tests:

| `cargo test --locked` suffix | Passed |
|---|---:|
| `--test game_sequences robber_` | 1 |
| `base_rules_br014_` | 1 |
| `--lib privacy_theft_` | 1 |
| `base_rules_br034_` | 1 |
| `base_rules_br035_` | 1 |
| `--test rules_conformance longest_route_` | 1 |
| `--test game_sequences road_` | 1 |
| `base_rules_br017_` | 2 |

## Independent inputs and review

The tracked `tests/support/longest-route.json` preserves the frozen hand-recorded 72
endpoints, 15 route witnesses, 16 award steps and two invalid controls. No production
graph/solver generated the expected lengths. Fresh Sol/high review reproduced every
fixture with independent exhaustive subset calculations, and a blocker-stop mutant
failed (5 versus required 3). Incumbent-retention removal failed the literal tie-at-five
step. Membership/privacy mutations were caught for theft, and removing the paid-road
victory call failed the winning road test. Isolated source/logs remain privately
retained; completed generated targets were removed. These are targeted controls, not a
mutation-coverage claim.

Public robber RNG inputs came from a standalone pinned rand/chacha oracle. The public
road oracle separately retained pinned Cargo inputs, RNG source/output and independent
endpoint candidate enumeration. Both original oracle sets are privately retained. The
selected seed15508 is u32 little-endian; fixed source incidence gives three eights and
two nines, exact before-payment hands/bank and next dice `[1,4]`. The route witness is
V31–E46–V36–E45–V30–E44–V35–E51–V40–E57–V44, using all five owned edges. Expectations
are literal tracked test data; no test reads these temporary paths.

Final Terra/high review independently checked all production rows, five-roll hands/bank,
every candidate list, the witness and next-roll bank. It approved T12b/C04 with no
required findings. Earlier dedicated reviewers approved every slice and control;
original T10a–T10c, T11a–T11b and T12a–T12b review reports remain privately retained.
Final gates were run by the implementing worker; the final reviewer inspected those
results without independently repeating the full suite.

Manual import/constraint review confirms model→rules→turn-engine→API direction, shared
validators/calculators, bounded position arrays, separate explicit RNG, borrowed
topology and caller-owned history. No reverse consumer imports,
UI/transport/storage/timer/global RNG, weakened constraints, suppression,
skipped/deleted assertions, ignored errors or executable stubs were introduced. The
extraction preserved all existing command test bodies and names; the private parent
qualification explains the instrumentation coverage denominator change.

## Limits at this historical checkpoint

T13 onward remains unchecked. Blocking settlements, city upgrades, trades and owned
development cards are later tasks. T16 must extend the same current score/victory
helpers with hidden VP and the public/true score distinction; T19 introduces
Knight/pre-roll continuation. A public complete ten-point history remains T21; the
current engineered fixture proves the live victory hook. Performance, wire/save/replay,
Wasm, hosted CI and full-game evidence remain pending. Nothing was published or
deployed. At this checkpoint, C04 was ready for Phase 5 under the existing autonomous
authorization.