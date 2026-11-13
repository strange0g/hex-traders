# T32: Historical independent review

**PUBLIC DERIVATIVE — historical summary.** This preserves the original native
engine assessment and evidence; it is not verification of the new release tree.
Historical source IDs and candidate hashes refer to privately preserved original
revisions, not new public-history commits or current renamed-file hashes.
T43 verified the original report SHA-256
`b3ece1d320653ec27b26ca41383693019c0ef95087b4962d2445a9e5f477f266` against the T35 preservation inventory and reconciled the relevant
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

**Verdict:** APPROVE — T32 at clean T31 commit `698e6ce98e5a6ca3cb9380550352a0e74065f098`.

**Overview:** The completed native engine preserves the approved four-module design, strict public/private contracts, atomic transitions and bounded current state. The native measurements justify retaining the implementation; no blocking correction, speculative optimization or quality exception is needed.

### Critical Issues

None.

### Required Changes

None for T32.

### Optional

No optimization follow-up is proposed. `Engine::legal_actions` checks bounded candidate families through the shared validator; its late-game medians are 6.976–7.024 microseconds. Accepted transitions clone the bounded Position and allocate its three-seat configuration Vec, but the measured ordinary roll/end-turn medians are approximately 0.24–0.30 microseconds. Neither measurement establishes a consumer bottleneck. Adding a phase dispatcher, legal-action cache or inline seat representation would require a concrete workload and repeatable before/after evidence with unchanged conformance/privacy/replay tests.

### Nits

None.

### What's Done Well

- One rule-provider layer supplies placement, economy, card, trade, score and exact-route checks to execution and queries. Native and wire interfaces agree through the same facade; importing snapshots uses the full native activation validator.
- Strict boundary diagnostics and explicit viewer/event allowlists protect hidden facts. Tests combine independent source/RNG/arithmetic literals with ordinary command replay and all-viewer comparisons; matching engine instances alone are not treated as independent truth.
- Measurements keep fresh validated command inputs, native/wire components and caller-owned receipts separate. Raw variability, profiler controls and earlier failed experiments remain available rather than being removed to improve the story.

### Architecture, correctness and security

The earlier full-source, tests-first preflight was reconciled with the final committed source. All 48 Rust source-tree files, including test modules, and the entire tests directory are byte-unchanged from C10. Root and all four provider specifications, capability map and constraints are also unchanged; their original exact hashes remain privately retained.

Imports follow `game_model` → `base_rules` → `turn_engine` → `engine_api`, with the facade also directly consuming providers below it. The model has no consumer import. Calls use typed borrowed values and owned transitions rather than inter-module JSON or a second rules implementation. Production modules remain reviewable: the largest mixed production/test file is 734 lines, with its production body 229 lines; only a test-only command file exceeds 1,000 lines. No unsafe path, executable stub, ignored ordinary error, lint/coverage suppression, skipped assertion, speculative subsystem or core transport/storage/timer was found. The replay encoder's local Write adapter bounds in-memory serialization; it performs no storage I/O.

Execution checks API version, actor/recipient and expected revision before rule eligibility. Shared validation precedes creation of an owned next Position, and counter exhaustion precedes mutation. The facade commits Position, revision and optional RNG only after successful evaluation. Queries are read-only and use the execution predicates. Nonrandom transitions return no replacement RNG; random generation is per-game and no global RNG exists. Topology is immutable and shared through LazyLock, while terrain, numbers and ports belong to the game.

Exact Longest Route searches edges, permits loops/revisited vertices and stops at the 15-road bound. An opponent building can terminate a trail but cannot be crossed. Paid/free roads share installation and refresh the affected owner; settlement placement refreshes only potentially blocked opponents with at least two incident roads. Incumbent tie retention and unique-maximum takeover are shared with score/projection/import. Economy, city and trade actions do not rescan routes. Knight, first-free-road and turn-start victory timing and winner-only VP revelation match the approved rules and retained direct tests.

Native commands, views, projected receipts and host-private snapshots/replay maintain their distinct contracts. Buyer card identity, discard types and theft kind use explicit permitted-viewer projections; terminal full scores remain private. Strict JSON rejects unknown fields, decoded duplicate keys, trailing input and invalid shapes/IDs/counters, bounds bytes to 65,536 and containers to depth 16, and restores only canonical supported RNG cursors. Creation/import validate resource/piece/card conservation, pending choices, source layouts, cached awards and final facts before activation. Imports prove coherent current facts, not authenticated historical reachability. Replay executes ordinary commands, requires contiguous exact private receipts and rolls back Position/RNG/revision/index on mismatch. It retains one current game and index rather than a match log; the host controls private storage, actor/viewer identity, seed choice and stream length.

### Measured performance and absolute budget decision

T31's actual raw evidence and fixture history were independently reviewed, including all 60 timing medians, all 20 PERF table rows, all 56 memory phase totals, 0/1/N controls, profiler commands and complete checksum verification. The hard public 14→15-road case has exact route 7→10, two cycles, branching and a blocked endpoint; it is demanding but not a proof of the global worst case. Every accepted benchmark command uses a fresh valid revision and intact validation; serialization, projection and result destruction semantics are documented.

Reference-machine hard-road medians are 4.824/4.892/5.191 microseconds; late legal query approximately 7.0 microseconds; player-view JSON encoding 8.46–8.58 microseconds; validated snapshot import 39.0–41.2 microseconds. These are median observations, not latency ceilings. Some same-binary cases differ by more than 5%, including occupied validation and projected-receipt encoding in both repeats. The CPU has an active SMT sibling and unisolated system activity. The approved 5% margin remains a manual repeatability review policy; Criterion does not enforce it, and same-binary drift is not a code regression or a reason to loosen the margin.

**Absolute budget proposal:** none is justified yet. One native desktop machine and fixed synthetic fixtures do not define target consumer concurrency, action/query frequency, supported hardware, latency percentile or a service objective. Selecting a numerical ceiling would invent a requirement. The measured values above provide a reference for a later concrete consumer budget proposal and human approval; no threshold was silently selected or changed.

Engine inline size is 1,232 bytes, including its 96-byte Board and 824-byte Position. Matching early revision-0 and late revision-191 controls add 3 requested heap bytes and 21 Massif extra bytes per three-seat game at both N=16 and N=64, after caller receipts are dropped. Shared graph storage is 6,448 static inline bytes plus 2,280 requested heap bytes and 4,056 extra bytes once per process. E58 receipts add 160 requested bytes and 8 extra bytes beyond their 104-byte inline values; this seed's creation receipts add 1,027 requested bytes and 29–30 observed extra bytes beyond 72 inline. Pre-reserved vectors are counted once. The corrected cold controls include the initialized stdin buffer; the earlier flawed capture remains excluded from the graph delta. These data support bounded retained state for the selected history, not a transient peak/RSS/stack budget. Creation receipts may grow under the approved tied-starter reroll policy, and private history/receipts belong to the host rather than Position.

### Verification Story

**Tests reviewed:** yes. Full production/provider and connected tests were inspected in the source preflight after reading the specifications. C10's retained independent mutation controls and source-frozen complete histories were reviewed; final source hashes match. Public native/wire/save/replay agreement, all-phase continuation, all-viewer privacy, rejection preservation and rule-clause references remain covered. T31 adds measured harnesses and guards without modifying production behavior.

**Build verified:** yes, through the exact retained final-source command/log evidence. T31 gates pass all-target check/Clippy, format, release build, rustdoc, 199 tests (131 unit/68 integration, none failed or ignored), 10 doctests and 20 benchmark smoke cases. Library line coverage is 98.72% (5,955/6,032), above the unchanged 90% floor. Advisory retry passes; its earlier database-lock tooling failure remains retained. These passed checks were not rerun without a new concern.

**Security checked:** yes. Locked runtime dependencies and safety floor are unchanged; refreshed advisory scan says `advisories ok`. Input bounds, private facts, static errors, sealed state/RNG and host responsibilities were inspected. External Massif/GDB profiling adds no allocator hook or public state export. Native results do not imply a browser binding, hosted CI, publication or deployment.

Actual timing binary SHA-256 is `36d509ff077e9183d8bed5f363a3fac1ea973761982338cdc4f1cefa56753955`, matching the recorded benchmark. Input hashes and raw evidence match the original committed files. Benchmark fingerprint has no Rust flags; relevant Cargo profile/config overrides are absent. The debuginfo inspection is explicitly a separate build with its own commands/hash. Archived verbatim stdout preservation was independently proven by reconstructing the original checksum manifest. Original raw evidence and the T31 specialist audit remain privately retained;
[PERF.md](../PERF.md) is the retained public performance summary.

### Historical candidate hashes

| File | SHA-256 |
|---|---|
| PERF.md | `f6af7ae7a75466751856a05040279748833d4f34a1ae1ba0e2db2da9fe48cda0` |
| Cargo.toml | `4af48a1cd7486a941d022fb656b7d695dfa54591d3e51b1c660c6e996a03ff19` |
| Cargo.lock | `47d1bf2874e9b173786a7f0ebe7f0fafd40ceaf87ba6eb0834ba495f15e5070c` |
| benches/engine.rs | `b6b92761341691b3fb10755c58c6dbe1820b34cb308b0d0128130bd4a101d40b` |
| benches/fixtures.json | `1dd002e026f4ef0530010866ded5447f80c05ae88d94e50c0c4bf6e330843d41` |
| Private original performance checksum manifest | `33daf6ab152ed61e1699e66bd1406c542c2818654296f6dad3fe5c1a9cecdd36` |
| Private original T31 performance audit | `e4e7600bbc9ffcc55a1beee069205513b39ac744c99726f727620ba8a4b695e4` |

Reviewer made no tracked edits or shared mutations. Full-source and timing preflights and the final T31 audit remain privately
retained.

T32's three acceptance criteria are met: architecture/contracts reviewed, measured hot paths and manual margin/budget decision recorded, and no blocking correction remains. Root owns task/report bookkeeping and a verified T32 documentation commit. T33/C11 still must finish accurate implementation/enforced/manual status prose, compiling consumer guidance and final agreed checks before declaring the engine handoff complete. At this historical checkpoint, status prose in CONSTRAINTS/spec/map was
assigned T33 work; approved rules and thresholds must remain intact.

Skipped redundant full gates/timing/mutations, exhaustive global worst-case route search, consumer load/latency percentile benchmarks and transient memory peaks. No finding or unapproved scope change requires those checks for T32 closure.
