# Native performance evidence

PUBLIC DERIVATIVE: historical T31 measurements and review facts. Exact original
reports, raw samples, proof sources and controllers are privately preserved.
This document is not a fresh release measurement; T51 records those results.

The first release-profile baseline was recorded on 10 November 2026 against
engine commit `03c60143af098037593d2c07fc2674acbd492c1d` and the benchmark
source identified in input hashes (privately preserved).
These are measurements on one machine, not an absolute budget or a speed
claim. Browser bindings and their boundary costs have not been measured.

Reference machine: Intel Core i7-10700, 8 cores/16 threads, 32,556,868 KiB RAM;
Fedora Linux 7.2.15 x86-64; Rust 1.98.1, LLVM 22.1.8, Cargo 1.98.1; locked
dependencies and the optimized Cargo bench profile. Criterion 0.8.2 used 40
samples, 500 ms warmup and 1 s measurement target per case. The recorded
baseline and repeats ran on logical CPU 7, whose SMT sibling is CPU 15, with
the `performance` governor (CPU 0 before and CPU 7 after the run). One-minute load average was 3.61 at the start and
1.26 at the end; the sibling and other system activity were not isolated.
Hardware (privately preserved), memory (privately preserved),
toolchain (privately preserved), affinity (privately preserved),
times and all commands are retained in private original evidence.

## Timings

Values are Criterion median estimates in nanoseconds, from the named approved
baseline and two immediate same-binary comparisons. The raw samples, confidence
estimates and logs are retained separately for each run.

| Case | Baseline | Repeat 1 | Repeat 2 |
|---|---:|---:|---:|
| Native roll with production | 280.76 | 276.24 | 300.91 |
| Native end turn | 242.40 | 244.06 | 253.81 |
| Native road with award | 1,508.46 | 1,621.16 | 1,582.72 |
| Native hard road | 4,823.88 | 4,892.22 | 5,190.77 |
| Reject stale revision | 7.30 | 7.29 | 7.76 |
| Reject occupied road | 13.49 | 13.75 | 14.38 |
| Validate legal road | 63.13 | 42.13 | 42.89 |
| Validate occupied road | 12.50 | 13.28 | 13.14 |
| Legal actions, variable setup | 2,696.35 | 2,741.78 | 2,729.53 |
| Legal actions, late game | 6,975.51 | 6,983.37 | 7,023.96 |
| Player view | 593.99 | 616.13 | 614.65 |
| Spectator view | 535.22 | 558.41 | 523.07 |
| Project receipt | 26.57 | 25.72 | 24.86 |
| Encode command | 116.45 | 119.35 | 118.97 |
| Decode command | 663.02 | 686.47 | 692.86 |
| Decode and execute; native receipt | 6,005.29 | 5,715.45 | 5,672.64 |
| Encode player view | 8,584.05 | 8,460.43 | 8,519.15 |
| Encode projected receipt | 252.58 | 268.05 | 275.43 |
| Export snapshot | 3,344.85 | 3,219.24 | 3,250.99 |
| Import and validate snapshot | 41,106.54 | 41,164.97 | 39,035.73 |

The median table (privately preserved) includes each
repeat's percentage difference. Same-binary variation exceeds 5% for several
cases, including both repeats of occupied validation and projected-receipt
encoding; the legal-validation baseline was especially noisy. The earlier
unpinned runs also varied substantially and are preserved. These are not code
regressions: no code changed between runs. The approved 5% threshold remains a
**manual review** margin, not a noise bound or Criterion exit-code gate. Repeat
ambiguous candidate comparisons under matching conditions and examine the raw
samples before deciding whether a tradeoff requires approval. The initial
pinned `--save-baseline` log compares with the previous unpinned baseline
before replacing it; its saved `approved` files are the pinned values above.

Accepted command iterations use `iter_batched_ref` with at most 16 independently
imported and validated pre-command games per batch. Import and input/output
drops are outside the timed command section; every command unwraps an accepted
receipt. The snapshot-import case likewise excludes its returned Engine's drop.
Ordinary `iter` cases include returned value destruction. Rejections are
repeated on an unchanged engine after exact error and snapshot/RNG checks.
JSON execution includes decoding and native execution, returning a native
receipt; viewer projection and JSON response encoding are separate cases.
No production validation or route calculation is bypassed.

## Fixtures and correctness

The synthetic fixed-three-seat seed is 15508, stored little-endian in the
first four seed bytes. A public 77-turn source-derived trace reaches the hard
road at turn 77, actor 0, revision 191. Its independent literal graph checker
finds 14 roads and exact route 7 before E58, then 15 roads and exact route 10.
The network contains branching and two physical cycles, with an opponent
building at V19 serving as a blocked endpoint. It is a demanding case, not a
proven global worst case. An earlier real E44 command gives the route award;
the ordinary roll fixture begins at revision 0. Variable setup and late-game
queries exercise both bounded enumeration contexts.

The complete trace was replayed through public commands on C10, comparing each
source-derived balance and route prefix. Both hard-road snapshots import and
round-trip byte-for-byte. Executing from the original and resumed pre-state
produces the same receipt and post-state. Saved RNG seed/stream/word `164` are
unchanged by E58; the next independent oracle roll is `[2,3]` at word `166`,
also checked on the resumed and no-road control paths. The benchmark checks
its frozen snapshots, exact results, errors and wire/native agreement before
sampling. All seeds and private-format snapshots here are synthetic. Real
snapshots and receipts contain private facts and must remain host-owned.

The privately preserved fixture proof contains the frozen trace, independent
endpoint checker, source literals, pinned RNG oracle, C10 public verifier and
its original output. Those sources/results retain their original byte hashes;
no archived historical script is an active public reproduction command.
Current source-traced assertions and benchmark guards use the checked-in synthetic
fixture [benches/fixtures.json](benches/fixtures.json) and the public
[fixture provenance](tests/support/full-games-source.md).

## Retained memory

External Valgrind Massif 3.28.0 sampled a release-profile public workload at
named phases, using `vgdb` detailed snapshots. The workload reserved identical
capacity for 0, 1 and N values and kept live inputs visible to the profiler.
The table gives incremental useful heap and Massif's extra allocator bytes;
inline values already held in reserved heap vectors must be counted only once.
The graph is shared once per process. The creation receipt is the specific
seed's receipt, whose event list can grow if starter rolls tie.

| Retained item | Inline bytes | Incremental useful heap | Incremental allocator extra |
|---|---:|---:|---:|
| Shared `BoardGraph` | 6,448 static | 2,280 | 4,056 |
| One game `Engine` | 1,232 | 3 | 21 |
| One E58 `Receipt` | 104 | 160 | 8 |
| One `CreationReceipt` | 72 | 1,027 | 29–30 observed |

At N=16 and N=64 the useful-heap increments are respectively 48 and 192 bytes
for games, 2,560 and 10,240 for E58 receipts, and 16,432 and 65,728 for
creation receipts. The two N=64 phase captures agree. Extra bytes for creation
receipts differ slightly across the two counts, so the range above is an
observed incremental value that may include profiler/harness effects. The
per-game board layout and position are **inside** Engine: a separate
release-with-debuginfo GDB inspection found `Board` 96 and `Position` 824
inline bytes; these are not additional heap allocations. GDB reported the
same 1,232-byte Engine as `std::mem::size_of` in the profiled binary. The
separate debug build and its warnings/commands are recorded; no public type
export or allocator hook was added.

The early game at revision 0 and late game at revision 191 retain the same
3 useful heap bytes and 21 extra bytes per game at both N values after caller
receipts are dropped. The position has bounded fields and contains no match
log; this comparison supports that property for the selected history. Massif
reported 5,112 stack bytes at the corrected retained phases, including the
profiler helper thread and workload; it is a process/thread total, not a
per-game stack figure. It does not describe transient peak stack use.

The first exploratory cold snapshot preceded initialization of an 8 KiB stdin
buffer. It is retained as `massif-16-a` but is excluded from the graph delta.
The corrected controller waits for that buffer in the allocation tree before
the cold snapshot. Raw phase totals (privately preserved),
allocation trees, controller commands, status and `ms_print` reports are
preserved in the four `massif-*.tar.gz` archives. Profiling is not a latency
measurement and no unsafe allocation instrumentation is used.

## Reproduction

The original baseline and two comparisons ran pinned to logical CPU 7 using the
privately preserved timing controller, with raw Criterion JSON retained between
runs. Original raw baseline, repeat, Massif and debug-size evidence remains
privately recoverable; its old scripts and paths are not active public commands.

For a new reference run, document CPU/RAM/OS/toolchain/profile/affinity/load and
matching fixtures, then run:

```bash
cargo bench --locked --bench engine -- --save-baseline approved
cargo bench --locked --bench engine -- --baseline approved
```

Saving overwrites the local approved baseline. Compare against preserved original
samples only under matching conditions, repeat ambiguous differences and retain
raw variability. The unchanged 5% margin is a manual review requirement.
T51 supplies a reviewed fresh controller using the current memory switches and
records actual N=16/N=64 heap measurements. Original Massif/GDB commands and
46-entry evidence checksums remain privately preserved; none were rerun in T40.

## Verification

Formatting, all-target check and clippy, release build and rustdoc passed.
The full suite passed 199 tests and 10 doctests; library line coverage was
98.72% (5,955/6,032), above the unchanged 90% floor. The Criterion `--test`
smoke run passed all 20 cases. The first advisory scan failed because the
database lock was on a read-only path; the narrowly escalated refresh then
passed with `advisories ok`. Gate commands and results (privately preserved)
and complete logs (privately preserved) retain both attempts.
