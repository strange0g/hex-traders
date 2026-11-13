# Contributing to Hex Traders

Keep changes focused on the native library and follow [SPEC.md](SPEC.md), the
provider specifications, [capability map](CAPABILITY-MAP.md) and
[CONSTRAINTS.md](CONSTRAINTS.md). Include a synthetic reproduction and traceable
tests for changed behavior. Preserve private information, rejection atomicity,
exact rules and deterministic replay. Never share real private seeds, snapshots,
replay data or credentials in issues, PRs or logs.

Use Rust 1.98.1 from rust-toolchain.toml, cargo-llvm-cov 0.9.1 and cargo-deny
0.20.2. Install the external tools with locked dependencies:

```bash
cargo install --locked --version 0.9.1 cargo-llvm-cov
cargo install --locked --version 0.20.2 cargo-deny
```

Run affected checks while editing and these gates before handing off a change:

```bash
cargo check --locked --all-targets
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --lib --tests
cargo test --locked --doc
cargo build --locked --release
cargo doc --locked --no-deps
cargo llvm-cov --locked --lib --tests --ignore-filename-regex '(^|/)(tests|benches)/' --fail-under-lines 90
cargo deny --locked fetch db
cargo deny --locked check advisories
cargo bench --locked --bench engine -- --test
python3 scripts/check-branding.py --self-test
python3 scripts/check-branding.py
python3 scripts/check-branding.py --history
cargo package --locked --list
git diff --check
```

Compiler, test, scanner and advisory-tool failures block the change. Keep the 90%
library line floor and all existing assertions; do not suppress failures to pass
CI. Architecture and the 5% repeatable native performance regression margin remain
manual review gates with matching-machine evidence; hosted benchmark smoke does
not establish a performance baseline. See [PERF.md](PERF.md).

Use a PR with the problem, resulting behavior and actual verification. Significant
changes need independent review evidence. Changes to rules, privacy, public
compatibility, quality requirements, RNG/sampler versions or replay-sensitive
behavior require an explicit maintainer compatibility decision before merging.
Review dependency and toolchain updates individually; they are not merged
unattended. Once repository protection is configured, main requires passing checks and resolved conversations;
zero mandatory human approvals accommodates the sole maintainer.

Report vulnerabilities through [SECURITY.md](SECURITY.md).
