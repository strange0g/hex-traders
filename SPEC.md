# Spec: Hex Traders Base Game Engine

## Hex Traders release initiative

The user approved the [release capability map](CAPABILITY-MAP-hex-traders-release.md)
on 13 November 2026 and advanced these release specifications to planning by
invoking `$plan`. They are accepted for planning, written in dependency order:

| Module id | Specification | Depends on |
|---|---|---|
| `hex-traders-branding` | [Complete identity and release-tree rename](SPEC-hex-traders-branding.md) | Existing engine contracts |
| `github-publication` | [Public repository and initial release](SPEC-github-publication.md) | `hex-traders-branding` |

Hex Traders becomes the sole published identity, with a fresh local checkout and
public Git history. Publish `strange0g/hex-traders` publicly with dual
MIT OR Apache-2.0 licensing and an initial `v0.1.0` release. Original history,
source URLs and historical evidence are preserved privately; existing saves need
no migration. The package is `hex-traders-engine`, imported as `hex_traders_engine`.

The approved engine baseline below remains authoritative for game behavior,
module direction, privacy, deterministic randomness and quality requirements.
The release specifications govern the identity changes and GitHub infrastructure
once approved. Native identity was verified in T37. Governing text and public provenance
use the new identity. Fresh history, relocation and the public repository are
verified in T45 and T50; the initial release remains subject to T52–T53. Do not overwrite completed tasks or reinterpret prior checks as fresh
release evidence. The [release plan](tasks/plan.md) and [tasks](tasks/todo.md)
were approved for autonomous implementation by the user's `$build auto` invocation
on 13 November 2026. Task completion requires new release verification.

## Approved engine baseline

Status: approved planning baseline, 9 November 2026. Rust and the four internal modules
are approved in [CAPABILITY-MAP.md](CAPABILITY-MAP.md). The user authorized
both card edge-case decisions and then invoked the plan skill to advance to
implementation planning. The [plan](tasks/plan.md) and [task list](tasks/todo.md)
are approved, including the carried-forward digital policies and quality defaults.
The user subsequently authorized `$build auto` to implement the complete plan
autonomously with subagents. [tasks/todo.md](tasks/todo.md) records implementation
and verification status. Hosted release checks are governed by the
`github-publication` provider specification.

## Objective

Build a complete, playable implementation of the official 2025 English
sixth-edition CN3081 base game for 3–4 human players. The engine owns the rules,
game state, legal decisions, randomness, and victory. A consuming UI submits
commands and displays projected results. It must not need its own rule logic.

Maintainability, modularity, functional design, reusable calculations, and
performance are mandatory. Success means both setup modes and every base-game
action work from creation to immediate victory, including hidden information,
save/resume, and deterministic replay.

## Specification index and authority

Read provider contracts in dependency order:

| Module id | Specification | Responsibility |
|---|---|---|
| `game-model` | [SPEC-game-model.md](SPEC-game-model.md) | Domain types, stable board graph, compact position and invariants. |
| `base-rules` | [SPEC-base-rules.md](SPEC-base-rules.md) | Normative BR001–BR038 rules, calculations, costs, legality, economy, cards and scoring. |
| `turn-engine` | [SPEC-turn-engine.md](SPEC-turn-engine.md) | Creation, phases, atomic commands, controlled randomness and victory timing. |
| `engine-api` | [SPEC-engine-api.md](SPEC-engine-api.md) | Consumer commands/queries, viewer-safe contracts, negotiation, save and replay. |

[CONSTRAINTS.md](CONSTRAINTS.md) is the approved written quality contract;
[quality-check evidence](tasks/quality-checks.md) records local tool controls
and [tasks/todo.md](tasks/todo.md) records verification results.
Provider specs own their interfaces; BR001–BR038 own game-rule meaning. Root
spec owns scope and shared conventions. Resolve contradictions here before
implementation rather than letting separate modules choose different answers.

The source baseline is the 2025 rulebook, CN3081, v6.250401 (original source URL privately preserved).
Use the official base-game FAQ (original source URL privately preserved) only for
compatible clarifications. The selected edition takes precedence over older
and extension-specific entries. Sources were checked on 9 November 2026.
Requirements paraphrase behavior; no publisher artwork is a product asset.

## Scope and user-visible behavior

- Fixed beginner setup and variable setup with official terrain, number,
  frame/port, placement, starting-resource, and starting-player procedures.
- Exact base-game economy: production and shortages, resource conservation,
  costs, inventories, road/settlement/city legality, and maritime/domestic trade.
- Two-dice turns, sevens, chosen discards, robber relocation and weighted theft.
- All 25 development cards with exact eligibility, pre-roll timing, one non-VP
  card per turn, hidden VP, and required multi-step choices.
- Exact Longest Route including loops, branches, blockers and ties; Largest
  Army; true/public scoring; immediate victory at 10 on the owner's turn.
- A stable API for commands, legal choices, player/spectator views, errors,
  activity events, host-private snapshots, and deterministic command replay.

A host owns accounts, display names, transport, storage, scheduling, and seed
generation. The engine verifies rule eligibility using the seat/viewer supplied
by that trusted host. It performs no I/O and has no timer, rendering dependency,
or global mutable state.

UI, networking, accounts, matchmaking, expansions, and house rules are outside
this release. Browser/native/mobile consumers can use a host adapter; Rust
consumers can embed the library directly. A browser binding is added when a
concrete UI needs it. No generic plugin system or transport framework is needed.

## Architecture and performance

One Rust library crate contains four internal modules. Imports go from
consumers to providers without cycles. Stable module IDs use kebab-case in
documents; Rust directories use underscores. Runtime cooperation uses direct
calls and borrowed inputs, with no per-module messages or JSON conversion.
`src/lib.rs` exposes the consumer facade and necessary public types.

Calculations are pure. Transition evaluation borrows board/state and produces
a next compact position, revision, and optional RNG update; the facade commits
only success. Nonrandom commands retain the existing generator. Rejected commands leave
state, RNG, revision, offers, and card/resource holdings unchanged. This does
not require a deeply copied object tree: topology is immutable and shared;
resources are five counts; bounded positions contain no complete replay log.

Use indexed adjacency and distinct dense graph IDs. Reuse the same validators
for queries and commands. Recalculate Longest Route only after a relevant road
or blocking settlement changes. Enumerate bounded placement choices and validate
specific resource bundles. No redundant full-state checks at internal boundaries.
Full invariants run on construction/import and in tests.

Rust permits explicit memory control and native compilation; it does not prove
this implementation is fast. Benchmark optimized complete commands, rejection,
legal choices, view projection, snapshot operations, difficult road graphs,
and memory per game on documented reference hardware. Measure core execution
and serialization separately. First establish a reproducible baseline, then
apply the proposed regression policy in CONSTRAINTS.md. Additional caches or
representation changes must earn their complexity through measurements.

## Tech Stack

Approved stack and pinned implementation versions:

| Item | Selection / purpose |
|---|---|
| Rust / Cargo | Stable 1.98.1, edition 2024; pin the toolchain when the project is created. Native `x86_64-unknown-linux-gnu` is the initial checked target. |
| Standard library | Arrays, enums, checked newtypes/arithmetic, results and direct calls. Safe Rust only. |
| `rand` | 0.10.3, default features disabled, `unbiased` enabled; deterministic unbiased sampling and shuffling, no global entropy source. |
| `rand_chacha` | 0.10.0, default features disabled; explicit ChaCha12 seeded generator. |
| `serde` | 1.0.229 with `derive`; wire/snapshot boundary types. |
| `serde_json` | 1.0.154; versioned JSON import/export. |
| Built-in Rust tests | Unit, integration and documentation tests. |
| `proptest` | 1.12.0, development dependency for invariants and shrinking. |
| `criterion` | 0.8.2, development dependency for release benchmarks. Use `std::hint::black_box`. |
| `cargo-llvm-cov` / `cargo-deny` | External coverage/advisory tools; pinned versions and controls are recorded in [quality-check evidence](tasks/quality-checks.md). |

Commit `Cargo.lock` when created. Pin the RNG/sampler dependency versions and
golden sequences; changes affecting random results require declared replay
compatibility decisions. Serde is a boundary tool, not an inter-module protocol.
Keep the runtime dependency list small; do not add a crate for a few ordinary
standard-library operations.

The core must avoid native-only OS APIs so it can compile for
`wasm32-unknown-unknown`. That target and `wasm-bindgen` are not installed or
implemented now. Native/Wasm determinism is a required check when a binding is
introduced; there is no second browser implementation of the rules.

Primary technical references: [Rust ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html),
[module visibility](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html),
[Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html),
[rand](https://docs.rs/rand/0.10.3/rand/),
[rand_chacha](https://docs.rs/rand_chacha/0.10.0/rand_chacha/),
[Serde](https://serde.rs/), [Criterion](https://docs.rs/criterion/0.8.2/criterion/),
[Proptest](https://docs.rs/proptest/1.12.0/proptest/),
[Wasm target](https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html),
and [wasm-bindgen](https://wasm-bindgen.github.io/wasm-bindgen/).

## Commands

Run these local checks against the implementation; actual results and
limitations are recorded in [tasks/todo.md](tasks/todo.md). No development
server is required for a library.

```bash
cargo check --locked --all-targets
cargo build --locked --release
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --lib --tests
cargo test --locked --doc
cargo doc --locked --no-deps
cargo llvm-cov --locked --lib --tests --ignore-filename-regex '(^|/)(tests|benches)/' --fail-under-lines 90
cargo deny --locked check advisories
cargo bench --locked --bench engine -- --save-baseline approved
cargo bench --locked --bench engine -- --baseline approved
```

The two baseline commands serve different steps: establish an approved baseline,
then compare a candidate. Criterion produces reports; it does not enforce the
regression percentage by itself. Coverage/advisory commands require their tools
and configuration first. Missing check tools are reported, not called a pass.

Once a browser consumer is authorized and the target is installed:

```bash
cargo check --locked --target wasm32-unknown-unknown --lib
```

This future compile check alone does not test JavaScript bindings or cross-target
replay; a binding needs executable contract and determinism tests.

## Project Structure

Representative layout; see the source tree for actual focused files:

```text
Cargo.toml / Cargo.lock / rust-toolchain.toml
src/
  lib.rs                  public facade and deliberate exports
  game_model/             shared values, graph, state, commands and events
  base_rules/             pure placement, economy, card and scoring calculations
  turn_engine/            creation, eligibility, phases, execution and RNG
  engine_api/             views, wire conversion, snapshots and replay
tests/
  support/                sourced fixtures and independent expected calculations
  model_invariants.rs
  rules_conformance.rs
  game_sequences.rs
  api_contract.rs
  privacy.rs
  snapshots.rs
  replay.rs
benches/
  engine.rs               representative immutable fixtures and measured operations
SPEC.md / SPEC-*.md / CAPABILITY-MAP.md / CONSTRAINTS.md / AGENTS.md
```

Create files when responsibilities need them; the per-module lists are guidance,
not a requirement to generate empty boilerplate. Colocate private-helper unit
tests. Integrations go through public contracts. Preserve the existing skill
installation and agent configuration directories.

## Code Style

Use Rustfmt, descriptive `snake_case` functions/fields, PascalCase types, distinct
ID newtypes, private validated fields, exhaustive enums and `Result` errors.
Comments explain a reason the code cannot show. Public rustdoc explains command
preconditions, result/error meaning, privacy, and determinism.

Illustrative transition style:

```rust
pub fn evaluate(
    board: &Board,
    state: &GameState,
    actor: PlayerId,
    command: &Command,
) -> Result<Transition, RuleError> {
    validate_action(board, state, actor, command)?;
    let next = apply_validated(board, state, actor, command)?;
    Ok(next)
}
```

The illustration is not product code. Implement shared predicates once; do
not rerun the full validation or calculation in `apply_validated`. Private
next-state mutation is compatible with a pure public evaluation contract.
Use simple functions rather than per-rule traits, generic registries, dynamic
dispatch, service wrappers, or speculative configuration. No `unsafe`, hidden
global RNG, floating-point game arithmetic, or panic for ordinary invalid input.

## Testing Strategy

1. **Rules:** directly asserted source-traced cases for every BR001–BR038,
   including rejections and conservation. Independently verify source-board
   fixtures, A–R sequence and frame ports before claiming setup conformance.
2. **Model and sequences:** graph/stock/phase invariants; complete 3- and
   4-player games in both setups; pending choices and immediate wins.
3. **Contracts and privacy:** every command/view/event/error variant; stale
   revision and negotiation behavior; exact hidden-data allowlists for every
   seat and spectator. Test malformed input at real import/wire boundaries.
4. **Persistence and randomness:** snapshot each phase, reject corrupt imports,
   resume with the same next result, replay accepted commands with exact
   state/revision/events, and retain random golden fixtures.
5. **Properties and performance:** legal-sequence invariant/property tests
   with reproducible shrunk failures; exact independent road fixtures;
   release-build benchmarks using realistic states with no skipped validation.

The approved coverage floor is 90% of library executable lines, excluding test and
benchmark files. Coverage is a backstop; every rule and security-sensitive
boundary still needs meaningful assertions. Do not mirror production code to
calculate expected answers or replace edge cases with coverage-only tests.
Run related checks while implementing and the complete required suite before
handing back an implementation. A benchmark or skipped check is not evidence
of game correctness.

## Boundaries

- **Always:** read the governing spec and constraints before coding; keep
  maintainability/performance mandatory; reuse rules; validate external input;
  protect hidden data; preserve atomicity/determinism; update contracts and
  traceable tests when behavior changes; report actual checks and limitations.
- **Ask first:** change approved module direction, language, scope, rules
  edition, approved card behavior, public compatibility, or privacy policy;
  relax the quality bar; add a speculative subsystem or externally visible
  release. Existing approval covers routine implementation choices that meet
  this contract; it does not require repeated approval for ordinary fixes.
- **Never:** silently weaken checks, skip failing tests, leave executable stubs,
  leak private state/seed, trust client dice/scores, introduce UI/transport/I/O
  into rules, or change editions by accident.

## Success Criteria

- BR001–BR038 pass source-traced conformance tests, including approved RQ1/RQ2.
- Both setup modes support full legal 3- and 4-player games ending at the exact
  first winning moment; every required choice is represented in the engine.
- Any consuming UI can play through the stable command/query/view contract.
- Every rejection preserves state, RNG, revision and offer; accepted commands
  commit exactly once and legal queries agree at the same revision.
- No view/event/error exposes another player's private hand, unplayed card IDs,
  hidden VP, seed, or full snapshot/replay.
- Saving/resuming every phase and replaying accepted history reproduce exact
  game state, event order and the next random result.
- The library builds/lints/tests cleanly, meets the approved quality contract,
  and has a reproducible release performance baseline. No unmeasured speed claim.
- The dependency direction remains acyclic and there is one implementation
  of each rule, with no UI-side rule calculation required.

## Resolved card cases and remaining review

Two card edge cases lack an explicit resolution in the consulted official
sources. The user authorized selecting the best behavior on 9 November 2026;
the approved engine policies are detailed in SPEC-base-rules.md:

- **RQ1, resolved:** Road Building places roads sequentially, up to two. Reject
  activation if no legal road/piece exists. After the first road, finish on
  victory or if no legal second road/piece remains; otherwise require the
  second. Voluntary forfeiture of a legal second road is unavailable.
- **RQ2, resolved:** Invention takes the sole bank card if only one exists,
  rejects activation when empty, and otherwise requires two available cards.
  Rejections preserve the card, non-VP turn allowance, and all game state.

Plan approval confirms the other explicit digital policies: highest-tied seats reroll
for starting player; fixed top-right variable-number starting corner; one
bilateral offer with defined replacement/closure; public bank availability;
decimal wire counters; strict bounded snapshots; approved quality thresholds.
Reference hardware and measured native performance are recorded in [PERF.md](PERF.md);
absolute performance budgets remain unapproved.

The plan, its baseline, and autonomous implementation are approved. Runtime and
performance claims require the recorded checks and measurements in the task list.
