# T02 local quality checks

Historical public derivative. The original document is privately preserved
under SHA-256 ID
`6cdcbb6a7a471091748b08c91dbe730b67a5960f4b0dfd0eaa2180d576819243`.
T35 verified inventoried original documents, archived members and recorded Git
blob copies byte-for-byte. Results below belong to the original T02 and
pre-T05 checks; they have not been rerun for this documentation change.
The [quality contract](../CONSTRAINTS.md) and [task status](todo.md) describe
current enforcement and later verification.

Rust/Cargo 1.98.1 supplies rustfmt, Clippy and `llvm-tools-preview`, pinned in
`rust-toolchain.toml`. Install the separately pinned tools with:

```sh
cargo install cargo-llvm-cov --version 0.9.1 --locked
cargo install cargo-deny --version 0.20.2 --locked
cargo llvm-cov --version
cargo deny --version
```

Official release documentation: [cargo-llvm-cov 0.9.1](https://github.com/taiki-e/cargo-llvm-cov/blob/v0.9.1/README.md),
[cargo-deny 0.20.2 advisories](https://github.com/EmbarkStudios/cargo-deny/blob/0.20.2/docs/src/checks/advisories/cfg.md).
The latter removed `vulnerability = "deny"`: vulnerability advisories are
errors by default. `deny.toml` names the canonical RustSec database and ignores
nothing. The ordinary advisory command fetches it; fetch errors must fail.

## Historical results, 9 November 2026

- `cargo check --locked --all-targets`, `cargo build --locked --release`,
  `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets -- -D warnings`,
  `cargo test --locked --lib --tests` and `cargo test --locked --doc` passed.
  The library and doc suites both contained zero tests.
- `cargo deny --locked check advisories` passed with RustSec database commit
  `d9cb059f3217db87aeccd190b81eaffe8cc29cb9` (2026-11-09 17:13:08 +01:00).
- The actual `cargo llvm-cov --locked --lib --tests --ignore-filename-regex
  '(^|/)(tests|benches)/' --fail-under-lines 90` failed (exit 1): no coverage
  data was found because the bootstrap has no executable library lines. This is
  not a coverage pass. Repeat on T03 and require at least 90% at C01.
- Separate disposable controls: clean compile, formatting and Clippy passed;
  type mismatch exited 101, `clippy::needless_bool` exited 101, and a formatting
  difference exited 1. Missing `rustc` failed check and Clippy; an unknown
  rustfmt option failed formatting. A clean coverage fixture passed at 100%;
  adding an untested function yielded 46.15% and exit 1, with its test passing.
  Missing `llvm-cov` failed report generation. `time = "=0.1.44"` failed with
  `RUSTSEC-2020-0071`; invalid advisory configuration failed; a copied existing
  database with an unreachable origin failed its fetch (exit 1). No control
  entered production source or skipped a test.

The filename filter excludes test and benchmark files, but inline unit-test
functions in `src/` are counted as covered lines by this tool. Interpret the
percentage with that limitation and retain meaningful rule assertions.

## Reproduce disposable controls

The missing-tool labels below are portable placeholders for nonexistent files
in the disposable working directory. Verify that they do not exist before
running the controls; the historical failed outcomes remain recorded above.

Use a temporary directory, not the production crate. Create a library with the
same toolchain, empty dependencies and a lockfile; put the following in its
`src/lib.rs` and run the four commands shown. The coverage command's clean
fixture must report 100% and exit 0.

```rust
pub fn increment(value: u32) -> u32 {
    value + 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn increment_works() {
        assert_eq!(super::increment(1), 2);
    }
}
```

```sh
cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo fmt --all -- --check
cargo llvm-cov --locked --lib --tests --ignore-filename-regex '(^|/)(tests|benches)/' --fail-under-lines 90
```

Replace the fixture source separately with `pub fn bad() -> u32 { "wrong" }`
for check, `pub fn bad(value: bool) -> bool { if value { true } else { false } }`
for Clippy, and `pub fn bad( )->u32{42}` for rustfmt; each respective command
must fail with the expected type, `needless_bool`, or formatting diagnostic.
Restore the clean source between controls. Append the following untested function
for the coverage failure, then restore the clean source:

```rust
pub fn untested(value: u32) -> u32 {
    let first = value + 1;
    let second = first + 1;
    let third = second + 1;
    let fourth = third + 1;
    fourth + 1
}
```

With clean source, `RUSTC=./no-such-rustc cargo check --locked --all-targets`
and the same prefix on the Clippy command must fail. `cargo fmt --all --
--check --no-such-option` must fail. Prefix the coverage command with
`LLVM_COV=./no-such-llvm-cov`; report generation must fail. Confirm each
nonzero exit and diagnostic, rather than treating a broken checker as a pass.

For the advisory violation, create another temporary library, copy this repo's
`deny.toml` and `rust-toolchain.toml`, add `time = "=0.1.44"` under
`[dependencies]`, run `cargo generate-lockfile`, then run `cargo deny --locked
check advisories`. It must fail with `RUSTSEC-2020-0071`.

For a refresh-error control, copy the fetched RustSec database directory from
`$CARGO_HOME/advisory-dbs` (or `~/.cargo/advisory-dbs`) into a temporary database
root. Change only the copy's Git origin to `https://does-not-exist.invalid/advisory-db`.
In a temporary deny config set the canonical `db-urls`, `db-path` to that root,
and `git-fetch-with-cli = true`. Run `cargo deny --locked --config <temporary-config>
check advisories` in this project: it must fail fetching latest changes even
though a prior database copy exists. Invalid config must also fail. Never disable
fetching or reuse that modified copy for the normal project scan.

## Historical RNG feature correction before T05

The pinned rand 0.10.3 manifest now enables `unbiased` with default features
disabled. Its feature definition is empty, so it introduces no dependency or
entropy source. [The integer sampler source](https://docs.rs/rand/0.10.3/src/rand/distr/uniform_int.rs.html)
documents the optional single-value bias, and [the shuffle source](https://docs.rs/rand/0.10.3/src/rand/seq/increasing_uniform.rs.html)
uses that sampler. Enabling the feature fulfills the approved exact-uniform
requirement before creation goldens freeze; dice/index draws additionally use
constructed `Uniform` distributions.

An independent disposable copy of committed graph/fixture code plus the changed
manifest passed check, release build, fmt, Clippy, all 11 tests, doc tests/docs,
and 95.77% library coverage. The lockfile and dependency entries were unchanged;
the normal feature graph includes `unbiased` and no `getrandom`. In-progress T05
code was excluded from that control. A separate pinned-library oracle regenerated
four manually traced starter tie scenarios with this feature; their literal
sequences were unchanged. T05 must still test actual creation against them.
