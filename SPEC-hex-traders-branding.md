# Spec: hex-traders-branding

Status: accepted for planning by the user's `$plan` invocation on 13 November 2026.
This module belongs to the approved
[release capability map](CAPABILITY-MAP-hex-traders-release.md). Depends on the
existing engine contracts; provides the verified release tree to
`github-publication`. No implementation is claimed by this document.

## Objective

Make Hex Traders the complete identity of the native Rust engine for developers
embedding it in a host. The final active checkout and publication must contain no
mentions of the former project name in filenames, contents, documentation,
compiled public examples, release assets or reachable Git history. A title-only
rename does not meet the requirement.

Keep the four approved engine modules, BR001–BR038, RQ1/RQ2, native public types,
command behavior, hidden-information policy and deterministic results intact.
The user approved a fresh initial release without legacy saved-game migration.

## Tech Stack and provider contract

Rust 1.98.1, edition 2024, and the currently locked dependency graph remain in use.
No new runtime dependencies. Any repository scanner uses Python's standard
library. Existing Rust tests, proptest and Criterion remain the test tools.

| Surface | Required identity |
|---|---|
| Product and document titles | `Hex Traders` |
| Local checkout folder | `Hex-Traders` |
| GitHub repository | `strange0g/hex-traders` |
| Cargo package | `hex-traders-engine` |
| Rust import | `hex_traders_engine` |
| Package version / initial tag | `0.1.0` / `v0.1.0` |
| Native `rules_version` and wire `rulesVersion` | `hex-traders-base-v1` |
| Benchmark switches | `HEX_TRADERS_MEMORY`, `HEX_TRADERS_MEMORY_COUNT` |

API, snapshot-schema and replay-format versions remain `1`; random algorithm
remains `chacha12-rand010-v1`. Only the rules identifier changes. Apply it to
creation receipts, command receipts, views, snapshots, replay headers and records,
their validators, contract examples and literal fixtures. Imports reject unknown
identifiers using existing typed errors and preserve state on failure. No legacy
aliases, input normalization or save-conversion utility are required.

The provider hands publication a clean Git repository with fresh history, full
engine source, complete tests and fixtures, updated specifications and constraints,
portable documentation and new verification evidence. All reachable branches and
tags belong to that fresh history; no original refs, alternates or Git object
archives are transferred. The package exports the same deliberate public facade.

## Rename and preservation scope

- Reach package manifests/lockfile, every caller and export example, source and
  rustdoc, integration tests, JSON fixtures, benchmarks, environment variables,
  scripts, guides, specifications, constraints, task records and retained public
  evidence. Source-only source scans must include hidden tracked configuration.
- Preserve original Git history and byte-for-byte checkpoint, oracle, performance
  and specialist evidence in private storage before excluding them from the new
  checkout. Verify the private copy and its hashes. Do not commit that backup or
  its original machine paths to the public repository.
- Replace public references to private historical files with accurate neutral
  provenance and fresh release evidence. Preserve original completed-task evidence
  privately, with one verified commit for each new implementation task. Do not
  relabel old logs as newly executed checks or fabricate historical results.
- Preserve behavioral fixture facts: seeds, graph IDs, resources, cards, moves,
  expected events and independent oracle answers. Only branding metadata and the
  declared rules identifier may change. Retain source-traced rule assertions.
- Public rules documentation cites the approved 2025 English sixth-edition
  baseline, document `CN3081`, revision `v6.250401`, source pages and BR IDs, using
  paraphrased requirements. Original source URLs stay in private provenance.
  Do not imply publisher endorsement or include publisher artwork/book text.
- Retain third-party licenses and attribution for shipped agent tooling or other
  materials. Exclude private original evidence rather than disguising it through
  encoding or indiscriminate replacement in archives or notices.
- Prepare and verify the destination checkout before retiring the old active
  location. Update project/workspace paths and references to local skills. Required
  filesystem access must be obtained through the normal tool approval mechanism;
  the destination must remain recoverable if moving or attachment fails.

## Commands

Run from the prepared release checkout. Scanner commands below describe a small
tool to be implemented; it does not exist at specification time.

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
python3 scripts/check-branding.py --self-test
python3 scripts/check-branding.py
python3 scripts/check-branding.py --history
cargo bench --locked --bench engine -- --test
cargo bench --locked --bench engine -- --save-baseline approved
cargo bench --locked --bench engine -- --baseline approved
```

The baseline commands respectively record and compare measurements; use matching
reference hardware/fixtures and retain variability. The unchanged 5% performance
margin remains a manual review rule. Hosted timing is not a substitute.

## Project Structure

Keep `src/{game_model,base_rules,turn_engine,engine_api}/`, `tests/`, `benches/`
and the existing public facade. Extend the repository only where needed:

```text
Cargo.toml / Cargo.lock / rust-toolchain.toml
README.md / SPEC.md / SPEC-*.md / CAPABILITY-MAP*.md / CONSTRAINTS.md
scripts/check-branding.py       zero-mention gate and disposable self-tests
tasks/plan.md / tasks/todo.md   preserved status plus new release tasks
tasks/evidence/                fresh release evidence safe to publish
performance/                   fresh measurements with hardware limitations
.agents/ / .codex/             preserved applicable tooling and notices
```

Private preservation lives outside the public checkout. Do not copy build trees,
credentials, real private games or original repository metadata into the release.

## Code Style

Follow the engine's Rustfmt, checked newtypes, explicit `Result` errors and private
state. The following example uses existing public contracts and a fixture seed:

```rust
use hex_traders_engine::{Engine, GameConfig, Viewer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (game, creation) = Engine::create(GameConfig::fixed(&[0, 1, 2])?, [0; 32])?;
    assert_eq!(creation.rules_version, "hex-traders-base-v1");
    let _public_view = game.view(Viewer::Spectator)?;
    Ok(())
}
```

Real hosts supply unpredictable private seeds and authenticate actors/viewers.
Update naming consistently; avoid unrelated refactoring or speculative wrappers.
Use a single small scan tool for local and hosted checks, with clear nonzero exits
for a detected mention or unreadable/malformed input. Detector control data may
construct the forbidden name without embedding a forbidden literal in source.

## Testing Strategy

Run every existing unit, integration and rustdoc test and the unchanged 90% line
coverage gate. Add focused assertions for the new identifier across all outputs,
imports and replay paths; mismatch rejection must remain atomic. Fixture audits
compare behavioral values with the privately preserved baseline rather than
regenerating expected results from production code. Compile a consumer using the
renamed dependency and published README examples.

The branding gate checks tracked paths and contents, supported archive member
names and decompressed contents, and, with `--history`, all reachable Git object
contents. Inspect other generated/release outputs before including them. Scanning
must not extract unsafe paths, execute archived scripts or silently skip malformed
archives. Self-tests use disposable controls for a clean tree, a mixed-case
mention, an archive-contained mention, a historical-only mention and a tool/read
error. Every negative control must cause the intended failure. Build/cache trees
are excluded from source scanning; distributable generated artifacts are not.

Archive retention/hash validation and private baseline preservation are separate
from the branding scanner. Fresh evidence records actual commands, revisions,
exit codes and limitations. Native benchmark smoke and matching-machine review
protect performance without changing the approved policy or claiming equivalence
from package renaming alone.

## Boundaries

- **Always:** read provider specs and `CONSTRAINTS.md`; preserve rules, assertions,
  privacy and RNG behavior; back up original evidence before excluding it; reach
  every affected caller; verify the destination and private copy; retain notices;
  commit each verified implementation task and report real results.
- **Ask first:** change approved gameplay, public semantics beyond the declared
  identity, dependencies, architecture, thresholds, publication destination or
  license; discard the private originals; add a host, UI or deployment.
- **Never:** weaken a check to eliminate a branding failure; delete rule coverage;
  publish original histories/private data; rewrite historical proof; claim the
  rename is complete while archives or reachable Git history still contain it.

## Success Criteria

1. Every identity surface above has the required value; executable consumers,
   rustdoc, fixtures and benchmark switches work with the renamed package.
2. The active checkout folder is `Hex-Traders`; its source/archive/history scans
   pass, and the private preservation is verified and excluded from publication.
3. BR001–BR038, approved card policies, deterministic sequences, privacy,
   atomicity and snapshot/replay behavior pass unchanged except identity metadata.
4. Every applicable build, lint, test, coverage and advisory gate passes with no
   new suppressions, exceptions or skipped tests. Performance review follows the
   existing contract and records the new evidence honestly.
5. `github-publication` receives a clean, complete tree and reproducible fresh
   evidence; original provenance remains privately recoverable.

## Open Questions

No unresolved scope choices. The user advanced this specification to planning.
Workspace access
and path updates are implementation prerequisites; do not claim relocation before
the destination is usable.
