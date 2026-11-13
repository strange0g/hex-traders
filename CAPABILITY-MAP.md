# Capability Map: Hex Traders Base Game Engine

Status: approved on 9 November 2026. Rust and four internal modules in one
library crate are selected. Performance is an explicit priority. Detailed
specifications and implementation plan are approved. The user subsequently
authorized autonomous implementation; [tasks/todo.md](tasks/todo.md) records
verified status.

## Objective

Build a complete, playable base-game engine that owns the rules and game state.
Any UI can submit commands and display the resulting views without implementing
game rules itself. Maintainability, functional design, reusable rule logic, and
performance are required constraints.

## Approved modules

| Module id | Responsibility | Depends on |
|---|---|---|
| `game-model` | Shared domain types and values; board graph with stable hex, vertex, and edge identities; resource and piece inventories; complete state representation. No screen coordinates or UI dependencies. | — |
| `base-rules` | Pure calculations and rule-specific validation for placement, building, production, trading, robber effects, development-card effects, and scoring. Each rule has one implementation. | `game-model` |
| `turn-engine` | Game creation and setup; turn and decision phases; command execution; required choices; controlled randomness; atomic state transitions; immediate victory checks. Owns the shared action-validation path used by both execution and legal-action queries. | `game-model`, `base-rules` |
| `engine-api` | Stable public commands, queries, results, and errors; external input validation; player and spectator views with hidden information protected; legal-action queries; versioned snapshots and deterministic replay. | `game-model`, `base-rules`, `turn-engine` |

Dependency direction: consumers import providers. Providers never import their
consumers. These are internal modules in one Rust library crate, with one public
entry point. They are not independent services, processes, or separately
published packages. Runtime cooperation uses ordinary function calls and
borrowed references. JSON is needed at external transport or persistence
boundaries, not between modules. A module boundary must not require cloning
state, allocating messages, dynamic dispatch, or repeating validation.

Build order: `game-model` → `base-rules` → `turn-engine` → `engine-api`.

The separation between `base-rules` and `turn-engine` keeps calculations
independently reusable while the turn engine handles sequencing and pending
decisions. This boundary is useful only if it remains small: do not split one
rule across duplicate execution and query pipelines. Rust's module system
provides organization and visibility without requiring runtime communication
infrastructure. Actual command costs still require benchmarks.

## Approved scope and stack

- Rules baseline: the official 2025 English sixth edition for 3–4
  human players. Use compatible official FAQ clarifications for edge cases;
  the selected edition takes precedence over older or extension-specific FAQ
  entries. Do not mix editions or invent house rules.
- Complete setup-to-victory play, including trading and every development card.
- An engine library that can be embedded locally or hosted by another program.
  A UI, networking, accounts, matchmaking, and expansions are later work.
- Stack: Rust for the engine, compiled natively for a host program.
  Browser use can compile the same core to WebAssembly with a thin JavaScript
  binding. The rules must not depend on a renderer or host-specific APIs.
- Rust trades direct TypeScript integration for explicit memory control and
  native compilation. A browser binding adds build and interoperability work;
  it does not require another implementation of the rules. Implement a binding
  when a concrete consuming UI needs it.
- Host programs own storage, transport, authentication, and scheduling. The
  engine owns validated commands, rules, state, visibility, and replay behavior.

## Required architecture constraints

- Functional core: explicit inputs and results, read-only borrowed inputs,
  value-based state transitions, and reproducible randomness. The public
  facade may own and commit the current position after successful evaluation.
  Rejected actions must not change position, random state, or revision.
- One source of truth for each rule. Gameplay and legal-action queries reuse
  the same validation, and UIs do not duplicate it.
- No cyclic module dependencies; no rendering, transport, storage, or framework
  imports in the core engine.
- Stable public contracts and versioned saved-state formats, with contract
  tests protecting consumers and rule tests covering edge cases.
- Future capabilities extend these boundaries when needed. Do not build a
  speculative plugin system or a separate service for each module.

## Architecture alternatives considered

| Direction | Benefit | Cost / concern | Assessment |
|---|---|---|---|
| One large engine file | Minimal initial organization | Rules and turn handling become difficult to isolate, reuse, and test as the complete game grows | Poor fit for the required maintainability |
| One library, three internal code areas | Direct calls, shared state types, focused rule files, and one stable consumer API | Calculations and command sequencing share a larger module | Viable simpler alternative |
| One library, four internal modules | Separates reusable rule calculations from command orchestration | Must avoid duplicated checks and artificial intermediate models | Recommended with ordinary calls and borrowed inputs |
| Separate published packages | Independent reuse and release schedules | Version coordination and more build/release work; no inherent command-speed advantage | Revisit only if independent consumers need them |
| Independent rule services | Independent deployment and operational scaling | Serialization, network or IPC latency, distributed consistency, and much greater operational complexity | No justification for splitting one match's rules |

## Performance approach

- The current objective is a complete, usable base-game engine. Define a
  reference runtime and representative hardware before setting numerical
  latency or memory budgets. Do not claim measured performance before there
  is an implementation and a reproducible benchmark.
- Benchmark complete command execution, targeted legal-action queries,
  player-view generation, snapshots, and memory per active match. Include
  realistic mid/late-game fixtures and difficult Longest Road graphs.
- Keep immutable board topology separate from the changing position and pass
  it by reference. Use dense board identifiers, indexed adjacency, and a fixed
  five-resource count vector. Do not allocate individual resource-card objects.
- Borrow state for validation and calculation. Produce the next compact
  position as a value and commit it only on success. Do not introduce a
  persistent-data-structure dependency or deeply clone an object tree. Compare
  the cost of position copies against measured alternatives before changing
  this correctness-friendly transition design.
- Recalculate Longest Road when roads or blocking settlements change, rather
  than during unrelated trades or dice rolls.
- Keep replay history separate from the current state so a longer match does
  not force every transition to copy all previous actions.
- Query placement candidates and validate specific trade proposals. Do not
  enumerate every possible trade and development-card choice for each update.
- Measure optimized release builds as well as the public API boundary cost.
  Any browser binding must be measured separately from native rule execution.
- Introduce additional caches, workers, or specialized representations only
  when measurements justify their complexity. Validation,
  deterministic behavior, and hidden-information protection remain required.

The four-module design avoids imposed runtime communication overhead. It is a
maintainability recommendation, not a claim about measured execution speed.

## Official sources for the specification

- 2025 English sixth-edition rulebook (original source URL privately preserved), linked from the publisher's base-game page (original source URL privately preserved).
- Official base-game FAQ (original source URL privately preserved).

Sources checked on 9 November 2026. The detailed rule specification and
rule-to-test checklist follow map approval.

## Technical sources for the architecture discussion

- [Rust ownership and memory management](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html).
- [Rust modules, scope, and visibility](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html).
- [Rust release and benchmark profiles](https://doc.rust-lang.org/cargo/reference/profiles.html).
- [JavaScript bindings through wasm-bindgen](https://wasm-bindgen.github.io/wasm-bindgen/).
- [Rust WebAssembly target support and host limitations](https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html).

## Specification index

`SPEC.md` indexes the complete engine specification and the approved
constraints in `CONSTRAINTS.md`. Provider contracts and acceptance criteria live in
`SPEC-game-model.md`, `SPEC-base-rules.md`, `SPEC-turn-engine.md`, or
`SPEC-engine-api.md`, written in dependency order. The specifications
cover the objective, executable project commands, structure, code style,
testing strategy, boundaries, and a rule-to-test checklist.

The specification and subsequent implementation plan were approved on
9 November 2026; autonomous implementation was authorized afterward.
See [tasks/todo.md](tasks/todo.md) for verified status.
