# C03: Setup, production and discard obligations

**PUBLIC DERIVATIVE — historical summary.** This records the 10 November 2026
checkpoint, not a new execution or verification of the release tree. Historical commit
IDs identify privately preserved source revisions; they are not public release-history
links. The original report, raw logs, oracle inputs and reviews remain privately
retained byte for byte under T35. T41 reconciled the original report's SHA-256
(`b494e5fdf7b40482ab9bded319e43fa14416c82a8af3a9a4d413e14ce0b2aeca`) with the private
preservation inventory and checked the relevant archived evidence hashes. No runtime
engine checks were rerun for T41. Private machine paths, raw archive/index links and
original source URLs are omitted from this derivative; no raw log has been renamed as
fresh evidence. Previously excluded generated build trees and nonexistent tool-error
controls remain exclusions; C03's unexecuted optional mutation remains a limitation.

Rule provenance: [approved base rules](../SPEC-base-rules.md), BR005–BR012, BR033 and
BR037. The approved 2025 English sixth-edition baseline is document `CN3081`, revision
`v6.250401`, pp.4–6, 8–10, 11–12, with the compatible FAQ qualifications recorded in the
catalogue. Requirements are paraphrased; no publisher endorsement is claimed. Historical
task IDs and completion context are retained in [todo.md](todo.md).

Verified on 10 November 2026 on `codex/build-engine`, through source commit `f5f29eb`.
The user approved the baseline and autonomous build on 9 November; the dates of that
authorization remain unchanged.

## Delivered capability

Both setup modes create source-verified games. Variable setup accepts free
settlement/road pairs in the pinned snake order, grants starting resources only at
completion, and begins the selected starter's first pre-roll turn. Native dice
production handles city values, the robber, aggregate claims and shortages. EndTurn
follows supplied seat order with checked counters and the current owner-turn victory
hook. Sevens create fixed resource-only obligations; any obligated seat may discard
once, exact stock/quantity is validated atomically, and only the last accepted discard
opens Seven→Action robber movement.

Public three-/four-seat setup and discard traces use accepted native commands with
retained literal expectations from source-incidence and independent pinned RNG
arithmetic. Discard types remain visible only to the discarder; public projection
retains original count, phase and receipt revision after later state changes. T09q
closes the independent review's wider-arithmetic contract finding with shared private
checked resource operations and checked narrowing.

## Verified commits

| Slice | Commit |
|---|---|
| Creation event discriminator | `637a397` |
| Setup settlement | `9e40c32` |
| Paired setup roads and completion | `46df854` |
| Public setup traces | `db5b0b2` |
| Native command extraction | `8e24c4a` |
| City-aware values and current score views | `b3ee5e3` |
| Dice, production and fixed obligations | `0432ab4` |
| EndTurn and current victory hook | `0e8ea7e` |
| Private discard fulfilment | `775e5aa` |
| Wider checked resource arithmetic | `f5f29eb` |

Each implementation slice has its recorded independent review and standing checks in
[todo.md](todo.md). The T07r cached-restoration correction remains explicit there; its
first restoration was not a passing control.

## Historical final standing gates

All commands below exited zero against the source now committed in `f5f29eb`. Only
evidence/status documentation changed afterwards.

| Command | Actual result |
|---|---|
| `cargo test --locked --lib --tests` | 49 passed: 32 library, 6 public sequences, 5 model invariants, 6 rules conformance; none ignored |
| `cargo test --locked --doc` | 5 passed, including compile-fail examples |
| `cargo check --locked --all-targets` | Passed |
| `cargo build --locked --release` | Passed |
| `cargo fmt --all -- --check` | Passed |
| `cargo clippy --locked --all-targets -- -D warnings` | Passed |
| `cargo doc --locked --no-deps` | Passed |
| `cargo llvm-cov --locked --lib --tests --ignore-filename-regex '(^\|/)(tests\|benches)/' --fail-under-lines 90` | 98.98% library line coverage, 3191/3224 |
| `cargo deny --locked check advisories` | Root's refreshed 10 November scan exited zero with `advisories ok`; `cargo deny --locked fetch` also exited zero |

The advisory tool is cargo-deny 0.20.2. Dependencies, Cargo.lock and the advisory
configuration were unchanged by these gameplay/correction commits. The scanner result
was supplied by the root agent; it was not duplicated by this worker.

Final focused commands also exited zero, with nonzero test counts:

| `cargo test --locked` suffix | Passed |
|---|---:|
| `--lib resource_arithmetic_` | 1 |
| `--lib setup_` | 7 |
| `--test game_sequences setup_` | 2 |
| `--lib turn_` | 11 |
| `--test game_sequences turn_` | 2 |
| `--lib base_rules_br008_` | 2 |
| `--lib base_rules_br01` | 3 |
| `--test game_sequences discard_` | 2 |
| `--lib privacy_initial_` | 3 |
| `--lib privacy_discard_` | 1 |

The original standing and final focused logs are retained privately. The tables above
preserve their historical commands and verdicts.

## Source and independent review

Manual source/import review confirms the four-module provider direction: game-model has
no consumer imports, base-rules imports the model, turn-engine imports model/rules, and
engine-api consumes them. Queries reuse execution predicates; exact discard
stock/quantity and required-seat eligibility each have one shared validator. Private
Position uses bounded arrays and contains no match history. RNG remains explicit and
separate, with no clock, storage, transport or global generator in the core. Successful
nonrandom actions leave the generator unchanged; rejected commands preserve
position/RNG/revision.

No approved spec/CONSTRAINTS/check configuration was changed, and no suppression,
skipped test, removed assertion, ignored error or executable stub was introduced.
Internal invariant assertions concern validated phases, checked graph IDs and conserved
resource bounds; ordinary player rejection returns typed safe errors.

Fresh C03 review initially requested changes for narrow resource arithmetic. After T09q
it approved all seven connected source edits and the new model test, with no remaining
correctness, privacy, architecture or atomicity findings. The discard reviewer had
separately approved the final three-/four-seat traces and owner-only receipt projection.
Its earlier isolated privacy leak mutant failed, and fresh restored source passed,
before the final fixture/mechanical refinements. The optional T09q mutation build
exhausted temporary disk quota before a control ran, was removed, and supplies no
mutation evidence. Completed standing gates were unaffected; final focused tests passed
after cleanup.

## Limits at this historical checkpoint

T10 robber movement/theft remains pending. T11 replaces the current exact
zero-/one-/two-setup-road helper with general trails. No card ownership exists yet: T16
must extend the same score/finish/reveal path with held VP and separate public from true
score; T19 introduces Knight/pre-roll continuation. Later construction, trade, wire,
snapshots, replay, full-game conformance, performance, Wasm and hosted CI are not
claimed by this checkpoint. Nothing was published or deployed. Phase 4 may proceed under
the existing autonomous authorization.