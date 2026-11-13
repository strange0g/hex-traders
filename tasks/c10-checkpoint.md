# C10: Complete rules, replay and shrinking properties

**Public derivative:** This is a historical summary of the original checkpoint
and its privately preserved evidence. Results below were recorded on 10
November 2026; remaining scope refers to that checkpoint, and no runtime checks
were rerun for this summary.

Verified on 10 November 2026 on `codex/build-engine`; source/test HEAD is
`380ca5e7532d0e5be3307519b9b6292387b9a8ea`. T28–T30 acceptance is complete.
Each verified leaf has an explicit-file commit and independent review. Existing autonomous authorization
permits the next phase after this checkpoint evidence is recorded.

## Delivered capability and independent evidence

Host-private format 1 replay consists of one validated creation/checkpoint header
and bounded accepted records. `ReplayVerifier` retains only current Engine/index,
uses strict snapshot/wire parsing and ordinary execution, compares exact ordered
private receipts/revisions/final scores, and restores complete state/RNG/index on
mismatch. Byte/depth bounds apply per header/record; stream length and history
storage belong to the host. Public projections remain separate allowlists.
The writer rejects crossing allocation growth at 65,536 bytes. Literal private
purchase and weighted theft/next dice controls supplement matching replay.

The historical [BR clause map](rules-conformance.md) records 38 ordered
BR001–BR038 rows, 106 resolved named assertion references and source pages 2–12
of the approved 2025 English sixth-edition baseline, document `CN3081` revision
`v6.250401`, with RQ1/RQ2 explicitly approved policies. Closed-command
nonactions are distinguished from executed negative assertions. Direct rare
cases preserve development locations through resource discard/theft, place two
independently connected free roads, build and trade beside the robber, and
produce from another same-resource/same-number hex.

Six `full_game_` tests reach exact victory across all four fixed/variable and
three/four-seat configurations through actual public setup/actions. The existing
235 fixed-three records remain unchanged; three new histories freeze 307 source
rolls, creation/layout/deck facts, exact payments/balances/cards/scores, route
witness, winner-only revelation, still-legal suppressed second free road and
independently sampled next dice/cursor. Historical fixture provenance
records independent all-row arithmetic and pinned sampler reproduction. Each
accepted command resumes and advances one complete persistent replay, comparing
private/projected receipts, every view, snapshots and unchanged nonrandom RNG.
All nine phases, both free-road continuations at remaining two/one, offers,
victim choices and partial discards additionally replay from reached checkpoints.

The historical [T30 property evidence](property-evidence.md) records the bounded synthetic
configuration, decision vectors, malformed families, deterministic seeds,
actual reach counts, mutation failures/restores and local shrink witnesses:

- 64 creation cases independently check literal resource/piece/card/score facts,
  awards/phases, spectator privacy and query/import byte purity.
- 32 vector cases reach 1,274 accepted commands across 16 families, including 12
  purchases/six Knights/one city and 34 trade accepts, and all 21 arbitrary
  rejection families. Assembly handles all 21 action kinds; every placement ID's
  candidate membership agrees with validation. Each query/rejection preserves
  complete bytes, and accepted commands replay/resume exactly.
- 64 malformed cases deliberately reject 1,024 snapshots and 1,024 wire inputs,
  all at publicly reached revision 1, preserving the next ordinary continuation.
  This totals 2,048 input instances and 3,072 rejection calls because wire
  decode and execution are verified separately.
  Counts/IDs/counters/RNG/phase/deck/duplicate/escaped/unknown/shape/UTF-8/size
  families stay invalid under shrinking; inputs are bounded to 65,537 bytes,
  including one-byte-over-bound valid padding probes. The direct parser unit
  independently proves depth 16 versus 17; the nested property input is a
  rejection family rather than isolated depth-guard proof.

Pinned runner seeds are `0xca7a0030`, `0xca7a0031` and `0xca7a0032`, with at most
2,048 shrink iterations and direct synthetic failure persistence. The accepted
EndTurn RNG control's actual retained vector `[0,0,16758,4083]` is a local
minimum, not global-minimality proof. The duplicate-key control shrinks to
three-seat fixed/seed 0/rotation 0/mutation 0/empty noise and persists. No genuine
engine bug was discovered in T30. Initial test-decoder failure and optional
partial hidden-score control remain explicitly distinguished from product bugs
and completed control verdicts.

## Verified commits and selected review controls

| Leaf | Commit | Independent completed controls |
|---|---|---:|
| T28: bounded accepted replay | `2608c3d` | 8 |
| T29a: four direct rare-rule assertions | `a8a408d` | 8 |
| T29b1: complete fixed-three replay/resume | `c9f95b1` | 5 |
| T29b2: missing complete configurations | `2e84578` | 8 |
| T29c: full clause map/same-resource production/checkpoint replay | `1422c4f` | 2 |
| T30a: independent creation facts | `a5fcf8f` | 6 |
| T30b: shrinking reached decision vectors | `603717f` | 8 |
| T30c: bounded malformed boundaries | `380ca5e` | 8 |

Eight independent slice reviews approved restored exact hashes and selected
compile-valid failures and green restores. The original reviews and controls
are privately preserved. These selected controls are not a whole-project
mutation score. Bounded random exploration does not claim later
settlement/free-road/Monopoly/Invention/terminal paths; the complete public
fixtures and all-phase checkpoints supply direct assertions for those families.

Prior survivors remain retained honestly: crossing replay allocation growth,
shared terminal RNG advancement and spectator private-record leak exposed direct
assertion gaps and are closed. Earlier quota/build and isolated Cargo mtime
failures remain tooling evidence; source hashes and forced recompilation resolve
stale-binary controls. The ninth optional T30b hidden-score experiment is partial
and is not counted in the eight completed controls.

## Final checkpoint verification

The original checkpoint logs refer to the final source/test HEAD above.
Checkpoint documentation changed no executable behavior. Every focused count
was nonzero; the recorded 16 commands took about 109 seconds.

| Command | Actual result |
|---|---|
| `cargo fmt --all -- --check` | Passed |
| `cargo check --locked --all-targets` | Passed |
| `cargo clippy --locked --all-targets -- -D warnings` | Passed |
| `cargo test --locked --lib --tests` | 199 passed: 131 unit/68 integration; none ignored |
| `cargo test --locked --doc` | 10 passed |
| `cargo build --locked --release` | Passed |
| `cargo doc --locked --no-deps` | Passed |
| `cargo llvm-cov --locked --lib --tests --ignore-filename-regex '(^\|/)(tests\|benches)/' --fail-under-lines 90` | 98.72% library lines, 5955/6032 |
| `cargo deny --locked check advisories` | Fresh scan passed, `advisories ok` |

The first advisory attempt failed to lock its database on a read-only path; the
authorized refresh passed. Locked dependencies were unchanged: Cargo.lock
SHA-256 `47d1bf2874e9b173786a7f0ebe7f0fafd40ceaf87ba6eb0834ba495f15e5070c`.

| Focused `cargo test --locked` suffix | Passed |
|---|---:|
| `--test model_invariants` | 5 |
| `--test property_sequences` | 3 |
| `--test api_contract` | 5 |
| `--test privacy` | 1 |
| `--test replay` | 6 |
| `--test rules_conformance` | 7 |
| `--test snapshot_roundtrips` | 3 |
| `--test game_sequences full_game_` | 6 |

The independent checkpoint evidence review checked exact
source/commit/count/log mapping, durable evidence, unchanged rules/quality
requirements and remaining scope. Architecture remained a written/manual
review; compilation did not prove dependency direction. The public provider
direction and safe-Rust floor were unchanged, with no
transport/storage/UI/second rules engine introduced.

## Limits and next phase

Import establishes coherent current facts, not historical honesty; current
award ties do not prove past incumbency. Snapshots and full replay remain
host-private. Recorded literals are explicitly synthetic fixtures.

Measured native performance/memory and final consumer handoff remain T31–T33/C11.
No timing/heap/baseline, hardware-independent performance budget, browser binding
or hosted-CI claim follows from conformance. This phase adds no dependency,
approved rule/policy/quality exception, publishing or deployment. C10 evidence
is available for human review under the existing autonomous authorization.
