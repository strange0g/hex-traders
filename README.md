# Hex Traders

[![CI](https://github.com/strange0g/hex-traders/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/strange0g/hex-traders/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](LICENSE-MIT)

Native Rust library for the 2025 English sixth-edition base-game rules baseline, with three or four seats, fixed and variable setup, all development cards, saving and deterministic replay. The host supplies identity, private randomness, storage and transport. The engine owns game rules, legal choices and state transitions.

Use the versioned Git source in your host's `Cargo.toml`:

```toml
[dependencies]
hex-traders-engine = { git = "https://github.com/strange0g/hex-traders", tag = "v0.1.0", version = "0.1.0" }
```

For local integration, add the checkout as a path dependency:

```toml
[dependencies]
hex-traders-engine = { path = "/path/to/Hex-Traders" }
```

The checked toolchain is Rust 1.98.1, edition 2024. This fixture demonstrates a fixed setup and one legal decision. The all-zero seed and simulated authentication are for this example only. In production generate an unpredictable private seed and derive actor and viewer from the authenticated host session; never accept the client's claimed seat as proof of identity.

```rust
use hex_traders_engine::{Command, CommandEnvelope, Engine, GameConfig, LegalAction, ReplayVerifier, Viewer};
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    // Test fixture only: a real host must generate and keep an unpredictable seed private.
    let (mut game, creation) = Engine::create(GameConfig::fixed(&[0, 1, 2])?, [0; 32])?;
    let public = game.view(Viewer::Spectator)?;
    // Simulate the host authenticating the seat whose turn it is.
    let actor = public.active_player;
    let viewer = Viewer::Seat(actor);
    let _creation_response = game.project_creation_receipt(&creation, viewer)?.to_json()?;
    let view = game.view(viewer)?;
    let legal = game.legal_actions(actor)?;
    if view.revision != legal.revision || !legal.choices.contains(&LegalAction::RollDice) {
        return Err(io::Error::other("refresh: roll is no longer available").into());
    }
    let command = Command::RollDice;
    game.validate(actor, &command)?; // No reservation; execute checks again.
    let replay_header = game.export_replay_header()?; // Private; save before the first record.
    let receipt = game.execute(CommandEnvelope {
        api_version: 1,
        actor,
        expected_revision: view.revision,
        command,
    })?;
    let _safe_response = game.project_receipt(&receipt, viewer)?.to_json()?;
    let replay_record = receipt.to_replay_record()?; // Host-private.
    let snapshot = game.export_snapshot()?; // Host-private.
    let resumed = Engine::from_snapshot(&snapshot)?;
    assert_eq!(resumed.export_snapshot()?, snapshot);
    let mut verifier = ReplayVerifier::from_header(&replay_header)?;
    assert_eq!(verifier.push(&replay_record)?, receipt);
    assert_eq!(verifier.into_engine().export_snapshot()?, snapshot);
    Ok(())
}
```

The supplied seats are the complete permutation `0..player_count` in the host's chosen order. For fixed setup that order also maps to preset roles; starting player comes from engine rolls and rotates the order. `GameConfig::variable` starts with required placements instead of the example's pre-roll state. Ask `view` for the public phase and `legal_actions` for each authenticated seat who may need to act; use `validate` for a complete chosen trade/discard/Invention bundle. The engine supplies bounded candidates and does not enumerate every resource bundle. Handle mandatory setup, discard, robber and free-road choices until the phase changes; never infer or reimplement rules. Seat-scoped legal choices may contain private card IDs and must go only to their owner.

Serialize access to a game at the host. Queries do not reserve an action: `execute` checks the expected revision and legality again, commits exactly once on success, and returns `Rejection` without changing state or RNG on failure. On a stale revision refresh the view and choice. After a lost response, do not change the old envelope's revision and blindly retry: the host must own request deduplication when recovering receipts. Native errors are typed; `BoundaryError` has a stable code, current revision and optional public field, and `ReplayError` exposes the first failing record index and code. The example propagates these errors with `?`; a real host should handle each at its boundary.

Select each `Viewer` only after host authorization. A seat view contains that seat's own hand, card identities and true score; spectator views omit them. Project the original creation or command receipt separately for each audience. Raw `CreationReceipt` and `Receipt` values, replay headers and records, and snapshots are host-private: they contain private facts, and snapshots/replay headers include the RNG seed and position. Never send them to players or put them in logs. `to_json` on projected views, choices, receipts and errors encodes the corresponding allowlist. The host is responsible for private, atomic snapshot storage and access controls; failed `from_snapshot` creates no replacement engine. Import validates current facts and invariants, not historical honesty. Replay verifies accepted records in order from a host-private starting snapshot; rejected commands are not records, and the host controls stream length and retention.

For JSON integration, `decode_command_envelope` and `Engine::execute_json` accept strict version-1 envelopes; the host must authenticate or replace `actor` before execution; do not pass an unverified client envelope directly to `execute_json`. Inputs reject unknown/duplicate fields, wrong shapes, malformed IDs and noncanonical counters; inputs, snapshots and individual replay units are limited to 64 KiB and container depth 16. Revision, turn and offer counters are canonical decimal strings, with checked ranges; format IDs are API 1, snapshot schema 1 and replay 1, rules `hex-traders-base-v1`, RNG `chacha12-rand010-v1`. Starter tie rerolls can make a native/projected creation receipt grow, so do not assume all output is bounded by the input limit. See the repository [engine API specification](https://github.com/strange0g/hex-traders/blob/v0.1.0/SPEC-engine-api.md) for the complete wire contract and error semantics.

## Verification and scope

Build and test the native library from the checkout:

```bash
cargo check --locked --all-targets
cargo test --locked --lib --tests
cargo test --locked --doc
cargo build --locked --release
```

The following evidence links refer to the complete Git repository; task history,
specifications and performance archives are omitted from the Cargo package.
Fixture provenance references also refer to the complete repository. Run the standing local commands in [the plan](tasks/plan.md) and [quality-check evidence](tasks/quality-checks.md); [C11 checkpoint](tasks/c11-checkpoint.md) records the final results. [Rule assertions](tasks/rules-conformance.md), [source setup fixture](tests/support/setup-source.md), [property evidence](tasks/property-evidence.md), [C10 checkpoint](tasks/c10-checkpoint.md), [performance baseline](PERF.md), and [T32 review](tasks/t32-review.md) describe the evidence and its limits. [Specifications](SPEC.md), [capability map](CAPABILITY-MAP.md), and [constraints](CONSTRAINTS.md) define the approved contract. Performance is measured on the recorded native reference machine; the 5% regression threshold is a manual review policy, and absolute budgets are not approved. This source release provides the native library. Browser/Wasm bindings, a user
interface, gameplay hosting and package-registry publication are outside its scope.
See [contribution guidance](CONTRIBUTING.md), [security reporting](SECURITY.md) and
[release notes](docs/releases/v0.1.0.md). Original code and documentation use
[MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE); third-party material retains its
own licensing and attribution.
