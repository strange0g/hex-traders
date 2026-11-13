# T37: Verified native identity rename

Input revision: `fa05bba`. Approved atomic scope: 44 inventoried files below.
Package/import are `hex-traders-engine` / `hex_traders_engine`; rules identity is
`hex-traders-base-v1`. API/schema/replay versions and RNG algorithm are unchanged.
No legacy alias or migration was added. Existing viewer allowlists remain intact.

RED: the coordinator observed the focused identity test fail with exit 101 before
replacements at the first creation identifier assertion. The root tool transcript
contains its stdout; no separate raw RED stdout file was saved. The corrected
focused native/snapshot/replay test passes after the rename, including unknown
identifier rejection and unchanged replay state on failure.

All ten fresh required checks below exit 0: 200 native tests, 11 rustdoc tests,
98.72% library line coverage (5,955/6,032), fresh advisories and benchmark smoke.
All eight JSON fixture files retain their exact game facts after identity/source
metadata normalization; all 111 locked packages retain their dependency graph
except root package naming. All 21 lower-provider source files are byte-identical.
No rule assertion, quality requirement, dependency, privacy or RNG behavior changed.

Independent Terra high review approves the final 44-file diff with no open
Critical/Required findings. Reviewed source diff SHA-256:
`a8ec4100a1be8750468f6f8338b62cdeca8cc1d872a3c5da9d1667706333c149`.
Review SHA-256: `5db0869c3a06841ed2bae051dd49c6e3b961f3117425c92fb63928d62097cff3`.
Provenance comments were narrowed where standalone historical scratch oracles
were not inventoried. Original excluded memory scripts remain unchanged; T40
removes public stale references and T51 supplies fresh memory reproduction.

## Fresh commands

- `cargo check --locked --all-targets`: exit 0, 1.95 seconds.
- `cargo build --locked --release`: exit 0, 2.84 seconds.
- `cargo fmt --all -- --check`: exit 0, 0.28 seconds.
- `cargo clippy --locked --all-targets -- -D warnings`: exit 0, 2.11 seconds.
- `cargo test --locked --lib --tests`: exit 0, 40.24 seconds.
- `cargo test --locked --doc`: exit 0, 0.42 seconds.
- `cargo doc --locked --no-deps`: exit 0, 0.9 seconds.
- `cargo llvm-cov --locked --lib --tests --ignore-filename-regex (^|/)(tests|benches)/ --fail-under-lines 90`: exit 0, 45.13 seconds.
- `cargo deny --locked check advisories`: exit 0, 1.15 seconds.
- `cargo bench --locked --bench engine -- --test`: exit 0, 4.81 seconds.
- `git diff --check`: exit 0.

## Inventoried paths

- `Cargo.lock`
- `Cargo.toml`
- `README.md`
- `benches/engine.rs`
- `benches/fixtures.json`
- `src/engine_api/command.rs`
- `src/engine_api/command_tests.rs`
- `src/engine_api/mod.rs`
- `src/engine_api/receipt.rs`
- `src/engine_api/receipt_tests.rs`
- `src/engine_api/replay.rs`
- `src/engine_api/settlement_tests.rs`
- `src/engine_api/snapshot.rs`
- `src/engine_api/snapshot_decode.rs`
- `src/engine_api/view.rs`
- `tests/api_contract.rs`
- `tests/game_sequences.rs`
- `tests/game_sequences/bank_trade.rs`
- `tests/game_sequences/city.rs`
- `tests/game_sequences/development.rs`
- `tests/game_sequences/free_road.rs`
- `tests/game_sequences/full_games.rs`
- `tests/game_sequences/invention.rs`
- `tests/game_sequences/knight.rs`
- `tests/game_sequences/monopoly.rs`
- `tests/game_sequences/road.rs`
- `tests/game_sequences/settlement.rs`
- `tests/game_sequences/trade_accept.rs`
- `tests/game_sequences/trade_offer.rs`
- `tests/game_sequences/verified_game.rs`
- `tests/game_sequences/victory.rs`
- `tests/model_invariants.rs`
- `tests/privacy.rs`
- `tests/property_sequences.rs`
- `tests/property_sequences/commands.rs`
- `tests/property_sequences/malformed.rs`
- `tests/replay.rs`
- `tests/rules_conformance.rs`
- `tests/snapshot_import.rs`
- `tests/snapshot_roundtrips.rs`
- `tests/snapshots.rs`
- `tests/support/replay-v1.json`
- `tests/support/setup.json`
- `tests/support/snapshot-v1.json`
