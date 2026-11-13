# C11: Historical native engine handoff evidence

**PUBLIC DERIVATIVE — historical summary.** This preserves the original native
engine assessment and evidence; it is not verification of the new release tree.
Historical source IDs and candidate hashes refer to privately preserved original
revisions, not new public-history commits or current renamed-file hashes.
T43 verified the original report SHA-256
`12f33dad75230791dd79e6893166bede29e58e277749c25ef92f602eeee58c36` against the T35 preservation inventory and reconciled the relevant
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

Verified on 10 November 2026, based on T32 commit
`2272e27da6003b0aa7d914de4793ab20a909e320`. The [independent final review](t33-review.md)
approved the exact pre-closure candidate; subsequent edits record that approval
and task/checkpoint completion. The documentation change replaces the crate doc
comment with an attribute, without changing engine behavior, dependencies,
configuration, rules or quality thresholds. The README's Rust example is the crate rustdoc source.

## Historical final local checks

Rust/Cargo 1.98.1, LLVM 22.1.8, `x86_64-unknown-linux-gnu`, cargo-llvm-cov
0.9.1 and cargo-deny 0.20.2. Original raw logs, exact argument arrays, durations, tool versions and 103 input
hashes remain privately retained. Inputs
include every tracked source, test and benchmark file, the README, specifications,
constraints, performance and review evidence, manifests and checker config.
The evolving task and checkpoint records are excluded to avoid self-reference.

| Command | Result | Seconds |
|---|---|---:|
| `cargo fmt --all -- --check` | pass | 0.262 |
| `cargo check --locked --all-targets` | pass | 0.999 |
| `cargo clippy --locked --all-targets -- -D warnings` | pass | 3.940 |
| `cargo test --locked --lib --tests` | pass: 199 tests (131 unit, 68 integration), none failed or ignored | 34.364 |
| `cargo test --locked --doc` | pass: 11 doctests (6 ordinary, including the README example, and 5 compile-fail); repeated after README EOF correction | 1.114 / 1.083 |
| `cargo build --locked --release` | pass | 2.335 |
| `cargo doc --locked --no-deps` | pass | 0.820 |
| `cargo llvm-cov --locked --lib --tests --ignore-filename-regex '(^\|/)(tests\|benches)/' --fail-under-lines 90` | pass: 98.72% (5,955/6,032 library lines) | 44.836 |
| `cargo deny --locked check advisories` | sandboxed attempt failed to acquire database lock; authorized refreshed scan passed | 0.254 / 0.835 |
| `cargo bench --locked --bench engine -- --test` | pass: 20 smoke cases | 4.169 |

The initial formatting check failed on a blank line between crate attributes;
it was corrected before the listed checks. A subsequent staged diff check found
an extra blank line at the README EOF; that whitespace alone was removed and
the README doctest was repeated. The other listed checks preceded this
whitespace-only correction. Both README input hashes, the initial formatting
and staged-diff logs, and both sandboxed advisory failures are retained. The authorized scan returned `advisories ok` after refreshing the
RustSec database at `d9cb059f3217db87aeccd190b81eaffe8cc29cb9`; its direct
tool output is retained in the private original archive. After the checks, Markdown table pipes
were escaped in this report and CONSTRAINTS.md; executable inputs were unchanged.
The approved coverage floor remains 90%. The coverage tool also counts inline source test functions, so the
percentage is not a substitute for direct behavioral assertions. Hosted CI is
not configured.

## Success-criterion evidence

[C10](c10-checkpoint.md) links direct source-traced BR001–BR038 conformance,
six full public-command victories across both setup modes and seat counts,
mandatory decision, privacy, rejection, replay, resume and malformed-input
properties. [Source fixture notes](../tests/support/setup-source.md),
[complete-game provenance](../tests/support/full-games-source.md), the
[rule map](rules-conformance.md), and [property evidence](property-evidence.md)
distinguish independent expected facts from product assertions. All 21 command
variants, exact road calculation, both approved card policies and all-viewer
projections were included in the earlier conformance and privacy work.

[T32 independent review](t32-review.md) approved architecture, shared validation,
private boundaries and measured hot paths with no required correction. It found
no justified absolute performance budget. [PERF.md](../PERF.md) records the
reference machine, three measurements and retained memory: Engine is 1,232
inline bytes with 3 useful and 21 extra allocator bytes per game in the
three-seat fixture; Board 96 and Position 824 are already included. Same-binary
variation exceeded 5% in some cases, so the unchanged 5% margin remains a
manual repeatability review, not a Criterion-enforced gate. Measurements do not
establish a hardware-independent latency ceiling or transient-memory budget.

The guide covers authenticated actor/viewer selection, private seed and state,
revision and loss handling, mandatory choices, safe original receipt projection,
strict bounded input, snapshot resume and accepted-command replay. Imports
validate coherent current facts, not historical honesty. Creation receipt size
can grow under repeated tied starter rolls. UI, transport, storage,
authentication, browser bindings, cross-target replay, hosted CI, publication
and deployment are outside this library handoff. The guide's repository-relative
evidence links refer to checkout files; when rendered inside rustdoc they are
not copied into the generated documentation tree.
