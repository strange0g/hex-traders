# Constraints: Hex Traders Base Game Engine

Status: approved written contract with the plan on 9 November 2026.
Maintainability, modularity, functional reuse, and performance are explicit
user requirements. Numerical defaults below are approved requirements, not
measurements. Local check tools and the pinned hosted checks are verified;
[release enforcement evidence](tasks/evidence/release/t50-hosted.md) records
the actual GitHub checks, protections and failure controls.
Implementation and verification status are recorded in [tasks/todo.md](tasks/todo.md).

Historical measurements and completed-checkpoint conclusions below describe the
approved native baseline. Exact original evidence is privately preserved; current
release verification is recorded in the task list. No threshold has changed; the release-only exception is recorded below.

## Floor

- Safe Rust only; use `#![forbid(unsafe_code)]` when source is created.
- Zero ignored errors, unimplemented executable stubs, `todo!()`/`unimplemented!()`
  production paths, or panics for ordinary player/input rejection.
- No new lint/coverage/security suppressions or ignored/skipped/deleted tests
  to make a change pass. Remove obsolete tests only with a documented reason
  and replacement coverage where behavior remains.
- No secrets, private seed, real private game snapshot, or private replay in
  source/logs/player output. Deterministic test seeds are explicitly fixtures.
- Do not weaken this file, rule assertions, or check configuration to pass a
  task. Exceptions require a concrete reason, human approval, owner, and expiry.

## Quality gates

Commands are local gates only after their clean and deliberately failing controls
and tool-error controls have been verified. [T02 evidence](tasks/quality-checks.md)
records those checks and the bootstrap coverage limitation. Behavior, privacy,
and architecture still require the listed tests or manual review as implemented.
The release also enforces the pinned `quality`, `coverage` and `advisories`
checks on GitHub, with deliberately failing and tool-error controls recorded in
[T50](tasks/evidence/release/t50-hosted.md). Architecture and native performance
remain manual review gates. Do not confuse a report with a gate.

| Dimension | Approved rule and reason | Verdict / verification | Runs when implemented |
|---|---|---|---|
| Types/build | Zero compiler errors; consumers need a usable library. | `cargo check --locked --all-targets` and `cargo build --locked --release` | Related edits / task end |
| Formatting/lint | Zero formatting differences and Clippy warnings; keep the code consistently readable. | `cargo fmt --all -- --check`; `cargo clippy --locked --all-targets -- -D warnings` | Task end, CI |
| Behavior | All tests pass; every BR001–BR038 has direct assertions and rejectable cases. | `cargo test --locked --lib --tests`; `cargo test --locked --doc`; review BR-to-test traceability | Related edits / task end, CI |
| Coverage | At least 90% library executable lines; backstop for a rules-heavy new library, not proof of correctness. | `cargo llvm-cov --locked --lib --tests --ignore-filename-regex '(^\|/)(tests\|benches)/' --fail-under-lines 90` | Task end, CI |
| Dependency security | Zero known vulnerabilities in the resolved dependency graph; small runtime dependency set makes the stricter floor practical. | `cargo deny --locked check advisories`, with explicit advisory-deny configuration and scanner/database errors treated as failure | Review, CI |
| Atomicity/privacy | Zero mutation on rejection; zero unauthorized private fields in views/events/errors. | Contract/privacy/replay assertions through the complete test command above | Task end, CI |
| Architecture | Zero reverse/cyclic imports; one validator/calculator per rule; core has no UI/transport/storage/timer/global RNG. | Review Rust module imports and diff against CAPABILITY-MAP.md; compilation checks visibility, not architectural intent | Review; written/manual gate initially |

The compiler check, rustfmt, Clippy, coverage threshold, and advisory scanner
have passed clean, violation, and tool-error controls locally. At T02 the empty
bootstrap correctly failed coverage for no data. At C01 the implemented graph
passes the 90% floor at 95.77% (136/142 lines), with independent integration
tests. The real advisory scan passed after a database refresh. Gameplay,
atomicity and privacy evidence is recorded in [C10](tasks/c10-checkpoint.md);
architecture and performance remain manual review gates, reviewed in
[T32](tasks/t32-review.md).

The Rust toolchain/components are pinned in `rust-toolchain.toml`, and external
checker versions and installation commands are recorded in the T02 evidence.
Cargo-deny supplies an external vulnerability database
opinion in addition to project tests. Do not pretend a manual architecture
review is an automated dependency-direction gate.

## Performance requirements and baseline

Native release measurements and their limitations are in [PERF.md](PERF.md).
The four-module design is not benchmark evidence. Regression policy:

- Establish and record native reference CPU/RAM/OS, Rust version, target,
  release profile, fixed fixtures/seeds, and benchmark command before comparing.
- Measure accepted/rejected commands, targeted legal-action queries, viewer
  projection, snapshot import/export, exact Longest Route, and per-game memory.
  Separate serialization from core execution; keep history outside the position.
- Use Criterion release benchmarks:
  `cargo bench --locked --bench engine -- --save-baseline approved` to record,
  then `cargo bench --locked --bench engine -- --baseline approved` to compare.
- Proposed review threshold: no repeatable regression greater than 5% in
  median time or per-game memory against the approved fixture baseline without
  an explained and approved tradeoff. Five percent is a starting review margin
  for a new project, not a universal measurement-noise bound. Repeat ambiguous
  results under the same conditions before claiming a change.
- Record raw measurements and run-to-run variability. Criterion reports do not
  fail the process solely because the 5% threshold is exceeded. This remains a
  review gate until a reproducible comparison gate is configured and tested.
- Absolute budgets must follow representative measurements. Do not invent a
  number/hardware claim or drop validation, exact road calculation, privacy, or
  deterministic replay to improve a benchmark.
- Position size must remain bounded by base-game data, independent of match
  length. Private receipts/logs belong to the caller. Additional caches or
  specialized representations require evidence and correctness checks.

| Metric | Current value | Direction / next evidence |
|---|---|---|
| Library coverage | T31: 98.72% (5,955/6,032 library lines) | Maintain at least 90%; inline source tests also count toward the tool report. |
| Command/query/view/snapshot timing | [Native baseline and variability](PERF.md#timings) | Record a fixed-fixture optimized native baseline and compare matching conditions; no absolute budget is approved. |
| Per-game memory / allocations | [Native retained-memory measurements](PERF.md#retained-memory) | Record position/board/receipt sizes and representative heap allocation; position/board are included in Engine, and state growth remains history-independent. |
| Browser boundary cost | Unmeasured; no binding | Measure separately when a concrete browser consumer exists. |

## Check cadence and protecting the bar

Run related checks while editing and full required checks before task handoff.
Proposed routine task-end budget is about 90 seconds on reference hardware;
move slow benchmarks/scanners to review or CI instead of weakening them. If a
required test takes longer, complete it and report the timing. No runtime UI
exists, so accessibility and Web Vitals are not engine-library gates.

Review each change for lowered thresholds, removed assertions/tests, added
suppression attributes, production stubs, ignored errors, and new exceptions.
Compare this contract with the approved version. Check tools that are missing
or broken block an enforcement claim; report the limitation clearly.

## Exceptions

Any exception must record rule, affected paths, concrete reason, approved owner,
and expiry no later than 90 days after approval. RQ1/RQ2 are approved engine
policies in SPEC-base-rules.md, not exceptions to the quality bar.

### R01: Initial source release with unresolved native timing acceptance

- **Approval and owner:** maintainer `strange0g`, 14 November 2026, explicitly
  instructed “proceed with the release” after being informed that T51 and the
  release were pending because clock/turbo conditions prevented acceptance.
- **Reason and scope:** publish the initial `v0.1.0` GitHub source release while
  disclosing the unresolved T51 comparison and incomplete current public
  performance evidence. This permits its T52/C20 release decision and T53
  publication to proceed; it does not complete T51 or establish a timing pass.
  Affected paths are this record, `docs/releases/v0.1.0.md`, `tasks/todo.md`, and
  `tasks/evidence/release/t52-decision.md`.
- **Limits:** the manual 5% rule, fixtures, assertions, security/build/test gates
  and all future release requirements remain unchanged. No confirmed regression
  tradeoff, machine tuning, missing security check or private evidence publication
  is approved by R01. Original measurements remain privately recoverable.
- **Expiry:** completion of `v0.1.0` publication or 28 November 2026, whichever
  comes first. R01 grants no continuing or future-release waiver; resolving T51
  still requires evidence under the standing policy.
