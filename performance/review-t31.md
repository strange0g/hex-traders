## Review Summary

PUBLIC DERIVATIVE: historical T31 review, not a fresh release approval.
Exact original review, evidence, controllers and hashes remain privately preserved.
Original report SHA-256: `516c9c5356289d9f232d7e7f7f2a3da7256ed32fba93dd49b36aaf9fd25d0f05`.

**Verdict:** APPROVE — exact T31 candidate below; not final T32/C11 closure.

**Overview:** T31 registers a native Criterion target, fixed synthetic public fixtures and reproducible timing/retained-memory evidence. Production source is unchanged from clean C10; the benchmark performs the claimed accepted/rejected work and PERF reports its limits accurately.

### Critical Issues

None.

### Required Changes

None.

### Optional

No code optimization is warranted by this evidence alone. The legal-action enumeration and short configuration Vec remain bounded, shared and simple; an optimization requires a real workload/budget and repeatable before/after evidence.

### Nits

None.

### What's Done Well

- Fresh validated pre-command imports and revision guards ensure accepted commands remain accepted on every iteration. Rejections assert the specified error and preserve state. Core execution, JSON decode, viewer projection and JSON encode remain separate measured cases.
- The demanding 14→15-road fixture has a complete source-derived public77-turn trace, independent exact7→10 route witness, cycles/junctions/blocker and unchanged saved RNG164. It is expressly not a proven global worst case. The original archive included missing historical proof sources; exact sources and original C10 reproduction commands remain privately preserved.
- All raw pinned/unpinned runs, corrected phase controls and the flawed exploratory cold profile remain retained. PERF reports same-binary >5% variability, the manual5% review policy, native-only scope and host-private synthetic facts without inventing absolute budgets or language speed claims.

### Verification Story

**Tests reviewed:** yes, connected correctness/privacy/import/replay tests and C10 independent controlled-failure evidence were reviewed first in the source preflight. T31 fixture guards and benchmark smoke checks were inspected. No production behavior changed, so no new source fault was introduced for this evidence-only candidate.

**Build verified:** yes, through retained final-candidate commands/logs. Gates.json records passing fmt/check/all-target Clippy/release/rustdoc,199 tests(131unit+68integration; zero failed/ignored),10 doctests,98.72% library lines(5955/6032), and20-case Criterion smoke. The first advisory database-lock failure is retained; the narrowly escalated refresh says advisories ok. Reviewer did not rerun expensive standing gates or timing.

**Security checked:** yes. Runtime dependencies/lock remain unchanged. Safe external Massif/GDB tooling adds no allocator hook/export. Synthetic snapshots/receipts remain host-private; no publication/deployment/binding/hosted-CI claim follows.

**Timing audit:** all60 raw median(time/iters) point estimates match the retained TSV to0.001ns; all20 PERF table rows match their rounded TSV values. Criterion0.8.2's installed iter_batched_ref excludes setup and input/output destruction; ordinary iter includes returned-value destruction. Batched loop/output bookkeeping is included, so these are realistic harness values rather than pure instruction-cycle measurements. Same binary produces native hard-road4.824/4.892/5.191µs, legal-validation63.131/42.125/42.893ns and projected-receipt encode252.584/268.049/275.432ns. No code-regression conclusion follows from those repeated measurements.

**Memory audit:** all56 TSV phase totals equal raw archive snapshots. Corrected16-b/64-a/64-b cold captures contain the already-initialized stdin buffer; all captured profiler/workload/ms_print commands succeed. The original16-a cold lacks the8KiB buffer and is explicitly excluded. Matching0/1/N controls independently give:

| Item | Inline | Incremental requested heap | Incremental Massif extra |
|---|---:|---:|---:|
| Shared graph |6448B static|2280B once/process|4056B|
| Early/late Engine |1232B|3B each|21B each|
| E58 Receipt |104B|160B each|8B each|
| Fixed-seed CreationReceipt |72B|1027B each|29–30B observed|

Pre-reserved Vec allocations are present in both controls; adding inline bytes only once avoids double-counting. Engine's96B Board/824B Position are inside1232B, not additional allocations. N16/N64 requested deltas are48/192B(game),2560/10240B(road receipt),16432/65728B(creation). BothN64 captures agree; creation extra varies across counts and PERF correctly reports an observed range rather than assigning it entirely to the receipt. Static graph and allocations are separate from Massif's5112B thread/process retained-stack total. Current state fields and early/late controls support selected-history boundedness; no transient peak/RSS proof is claimed.

**Attribution/reproduction:** actual timing binarySHA36d509ff077e9183d8bed5f363a3fac1ea973761982338cdc4f1cefa56753955 matches recorded evidence. All four input hashes match. Benchmark fingerprint rustflags[]; current relevant Cargo configs/profile overrides are absent. Profile is ordinary optimized native bench; the separate GDB build explicitly sets bench debug2 and has its own hash/commands. All46 checksummed evidence files pass sha256sum --check. All48 production files equal C10HEAD03c60143af098037593d2c07fc2674acbd492c1d.

### Exact reviewed candidate hashes

- PERF.md: f6af7ae7a75466751856a05040279748833d4f34a1ae1ba0e2db2da9fe48cda0
- Cargo.toml:4af48a1cd7486a941d022fb656b7d695dfa54591d3e51b1c660c6e996a03ff19
- benches/engine.rs:b6b92761341691b3fb10755c58c6dbe1820b34cb308b0d0128130bd4a101d40b
- benches/fixtures.json:1dd002e026f4ef0530010866ded5447f80c05ae88d94e50c0c4bf6e330843d41
- Private original evidence checksum manifest:33daf6ab152ed61e1699e66bd1406c542c2818654296f6dad3fe5c1a9cecdd36
- Private original Criterion log archive:4a2101e9988975d15c8c7b2a2807c266e21ca486df17ad0e0e2bb148ba62d627
- Private original gate records:2cd8a639125a36cc7160a80831ff52483239588a2df55773703ce8056008d842
- Cargo.lock:47d1bf2874e9b173786a7f0ebe7f0fafd40ceaf87ba6eb0834ba495f15e5070c

The original reviewer produced only the retained review report. Source/timing
preflight and independently computed median tables remain privately preserved.

Skipped repeating passed suites/profiles/timings/fault controls; exhaustive global-worst-case route, performance budget, transient peak memory and consumer bindings were not established. T31 task/checkpoint bookkeeping and clean verified commit belong to root/owner. T32 needs explicit root activation after that commit; T33/C11 remains required before declaring the engine handoff complete.

### Packaging-only recheck

Six verbatim Criterion stdout logs were moved into criterion-logs.tar.gz after terminal blank lines triggered Git whitespace checks. Each archive member remains valid UTF-8, retains20 timing results and its final blank line. Independently rebuilding the original51-entry checksum manifest from the current unchanged45 non-log entries and these six archived member digests exactly reproduces prior reviewed manifestSHA ec25a9122e7f8c5e3ce3b5e53fb6b4e0349b32e41efa81958f47a84377c78e06. This establishes byte preservation for every original log and unchanged raw inputs/results. The refreshed46-entry manifest verifies; staged git diff --check passes. No source, measurement, gate or scope changed. T31 approval remains valid at the refreshed exact hashes above. Member digests and the independent reconstructed manifest remain privately preserved.
