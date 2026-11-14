# Spec: github-publication

Status: accepted for planning by the user's `$plan` invocation on 13 November 2026.
Belongs to the approved
[release capability map](CAPABILITY-MAP-hex-traders-release.md) and depends on the
[branding provider](SPEC-hex-traders-branding.md). This document specifies
publication. The public repository and enforced checks are verified in
[T50](tasks/evidence/release/t50-hosted.md). The initial `v0.1.0` assessment and
publication are verified in [T52–T53](tasks/evidence/release/t53-publication.md)
under the release-only R01 exception; T51 performance acceptance remains open.

## Objective

Publish the fully renamed engine as public `strange0g/hex-traders`, with useful
consumer documentation, enforced quality checks and a verified initial source
release. Developers must be able to clone it, build it and use
`hex-traders-engine` without implementing their own game rules.

The user selected public visibility and dual MIT OR Apache-2.0 licensing. Version
`0.1.0` and tag `v0.1.0` form the initial Hex Traders release. Publish the GitHub
repository and release after approval and verification; package-registry
publication, hosted gameplay, deployment and paid features are outside scope.

## Tech Stack and consumer contract

Carry forward Rust 1.98.1/edition 2024, the committed lockfile and native Linux
target. Use GitHub Actions on a fixed Ubuntu runner image, Python's standard
library for the branding gate, cargo-llvm-cov `0.9.1` and cargo-deny `0.20.2`.
Verify chosen upstream action revisions before pinning their full commit SHAs.
GitHub CLI or its authenticated API performs repository management; no new engine
dependencies are introduced.

The provider must first supply a verified zero-mention release tree and fresh Git
history. The public repository contains all source, tests, fixture facts, required
licenses, specs and safe release evidence; private preservation is never pushed.
Cargo metadata includes description, repository URL, README, SPDX expression
`MIT OR Apache-2.0`, suitable keywords/categories and an accurate package file
selection. No unavailable hosted documentation or registry badge is advertised.

## Repository and maintenance requirements

- Use `main` as the default branch and add an accurate description/topics. Keep
  Issues enabled and the Wiki disabled; avoid unused discussion/project surfaces.
- Ship complete `LICENSE-MIT` and `LICENSE-APACHE` texts for original project
  code. Retain applicable third-party licenses and source attribution. Do not
  grant rights to publisher assets, copied rulebook text or third-party tooling
  under the project's dual license.
- README includes a concise description, a working Git/path dependency example,
  create → query → execute → safe projection → save/replay guide, build/check
  commands and host authentication, seed, storage and privacy responsibilities.
  Link successful CI and licensing with accurate badges after hosted verification.
- Provide `CONTRIBUTING.md`, `SECURITY.md`, curated `CHANGELOG.md`, a focused PR
  template and bug/feature issue forms. Use GitHub private vulnerability reporting
  as the security contact when enabled; do not invent an email or response SLA.
  Templates must not invite real private snapshots, seeds or credentials.
- Configure weekly Dependabot PRs for Cargo and GitHub Actions, bounded to five
  open PRs per ecosystem. Updates are reviewed; there is no unattended merge or
  automatic RNG/toolchain drift. Replay-sensitive changes follow existing
  compatibility approval requirements.
- Enable available dependency alerts/security updates, secret scanning/push
  protection and private vulnerability reporting. Read settings back through the
  API and document any feature the account cannot enforce. The RustSec scan is
  the required dependency-security gate; do not imply unsupported scanners ran.
- After bootstrap checks succeed, enforce a `main` ruleset requiring PRs,
  conversation resolution, up-to-date passing CI checks, linear history and no
  branch deletion or force pushes. Bind required checks to GitHub Actions where
  supported. Use squash merges and delete merged feature branches.
- A sole maintainer needs zero mandatory human approvals because they cannot
  approve their own PR; independent code/security/test review evidence still
  precedes merging significant changes. Do not add a routine bypass of checks.
  Verify rejected update/merge controls through safe disposable branches or
  equivalent ruleset evaluation, without weakening `main` to run a control.
- Protect published `v*` tags against changes/deletion; allow the maintainer to
  create version tags. Never silently move a release tag to repair a mistake.

These settings implement the verified GitHub
[secure workflow guidance](https://docs.github.com/en/actions/reference/security/secure-use),
[repository rules](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/available-rules-for-rulesets)
and [Dependabot configuration](https://docs.github.com/en/code-security/reference/supply-chain-security/dependabot-options-reference),
checked on 13 November 2026. Read actual account capabilities during publication.

## CI quality contract

Workflow `ci.yml` runs on every PR, pushes to `main`, release-tag pushes and manual
dispatch. A weekly schedule refreshes the same advisory opinion. Do not filter
away documentation-only PRs when required checks would be left missing.

| Required check | Commands / responsibility |
|---|---|
| `quality` | Compiler check, formatting, warnings-denied Clippy, library/integration tests, rustdoc tests, release build, public documentation build, benchmark smoke, branding self-tests, tree and reachable-history scans. |
| `coverage` | Existing library line command with unchanged 90% floor and filename exclusions. |
| `advisories` | Fresh `cargo deny --locked check advisories`; database/tool/network failures fail the job. |

Each job uses the pinned Rust toolchain and locked dependencies. Cache keys
include the runner/toolchain/lockfile; coverage data is clean for the current run.
Bound job duration, cancel superseded PR runs and retain useful logs/coverage
reports without private data. No job uses `continue-on-error` for a gate.

Default token permissions are read-only repository contents. Check out PR code
without persisted credentials and without repository secrets. Fetch the complete
candidate/default/release history required by the branding gate. Use the ordinary
`pull_request` event; do not run untrusted code with privileged triggers. Avoid
shell interpolation of untrusted event text. Repository publication remains a
maintainer CLI operation; PR workflows receive no release-write authority.

Keep the manual architecture/performance gate and 5% native regression policy.
Do not claim hosted runners provide a stable benchmark baseline. No UI/browser
or cross-platform assurance is added to this native Linux release.

## Commands

Run the local commands below from the renamed checkout before publication:

```bash
cargo check --locked --all-targets
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --lib --tests
cargo test --locked --doc
cargo build --locked --release
cargo doc --locked --no-deps
cargo llvm-cov --locked --lib --tests --ignore-filename-regex '(^|/)(tests|benches)/' --fail-under-lines 90
cargo deny --locked check advisories
cargo bench --locked --bench engine -- --test
python3 scripts/check-branding.py --self-test
python3 scripts/check-branding.py
python3 scripts/check-branding.py --history
cargo package --locked --list
gh auth status --hostname github.com
gh api user --jq .login
```

Publishing commands are specified for later execution against the verified fresh
repository, after inspecting whether the destination already exists. Never
overwrite an unrelated repository. The ruleset file is created during
implementation and contains the verified hosted check names and app identity.

```bash
gh repo create strange0g/hex-traders --public --description "Hex Traders: deterministic Rust game engine" --disable-wiki
git remote add origin https://github.com/strange0g/hex-traders.git
git push --set-upstream origin main
gh api --method POST repos/strange0g/hex-traders/rulesets --input .github/rulesets/main.json
git tag -a v0.1.0 -m "Hex Traders v0.1.0"
git push origin refs/tags/v0.1.0
gh release create v0.1.0 --repo strange0g/hex-traders --verify-tag --title "Hex Traders v0.1.0" --notes-file docs/releases/v0.1.0.md
```

The tag must identify the exact reviewed and hosted-green commit, matching the
manifest and changelog version. Release notes are prepared locally first.
`--verify-tag` avoids creating a tag implicitly; see the
[official CLI release command](https://cli.github.com/manual/gh_release_create).
Do not attach compiled artifacts without a concrete consumer requirement.

Read back and record the result:

```bash
gh repo view strange0g/hex-traders --json nameWithOwner,visibility,defaultBranchRef,url
gh api repos/strange0g/hex-traders/rulesets
gh run list --repo strange0g/hex-traders --workflow ci.yml --branch main --limit 5
gh release view v0.1.0 --repo strange0g/hex-traders --json tagName,name,isDraft,url
```

Use the actual run IDs to wait and verify conclusion and head revision. A created
workflow or listed run is not a passing check.

## Project Structure

```text
Cargo.toml / README.md
LICENSE-MIT / LICENSE-APACHE / CONTRIBUTING.md / SECURITY.md / CHANGELOG.md
.github/workflows/ci.yml
.github/dependabot.yml
.github/ISSUE_TEMPLATE/bug.yml / feature.yml
.github/pull_request_template.md
.github/rulesets/main.json     reviewable repository rule configuration
docs/releases/v0.1.0.md        curated initial release notes
tasks/evidence/                fresh publication and settings evidence
```

Keep the engine's existing source/test layout. Add only files implementing these
requirements; private backups, caches and credentials remain outside the release.

## Code Style

YAML uses two-space indentation, explicit job names and plain commands shared
with local verification. This is a partial style example, not a complete workflow:

```yaml
permissions:
  contents: read

jobs:
  quality:
    runs-on: ubuntu-24.04
    timeout-minutes: 15
    steps:
      # Checkout and pinned Rust setup precede this gate.
      - name: Check formatting
        run: cargo fmt --all -- --check
```

Full workflows pin reviewed action SHAs and state tool versions. Public prose
uses Hex Traders consistently, describes observable engine behavior and gives
copyable consumer commands. Avoid generic framework or deployment boilerplate.

## Testing Strategy

Verify YAML/workflow syntax with an available maintained validator or equivalent
inspection, execute the exact gate commands locally, then observe every required
hosted check passing for the release commit. Hosted check controls must demonstrate
that a failing formatting/test/coverage/advisory gate cannot merge; validate a
branding failure and scanner/tool error as well. Keep controls in disposable
checks and remove temporary branches after retaining evidence. Branding controls
construct prohibited payloads only at runtime in temporary repositories, with
neutral diagnostics; never commit the former-name literal into public control
branches, PR bodies or logs.

Review workflow permissions and untrusted input handling independently before
publication. Confirm a PR from an unprivileged context receives no credentials
or write scope. Read back actual rulesets/security settings and verify enforcement
with the intended actor; configured-but-inactive settings do not satisfy the gate.

Clone the remote repository into a new disposable directory and verify that its
`v0.1.0` tag resolves to the assessed commit. Run full relevant checks and the
tree/history scanner there; compile a minimal external consumer with
`hex_traders_engine`. Inspect automatically generated source archives, the Cargo
file list and release notes for private/old material, missing licenses and broken
links. Retain fresh commands, revision IDs and results without credentials.

## Boundaries

- **Always:** publish only the verified provider tree; authenticate the confirmed
  owner; preserve all constraints and notices; verify actual hosted results,
  protections, release revision and clean-clone integration; report unavailable
  security features and material limitations accurately.
- **Ask first:** use another owner/visibility/license, overwrite existing remote
  content, purchase features, publish to a package registry, deploy a host, change
  approved rules/quality requirements or grant broader account access.
- **Never:** publish private history/evidence/seeds; commit authentication tokens;
  disable failing gates; mark unsupported checks as passing; automatically merge
  replay-sensitive updates; force-push shared history or silently replace tags.

The user has explicitly requested this GitHub publication and confirmed its
destination. Routine setup within the approved specification does not require
repeated publication permission. The invoked specification workflow still requires
approval before planning/implementation.

## Success Criteria

1. Public `strange0g/hex-traders` exists with the intended `main` branch, accurate
   metadata, licenses, docs, templates and a clean history free of former mentions.
2. All three required checks pass for the assessed revision. PR/check controls
   verify that failing changes are blocked; enforced repository protections and
   available security settings are read back and documented.
3. Weekly dependency-update PRs and advisory refresh are configured without
   automatic merge or suppression of replay compatibility review.
4. Published `v0.1.0` matches Cargo/changelog versions and the reviewed commit.
   Source archives and package file selection contain only intended safe files.
5. A clean remote clone passes relevant build/test/branding checks and compiles a
   real external consumer. Publication evidence links actual successful runs and
   the release, with all material limitations recorded.

## Open Questions

No unresolved scope choices. The user advanced this specification to planning.
The last CLI
authentication check reported invalid credentials; restore access through a
secure sign-in flow when remote work begins. Never request a token in chat.
