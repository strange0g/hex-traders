# T33: Historical integration and handoff review

**PUBLIC DERIVATIVE — historical summary.** This preserves the original native
engine assessment and evidence; it is not verification of the new release tree.
Historical source IDs and candidate hashes refer to privately preserved original
revisions, not new public-history commits or current renamed-file hashes.
T43 verified the original report SHA-256
`e77bdb8a029f9c64bf1ceb0f8c9149fa7e17a33696f229620a43310e0195c68f` against the T35 preservation inventory and reconciled the relevant
original evidence hashes. Original specialist reports, raw logs, C11/ship
archives, manifests, oracle sources and control artifacts remain private and
byte-preserved; none has been renamed as fresh evidence. Private machine paths,
source URLs and links to excluded raw evidence are omitted from this derivative.
Previously excluded generated build trees and nonexistent tool-error controls
remain exclusions. No engine tests, benchmarks, profilers or rollback procedures
were rerun for T43. T52 owns the fresh release assessment; historical approval
here grants no new publication or deployment authorization.

Public context: [approved rules](../SPEC-base-rules.md), BR001–BR038 and RQ1/RQ2;
[constraints](../CONSTRAINTS.md); [historical plan](plan.md) and
[task status](todo.md). The rule baseline is the 2025 English sixth edition,
document `CN3081`, revision `v6.250401`, with pages and compatible FAQ
qualifications recorded in the paraphrased catalogue. No publisher endorsement
is claimed.

**Verdict:** APPROVE — final T33/C11 technical candidate based on T32 `2272e27da6003b0aa7d914de4793ab20a909e320`, at the exact hashes below.

**Overview:** The shared README/crate rustdoc provides a compiling native integration guide, and factual status updates reconcile the specifications and constraints with verified implementation. All original engine success criteria and final agreed checks are supported; no rule, quality threshold, dependency or executable behavior changed.

### Critical Issues

None.

### Required Changes

None. The two Markdown coverage-table repairs were verified before final approval.

### Optional

None needed for handoff.

### Nits

None.

### What's Done Well

- The same README is included as crate rustdoc and exercised by the real documentation-test runner. Its example demonstrates create → view/legal choice → validate/execute → safe original receipt projection → private save/resume and accepted-record replay.
- The guide separates engine rules from host authentication, private randomness, serialization access, storage, transport and deduplication. Mandatory choices, private card candidates, stale revisions and lost-response recovery are concrete rather than delegated to duplicated UI rules.
- C11 records exact command arguments, actual counts/durations, input hashes and original formatting/advisory failures. The approved bar remains intact and the privately preserved archive supplies historical evidence.

### Consumer/API and normative review

README uses the actual public API, fixed three-seat configuration, a clearly marked synthetic all-zero seed and simulated authenticated actor. Production guidance requires an unpredictable private seed and host-derived actor/viewer; it explicitly forbids executing an unverified client JSON envelope. Seats must be a complete permutation, setup mode/starting rotation are described correctly, legal choices are scoped to owners, complete bundles use validate, and mandatory setup/discard/robber/free-road phases remain engine-owned.

Queries reserve nothing. Expected revision and shared legality checks precede atomic execution; stale views refresh, and lost responses require host request deduplication rather than a newly revisioned retry. Native, boundary and replay errors are described accurately. Original receipts are projected per viewer; raw receipts, snapshots and replay remain host-owned. Snapshot import returns a new validated engine and proves coherent current facts, not honest history. Replay consumes ordered accepted records and host-managed stream retention. Private identities, hand/card/VP/RNG facts, stable version IDs, canonical decimal counters, 64-KiB/depth-16 input units and possibly growing creation receipts are explained without claiming universal output limits.

All root/provider/map/CONSTRAINTS diff hunks were inspected. Changes replace obsolete implementation/tooling/coverage/performance status, label earlier illustrative file plans honestly, and correct benchmark filters to real cases. BR001–BR038, RQ1/RQ2, module direction, atomicity/privacy, safe Rust, no-suppression rules, the 90% coverage floor, advisory requirements and the manual 5% repeatability margin are preserved. No approved normative requirement was removed or weakened. Local compiler/format/Clippy/coverage/advisory enforcement remains distinct from manual architecture/performance review and absent hosted CI. Absolute performance budgets remain unapproved; noisy same-binary native measurements retain their original limitations.

Every checkout-relative local link in the README, root/provider specifications, map and constraints resolves to an existing file. README's evidence links are expressly repository files; generated rustdoc does not copy those documents under target/doc. That known limitation is recorded in C11 and is acceptable for this local path-dependency handoff with no configured repository URL. The integration example itself is self-contained. No browser/Wasm binding, UI, network, authentication service, cross-target replay, package publication or deployment is claimed.

### Root success criteria

| Criterion | Evidence and verdict |
|---|---|
| Source-traced BR001–BR038 and approved card policies | C10 rule map, direct tests and independent retained review; unchanged final suite passes. |
| Full legal fixed/variable games with 3/4 seats and exact first victory | Six passing final full_game tests, source-frozen histories and mandatory-phase assertions. |
| Consumer command/query/view integration | Compiling README/rustdoc flow and public contract tests; no consumer-side rule calculation. |
| Pure rejection, exactly-once accepted transition and query agreement | Native contract/privacy/counter/property tests, complete snapshots and replay; host dedup distinction documented. |
| Safe views/events/errors | Explicit all-viewer allowlist tests and private boundary review; raw host facts are identified. |
| Every-phase save/resume and accepted-history replay with next RNG | Complete games and phase checkpoints, exact private/projected receipts, snapshots and independently sampled RNG facts. |
| Clean build/lint/tests, approved quality contract and reproducible native baseline | Final C11 gates below; committed T31 raw timing/memory and T32 review. |
| Acyclic four-module direction and one implementation per rule | Independent complete-source T32 review; production/tests/configuration remain unchanged. |

### Verification Story

**Tests reviewed:** yes, connected tests were reviewed before implementation in T32/source preflight. Final documentation flow was checked against actual exports/semantics and the new passing README doctest. All six complete-game tests are present and pass in the final log. No new source fault control is warranted for doc attributes and factual prose.

**Build verified:** yes through final exact-source raw logs and all 103 matching archived input hashes. Final manifest contains ten local commands; all succeed except the preserved sandbox advisory lock attempt, superseded by a separately recorded successful authorized scan. Actual results are 199 tests (131 unit/68 integration), 11 doctests (6 ordinary including README/5 compile-fail), 20 Criterion smoke cases, zero failed/ignored tests, and 98.72% library lines (5,955/6,032). Format, all-target check/Clippy, release build and public rustdoc pass with no final warnings. The ten commands total 93.093 seconds, plus 0.835 seconds for the authorized advisory refresh; the approximate 90-second routine target did not truncate required checks.

**Security checked:** yes. Runtime/development dependencies, lockfile, toolchain and checker configuration are unchanged. Refreshed RustSec database revision is `d9cb059f3217db87aeccd190b81eaffe8cc29cb9`; direct execution-tool output says advisories ok, exit 0. Both sandbox advisory failures and the initial crate-attribute blank-line fmt failure remain archived. No production state export/hook or permission/policy exception was added.

The only source change from T32 is `#![doc = include_str!("../README.md")]` replacing the short crate doc comment; all exports and `#![forbid(unsafe_code)]` remain intact. Tests, benchmarks, manifests and checker configuration have zero diff from T32. Final source/README/spec/fixture/review/config hashes in the archive match all 103 then-reviewed files. After gates, two coverage-command table rows received Markdown pipe escaping; the checkpoint discloses these presentation-only repairs. Removing only that escaping reproduces the earlier CONSTRAINTS hash exactly. The 103-file manifest was refreshed for that document; executable inputs and the original full-gate command/log bytes remain unchanged. The later README EOF correction and its doctest rerun are recorded below. Independent audit details remain privately retained.

### Historical reviewed candidate hashes

| File | SHA-256 |
|---|---|
| README.md | `77b048af9187f69d8694c05df4b85b8f96a2903769fdc8874d93bee7e56e60b6` |
| src/lib.rs | `d1a156ad6bb91880a66d94886e70ef3752551130c981aac5ba2ef5f33dafe7b0` |
| SPEC.md | `f3e30ef54954ca0d1970b674361f2e99cfc1c1f789ab02ae691cfb42ba2a9c60` |
| SPEC-game-model.md | `9555451698224d0aa2dafb317e0601db42108b103ea6afb6822fd05a0b591baa` |
| SPEC-base-rules.md | `52d994555323976a8e023bd7aad261573c6c5bde53eeadee3883092ac9dc1231` |
| SPEC-turn-engine.md | `c3789f573484be846b997170dcb41bd0bb65714b423f67c4b1de79253a30e03f` |
| SPEC-engine-api.md | `223d53409c1b0f8597f2f441bc305ff5b25e3ea60c2bc1630f9b40f3246e0feb` |
| CAPABILITY-MAP.md | `061843a0e35e4f9818cb2cff56b762b484d129a756daa120a3360ba99f2a6906` |
| CONSTRAINTS.md | `d9d32c007e00b59b0c06b68dbdfaf4616d2f5dca86229931c3c6732d629ced76` |
| Private original C11 evidence archive | `2740fcc4e6e42fea50f78e0d8ff945d60bbe05288ae6e62b53c255870dd27903` |
| tasks/c11-checkpoint.md | `12f33dad75230791dd79e6893166bede29e58e277749c25ef92f602eeee58c36` |

Reviewer made no tracked edit or source mutation. Root/owner retain completion marks, final evidence presentation and verified commit ownership. The technical T33 acceptance and required C11 verification are satisfied at this exact candidate; bookkeeping and human checkpoint presentation must preserve the completed verification and commits. This review authorizes no external publication/deployment or changes to approved rules/quality requirements.

Skipped redundant suites, benchmark/profiler reruns, new fault controls, cross-target/binding tests and transient/load/percentile performance work. Those excluded capabilities are explicit scope limits, not hidden unfinished requirements of the native library.

### Final README whitespace recheck

One terminal newline was removed after the initial staged diff check flagged an extra README EOF blank line. Adding that single byte back exactly reproduces the previously reviewed README SHA. The archive retains both 103-file manifests; they differ only in the README entry. All 103 final entries match the then-reviewed candidate. Original full-suite/build/lint/coverage/smoke/advisory command/log bytes remain unchanged, and the checkpoint explicitly says those checks preceded this whitespace-only correction. The actual `cargo test --locked --doc` rerun passes 11 tests in 1.083 seconds and is archived separately. Original staged-diff failure is retained; after restaging the corrected files, independent `git diff --cached --check` passes. Final approval remains valid at the updated exact hashes above.
