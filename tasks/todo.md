# Task List: Hex Traders Release

<a id="hex-traders-release"></a>
## Current release work

Status: approved for autonomous implementation on 13 November 2026 by `$build auto`,
including the T37 atomic size exception and Terra/Sol subagent execution.
No release task is marked complete until its verification passes. [plan.md](plan.md) owns approach, ordering,
checks and risks. This file is the single task/checkpoint completion source.

The release plan has 19 tasks, T35–T53, and ten checkpoints, C12–C21.
Focused review repairs are recorded alongside their owning tasks. Read the
[root](../SPEC.md), [branding](../SPEC-hex-traders-branding.md),
[publication](../SPEC-github-publication.md) specifications and
[constraints](../CONSTRAINTS.md) before implementation. The completed engine
history follows this section unchanged during planning.

Each task requires its acceptance, relevant standing checks, real evidence and
one verified commit. Original history/evidence is privately preserved before
exclusion from publication. Only a checked fresh export is initialized/pushed as
public history. Checkpoints require reviewable evidence; applicable autonomous
build authorization can permit progressing between them.

T37 is the documented 44-file mechanical size exception for plan review.
Other tasks keep implementation edits to about five files or fewer. Do not lower
the quality bar, add temporary aliases, erase original proof or mark pending
work done to satisfy sizing or timing.

<a id="t35"></a>
### T35: Preserve the recoverable original baseline

- [x] T35 complete

**Module:** hex-traders-branding. **Dependencies:** None.

**Description:** Prepare the private safety copy before excluding historical material. Capture the approved source, all original refs and per-task commits, checkpoint/oracle/performance evidence, notices and planning inputs. Use a preparation branch; publish none of its refs.

**Acceptance criteria:**

- [x] Private preservation restores the assessed original source/history and original evidence byte-for-byte, including uncommitted planning inputs if not already committed.
- [x] A portable inventory records hashes and exclusions, and a disposable restore verifies original archive members and baseline fixture/config facts.
- [x] The original active checkout is preserved; no credentials, real game data or private backup paths enter public output.

**Verification:**

- [x] Run `git bundle verify` on the actual saved bundle and `sha256sum --check` on its retained manifest; restore into a disposable directory, compare source/fixture/config hashes and record the original refs. Inspect the complete backup and export exclusions manually.
- [x] Relevant standing definition of done, scoped diff/provenance audit and actual evidence pass before completion is committed.

**Files likely touched:** Private original Git bundle/archive, hash manifest; `tasks/evidence/release/t35-preservation.md`.

**Estimated scope:** Small; private archive/bundle, hash manifest and preservation record.

**Fresh evidence:** [T35 preservation](evidence/release/t35-preservation.md).

<a id="t36"></a>
### T36: Implement the fail-closed branding gate

- [x] T36 complete

**Module:** hex-traders-branding. **Dependencies:** [T35](#t35).

**Description:** Build the single reusable scanner for tracked paths/content, supported archive names/decompressed contents and reachable Git objects. Its forbidden-pattern representation must not introduce a literal mention in its own published source.

**Acceptance criteria:**

- [x] Ordinary tree and `--history` modes reject prohibited mixed-case content/paths, including tar/gzip and ZIP artifacts, with nonzero detection or read/tool-error exits.
- [x] Disposable self-tests cover clean input, text/path hits, nested archive hits, historical-only hits, malformed/unsafe archives and tool/read failures; controls never execute archived scripts or expose the prohibited literal in public logs.
- [x] Reads and archive traversal are bounded and fail closed; the tool adds no dependency, suppression or production exception.

**Verification:**

- [x] Run `python3 scripts/check-branding.py --self-test`; exercise both modes on clean and deliberately failing disposable repositories. Record expected exits; remaining preparation-tree hits are unresolved scope, not a passing release scan.
- [x] Relevant standing definition of done, scoped diff/provenance audit and actual evidence pass before completion is committed.

**Files likely touched:** `scripts/check-branding.py`; `tasks/evidence/release/t36-scanner.md`.

**Estimated scope:** Small; one stdlib Python tool plus compact evidence.

**Fresh verification record:** [T36](evidence/release/t36-scanner.md).

<a id="c12"></a>
### C12: Private preservation and detector

**Dependencies:** T35–T36.

- [x] Original source/history/evidence restores with verified hashes; scanner clean, negative and tool-error controls pass. Remaining preparation naming findings remain explicitly unresolved.
- [x] Required affected checks and evidence pass; review the diff for weakened constraints, lost assertions, private material and unsupported claims.
- [x] Present the checkpoint evidence for human review or proceed under applicable autonomous build authorization; record actual status before continuing.

**Evidence:** T35/T36 records; independent review complete under autonomous approval. Preparation branding remains unresolved.

<a id="t37"></a>
### T37: Rename the complete native engine identity atomically

- [x] T37 complete

**Module:** hex-traders-branding. **Dependencies:** [T35](#t35), [T36](#t36).

**Description:** Apply the coupled crate/import/rustdoc/rules-identifier and benchmark-switch changes in one working commit. Audit the exact target list before editing, including callers and golden fixtures. Avoid temporary aliases and unrelated rewrites.

**Acceptance criteria:**

- [x] Package/imports, all rules-version outputs/validators, literal contract fixtures and benchmark switches use the approved Hex Traders values; every consumer and rustdoc example compiles.
- [x] API/schema/replay versions, RNG versions, locked non-root dependency graph, seeds, graph IDs, game actions and independent expected answers remain unchanged; unknown identifiers still reject atomically.
- [x] The complete native quality gates and benchmark smoke pass with relevant identifier assertions, and the diff contains only identity/provenance edits and their necessary checks.

**Verification:**

- [x] Run all compiler/build/fmt/Clippy/library/integration/rustdoc/documentation/coverage/advisory commands in the release plan, plus `cargo bench --locked --bench engine -- --test`. Compare resolved dependency and parsed fixture facts against T35; inspect every repeated rules identifier and public caller.
- [x] Relevant standing definition of done, scoped diff/provenance audit and actual evidence pass before completion is committed.

**Files likely touched:** `Cargo.toml`, `Cargo.lock`, `README.md`; the inventoried affected `src/engine_api/`, `tests/`, and `benches/` files (44 total); compact T37 audit record.

**Estimated scope:** Explicit mechanical size exception: 44 coupled files, one focused session.

**Fresh verification record:** [T37](evidence/release/t37-identity.md).

<a id="t38"></a>
### T38: Retain portable fixture provenance

- [x] T38 complete

**Module:** hex-traders-branding. **Dependencies:** [T37](#t37).

**Description:** Replace old source URLs and temporary-path references with neutral edition/document/page/rule IDs and accurate private-original provenance. Keep complete rule-to-assertion traceability and independent expected fixture facts.

**Acceptance criteria:**

- [x] All source fixtures retain the approved edition, document revision/pages, deterministic facts and independent verification origin without former-name URLs or machine paths.
- [x] Every BR001–BR038 and RQ1/RQ2 still maps to direct retained assertions; no rule expectation is weakened or replaced by production-derived answers.
- [x] Public references resolve to portable documents; exact original URLs/scripts/results remain available in T35 private preservation.

**Verification:**

- [x] Compare fixture fact hashes/parsed values with T35 and review the BR assertion map. Check all local Markdown links and run `cargo test --locked --test rules_conformance`, `cargo test --locked --test game_sequences`, and `cargo test --locked --test snapshot_import`.
- [x] Relevant standing definition of done, scoped diff/provenance audit and actual evidence pass before completion is committed.

**Files likely touched:** `tests/support/setup-source.md`, `tests/support/victory-source.md`, `tests/support/full-games-source.md`, `tasks/rules-conformance.md`.

**Estimated scope:** Medium; four provenance/traceability documents.

**Fresh verification record:** [T38](evidence/release/t38-provenance.md).

<a id="c13"></a>
### C13: Native identity and fixture facts

**Dependencies:** T37–T38.

- [x] The renamed create → view → command → snapshot → replay path and all native gates pass; dependency/RNG/game facts and BR traceability match the preserved baseline.
- [x] Required affected checks and evidence pass; review the diff for weakened constraints, lost assertions, private material and unsupported claims.
- [x] Present the checkpoint evidence for human review or proceed under applicable autonomous build authorization; record actual status before continuing.

**Evidence:** T37/T38 records; native identity, fixture facts and full gates verified under autonomous review.

<a id="t39"></a>
### T39: Rename the governing specifications

- [x] T39 complete

**Module:** hex-traders-branding. **Dependencies:** [T37](#t37).

**Description:** Complete the identity and source-reference cleanup in the approved root/provider specifications while preserving normative rules and the existing four-module direction.

**Acceptance criteria:**

- [x] Root/provider documents use Hex Traders and the declared new rules identifier, with neutral original-source IDs and consistent reference links.
- [x] BR001–BR038, RQ1/RQ2, host responsibilities, privacy, schema/replay/RNG semantics and module contracts retain their approved meaning.
- [x] Release status describes verified progress accurately; no source or hosted capability is advertised before its task passes.

**Verification:**

- [x] Review a clause-level diff against T35; check local links and rules-identifier consistency with the implemented crate. Run `git diff --check`; any behavioral contract change requires separate approval before implementation.
- [x] Relevant standing definition of done, scoped diff/provenance audit and actual evidence pass before completion is committed.

**Files likely touched:** `SPEC.md`, `SPEC-base-rules.md`, `SPEC-engine-api.md`, `CAPABILITY-MAP.md`.

**Estimated scope:** Medium; four governing documents.

**Fresh verification record:** [T39](evidence/release/t39-specifications.md).

<a id="t40"></a>
### T40: Preserve the quality and performance contract

- [x] T40 complete

**Module:** hex-traders-branding. **Dependencies:** [T37](#t37).

**Description:** Rename quality/performance references and distinguish privately preserved historical measurements from the future fresh release runs. Keep all numerical thresholds, enforcement limitations and measurement caveats.

**Acceptance criteria:**

- [x] The 90% coverage floor, zero-vulnerability/error/suppression policies and manual 5% native performance margin are unchanged, with no new exception.
- [x] Historical measurement facts are accurately labelled and their exact original evidence remains privately preserved; public references have no old paths/names.
- [x] No fresh, hosted or cross-platform measurement is claimed before it exists; T51 has clear baseline/variability requirements.

**Verification:**

- [x] Compare the policy tables and benchmark conditions with T35 and inspect every threshold/exception clause. Check links and `git diff --check`; do not rerun native benchmarks for this documentation-only task.
- [x] Relevant standing definition of done, scoped diff/provenance audit and actual evidence pass before completion is committed.

**Files likely touched:** `CONSTRAINTS.md`, `PERF.md`, `performance/review-t31.md`.

**Estimated scope:** Medium; three quality/performance documents.

**Fresh verification record:** [T40](evidence/release/t40-quality.md).

<a id="c14"></a>
### C14: Governing contracts

**Dependencies:** T39–T40.

- [x] Renamed governing text preserves all approved rule, privacy, architecture and quality semantics; historical versus fresh performance status is explicit.
- [x] Required affected checks and evidence pass; review the diff for weakened constraints, lost assertions, private material and unsupported claims.
- [x] Present the checkpoint evidence for human review or proceed under applicable autonomous build authorization; record actual status before continuing.

**Evidence:** T39/T40 records; governing rules and complete quality-policy body preserved under autonomous review.

<a id="t41"></a>
### T41: Publish accurate C03–C06 historical summaries

- [x] T41 complete

**Module:** hex-traders-branding. **Dependencies:** [T35](#t35).

**Description:** Produce neutral public derivative summaries of these completed checkpoints without distributing rewritten raw logs or losing original conclusions.

**Acceptance criteria:**

- [x] Each document explicitly identifies its historical summary status, preserves verified results and resolves public rule/task references.
- [x] Former-name/path literals and references to non-exported raw archives are removed from the public derivative; no new test execution is claimed.
- [x] Original reports/logs and their hashes are verified in T35; documented omissions remain explicit.

**Verification:**

- [x] Reconcile each summary with the privately preserved original, verify source IDs/hashes and check local links plus `git diff --check`. Runtime engine checks are unchanged by this task.
- [x] Relevant standing definition of done, scoped diff/provenance audit and actual evidence pass before completion is committed.

**Files likely touched:** `tasks/c03-checkpoint.md`, `tasks/c04-checkpoint.md`, `tasks/c05-checkpoint.md`, `tasks/c06-checkpoint.md`.

**Estimated scope:** Medium; four checkpoint documents.

**Fresh verification record:** [T41](evidence/release/t41-summaries.md).

<a id="t42"></a>
### T42: Publish accurate C07–C10 historical summaries

- [x] T42 complete

**Module:** hex-traders-branding. **Dependencies:** [T35](#t35).

**Description:** Apply the same verified historical-summary policy to the later gameplay, contract and conformance checkpoints, preserving their actual limitations.

**Acceptance criteria:**

- [x] All checkpoint results, rejected-case/privacy conclusions and source-traced coverage remain accurately represented as historical facts.
- [x] Public summaries contain no former-name literals, private paths or broken raw-archive links, and do not imply fresh reruns.
- [x] T35 retains each original and its corresponding passing/failing/control evidence without alteration.

**Verification:**

- [x] Compare summaries against the original checkpoint/oracle records, check evidence identities and all local links, and run `git diff --check`.
- [x] Relevant standing definition of done, scoped diff/provenance audit and actual evidence pass before completion is committed.

**Files likely touched:** `tasks/c07-checkpoint.md`, `tasks/c08-checkpoint.md`, `tasks/c09-checkpoint.md`, `tasks/c10-checkpoint.md`.

**Estimated scope:** Medium; four checkpoint documents.

**Fresh verification record:** [T42](evidence/release/t42-summaries.md).

<a id="c15"></a>
### C15: Checkpoint provenance

**Dependencies:** T41–T42.

- [x] Historical public derivatives reconcile with the private originals; all preserved conclusions and public references are accurate.
- [x] Required affected checks and evidence pass; review the diff for weakened constraints, lost assertions, private material and unsupported claims.
- [x] Present the checkpoint evidence for human review or proceed under applicable autonomous build authorization; record actual status before continuing.

**Evidence:** T41/T42 records above; autonomous review completed 13 November 2026.

<a id="t43"></a>
### T43: Preserve review and handoff provenance

- [x] T43 complete

**Module:** hex-traders-branding. **Dependencies:** [T41](#t41), [T42](#t42).

**Description:** Complete portable historical review, property and handoff summaries. Original specialist reports and raw C11/ship archives remain private; fresh reports will be produced in T52.

**Acceptance criteria:**

- [x] Public summaries retain original scope, conclusions, failure/control/property facts and host/performance limitations without former-name references.
- [x] Original specialist/C11/ship artifacts are marked for private preservation rather than rehashed renamed originals, and public links point to real summaries or forthcoming release evidence honestly.
- [x] No historic GO decision is presented as the new GitHub release assessment; T52 owns the fresh release decision.

**Verification:**

- [x] Reconcile original reports, property facts and C11 counts with T35. Check local references and absence of unsupported fresh claims; run `git diff --check`.
- [x] Relevant standing definition of done, scoped diff/provenance audit and actual evidence pass before completion is committed.

**Files likely touched:** `tasks/t32-review.md`, `tasks/t33-review.md`, `tasks/ship-decision.md`, `tasks/property-evidence.md`, `tasks/c11-checkpoint.md`.

**Estimated scope:** Medium; five historical-summary documents.

**Fresh verification record:** [T43](evidence/release/t43-summaries.md).

<a id="t44"></a>
### T44: Complete portable task and tooling history

- [x] T44 complete

**Module:** hex-traders-branding. **Dependencies:** [T38](#t38), [T39](#t39), [T40](#t40), [T43](#t43).

**Description:** Prepare the public derivative of the completed task history and current release plan. Preserve old completion facts and evidence privately while removing historical machine paths and updating project-local skill references.

**Acceptance criteria:**

- [x] Original completed T01–T34/C01–C11 facts and current T35–T53 status remain accurate; no completed work is silently unchecked or a pending release task marked complete.
- [x] Public plan/task/tooling references are portable and free of old naming, with third-party skill contents/licenses preserved and no private-source claims masquerading as fresh evidence.
- [x] The final export set and excluded original evidence directories are identified, and every public local reference resolves within that intended set.

**Verification:**

- [x] Compare original completion IDs/counts with T35, resolve all plan/task/provider/skill links and inspect original-versus-public provenance. Run `git diff --check`; leave original bytes recoverable privately.
- [x] Relevant standing definition of done, scoped diff/provenance audit and actual evidence pass before completion is committed.

**Files likely touched:** `tasks/plan.md`, `tasks/todo.md`, `AGENT-SKILLS-INSTALLATION.md`; `.agents/SOURCE.json` only if a stale path is found.

**Estimated scope:** Medium; three documents, one optional stale-path metadata file.

**Fresh verification record:** [T44](evidence/release/t44-history.md).

<a id="c16"></a>
### C16: Export readiness

**Dependencies:** T43–T44.

- [x] Historical task/review/property/handoff facts are preserved; the public export set/exclusions and local references are complete, without fabricated fresh proof.
- [x] Required affected checks and evidence pass; review the diff for weakened constraints, lost assertions, private material and unsupported claims.
- [x] Present the checkpoint evidence for human review or proceed under applicable autonomous build authorization; record actual status before continuing.

**Evidence:** T43/T44 records; original history and reviewed export exclusions preserved under autonomous review.

<a id="t45"></a>
### T45: Create the clean relocated release repository

- [x] T45 complete

**Module:** hex-traders-branding. **Dependencies:** [T36](#t36), [T44](#t44).

**Description:** Export the reviewed source/fixture/docs/tooling set to the local Hex-Traders destination without private original archives/indices/reports or Git metadata. Initialize fresh history only after tree/archive checks pass; verify destination usability before retiring the old active location.

**Acceptance criteria:**

- [x] A checksummed relative-path manifest accounts for every included file and intentional private-original exclusion; required fixtures, notices and public links are complete, with safe paths and no credential/private-game data.
- [x] The destination folder is `Hex-Traders`, has fresh independent Git history without preparation refs/objects/alternates, and both tree/archive and reachable-history scans pass.
- [x] The app/project/workspace uses the verified destination or a concrete handoff resolves it; subsequent commands use explicit destination workdirs, and original source/history remains privately recoverable.

**Verification:**

- [x] Compare export bytes against the manifest, run `python3 scripts/check-branding.py` before the initial commit and `python3 scripts/check-branding.py --history` after it. Run the standing native gates and verify local links in the exported tree; inspect all refs, repository configuration and private-preservation exclusions.
- [x] Relevant standing definition of done, scoped diff/provenance audit and actual evidence pass before completion is committed.

**Files likely touched:** `tasks/evidence/README.md`, `tasks/evidence/release/export-manifest.json`, `tasks/evidence/release/t45-export.md`; fresh destination repository/workspace metadata.

**Estimated scope:** Small configuration/evidence footprint; validated copy of the approved tree.


**Fresh verification:** [T45 export](evidence/release/t45-export.md); workspace handoff and app readback pass.

<a id="t46"></a>
### T46: License the consumable package

- [x] T46 complete

**Module:** github-publication. **Dependencies:** [T45](#t45).

**Description:** Provide the selected dual license for original code, accurate repository/package metadata and a curated initial-version changelog without relabelling third-party materials.

**Acceptance criteria:**

- [x] Complete MIT and Apache-2.0 texts and Cargo `MIT OR Apache-2.0` metadata apply correctly to original project code; shipped third-party notices retain their original attribution/licensing.
- [x] Manifest/version/repository/README/keywords/categories/file selection describe Hex Traders 0.1.0 accurately, with no dependency or RNG graph changes.
- [x] The changelog describes the initial consumer capability and declared format identity; Cargo packaging includes required source/docs/licenses and excludes private/tooling/build material appropriately.

**Verification:**

- [x] Run `cargo package --locked --list`, inspect the exact package file selection/notices, and compare the non-root lock graph with T35. Run `cargo check --locked --all-targets`, `cargo test --locked --doc`, both naming gates and `git diff --check`.
- [x] Relevant standing definition of done, scoped diff/provenance audit and actual evidence pass before completion is committed.

**Files likely touched:** `LICENSE-MIT`, `LICENSE-APACHE`, `Cargo.toml`, `CHANGELOG.md`.

**Estimated scope:** Medium; four package/licensing files.

**Fresh verification record:** [T46](evidence/release/t46-package.md).

<a id="c17"></a>
### C17: Clean local release repository

**Dependencies:** T45–T46.

- [x] The relocated fresh repository passes complete tree/archive/reachable-history gates, native checks and package/license inspection; private preservation is recoverable and excluded.
- [x] Required affected checks and evidence pass; review the diff for weakened constraints, lost assertions, private material and unsupported claims.
- [x] Present the checkpoint evidence for human review or proceed under applicable autonomous build authorization; record actual status before continuing.

**Checkpoint:** [T45 export](evidence/release/t45-export.md) and [T46 package](evidence/release/t46-package.md) pass with independent review; the app path readback confirms the fresh checkout. Autonomous authorization permits proceeding.

<a id="t47"></a>
### T47: Deliver the consumer and security guide

- [x] T47 complete

**Module:** github-publication. **Dependencies:** [T46](#t46).

**Description:** Reuse the existing host-integration example and privacy guidance with the renamed package and real repository location. Prepare contribution/security paths and initial release notes before candidate review.

**Acceptance criteria:**

- [x] README examples compile and describe create/query/execute/projection/save/replay plus authenticated actor/viewer, private seed/storage and deduplication duties; Git/path dependency examples use the intended version/repository.
- [x] Contribution/security guidance specifies unchanged quality gates and a real enabled or provisionable private-reporting contact, without invented email/SLA or requests for real private snapshots/seeds.
- [x] Release notes describe actual 0.1.0 behavior and limitations; badges and links are activated only when their hosted resources exist and succeed.

**Verification:**

- [x] Run `cargo test --locked --doc`, `cargo doc --locked --no-deps`, and compile an external path consumer. Check Markdown links and both naming gates; validate the Git dependency and hosted links once T53 provides the tag.
- [x] Relevant standing definition of done, scoped diff/provenance audit and actual evidence pass before completion is committed.

**Files likely touched:** `README.md`, `CONTRIBUTING.md`, `SECURITY.md`, `docs/releases/v0.1.0.md`.

**Estimated scope:** Medium; four documentation files.

**Fresh verification record:** [T47](evidence/release/t47-guide.md).

<a id="t48"></a>
### T48: Enable useful contribution and update requests

- [x] T48 complete

**Module:** github-publication. **Dependencies:** [T46](#t46).

**Description:** Add focused bug/feature and PR intake plus weekly Cargo/GitHub Actions dependency-update PRs, all scoped to the native library.

**Acceptance criteria:**

- [x] Issue/PR forms ask for useful public reproduction and verification information and avoid secrets, real private game data and unsupported hosting/UI assumptions.
- [x] Dependabot covers Cargo and GitHub Actions at the repository root weekly, with five open PRs per ecosystem and reviewed replay-sensitive changes.
- [x] Configuration is valid, all public text uses Hex Traders, and no automatic merge or unrequested workflow subsystem is introduced.

**Verification:**

- [x] Validate YAML/form/Dependabot syntax with an available maintained validator or equivalent review, inspect update policy and local references, run both naming gates and `git diff --check`; verify GitHub recognizes the forms/configuration in T50.
- [x] Relevant standing definition of done, scoped diff/provenance audit and actual evidence pass before completion is committed.

**Files likely touched:** `.github/ISSUE_TEMPLATE/bug.yml`, `.github/ISSUE_TEMPLATE/feature.yml`, `.github/pull_request_template.md`, `.github/dependabot.yml`.

**Estimated scope:** Medium; four maintenance configuration files.

**Fresh verification record:** [T48](evidence/release/t48-intake.md).

<a id="c18"></a>
### C18: Consumer and contributor paths

**Dependencies:** T47–T48.

- [x] External path integration and compiling docs work; contribution/security/issue/PR/update configuration is valid and public text uses the intended identity.
- [x] Required affected checks and evidence pass; review the diff for weakened constraints, lost assertions, private material and unsupported claims.
- [x] Present the checkpoint evidence for human review or proceed under applicable autonomous build authorization; record actual status before continuing.

**Checkpoint:** [T47 guide](evidence/release/t47-guide.md) and [T48 intake](evidence/release/t48-intake.md) pass with independent review and external path integration. Hosted links/config recognition remain the declared T50/T53 checks. Autonomous authorization permits proceeding.

<a id="t47a"></a>
### T47a: Resolve remaining packaged-document links

- [x] T47a complete

**Dependencies:** T47–T48; required before T49. Independent review found that the
packaged security guide linked an API specification omitted from the Cargo file
set. It now links the versioned repository contract; contribution guidance
explicitly requires the full Git checkout for its specification/evidence links.
No source, dependency or requirement changed. Link/scope inspection, all 11
rustdoc tests, branding tree/history and scoped diff checking pass. Published
version URLs remain the declared T53 verification.

**Evidence:** [T47a link correction](evidence/release/t47a-links.md).

<a id="t49"></a>
### T49: Implement the enforced CI contract

- [x] T49 complete

**Module:** github-publication. **Dependencies:** [T36](#t36), [T45](#t45), [T46](#t46), [T47](#t47), [T48](#t48).

**Description:** Implement the pinned native pipeline and reviewable main/tag rulesets using the same local commands, including complete candidate-history scanning and safe PR execution.

**Acceptance criteria:**

- [x] Stable `quality`, `coverage`, `advisories` checks run on PR/main/tag/manual events, with weekly refreshed advisories, pinned tool/action revisions, locked dependencies and unchanged failure thresholds.
- [x] Read-only token scope, no persisted checkout credentials/secrets for untrusted PR code, bounded jobs, clean coverage data, safe interpolation and complete naming-scan history are verified.
- [x] Rulesets encode up-to-date required checks, PR/conversation/linear-history requirements, no main deletion/force pushes and immutable version tags, while zero mandatory approvals permits sole-maintainer PRs.

**Verification:**

- [x] Validate workflow/ruleset syntax and verified upstream action commits; execute every exact job command locally. Inspect permissions/events/cache keys/history depth and negative/tool-error controls without changing production thresholds. Run both naming gates before any push.
- [x] Relevant standing definition of done, scoped diff/provenance audit and actual evidence pass before completion is committed.

**Files likely touched:** `.github/workflows/ci.yml`, `.github/rulesets/main.json`, `.github/rulesets/tags.json`.

**Estimated scope:** Medium; three workflow/ruleset configuration files.

**Fresh verification record:** [T49](evidence/release/t49-ci.md).

<a id="t50"></a>
### T50: Publish the protected GitHub repository

- [x] T50 complete

**Module:** github-publication. **Dependencies:** [T49](#t49); valid authenticated `strange0g` access.

**Description:** Create the confirmed public destination from the verified fresh tree, observe bootstrap checks, enforce protections/security features and prove failures block updates. Inspect an existing destination before mutation.

**Acceptance criteria:**

- [x] Public `strange0g/hex-traders` has the intended main branch, metadata/topics, Issues/forms/Dependabot/security reporting and verified available security settings; no preparation/private original refs are pushed.
- [x] All three hosted checks pass for the assessed commit; main/tag rulesets are active and read back with actual check/app identities and no routine quality bypass.
- [x] Disposable formatting/test/coverage/advisory/branding/tool-error controls fail as intended and cannot merge; actor-aware rejected-update/ruleset proof is retained without publishing the former-name literal or weakening protected branches.

**Verification:**

- [x] Use `gh auth status --hostname github.com`, `gh api user --jq .login`, repository/run/ruleset/settings readbacks and actual run IDs to observe conclusions/head SHAs. Retain control exits and blocked merge/update evaluations. Never print tokens or merge a negative control; subsequent main changes use required-check PRs.
- [x] Relevant standing definition of done, scoped diff/provenance audit and actual evidence pass before completion is committed.

**Files likely touched:** Actual GitHub repository/settings; finalized `.github/rulesets/main.json` and `tags.json` if hosted app/check IDs require it; `tasks/evidence/release/t50-hosted.md`.

**Estimated scope:** Small repository-settings and compact hosted-evidence footprint.

**Fresh verification record:** [T50](evidence/release/t50-hosted.md).

<a id="c19"></a>
### C19: Hosted enforcement

**Dependencies:** T49–T50.

- [x] All required hosted checks are green for the actual SHA; active rules/security settings and deliberately failing/tool-error/rejected-update controls are verified.
- [x] Required affected checks and evidence pass; review the diff for weakened constraints, lost assertions, private material and unsupported claims.
- [x] Present the checkpoint evidence for human review or proceed under applicable autonomous build authorization; record actual status before continuing.

**Checkpoint record:** [T50 hosted enforcement](evidence/release/t50-hosted.md) records the exact bootstrap checks, active rules/security settings, six failing blocked controls, rejected owner update, cleanup and fresh native gates. Existing autonomous authorization permits T51–T53; native performance, independent GO and the immutable release remain pending.

<a id="t50a"></a>
### T50a: Run CI for every PR base branch

- [x] T50a complete

**Module:** github-publication. **Dependencies:** [T50](#t50).

**Description:** Repair the Required/P2 finding from the requested T49/T50 review.
The publication specification requires every PR, but the workflow restricted PR
base branches to `main`. Remove only that PR filter.

**Acceptance and verification:**

- [x] The ordinary PR event has no branch/path filter; main/version-tag pushes,
  manual/scheduled events, job permissions and quality requirements are retained.
- [x] A disposable check fails on the original filter and passes on the fix;
  restoring the filter fails again. All standing native/naming gates pass.
- [x] Fresh independent review approves the focused diff; evidence is privately
  preserved, a compact public record is linked, and the task has one verified commit.

**Evidence:** [T50a CI correction](evidence/release/t50a-ci-every-pr.md).

<a id="t51"></a>
### T51: Record fresh native performance evidence

- [ ] T51 complete

**Release-only authorization, 14 November 2026:** the maintainer instructed
publication despite the disclosed timing uncertainty. [R01](../CONSTRAINTS.md#r01-initial-source-release-with-unresolved-native-timing-acceptance)
permits the initial T52/C20/T53 release path to proceed with this limitation
explicitly acknowledged. T51 remains incomplete; no performance pass or future
release waiver is implied.

**Module:** hex-traders-branding, github-publication. **Dependencies:** [T49](#t49); can proceed while T50 awaits authentication.

**Description:** Measure the renamed candidate on the documented native reference machine with fixed fixtures and repeat variability. Retain timing and actual retained-memory/allocation evidence separately from hosted checks.

**Acceptance criteria:**

- [ ] The original benchmark cases, fixture facts, sampling conditions and environment are accounted for; fresh accepted/rejected/query/view/snapshot/route and 16/64-game memory evidence is recorded.
- [ ] A same-condition comparison and repeats resolve any repeatable regression over the unchanged manual 5% margin, with original baseline privately recoverable and no invented absolute/cross-target claim.
- [ ] Public evidence is checksummed, complete, free of original/private paths and accurately distinguishes measurements from estimates and shared/external retained data.

**Verification:**

- [ ] Run the plan's Criterion baseline/comparison and `HEX_TRADERS_MEMORY` commands with actual profiling/allocation tooling where required; record CPU/RAM/OS/toolchain/affinity/load and variability. Audit fixture/preparation facts and scan the final evidence archive plus full history.
- [ ] Relevant standing definition of done, scoped diff/provenance audit and actual evidence pass before completion is committed.

**Files likely touched:** `PERF.md`, `performance/review-hex-traders.md`, `performance/evidence/hex-traders-native.tar.gz`, `tasks/evidence/release/t51-performance.md`.

**Estimated scope:** Medium; three documents/artifacts plus compact evidence.

<a id="t52"></a>
### T52: Review the frozen release candidate independently

- [x] T52 complete for v0.1.0 under R01

**Completion evidence, 14 November 2026:** [Verified publication](evidence/release/t53-publication.md)
and [independent decision](evidence/release/t52-decision.md#published-decision-and-completed-verification)
record the exact immutable release, required checks and consumer/archive proof.
T51 remains incomplete; R01 permits only this initial publication.

**Module:** github-publication. **Dependencies:** [T50](#t50), [T50a](#t50a), [T51](#t51).

**Description:** Freeze the exact candidate and obtain independent code, security and test assessments of the complete source/fixture/privacy/workflow/settings/evidence path. These are new reports; original reports stay in T35 private preservation.

**Acceptance criteria:**

- [x] Independent reviews assess correctness/readability/architecture/performance, trust boundaries/workflow/private material, and rule/fixture/control coverage against the exact candidate and actual hosted/local evidence.
- [x] No unresolved release-blocking finding remains; necessary repairs become focused tasks and repeat affected checks/review without weakening any approved requirement.
- [x] A fresh release decision names assessed commit, required checks, performance limitations, immutable-tag/recovery procedure, prepared notes/changelog and repository enforcement evidence.

**Verification:**

- [x] Run the complete fresh native and naming gates against the frozen candidate and verify all hosted checks on its actual SHA. Resolve specialist findings before GO; verify notice/package/source-archive selection and inspect every report for prohibited/private material.
- [x] Relevant standing definition of done, scoped diff/provenance audit and actual evidence pass before completion is committed.

**Files likely touched:** `tasks/ship-reports/code-review.md`, `tasks/ship-reports/security-audit.md`, `tasks/ship-reports/test-analysis.md`, `tasks/evidence/release/t52-decision.md`.

**Estimated scope:** Medium; three fresh specialist reports plus one decision record.

<a id="c20"></a>
### C20: Assessed release candidate

**Dependencies:** T51–T52.

- [ ] Fresh native timing/memory evidence satisfies the unchanged review policy; independent reviews and exact-revision full local/hosted checks leave no release blocker.
- [x] Required affected checks and evidence pass; review the diff for weakened constraints, lost assertions, private material and unsupported claims.
- [x] Present the checkpoint evidence for human review or proceed under applicable autonomous build authorization; record actual status before continuing.

**R01 disposition:** the combined timing/memory criterion stays unchecked.
Independent reviews and exact-revision local/hosted source/security/test checks
passed; the maintainer approved initial publication with T51 unresolved.
[Decision](evidence/release/t52-decision.md#published-decision-and-completed-verification)
and [publication evidence](evidence/release/t53-publication.md) record the result.

<a id="t53"></a>
### T53: Publish and verify the immutable initial release

- [x] T53 complete for v0.1.0 under R01

**Documentation closure:** the T53 completion entries take effect only after
this documentation PR passes all required checks and merges through protected
main. Until then, the release is verified and documentation closure is pending.

**Completion evidence, 14 November 2026:** [Verified publication](evidence/release/t53-publication.md)
and [independent decision](evidence/release/t52-decision.md#published-decision-and-completed-verification)
record the exact immutable release, required checks and consumer/archive proof.
T51 remains incomplete; R01 permits only this initial publication.

**Module:** github-publication. **Dependencies:** [T52](#t52).

**Description:** Publish the prepared initial source release from the exact reviewed hosted-green commit. Validate remote clone, tag and automatically generated source archives before declaring the request complete.

**Acceptance criteria:**

- [x] Annotated `v0.1.0` and the non-draft GitHub release identify the exact assessed commit, match Cargo/changelog version and prepared notes, and are protected from silent movement/deletion.
- [x] A clean remote clone at the tag passes relevant full build/test/coverage/advisory/naming checks and compiles a Git/path consumer; GitHub source ZIP/tar archives and Cargo file lists have only intended safe files/notices.
- [x] Readback evidence links the real repository/release/check runs and verified workspace location; post-publication status records use a checked documentation-only PR and distinguish later HEAD from the immutable release tag.

**Verification:**

- [x] Run the specified annotated-tag/push and `gh release create ... --verify-tag --notes-file docs/releases/v0.1.0.md` commands only after revision verification. Clone and peel the remote tag, inspect generated archives without executing their contents, run the relevant standing gates and external consumer, then read back release visibility/tag/run conclusions.
- [x] Relevant standing definition of done, scoped diff/provenance audit and actual evidence pass before completion is committed.

**Files likely touched:** Annotated `v0.1.0` tag and GitHub release; `tasks/evidence/release/t53-publication.md`; final task/checkpoint status via a documentation-only PR.

**Estimated scope:** Small; GitHub tag/release plus completion evidence/status.

<a id="c21"></a>
### C21: Published Hex Traders

**Dependencies:** T53.

- [x] The public release/tag, clean remote consumer, generated archives, active local checkout and completion evidence are verified; the release tag stays immutable.
- [x] Required affected checks and evidence pass; review the diff for weakened constraints, lost assertions, private material and unsupported claims.
- [x] Present the checkpoint evidence for human review or proceed under applicable autonomous build authorization; record actual status before continuing.

**Completion evidence, 14 November 2026:** [T53](evidence/release/t53-publication.md)
verifies publication, clean consumer/archive checks, the synchronized active
checkout and privately preserved originals. This documentation PR records
completion separately from immutable v0.1.0. C21 completion entries take effect
only after this PR passes all required checks and merges through protected main;
until then, documentation closure is pending. T51 stays open under the standing
policy; the initial-release R01 exception is consumed.

## Release execution notes

Split newly discovered independent remediation into focused tasks with explicit
dependencies. Keep T01–T34/C01–C11 original completion evidence and history
privately recoverable. Public control commits/bodies/logs must never expose the
former identity: generate negative payloads in runtime disposable fixtures with
neutral diagnostics. After publication, record completion through a required-check
documentation PR without moving `v0.1.0`.

---

## Completed engine tasks (historical public derivative)

# Task List: Hex Traders Base Game Engine

PUBLIC DERIVATIVE: original completed-task facts and conclusions are preserved
here as historical summaries. Exact original task text, commits and indexed raw
evidence remain privately recoverable. Uninventoried scratch bytes are not claimed
retained; fresh release verification is recorded above.

Status: approved on 9 November 2026. The user subsequently authorized `$build auto`
to implement the entire plan autonomously with subagents. Only verified tasks
are checked below.
Approach and standing checks: [plan.md](plan.md). Governing requirements:
[SPEC.md](../SPEC.md), the four provider specs and [CONSTRAINTS.md](../CONSTRAINTS.md).
This checklist is the single execution-status source; do not maintain a second
copy of task completion in the plan.

Implementation is authorized. Mark a task complete only after acceptance,
its focused verification and the standing definition of done. Run the checkpoint
commands from plan.md and present evidence for human review before the next phase.
Module entry filenames are estimates: substitute focused files as code grows,
include affected callers/exports, and split any slice that becomes too large.

## Phase 1: Bootstrap and board identity

<a id="t01"></a>
### T01: Create the Rust library toolchain

- [x] T01 complete

**Description:** Create the smallest buildable library with the selected Rust toolchain and dependency versions; add no engine scaffolding or unimplemented command handlers.

**Modules:** Project tooling. **Dependencies:** None.

**Rule trace:** Model/API/quality contract; no additional tabletop rule.

**Acceptance criteria:**

- [x] Cargo uses Rust 1.98.1/edition 2024, the selected runtime/dev dependencies, and a committed-ready lockfile; the RNG dependencies disable default entropy features.
- [x] The library forbids unsafe code and builds in debug and release; target/ and generated reports are ignored without touching existing agent configuration.
- [x] Only required project files exist; no renderer, async runtime, bindings, or blank per-feature files are generated.

**Verification:**

- [x] `cargo check --locked --all-targets`
- [x] `cargo build --locked --release`
- [x] `cargo fmt --all -- --check`

**Evidence:** Pinned toolchain/dependencies resolved; debug/all-target check, release build, formatting, Clippy, library/integration tests, doc tests and public rustdoc passed. This configuration-only slice has zero executable tests; no engine behavior or coverage claim is made. The runtime dependency tree contains no `getrandom`.

**Files likely touched:** `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `src/lib.rs`, `.gitignore`.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

<a id="t02"></a>
### T02: Make the written quality checks executable

- [x] T02 complete

**Description:** Set up the selected local coverage and dependency-advisory checks before adding rules. Pin tool versions from their official releases at execution time and validate that the commands actually enforce their stated verdicts.

**Modules:** Project tooling. **Dependencies:** [T01](#t01).

**Rule trace:** Model/API/quality contract; no additional tabletop rule.

**Acceptance criteria:**

- [x] Clippy/rustfmt, coverage with the 90% library-line floor, and cargo-deny with zero known vulnerabilities have reproducible commands and pinned tool/config versions.
- [x] Disposable clean/violating controls prove compiler/lint/coverage/advisory failures and checker errors produce failure; controls never become skipped production tests.
- [x] CONSTRAINTS.md distinguishes installed local gates from manual review and absent hosted CI; tool absence or failed database refresh cannot be reported as green.

**Verification:**

- [x] `cargo clippy --locked --all-targets -- -D warnings`
- [x] `cargo llvm-cov --locked --lib --tests --ignore-filename-regex '(^|/)(tests|benches)/' --fail-under-lines 90` — executed; expected no-data failure on the empty bootstrap, not a coverage pass. C01 requires real coverage after T03.
- [x] `cargo deny --locked check advisories`

**Evidence:** [Pinned tool versions, official sources, actual results, and disposable clean/violation/error controls](quality-checks.md). Compiler, formatting, Clippy, release build and current test commands passed; advisory refresh/scan passed. Coverage controls pass at 100% and fail at 46.15%; the real empty crate correctly fails for no data. No gate threshold or assertion was weakened.

**Files likely touched:** `deny.toml`, `rust-toolchain.toml`, `CONSTRAINTS.md`.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

<a id="t03"></a>
### T03: Build the stable board graph

- [x] T03 complete

**Description:** Build the immutable logical graph and checked domain identifiers required by every later rule. This is a foundational provider slice, not an extra public layout-preview API.

**Modules:** game-model. **Dependencies:** [T01](#t01), [T02](#t02).

**Rule trace:** Model/API/quality contract; no additional tabletop rule.

**Acceptance criteria:**

- [x] The integer-coordinate construction produces exactly 19 hexes, 54 vertices, and 72 edges with the specified stable ordering and reciprocal, deterministic incidence.
- [x] Graph data identifies coast, endpoints and neighbors without UI coordinates; checked graph IDs reject before lookup and native rule consumers borrow the same topology.
- [x] Graph tests use independently recorded expected IDs/incidence, including coastal vertices; queries borrow shared topology without cloning it.

**Verification:**

- [x] `cargo test --locked --test model_invariants`
- [x] `cargo test --locked game_model_`
- [x] `cargo check --locked --all-targets`

**Evidence:** Five integration tests passed after an initial missing-export failure. Literal fixtures cover all 19 perimeters, 72 endpoint pairs and 30 coastal vertices/edges; expected IDs were independently hand-counted from vertex bands of sizes `3,4,4,5,5,6,6,5,5,4,4,3`, then hex perimeters were traced. Formatting only numbered/inverted those literal rows, without constructing or deduplicating corners. Coast-by-degree and off-by-one ID mutations failed their regression tests; restored code passed. All standing commands passed, including release/rustdoc and a refreshed advisory scan. Library coverage is **95.77% (136/142 lines)**; board construction/accessors are 100%. Independent C01 review approved with no required findings. No gameplay or performance claim follows from graph verification.

**Files likely touched:** `src/game_model/mod.rs`, `src/game_model/board.rs`, `src/lib.rs`, `tests/model_invariants.rs`.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

<a id="c01"></a>
### C01: Tooling and stable graph

- [x] T01–T03 acceptance and focused checks pass.
- [x] Build/lint/local quality controls pass; the graph provider has source-independent 19/54/72 and adjacency checks.
- [x] Run all applicable standing checkpoint commands in plan.md; record actual results and limitations.
- [x] Present checkpoint evidence for human review before proceeding.

**Evidence:** T01–T03 and all C01 commands passed; gate failure controls are recorded in [quality-checks.md](quality-checks.md). Independent review verified immutable borrowed topology, checked IDs, acyclic imports, no runtime entropy or weakened gates. Evidence is presented under the user's autonomous checkpoint authorization.

## Phase 2: Verified layouts and game creation

<a id="t04"></a>
### T04: Verify the official setup fixtures

- [x] T04 complete

**Description:** Turn the official diagrams into explicit, independently verified source fixtures before gameplay depends on them. Include the fixed roads/ports and six variable frame templates. Production setup calculations are introduced with their creation consumers in T05/T06, and must be checked against these same fixtures.

**Modules:** game-model → base-rules. **Dependencies:** [T03](#t03).

**Rule trace:** BR001, BR002, BR003, BR004, BR005, BR006.

**Acceptance criteria:**

- [x] Fixed terrain/tokens, every preset settlement and road, and all nine ports match the 2025 diagrams by stable IDs; the three-player fixture omits White and yields the stated second-settlement hands.
- [x] Six sea-frame templates retain their printed port kinds and spacing; A–R values and the top-right counterclockwise spiral are independently checked against the source.
- [x] Source URL/page and capture notes live with the verified fixture; visual uncertainty blocks conformance rather than inviting invented coordinates. T05/T06 must verify their production setup calculations against this captured data.

**Verification:**

- [x] `cargo test --locked --test rules_conformance base_rules_br00`
- [x] `cargo test --locked --test model_invariants`
- [x] Compare fixture IDs with rendered official pages 3–5 and 11; use an original diagnostic board render only to verify geometry.

**Evidence:** Six source-fixture tests failed with the missing fixture, then passed with the verified capture. The full current suite is 11 passing tests; build, release, formatting, Clippy, doc checks and 95.77% library coverage passed. Independent review compared captured data and rendered source diagrams; changing Blue's first road to another incident edge failed the source-orientation assertion, and restoration passed. Exact source hash, pages, capture procedure, coordinates, frame templates and confidence are in [setup-source.md](../tests/support/setup-source.md). No source artwork is a product asset. Production setup verification remains required in T05/T06.

**Files likely touched:** `tests/support/setup.json`, `tests/support/setup-source.md`, `tests/rules_conformance.rs`, `SPEC-base-rules.md`.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

<a id="t05"></a>
### T05: Create a fixed game through the native facade

- [x] T05 complete

**Description:** Implement the first complete consumer path: Engine::create with a host-private seed returns a valid fixed game, creation receipt, and seat/spectator views. Introduce shared fixed-setup calculations here and verify them against T04's independent source fixtures.

**Modules:** All four modules. **Dependencies:** [T04](#t04).

**Rule trace:** BR001, BR003, BR005, BR007, BR008, BR033.

**Acceptance criteria:**

- [x] Three/four-seat configurations and their order are checked; fixed role mapping, stock, full shuffled development deck, starting hands, robber, starter/tied-highest rolls, revision 0 and turn 1 are correct.
- [x] The facade owns board/position/RNG separately; creation is deterministic in the specified operation order and rejects invalid input without returning a partial engine.
- [x] Initial views/events expose only public facts and the owning seat's hand; raw state, seed, deck order, private card IDs and true opponent scores are inaccessible through a player view.

**Verification:**

- [x] `cargo test --locked --lib fixed_creation_`
- [x] `cargo test --locked --lib privacy_initial_`
- [x] `cargo check --locked --all-targets`

**Files likely touched:** `src/game_model/mod.rs`, `src/base_rules/setup.rs`, `src/turn_engine/mod.rs`, `src/engine_api/mod.rs`, `src/lib.rs`.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

**Implementation slices:** Complete all three before checking T05.

- [x] T05a1: Checked native configuration, resource values and public errors. Depends on T04. Own `src/game_model/mod.rs` and `src/lib.rs`, with focused positive/negative tests. Introduce no unused private state, card scaffolding or unsupported creation option.
- [x] T05a2: Complete fixed creation and host-private receipt. Depends on T05a1. Own `src/game_model/mod.rs`, `src/base_rules/mod.rs`, `src/turn_engine/mod.rs`, `src/engine_api/mod.rs`, and `src/lib.rs`; colocate focused tests. Verify seat order, source layout/stock/hands/deck, starter/tied-highest RNG goldens, separate board/state/RNG, initial invariants and creation rejection. Expose only implemented creation behavior.
- [x] T05b: Initial viewer allowlists and creation-receipt projection. Depends on T05a2. Extract the growing creation state into `src/game_model/state.rs` with its module declaration; add `src/engine_api/view.rs`, its facade declaration, and deliberate exports. Verify every valid seat/spectator, invalid viewers, original receipt facts and absence of seed/deck/opponent-private data. Run complete standing checks and source-fixture comparisons before T05 completion.

Each slice runs its focused RED/GREEN tests, full current tests/build/lint/coverage,
and receives its own commit. No unused modules, stubs, or public preview API are
introduced to fit a file count.

**T05a1 evidence:** Checked configuration/resource tests cover valid seat orders,
all u8 seat values, every invalid resource count in each position, and stable
errors. The 13-test suite, check/release/fmt/Clippy/docs and 97.60% library line
coverage (244/250) passed; the refreshed advisory scan passed. Independent
review found no required changes. In an isolated copy, allowing seat 4 made
the rejection test fail; restoration passed. Creation, views and T05 completion
remain pending.

**T05a2 evidence:** Complete fixed creation agrees with the captured source
terrain/numbers/ports/presets and independent four-case deck/starter/RNG
goldens. Twenty-two initial-state corruptions reject; topology is borrowed and
position/RNG are separate. All 19 current tests, two consumer/privacy doctests,
check/release/fmt/Clippy/docs and 99.14% library line coverage (919/927) passed.
The refreshed advisory scan remains applicable to the unchanged lockfile.
Independent review found no required changes; changing the leader comparison
from `>` to `>=` failed the golden test, and restoration passed all eight
focused tests. Creation records every starter roll, so its receipt can grow
with tied rounds under the approved reroll-until-one policy; it is separate
from the bounded gameplay-command transition contract. Views and T05b remain
pending; no gameplay, performance or hosted CI completion is claimed.

**T05b evidence:** Explicit native views and projected creation receipts cover
every seat/spectator and reject absent seats. Own hands/scores are separate from
public totals/board/bank/pieces; hidden hand/deck/RNG variations leave uninvolved
views unchanged. Old receipt projection retains its original facts/revision.
State was extracted into a focused provider file. All 22 tests, five compiling
or privacy doctests, check/release/fmt/Clippy/docs and 99.31% library line coverage
(1151/1159) passed. Independent review found a focused-test naming mismatch;
it was corrected and the exact `privacy_initial_` command now runs three passing
tests. A reversed viewer-membership guard failed all three tests; restoration
passed. No required review findings remain. The advisory result remains
applicable to the unchanged lockfile. Fixed creation is complete; general
gameplay scoring replaces the explicitly documented creation-road shortcut
when its consumer arrives. No full-game/performance/hosted CI claim is made.

<a id="t06"></a>
### T06: Create a variable game deterministically

- [x] T06 complete

**Description:** Extend the same creation path to shuffle terrain and frame templates, place numbered tokens by the fixed spiral, and return the initial setup decision.

**Modules:** All four modules. **Dependencies:** [T05](#t05).

**Rule trace:** BR002, BR003, BR004, BR006, BR007.

**Acceptance criteria:**

- [x] Variable creation uses the source frame permutation, correct terrain/token distributions, desert skip, robber location and prescribed RNG sequence; no number shuffle or hidden balance reroll is added.
- [x] Three/four seats enter SetupSettlement with empty hands, no buildings/roads, turn 0 and revision 0; views explain the current setup actor.
- [x] Seed/config golden fixtures reproduce layouts, starter rolls and next RNG state, including tied-highest rerolls and two matches with separate terrain layouts.

**Verification:**

- [x] `cargo test --locked --lib variable_creation_`
- [x] `cargo test --locked base_rules_br006_`
- [x] `cargo check --locked --all-targets`

**Files likely touched:** `src/game_model/mod.rs`, `src/game_model/state.rs`, `src/base_rules/mod.rs`, `src/turn_engine/mod.rs`, `src/engine_api/mod.rs`; colocate focused provider/facade tests. Shared public phase/events allow the existing view/projection path to represent variable setup without a second converter.

**Preflight:** Independent source-index RNG fixtures cover seeds 6/21 for
both player counts, seed 8 for a single tied tier, and seeds 12/17 for repeated
ties. Expected layouts, ports, full deck, ordered rolls, starter, word position
and next dice were produced without reading production creation code. Pin the
canonical source-array inputs as described in SPEC-turn-engine.md and retain
literal expectations in committed tests; temporary oracle files are not runtime
inputs. The preflight preceded T05b completion and T06 implementation.

**T06 evidence:** Eight retained independent goldens assert complete layouts,
source frame ports/spiral, deck IDs, starter/tied-tier rolls, word positions and
next dice. The source sample and all 19 desert positions pass. Eleven variable
corruptions and 22 retained fixed corruptions reject; invalid frame IDs reject
before template indexing. Public facade tests cover every seat/spectator,
empty stock/phase/turn facts, distinct match layouts and unchanged query RNG.
Focused filters ran four variable, three BR006, three initial-privacy and eight
fixed tests. The full 27-test suite, five doctests, check/release/fmt/Clippy/docs
and 99.36% library line coverage (1564/1574) passed. The refreshed advisory gate
passed. Independent creation-checkpoint review found no required findings;
isolated viewer and source-identity mutations failed, restoration passed, and
shared source hashes remained unchanged.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

<a id="c02"></a>
### C02: Official and deterministic creation

- [x] T04–T06 acceptance and focused checks pass.
- [x] Fixed fixtures/frame templates are visually checked; both game counts/setup modes create deterministic valid viewer-safe games.
- [x] Run all applicable standing checkpoint commands in plan.md; record actual results and limitations.
- [x] Present checkpoint evidence for human review before proceeding.

**Checkpoint evidence:** The source capture/visual review in
[setup-source.md](../tests/support/setup-source.md), fixed creation/privacy
tests and eight independent variable RNG fixtures pass together. All current
standing checks above and a refreshed advisory scan passed; independent review
approved the complete creation path. Evidence was presented on 9 November
2026; existing autonomous authorization permits continuing. Placement commands,
complete games, wire/save/replay, benchmarks, Wasm and hosted CI remain outside
this checkpoint and have no completion claim.

## Phase 3: Setup, production and discard decisions

<a id="t07"></a>
### T07: Complete the variable setup placement sequence

- [x] T07 complete

**Description:** Add placeSetupSettlement and placeSetupRoad as atomic commands with legal candidates, projected events, and the snake placement order.

**Modules:** All four modules. **Dependencies:** [T06](#t06).

**Rule trace:** BR007, BR008, BR018, BR020.

**Acceptance criteria:**

- [x] Forward/reverse placement, the final seat's consecutive pairs, paired-road adjacency and settlement distance work for both player counts; disconnected starting pairs are legal.
- [x] Only the second settlement grants starting resources, after all setup pairs; the last road starts turn 1/PreRoll without an extra UI instruction.
- [x] Wrong seats/phases/locations and repeated placements leave all state unchanged; accepted commands increment revision once, and legal candidates agree with execution.

**Verification:**

- [x] `cargo test --locked --test game_sequences setup_`
- [x] `cargo test --locked base_rules_br008_`
- [x] `cargo check --locked --all-targets`

**Files likely touched:** `src/game_model/mod.rs`, `src/base_rules/mod.rs`, `src/turn_engine/mod.rs`, `src/engine_api/mod.rs`, `tests/game_sequences.rs`.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

**Implementation slices:** Complete all before checking T07. Preflight found
that the first command needs six files including two small export changes;
retain the existing provider-root exports rather than adding a namespace only
to reduce the file count. No unsupported command or executable stub is exposed.

- [x] T07p: Extract existing creation into `src/turn_engine/creation.rs`; keep `src/turn_engine/mod.rs` as its provider index. Pure move, no behavior change or new logic tests; verify all existing source/RNG/privacy tests and standing checks. Depends on T06/C02.
- [x] T07q: Retain designated second-settlement facts in `src/game_model/state.rs`, share their current fixed starting-resource calculation in `src/base_rules/mod.rs`, and initialize/verify them in `src/turn_engine/creation.rs`. The helper and facts have real fixed-creation consumers; add corruption/resource assertions without changing current results. Depends on T07p.
- [x] T07r: Select creation event kind from the actual setup mode, preserving existing creation facts while allowing the shared phase enum to grow without unrelated exhaustive gameplay arms. Own `src/turn_engine/creation.rs`; verify existing creation/source/RNG tests and all standing gates. Depends on T07q.
- [x] T07a: Implement the complete native setup-settlement path in `src/game_model/state.rs`, `src/base_rules/mod.rs`, `src/turn_engine/mod.rs`, `src/engine_api/mod.rs`, with deliberate exports in `src/game_model/mod.rs` and `src/lib.rs`. Include envelope/version/seat/revision rejection, shared validation/candidates, atomic transition, host receipt and opt-in viewer projection. Only the settlement command exists in this slice; its public SetupRoad phase is truthful and has no supported command until T07b. Colocate focused contract/privacy tests. Depends on T07q.
- [x] T07b: Implement paired setup roads, forward/reverse actor order and final starting-resource grant in `src/game_model/state.rs`, `src/base_rules/mod.rs`, `src/turn_engine/mod.rs`, `src/engine_api/mod.rs`, `src/engine_api/view.rs`. Replace the presence-only road shortcut with exact current 0/1/2-road scoring; retain its at-most-two bound until T11 supplies the general calculator. Reuse the same predicates and second-settlement calculation; no RNG consumption. Depends on T07a.
- [x] T07c: Add literal three-/four-seat setup sequences in `tests/game_sequences.rs` through public commands only. Independently captured traces cover rotated order, boundary consecutive pairs, joined roads, coastal/desert/duplicate-resource grants and negative examples. Verify the required `setup_` target and full standing checks. Depends on T07b.

Every slice receives its own verified commit. Actual growth may require another
focused extraction before adding behavior; it cannot justify giant module
indices, duplicate rules, omitted callers, unused scaffolding or weakened tests.

**T07p evidence:** Creation source/tests moved byte-for-byte; parent independently
confirmed matching SHA-256
`99ec1064357dffe504ce7dae8419a6076e7768a3ef5252c4945f0dd4014bd6a2`.
The new index declares the private module and two existing provider exports.
All focused source/RNG/privacy filters, 27 tests, five doctests, check/release/
fmt/Clippy/docs and unchanged 99.36% coverage (1564/1574) passed. No new logic,
tests, behavior or dependencies; C02 advisory evidence remains applicable.

**T07q evidence:** Designated second vertices and one pure starting-resource
calculation have current fixed-creation/invariant consumers. Source ownership,
exact grants and coastal/desert/duplicate-resource cases pass; eight additional
identity/ownership/grant corruptions reject. All 29 tests, five doctests,
check/release/fmt/Clippy/docs and 99.46% coverage (1644/1653) passed without
changing source layouts/hands/events/RNG. Independent review found no required
changes; disabling the second-identity guard failed the corruption test, and
restoration passed both focused checks. Dependency lock remains unchanged.

**T07r evidence:** Initial event selection now follows the actual fixed/variable
creation discriminator; gameplay phase additions do not require unrelated
creation arms. All 29 current tests, five doctests, check/release/fmt/Clippy/docs
and 99.46% library line coverage (1644/1653) passed. Independent read-only review
approved with no required findings and reran all 29 tests. A disposable mutant
emitting TurnStarted for variable setup failed the facade event assertion. The
initial restoration run incorrectly reused the cached mutant binary after file
timestamps were restored; it did not pass. Source equality was then checked,
and a fresh `private historical evidence` rebuilt the restored copy: the
required variable-facade test actually ran (one test) and passed. Disposable
controls now use a separate target to avoid that cache error. Unfinished T07a
changes remained outside the T07r commit.

**T07a evidence:** The two native settlement tests first failed for missing APIs,
then passed through shared validation, bounded candidates, one atomic revision,
and separate safe receipt projection. Version/seat/revision/rule rejection and
private position/RNG preservation pass. Independent review exposed candidate and
old-receipt test gaps: new independent V0/V3/V4 exclusions and changed-current-
phase assertions reject both disposable mutants; the restored fresh source and
fresh target passed both focused tests. All 31 tests, five doctests, check/release/
fmt/Clippy/docs and **98.66% coverage (1991/2018 lines)** passed. Review approved;
lock/dependencies are unchanged and C02 advisory evidence remains applicable.
Only the settlement command exists; SetupRoad becomes executable in T07b.
T07r's cached restoration evidence is corrected above.

**T07b evidence:** Road/route tests failed for missing vocabulary/calculators,
then passed. The literal three-seat trace verifies paired adjacency, snake
boundary, deferred exact grants, stocks, stale/occupied/disconnected rejection
and unchanged RNG next draws. The shared current route helper directly asserts
0/1/2, disconnected roads and an opponent blocker. All 33 tests, five doctests,
check/release/fmt/Clippy/docs and **98.80% coverage (2311/2339 lines)** passed.
Independent review required exact projected facts rather than event counts:
all seats/spectator now retain exact original grants/turn/phase through later
state changes; an incorrect grant-seat mutant fails, and fresh restored source/
target passes. Paired-adjacency mutation also fails. Review approved. T07c's
public three-/four-seat proof remains required; lock/advisories are unchanged.

**T07c evidence:** Two public tests execute all 28 literal accepted setup
commands for three/four seats; independent captured expectations cover every
phase/actor/turn/board slot, stock, route, public count, own hand and final bank.
The coastal/desert-only/duplicate-resource and boundary consecutive-pair cases
pass. Wrong actor, nonincident/occupied road, occupied/distance vertex checks
preserve observed state. Independent review matched all literals with its
source-backed oracle; an isolated zero-grant mutation failed both tests, and
restored source passed in a separate fresh target. Final added active-player
and occupied-E1 assertions were subsequently reviewed and passed 2/2. All 35
tests, five doctests, check/release/fmt/Clippy/docs and **98.80% coverage
(2311/2339 lines)** passed. T07 is complete; broader gameplay remains pending.

<a id="t08"></a>
### T08: Roll dice and advance ordinary turns

- [x] T08 complete

**Description:** Implement rollDice and endTurn using explicit RNG, aggregate production, and an immediate turn-start score check. A seven returns the complete mandatory-discard/robber decision, whose resolution follows next.

**Modules:** All four modules. **Dependencies:** [T07](#t07).

**Rule trace:** BR009, BR010, BR011, BR012, BR015, BR037.

**Acceptance criteria:**

- [x] Dice are two independent unbiased d6 samples; non-seven production honors cities, robber blocking, multiple matching hexes and exact single/multiple-claimant shortages.
- [x] A seven pays nothing and fixes discard obligations once; it enters Discard or RobberMove directly when no discard is required. EndTurn is accepted only in Action and checks the next seat's true score before a roll.
- [x] Queries use no randomness; wrong phase/stale/repeated rolls do not consume a draw; public dice/production/turn events are deterministic.

**Verification:**

- [x] `cargo test --locked --test game_sequences turn_`
- [x] `cargo test --locked base_rules_br01`
- [x] `cargo check --locked --all-targets`

**Files likely touched:** `src/game_model/mod.rs`, `src/base_rules/mod.rs`, `src/turn_engine/mod.rs`, `src/engine_api/mod.rs`, `tests/game_sequences.rs`.

**Implementation slices:** Actual caller preflight approved by the parent under
the autonomous plan. Keep existing module-root exports and one verified commit
each; no unsupported command is advertised.

- [x] T08p: Mechanically extract existing native command types/methods and setup tests to `src/engine_api/command.rs`, leaving creation in its index; move the current command provider unchanged to `src/turn_engine/setup.rs`. Four source files plus status; verify moved body hashes and all current tests/gates. The native index has reached 1,064 lines before this extraction.
- [x] T08a: Add city-aware occupancy and shared current building-score calculation with real view consumers. Actual seven source files: `src/game_model/state.rs`, `src/game_model/mod.rs`, `src/lib.rs`, `src/base_rules/mod.rs`, `src/turn_engine/creation.rs`, `src/turn_engine/setup.rs`, `src/engine_api/view.rs`. This exception includes necessary initializers, fixed-creation kind invariants, two small existing export edits, and city-aware stock/score views with rare private fixtures. No city command or speculative card ownership is added.
- [x] T08b: Implement the complete RollDice native path, production and fixed seven obligations. Actual approved ten source callers: state, existing model/lib exports for live typed robber origin/continuation, new economy plus base-rules index, new gameplay plus turn index, native command, creation sampler visibility, and the existing setup predicate caller required to share context validation once. `tests/game_sequences.rs` supplies the public turn proof. Reuse the existing unbiased dice sampler; stored Seven→Action is the only currently represented robber effect. Include city/robber/aggregate shortages and nonrandom queries/rejections.
- [x] T08c: Implement EndTurn in Action with checked counters and the next owner's immediate current true-score/finish hook. All 25 cards are currently in the deck, so there are no held hidden VP facts. T16 must extend the same helper and finish/reveal path with held VP when purchase supplies the real card-ownership consumer; no duplicated deck cards or separate score implementation.

**Estimated scope:** Split as above; any further actual caller growth is recorded
before implementation rather than hidden in an oversized index.

**T08p evidence:** Existing native command/type/setup-test and turn-provider
bodies moved unchanged. SHA-256 identity checks passed for declarations/methods,
setup tests and the complete turn-provider body; independent review also compared
retained creation code and prior HEAD. Seven focused tests, all 35 tests, five
doctests, check/release/fmt/Clippy/docs and unchanged **98.80% coverage
(2311/2339)** passed. Public exports and dependencies are unchanged. No new logic
or mutation-control claim; review approved.

**T08a evidence:** The private city-view test failed for missing kind/value,
then passed with shared current score and derived city/settlement stocks. Fixed
creation rejects a corrupted City and all existing initializers create Settlement.
All 36 tests, five doctests, check/release/fmt/Clippy/docs and **98.84% coverage
(2384/2412)** passed. Independent review approved; a disposable City-value 2→1
mutation fails the score assertion, and fresh restored source/target passes.
Only represented building points exist currently; T16 must separate public
building/award score from true held-VP score while extending the same finish path.

**T08b evidence:** Native dice/seven tests first failed for missing commands/
phases, then passed with the existing unchanged unbiased sampler. A separate
pinned-dependency oracle with no engine imports supplied exact raw/after-creation
dice pairs. Pure BR010/011/012 tests assert matching-hex accumulation, cities,
robber blocking, distinct-claimant shortage, sole remainder, empty stock and fixed
quantities. The public fixed-roll expectation initially missed Orange V17;
the independent H8 perimeter/source preset corrected that test, not production.
All 42 tests, five doctests, check/release/fmt/Clippy/docs and **98.95% coverage
(2727/2756)** passed. Independent review reran all 42 tests; a multiple-claimant
shortage payout mutant fails BR011 and a fresh restored target passes. Review
approved. Mandatory phases are truthful; their commands follow in T09/T10.

**T08c evidence:** End-turn/finish tests failed for missing command/phase,
then passed with checked counters, supplied-order rotation and unchanged RNG.
A public three-seat wrap preserves hands and the independent next dice pair;
a private four-seat test covers nonnumeric order. An exact-ten valid-stock/
distance next-owner fixture finishes within EndTurn and exposes no subsequent
required decision. All 45 tests, five doctests, check/release/fmt/Clippy/docs and
**98.99% coverage (2951/2981)** passed. Independent review reran all 45 tests and
approved; an isolated threshold-11 mutant fails the exact-ten fixture. Current
score reflects represented buildings only, with no held development cards;
T16 extends the same score/finish path when ownership exists. T08 is complete.

<a id="t09"></a>
### T09: Fulfil required discards safely

- [x] T09 complete

**Description:** Implement discardResources for any obligated seat, with exact quantities and private resource-type projection.

**Modules:** All four modules. **Dependencies:** [T08](#t08).

**Rule trace:** BR012, BR015.

**Acceptance criteria:**

- [x] More than seven discards floor(total/2), exactly seven does not, development cards are excluded, and obligations can complete in either arrival order without repeated halving.
- [x] Only the last successful discard opens robber movement; no trade, card, roll or endTurn interrupts mandatory resolution.
- [x] Wrong quantity/stock/seat or a duplicate submission changes nothing; public events show seat/count and only the discarder sees types.

**Verification:**

- [x] `cargo test --locked --test game_sequences discard_`
- [x] `cargo test --locked --lib privacy_discard_`
- [x] `cargo check --locked --all-targets`

**Files likely touched:** `src/game_model/mod.rs`, `src/base_rules/mod.rs`, `src/turn_engine/mod.rs`, `src/engine_api/mod.rs`, `tests/game_sequences.rs`.

**Actual scoped implementation:** Six source files plus public integration proof:
`src/game_model/state.rs`, `src/base_rules/economy.rs`, `src/base_rules/mod.rs`
(the required existing helper export), `src/turn_engine/mod.rs`,
`src/turn_engine/gameplay.rs`, `src/engine_api/command.rs`, and
`tests/game_sequences.rs`. The parent authorized this complete path rather than
unused partial helpers. Required-discard eligibility is shared by query and
validation; bundle stock/quantity rules are implemented once.

**Estimated scope:** One focused consumer slice including the small provider
export; one verified commit after all acceptance and standing checks.

**T09 evidence (10 November 2026):** Native/public discard tests initially failed
for missing command, legal-action and event types, then passed. The public
three-seat fixed `[0,1,2]` seed-12 trace reaches turn 20/revision 39 and fixed
`[7,7,7,0]` obligations; both arrival orders preserve exact hands/bank and no
second halving, even when a remaining hand still holds eight. The independent
four-seat `[2,0,3,1]` seed-220 trace reaches turn 6/revision 11 with
`[0,4,4,0]`; the exactly-seven seat has no discard, and both arrival orders
reach the same conserved hands/bank and RobberMove at revision 13. Literals
come from standalone pinned-RNG/source-incidence arithmetic, with no runtime
oracle or private mutation in those public traces. Native tests cover exact
quantity/stock, atomic rejections, unchanged RNG and owner-only original receipt
projection despite later state/revision changes. Mandatory phases reject all
currently exposed ordinary commands; trade/card commands are still absent.
All **48 tests**, five doctests, check/release/fmt/Clippy/docs and **98.96% line
coverage (3145/3178)** passed. Clippy first rejected a helper below the test
module; moving its unchanged body above the tests fixed this without suppression.
Independent review approved, then re-reviewed the four-seat fixture and
mechanical move. Its isolated spectator-discard-leak mutant failed and fresh
restored source passed before those final refinements; the final review reran
both public discard tests. Robber execution remains T10; no owned development
cards exist until T16 and resource-only obligations exclude the untouched deck.

<a id="c03"></a>
### C03: Setup and turn obligations

**C03 correction preflight (10 November 2026):** Fresh checkpoint review found
that current reachable resource values cannot overflow, but narrow intermediates
do not meet SPEC-game-model's approved wider/check-bounds contract. T09q will
introduce private shared checked resource arithmetic and total calculation,
apply it to existing source/setup/production/discard/view/receipt consumers,
and verify conservation and rejected overdraw/overflow. Actual seven source
files are `src/game_model/mod.rs`, `src/base_rules/mod.rs`,
`src/base_rules/economy.rs`, `src/turn_engine/setup.rs`,
`src/turn_engine/gameplay.rs`, `src/engine_api/view.rs`, and
`src/engine_api/command.rs`; small connected caller edits keep one calculation
per rule and add no public API. One verified correction commit follows new
failing-then-passing model tests, standing gates and independent re-review.

- [x] T09q wider checked resource arithmetic correction complete.

**T09q evidence (10 November 2026):** The new model test first failed for
missing private helpers, then passed exact conserved transfers, overdraw,
per-type overflow, wide 256/65535 rejection and total 95. Shared private
Resources construction/checked add/subtract/total now supply the existing
setup, production, discard, public-count and receipt consumers; claims and
source grants accumulate in u16 and all narrowing checks bounds. Public
signatures and approved rules remain unchanged. All **49 tests**, five doctests,
check/release/fmt/Clippy/docs and **98.98% line coverage (3191/3224)** passed.
Final focused setup/turn/BR008/BR010–012/discard/initial-privacy/discard-privacy
filters all ran nonzero tests and passed after the correction. Fresh checkpoint
review approved all seven source edits and the model test. Its optional isolated
mutation build failed before running the control due to temporary disk quota;
that target was removed and supplies no mutation evidence. Existing passing
gate results were unaffected, and final focused checks passed after cleanup.

- [x] T07–T09 acceptance and focused checks pass.
- [x] Variable setup completes; production shortages/sevens/discards follow the pinned rules and rejected actions preserve state.
- [x] Run all applicable standing checkpoint commands in plan.md; record actual results and limitations.
- [x] Present checkpoint evidence for human review before proceeding.

**C03 evidence (10 November 2026):** [Retained checkpoint report](c03-checkpoint.md)
records verified slice commits, all 49 tests/five doctests, 98.98% line coverage,
final nonzero focused checks, root's refreshed advisory pass, manual source/
constraint audit, and fresh review approval after T09q. Evidence was presented
for handoff; the existing autonomous authorization permits Phase 4 to proceed.
The report explicitly retains unfinished T10 onward and unavailable mutation,
full-game, wire/save/replay, performance, Wasm and hosted-CI evidence.

## Phase 4: Robber and exact road construction

<a id="t10"></a>
### T10: Resolve robber movement and theft

- [x] T10 complete

**Refined execution (10 November 2026):**
- [x] T10a: Mechanically extract the 982-line existing command test modules into `src/engine_api/command_tests.rs`; preserve test bodies, names, provider exports and behavior. Independently review and verify before a separate commit.
**T10a evidence:** `private historical evidence,build,clippy,check,doc-tests,doc,coverage}.log` and `private historical evidence`: 49 tests, 5 doctests, all listed gates passed; library line coverage 98.64% (2389/2422). Reviewer verified production and all four test modules byte-for-byte against `ea1813d`; fully qualified test paths gain only the private `command_tests` parent. No rule or quality changes.

- [x] T10b: Implement both live robber commands together in `src/game_model/state.rs`, `src/base_rules/robber.rs` and its provider export, `src/turn_engine/robber.rs` and dispatcher, and `src/engine_api/command.rs`. These seven connected files are the smallest complete path: separating the chooser would advertise an unsupported mandatory decision. Update existing dice/discard query assertions in `src/engine_api/command_tests.rs` and `tests/game_sequences.rs` from the interim empty choices to exact now-supported robber destinations. Include focused private transition/RNG/privacy tests in `src/engine_api/robber_tests.rs`; retain literal offline sampler expectations.
**T10b evidence:** `private historical evidence` records absent-command/phase RED; focused `private historical evidence,br014,privacy}.log` ran 4/1/1 tests. `private historical evidence,check,tests,doc-tests,build,clippy,doc,coverage}.log`: 54 tests, 5 doctests, all listed gates pass, 98.81% library lines (2578/2609). `private historical evidence` approves the connected source/privacy path and records detected membership/privacy mutations in isolated fresh targets, with retained sources/logs and completed targets removed. Ten independent pinned sampler literals cover all card indices 0–6 and positions 0/37; no runtime `/tmp` fixture reads. New origin/continuation consumers remain Seven/Action; Knight/PreRoll arrive in T19. Existing dice/discard assertions explicitly cover newly supported movement choices.

- [x] T10c: Add focused public robber sequences in the existing `tests/game_sequences.rs`, without mechanical reshuffling. Verify all T10 acceptance and standing gates, retain evidence, then commit the sequence proof/status separately. Depends on T10b.


**Description:** Implement moveRobber and chooseRobberVictim, including the automatic zero/one-opponent path and weighted resource theft.

**Modules:** All four modules. **Dependencies:** [T09](#t09).

**Rule trace:** BR013, BR014, BR015.

**Acceptance criteria:**

- [x] The destination differs from the current hex, desert is legal, victims are distinct adjacent opponents including empty hands, and zero/one/multiple-victim flows resume the stored phase correctly.
- [x] A nonempty theft samples an actual card index in fixed resource order; an empty victim consumes no RNG and transfers nothing. Buildings/development cards are never stolen.
- [x] Public projection reveals only parties/count, with the resource type to thief and victim; invalid choices preserve position/RNG/revision.

**Verification:**

- [x] `cargo test --locked --test game_sequences robber_`
- [x] `cargo test --locked base_rules_br014_`
- [x] `cargo test --locked --lib privacy_theft_`

**Files likely touched:** `src/game_model/mod.rs`, `src/base_rules/mod.rs`, `src/turn_engine/mod.rs`, `src/engine_api/mod.rs`, `tests/game_sequences.rs`.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

**T10c evidence:** `private historical evidence,br014,privacy}.log` each runs one passing focused test. `private historical evidence,check,build,clippy,doc-tests,coverage}.log`: 55 tests, 5 doctests, all listed gates pass; coverage 98.85% (2579/2609). `private historical evidence` independently approves public zero/automatic/chosen traces and checks the standalone pinned-rand oracle at `private historical evidence,Cargo.lock,src/main.rs,results.log}`. Its literal first/next dice and card indices are retained directly in tracked tests. Its completed generated target was removed; source and output remain. No snapshots, Knight consumer or hosted CI is claimed.

<a id="t11"></a>
### T11: Calculate exact Longest Route from a board

- [x] T11 complete

**Refined execution (10 November 2026):**
- [x] T11a: Replace the at-most-two shortcut with the sole exact bounded edge-visited DFS in `src/base_rules/longest_road.rs`, exported from the provider root. Retain all six existing setup-route assertions and add the 15 independently witnessed graph fixtures. Wire bounded cached lengths into `src/game_model/state.rs`, real fixed/variable creation and initial validation (`src/turn_engine/creation.rs`), setup-road transitions (`src/turn_engine/setup.rs`) and the existing `Engine::view` field. Update the economy Position literal and reset synthetic fixture caches in `src/engine_api/{command_tests,robber_tests}.rs`. These connected caller changes exceed five files and cannot be replaced by disconnected scaffolding. Add the public graph/witness conformance proof. Independently Sol/high review algorithm/fixtures and verify a mutation before commit.
**T11a evidence:** `private historical evidence` fails the old shortcut on the independent five-edge fixture. `private historical evidence,cache,witness}.log` run 1/2/1 focused tests; full `{fmt,check,tests,build,clippy,doc-tests,doc,coverage}.log` record 59 tests, 5 doctests and all listed gates passing, 98.88% library lines (2736/2767). Fresh Sol/high `private historical evidence` independently reruns all 15 witnesses/16 award inputs/two invalid controls, reproduces the frozen JSON, and detects an isolated fresh-target blocker-stop mutation (5 versus required 3). Original/mutant source and logs remain; completed target removed. Views read four bounded cached bytes; real creation validates them, setup roads refresh only their owner. Setup settlements cannot block an existing paired road because distance excludes vertices adjacent to its owned settlement. Test-only thread-local search counters verify zero unrelated roll/end/view/query searches; no production global state or benchmark claim.

- [x] T11b: Reconcile the actual incumbent and qualifying maxima through real creation/setup/view consumers, bounded position holder, shared scoring and projected award event. Independently assert all 16 frozen award steps and exact view/score facts; keep unrelated views/commands free of route searches. Verify and separately commit; no separate public rule-query API.


**Description:** Implement the exact road-length provider and its existing Engine::view fields, using edge-visited traversal. Independently test award reconciliation before construction depends on it; add no separate public rule-query API.

**Modules:** game-model → base-rules → engine-api. **Dependencies:** [T03](#t03), [T10](#t10).

**Rule trace:** BR033, BR034, BR035.

**Acceptance criteria:**

- [x] The calculator counts trails with each edge once, including cycles/branches/tails/touching loops and length 15; entering an opponent building ends traversal but starting there may leave it.
- [x] Threshold 5, incumbent ties, unique maximum takeover and award-to-supply cases have independently asserted expected paths/results.
- [x] Queries do not alter state/RNG; calculator/award helpers are shared with later command execution and make no approximation.

**Verification:**

- [x] `cargo test --locked base_rules_br034_`
- [x] `cargo test --locked base_rules_br035_`
- [x] `cargo test --locked --test rules_conformance longest_route_`

**Files likely touched:** `src/base_rules/mod.rs`, `src/base_rules/longest_road.rs`, `src/engine_api/mod.rs`, `tests/rules_conformance.rs`.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

**T11b evidence:** `private historical evidence` records missing-award-interface RED; `{green,cache,br034,witness}.log` run 1/4/1/1 focused tests. `{fmt,check,tests,doc-tests,build,clippy,doc,coverage}.log` pass 62 tests, 5 doctests and all listed gates, with 98.95% library line coverage. `private historical evidence` independently verifies all 16 frozen award steps and shared holder/score/projection consumers, and kills incumbent-tie retention removal at the literal tie-at-five step. Original/mutant source and logs remain in its recorded control directory; completed target removed. Actual nine connected source files include holder and event in state, exact-provider reconciliation/export and score, economy Position fixture, creation validation, shared turn refresh/setup caller, and API view/event projection. No rule-query API or hidden-card score claim added; T16 extends the same current score/victory helpers.

<a id="t12"></a>
### T12: Build a paid road with atomic award updates

- [x] T12 complete

**Refined execution (10 November 2026):**
- [x] T12a: Complete the live paid-road path through `src/game_model/state.rs`, `src/base_rules/building.rs`/provider export, `src/turn_engine/building.rs`/dispatcher and `src/engine_api/command.rs`. The six connected production files share placement legality with candidate enumeration, checked cost/bank transfers, exact route refresh/reconciliation and the existing shared score/finish hook in one atomic transition. Include focused base-rule and `src/engine_api/road_tests.rs` tests for every rejection, blockers, search invalidation, payment/event order and a valid bounded engineered 8→10 win. This is the smallest complete supported path; no deferred award/finish step or public preview API. Independently review/mutate and run standing gates before its commit.
**T12a evidence:** `private historical evidence` records the missing-command RED; focused road and BR017 tests pass. `private historical evidence,tests,check,clippy,build,doc-tests,doc,coverage}.log` record 68 tests, 5 doctests, all standing gates passing and 99.00% library line coverage (3061/3092). Placement/candidate consumers share one validator; empty shared vertices remain usable beside opponent roads, either unblocked owned connection suffices, and exhausted pieces, occupied edges, costs, actor/phase/counter failures preserve state/RNG/revision. Checked payment precedes placement, exact owner refresh, award and the existing active-score victory hook in one transition. The valid distance-bounded engineered 8→10 fixture asserts finish before another action. `private historical evidence` independently approves and records a caught isolated victory-call removal; source/log remain in its control directory and the completed target was removed. Public paid/award sequences remain T12b; no hidden-card or complete-game history claim.

- [x] T12b: Add literal public paid-road/candidate/award sequences in `tests/game_sequences/road.rs`, referenced by a small module declaration from the existing integration root to preserve the approved focused filter, retaining independent graph/production facts. Run all nonzero T12 focused checks, standing gates and independent review; record truthful T12 completion and commit separately. Public 10-point history awaits later point sources in T21; T12a still proves the actual paid transition invokes the same active-score finish hook now.


**Description:** Add buildRoad through the existing execute/validate/query/view pipeline, paying its cost and reconciling Longest Route inside the same transition.

**Modules:** All four modules. **Dependencies:** [T11](#t11).

**Rule trace:** BR016, BR017, BR020, BR033, BR035, BR037, BR038.

**Acceptance criteria:**

- [x] Empty edge/connectivity/opponent-blocker/piece/affordability rules pass; brick/wood return to the bank and no failure spends resources or a piece.
- [x] Affected road length and award ownership update immediately; the shared true-score/victory hook runs before another decision and finishes if appropriate.
- [x] Legal candidates use the execution validator, events describe payment/road/award in order, and unrelated dice/trades do not trigger road searches.

**Verification:**

- [x] `cargo test --locked --test game_sequences road_`
- [x] `cargo test --locked base_rules_br017_`
- [x] `cargo check --locked --all-targets`

**Files likely touched:** `src/game_model/mod.rs`, `src/base_rules/mod.rs`, `src/turn_engine/mod.rs`, `src/engine_api/mod.rs`, `tests/game_sequences.rs`.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

**T12b evidence:** `tests/game_sequences/road.rs` is wired by only a small declaration in the integration root. The standalone pinned rand/chacha source/results and independent endpoint enumeration are retained in `private historical evidence`; its completed generated target was removed. Seed15508 (u32 little-endian) gives five accepted public turns, exact source-produced hands/bank, three payments on E45/E44/E51, candidate agreement for all 72 edges at each step, route lengths 2→3→5 and ordered award acquisition at revision 12. Every seat/spectator sees the public payment/placement/award facts; occupied attempts preserve all views and the exact next dice [1,4] proves the intervening nonrandom actions/queries leave the stream unchanged. The witness V31–V36–V30–V35–V40–V44 uses every owned edge once. Final `private historical evidence,check,clippy,build,tests,doc-tests,doc,coverage}.log` pass 69 tests, 5 doctests and 99.00% library lines (3061/3092). All T10–T12 focused filters run nonzero and pass. The ordinary advisory call failed on a read-only database lock; the authorized refreshed `private historical evidence` exits zero with `advisories ok`. `private historical evidence` independently approves the source/production/RNG/candidate/witness arithmetic and C04 contracts. Public 10-point histories remain T21; T12a already proves the live active-score finish hook with a valid bounded engineered fixture. No full-game, benchmark, wire/replay, Wasm or hosted-CI claim.

<a id="c04"></a>
### C04: Robber and exact road rules

- [x] T10–T12 acceptance and focused checks pass.
- [x] Weighted theft/privacy, independent road graph fixtures, paid construction and atomic award updates pass.
- [x] Run all applicable standing checkpoint commands in plan.md; record actual results and limitations.
- [x] Present checkpoint evidence for human review before proceeding.

**C04 evidence (10 November 2026):** [Retained checkpoint report](c04-checkpoint.md) records seven verified slice commits through `65d2d23`, 69 tests/five doctests, 99.00% library line coverage, all nonzero focused filters, refreshed advisory pass, independent source/RNG/route witnesses and detected privacy/block/tie/victory mutations. Final review approved with no required findings; manual source/constraint audit confirms the provider direction, shared validators, bounded caches and unchanged quality bar. Evidence is presented under the existing autonomous authorization. T13 onward remains unchecked, and this Phase 4 worker stops at C04.

## Phase 5: Settlement, city and bank trades

<a id="t13"></a>
### T13: Build a settlement that can interrupt opponents

- [x] T13 complete

**Refined execution (10 November 2026):**
- [x] T13a: Complete the native paid-settlement path in `src/game_model/state.rs`, the existing `src/base_rules/building.rs` and provider export, `src/turn_engine/building.rs` and dispatcher, and `src/engine_api/command.rs`. These six connected source files include command/candidate declarations, shared distance/connection/cost/stock validation, checked payment, placement, affected-opponent route refresh and active victory in one atomic transition. Add focused private transition tests in `src/engine_api/settlement_tests.rs`, retaining nonzero BR018/BR035 filters. Reuse existing payment/settlement events, score and exact route helpers. Independently review/control and run standing gates before a separate commit.
**T13a evidence:** `private historical evidence` records the absent-command RED; final focused settlement/BR018/BR035 filters run 11/1/2 passing tests. `{fmt,check,tests,doc-tests,clippy,build,doc,coverage}.log` record 76 tests, 5 doctests, all gates passing and 98.96% library lines (3141/3174). The ordinary advisory call failed on the read-only database lock; the authorized refreshed scan passes `advisories ok`. Shared distance/cost/payment validation supplies the live candidate/command path. Only a non-builder with two incident roads is searched; payment/placement/award/active victory commit at one revision with unchanged RNG. Independent subset-DP fixtures verify 7→10 active transfer, off-turn 8→10 waiting until its own turn, and no impossible simultaneous opponent interruption. Fresh Sol/high `private historical evidence` approves all connected paths, independently reruns 76 tests/five doctests and the 15 frozen + four new graph fixtures, and kills a >=2→>2 blocker-selection mutation in three tests; restored source passes. Retained isolated source/logs remain, completed target removed. Public sequences remain T13b; engineered city/score fixtures are not complete public histories.

- [x] T13b: Add source-backed public settlement/payment/candidate/blocking sequences in `tests/game_sequences/settlement.rs`, referenced from the integration root. Preserve literal independent endpoint/production facts, old receipt projection and unchanged next RNG. Verify focused/standing checks, independently review and commit completion separately. At this point cities/held cards are unavailable, so public ten-point histories remain later tasks; a valid bounded engineered fixture proves the live finish hook.

**Description:** Add buildSettlement through the native command path and reconcile road awards affected by the new blocking vertex.

**Modules:** All four modules. **Dependencies:** [T12](#t12).

**Rule trace:** BR016, BR018, BR020, BR033, BR035, BR037, BR038.

**Acceptance criteria:**

- [x] Distance, owned-road connection, coast/port sites, free stock and exact four-resource payment hold; no voluntary removal/movement/refund path is introduced.
- [x] Only opponents whose roads can be blocked need new lengths; award transfer/loss and immediate active-player victory happen atomically.
- [x] Queries and projected events agree with execution, and insufficient/occupied/disconnected attempts preserve the original state.

**Verification:**

- [x] `cargo test --locked --test game_sequences settlement_`
- [x] `cargo test --locked base_rules_br018_`
- [x] `cargo test --locked base_rules_br035_`

**Files likely touched:** `src/game_model/mod.rs`, `src/base_rules/mod.rs`, `src/turn_engine/mod.rs`, `src/engine_api/mod.rs`, `tests/game_sequences.rs`.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

**T13b evidence:** `private historical evidence` records the missing public test module; final `--test game_sequences settlement_` runs two passing public histories. Seed29573 independently funds Blue roads E47/E53 and coastal brick-port settlement V42; seed36673 funds Blue's five-edge trail and Orange E29/E36/V30, reducing Blue's length 5→3 and returning its award. Literal source-backed hands/bank, all-54 candidate agreement, cost/placement/award ordering for every viewer, old-receipt facts and next dice [3,6]/[6,1] pass. Full `{fmt,check,tests,doc-tests,clippy,build,doc,coverage}.log` pass 78 tests/five doctests and 98.96% lines (3141/3174); the refreshed T13a advisory verdict remains applicable to unchanged dependencies. Fresh Terra/high `private historical evidence` independently verifies graph, production, payments, all candidates and standalone pinned RNG, approves, and catches an isolated blocker-threshold mutation in the public award test. Standalone oracle and original/mutant source/logs remain under `private historical evidence` and the review paths; completed review targets removed. T13 is complete; public full-game/ten-point histories remain later tasks.

<a id="t14"></a>
### T14: Upgrade a settlement to a city

- [x] T14 complete

**Refined execution (10 November 2026):**
- [x] T14a: Add owned-city upgrade to the existing atomic construction path in `src/game_model/state.rs`, `src/base_rules/building.rs` and export, `src/turn_engine/building.rs` and dispatcher, and `src/engine_api/command.rs`. These six connected source files carry the live command, stable rejection, candidate and original-fact event projection; occupancy remains the sole stock representation, costs use checked Resources, and the shared score/finish helper runs without route invalidation. Focused `src/engine_api/city_tests.rs` covers exact payment, all target/cost/stock/context rejections, returned fifth settlement stock used by a later paid command and an independently traced 9→10 upgrade. Verify standing gates and fresh review/control before commit.
**T14a evidence:** Missing UpgradeCity/CityUpgraded declarations produce RED in `private historical evidence`; final city/BR019 filters run 8/1 tests. `{fmt,check,tests,doc-tests,clippy,build,doc,coverage}.log` pass 82 tests/five doctests and 98.98% library lines (3201/3234). Owned settlement, stock and exact wheat/ore payment share one validator with all-54 candidate enumeration. The checked construction transition replaces the kind once, derives returned stock from occupancy, projects original public facts and checks immediate victory without route searches. A five-settlement fixture uses the returned piece in a subsequent BuildSettlement; the independent eight-edge fixture wins 9→10 by upgrading V48. Fresh Sol/high `private historical evidence` approves, independently checks graph/distance/resource/stock/score arithmetic and 82 tests/five docs, and kills separate missing-victory and reduced-ore-cost mutations; restored source passes. Source/logs remain in the recorded control paths; completed target removed. T13's refreshed unchanged-dependency advisory verdict remains applicable. Public next-production proof remains T14b; isolated threshold positions are engineered, not public full-game histories.

- [x] T14b: Add the independent fixed-game upgrade/next-production sequence in `tests/game_sequences/city.rs` and a small root declaration, proving public stock/score changes and two-card production using the preserved standalone pinned RNG oracle. Verify nonzero city/BR019 filters, full gates and independent review before completion/commit; complete four-game victory histories remain T21.

**Description:** Add upgradeCity with returned settlement stock, correct scoring/production, and the shared immediate-win hook.

**Modules:** All four modules. **Dependencies:** [T13](#t13).

**Rule trace:** BR010, BR016, BR019, BR020, BR033, BR037, BR038.

**Acceptance criteria:**

- [x] Only an owned settlement can be upgraded; the city consumes three ore/two wheat and one city piece while returning the settlement piece.
- [x] The replacement counts as one two-point building and produces two cards; a later settlement may use its returned piece; it introduces no new road blocker.
- [x] Insufficient resources/pieces, foreign or already-city targets reject atomically, and a winning upgrade freezes play immediately.

**Verification:**

- [x] `cargo test --locked --test game_sequences city_`
- [x] `cargo test --locked base_rules_br019_`
- [x] `cargo check --locked --all-targets`

**Files likely touched:** `src/game_model/mod.rs`, `src/base_rules/mod.rs`, `src/turn_engine/mod.rs`, `src/engine_api/mod.rs`, `tests/game_sequences.rs`.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

**T14b evidence:** Missing public module RED and the passing `--test game_sequences city_` are `private historical evidence,green}.log`. Independent seed8433 supplies four accepted fixed-game rolls, Blue hand [1,0,0,2,3] and bank [16,14,16,15,15]. Public UpgradeCity40 changes score 2→3, remaining settlement pieces 3→4 and city pieces 4→3; the next Red roll [2,2] pays Blue two ore, Red/Orange one wood each, with literal bank/hand and old-receipt projections. All-54 target agreement uses the shared validator. Full `{fmt,check,tests,doc-tests,clippy,build,doc,coverage}.log` pass 83 tests/five doctests and 98.98% lines (3201/3234); prior refreshed advisory verdict applies to unchanged dependencies. Fresh Terra/high `private historical evidence` independently reruns the public/BR019 test and pinned RNG oracle, verifies source incidence and resource/stock arithmetic, approves and kills a production-only city multiplier mutation. Retained oracle/control source/logs remain; completed targets removed. T14 is complete; complete reachable ten-point game histories and card points remain later tasks.

<a id="t15"></a>
### T15: Trade with the bank using the best port

- [x] T15 complete

**Refined execution (10 November 2026):**
- [x] T15a: Complete the bank-trade path in `src/game_model/state.rs`, `src/base_rules/trade.rs` and provider export, `src/turn_engine/trade.rs` and dispatcher, and `src/engine_api/command.rs`. These six connected source files include command/error/event/candidate declarations and one pure best-port/checked-exchange calculation. The shared internal validator returns the small prepared exchange only for BankTrade; native validate/query discard it and execution consumes it once without repeated port scans, state caches, extra RNG or a second validation path. Focused `src/engine_api/bank_trade_tests.rs` covers every port endpoint/kind, best-rate fallback, all resource pairs, immediate new port/city access, bank/hand conservation, original public receipt and atomic rejection. Adapt any existing query assertions only for now-supported real pairs. Run standing gates and independent review/control before commit.
**T15a evidence:** Missing BankTrade/event/choice declarations produce `private historical evidence`; final bank/BR023/BR024 filters run 4/1/1 passing tests. `{fmt,check,tests,doc-tests,clippy,build,doc,coverage}.log` pass 87 tests/five doctests and 99.02% library lines (3342/3375). The private BankExchange result is calculated once by `validate_action`; its two callers are `Engine::validate` (discard prepared success) and `evaluate` (consume the explicit Some for BankTrade). Checked hand/bank values and public rate form one atomic no-RNG receipt. All nine source ports, both endpoints/building kinds, default/special/generic fallback, non-stacking and foreign buildings pass. All 20 different pairs agree with native queries/execution; same-kind/unaffordable/empty-bank/context/stale/counter failures preserve complete state/RNG/revision. New settlement and actual city upgrade retain immediate 2:1 access; unrelated hand variation leaves the querying seat's pairs unchanged. Search-counter tests reset after fixture creation, proving queries and exchanges themselves search zero routes. Fresh Terra/high `private historical evidence` approves source/atomicity/provider simplicity and kills an inverted port-owner mutant; restored sources/logs remain, completed target removed. No legacy query assertion required modification. Prior refreshed advisory verdict applies to unchanged dependencies. Public all-three-rate histories remain T15b.

- [x] T15b: Add literal public default 4:1, newly built special 2:1 and generic 3:1 exchange histories in `tests/game_sequences/bank_trade.rs` with a small root declaration. Preserve standalone pinned RNG/source graph/production witnesses, exact deltas and next dice. Verify nonzero bank/BR023/BR024 filters and standing gates, independently review, then commit completion and retain C05 checkpoint evidence.

**Description:** Add bankTrade and its bounded legal resource-pair query, deriving the correct rate rather than trusting a client-supplied quantity.

**Modules:** All four modules. **Dependencies:** [T14](#t14).

**Rule trace:** BR004, BR023, BR024.

**Acceptance criteria:**

- [x] One receive card costs the best applicable 4:1/3:1/2:1 rate for that give resource; rates do not stack, receive differs, and cities retain port access.
- [x] A newly built port applies immediately and an absent bank card rejects; inactive seats cannot make maritime trades.
- [x] Payments/transfers conserve each resource, failures change nothing, and the view/candidates expose only the caller's relevant affordability.

**Verification:**

- [x] `cargo test --locked --test game_sequences bank_trade_`
- [x] `cargo test --locked base_rules_br023_`
- [x] `cargo test --locked base_rules_br024_`

**Files likely touched:** `src/game_model/mod.rs`, `src/base_rules/mod.rs`, `src/turn_engine/mod.rs`, `src/engine_api/mod.rs`, `tests/game_sequences.rs`.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

**T15b evidence:** `private historical evidence,green}.log` record missing public module RED and three passing native histories. Seed50456 gives both an ordinary 4:1 exchange and a separate accepted E47/E53/V42 settlement path whose port immediately permits 2:1; seed27147 gives accepted E13/E7/E1/E0/V3 construction and generic 3:1. Literal source-backed resources/bank, all resource-pair agreement, public original receipts and exact next dice [2,6]/[3,5] pass. Final `private historical evidence,check,tests,doc-tests,clippy,build,doc,coverage}.log` pass 90 tests/five doctests and 99.02% library lines (3342/3375); authorized refreshed advisory scan passes `advisories ok`. All final T13–T15 focused filters run nonzero. Fresh Terra/high `private historical evidence` independently verifies graph/distance/port/production/payment arithmetic, reruns both pinned RNG oracles and three public tests, approves C05 import/unchanged-contract evidence, and catches a 2→3 special-rate mutation at the public candidate assertion. Retained standalone/mutant source/logs remain; completed review targets removed. T15 is complete; full game/card/wire/save/replay/performance evidence remains later work.

<a id="c05"></a>
### C05: Construction and maritime economy

- [x] T13–T15 acceptance and focused checks pass.
- [x] Blocking settlements, returned city stocks, immediate scoring and 4:1/3:1/2:1 trades work through public commands.
- [x] Run all applicable standing checkpoint commands in plan.md; record actual results and limitations.
- [x] Present checkpoint evidence for human review before proceeding.

**C05 evidence (10 November 2026):** [Retained checkpoint report](c05-checkpoint.md) records six independently reviewed verified slice commits through `c39bc91`, 90 tests/five doctests, all build/format/lint/docs gates, 99.02% library lines (3342/3375), refreshed advisory pass and nonzero final T13–T15 focused filters. Public paths prove settlement blocking/award return, coastal placement, city stock/score/two-card production and all three maritime rates; valid engineered positions prove active/off-turn immediate victory timing. Independent graph/source/RNG/arithmetic and targeted mutation reviews approve the unchanged four-module/quality/privacy contract. Evidence is presented for clean handoff; existing autonomous authorization permits Phase 6 to proceed. Cards, complete public setup-to-win histories, wire/save/replay, performance/memory, Wasm and hosted CI remain explicitly unfinished; nothing was published.

## Phase 6: Card purchases and resource effects

<a id="t16"></a>
### T16: Buy development cards without leaking their identity

- [x] T16 complete

**Refined execution (10 November 2026):**
- [x] T16a: Deliver the complete native purchase/ownership/privacy/hidden-VP/finish path. Necessarily connected files are `game_model/state.rs` and its export, `turn_engine/creation.rs`, the existing economy-test initializer, `base_rules/development.rs` and shared scoring/export, `turn_engine/mod.rs`, the existing paid-construction executor and finish hook, API command/view/export, `lib.rs`, focused `engine_api/development_tests.rs`, and this evidence. Keep one shared payment/validation/score path; preserve the bank exchange's prepared result and pinned shuffle/RNG. Fixed-size card locations/cursor carry individual acquisition turns. Winner-only VP reveal precedes the existing terminal event; final scores stay in host-private position/receipt facts. This is one inseparable truthful consumer slice despite the cross-module file count. Verify RED/GREEN, full standing gates and fresh independent privacy/score review before its explicit-file commit.
- [x] T16b: Add independent seeded public purchase/owner-view/original-receipt histories in `tests/game_sequences/development.rs` and a small root declaration. Assert literal draws, exact payment/bank, multiple purchases, old/new same-kind acquisition turns and unchanged next dice. Verify nonzero filters, standing gates and independent review before completion/commit. Complete public ten-point histories remain T21/T29.

**T16a evidence:** `private historical evidence` records the missing purchase/ownership/view/finish contracts; final focused purchase filter passes three tests, BR025 two, and privacy three. Six new focused tests assert exact 25-ID draw order, cost/bank/cursor, every private rejection fact, same-kind individual timing, all four seats/spectator and original purchase receipts, newly bought 9→10 VP and multiple-VP turn-start victory. The existing score/finish/payment paths are shared, route searches and RNG remain unchanged, and initial creation rejects nonzero cursor/held-card/final-score corruption. Full standing `{fmt,check,tests,doc-tests,clippy,build,doc,coverage}.log` plus final fmt/lint/privacy logs pass 96 tests/five doctests and 99.06% library lines (3466/3499). The initial Clippy nested-test-if finding was corrected without changing assertions; its failed log is retained. Ordinary advisory scan failed on the read-only database lock; authorized refreshed `private historical evidence` passed `advisories ok`. Fresh Sol/high `private historical evidence` approves with no required findings, independently passes 37 command/16 creation tests and all-target compile, reproduces all seed21 IDs/starter/next dice using pinned rand/chacha alone, and catches an isolated purchase-visibility mutation. Original/mutant/restored sources and logs remain in `private historical evidence`; reviewer-generated target was removed. Optional stale view wording is refreshed. Public seeded purchase histories remain T16b; actual card-play eligibility/allowance/spent transitions remain T17 onward and no unsupported commands are exposed.

**T16b evidence:** `private historical evidence` records the missing public module; final `--test game_sequences development_purchase_` passes two independent fixed-game histories. Seed20498 (little-endian first four bytes, remaining zero) independently yields VP24, Knight5 and Knight4 in both three/four-seat games; public turns7/7/10 and5/5/9 prove multiple purchases and distinct older/newer same-kind cards. Literal starting/next dice, exact cost/hand/bank, every seat/spectator owner allowlist, public counts/hidden score, stale rejection and original receipts after later turns all pass, with next dice [2,2]. No test reads temporary inputs. Standalone pinned rand/chacha and literal CN3081 production inputs remain under `private historical evidence`; its completed generated target was removed. Final standing `{fmt,clippy,check,tests,doc-tests,build,doc,coverage}-final.log` files pass 98 tests/five doctests and 99.06% library lines (3466/3499). The authorized T16a refreshed advisory pass applies to the unchanged resolved dependencies. Fresh Terra/high `private historical evidence` approves after correcting a required test-only viewer-identity guard gap: spectator own view must be absent and every seat's own view must identify exactly that seat. The reviewer independently reconstructs source geometry/production, reruns the pinned oracle, and proves wrong-cost, spectator-private-view and wrong-own-seat mutations each fail both public tests; restored originals pass. Source/control/oracle/log evidence remains in `private historical evidence`, with generated review targets removed. T16 is complete; actual card-play eligibility/allowance/spent effects, full public winning histories, wire/save/replay, and performance remain later approved tasks.

**Description:** Add buyDevelopmentCard using the already shuffled deck, individual acquisition turns, own-card views and immediate hidden-VP scoring.

**Modules:** All four modules. **Dependencies:** [T15](#t15).

**Rule trace:** BR016, BR025, BR026, BR031, BR032, BR033, BR037, BR038.

**Acceptance criteria:**

- [x] Wool/wheat/ore payment, deck cursor and unique card location update once; exhausted/unaffordable purchases reject without consuming stock or randomness.
- [x] The owner sees card ID/kind/acquisition eligibility; everyone else sees buyer/count only. Buying multiple cards is legal and old/new same-kind cards remain distinguishable.
- [x] A bought VP counts immediately and may finish the active turn; only winner VP cards reveal. Deck/hand/spent locations conserve all 25 cards.

**Verification:**

- [x] `cargo test --locked --test game_sequences development_purchase_`
- [x] `cargo test --locked base_rules_br025_`
- [x] `cargo test --locked --lib privacy_purchase_`

**Files likely touched:** `src/game_model/mod.rs`, `src/base_rules/mod.rs`, `src/turn_engine/mod.rs`, `src/engine_api/mod.rs`, `tests/game_sequences.rs`.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

<a id="t17"></a>
### T17: Play Monopoly through shared card eligibility

- [x] T17 complete

**Refined execution (10 November 2026):**
- [x] T17a: Complete the connected native Monopoly path across the model, creation/test initializers, shared base-rule eligibility/effect, turn validation/execution/reset, and safe API choices/views/original receipts. Focused internal tests cover ownership, per-ID acquisition timing, every context and allowance rejection, exact conserved transfer, public spent cards, all viewers and RNG/revision atomicity. Preserve the prepared bank exchange and existing score/victory paths. RED, standing gates and fresh independent privacy review precede the explicit-file commit.
- [x] T17b: Add independently derived, seeded public purchase-to-Monopoly histories and a focused integration module; assert literal card and transfers, old purchase and play receipts, validator/choices agreement, no RNG draw and the next dice. Run nonzero focused filters, standing gates and independent review before completion and commit.

**T17a evidence:** `private historical evidence` records missing-play RED; the BR026/BR028 focused filters pass five tests across old/new individual cards, phase, actor, turn allowance, exact transfers for five resources including zero, all-viewer spent ownership and original receipt projection. Full `private historical evidence,fmt-verified,clippy-verified,coverage-verified}.log`, plus `{check-final,build-final,doc-final,doc-tests-final}.log`, pass 103 tests/five doctests and 99.06% executable library lines (3577/3611). The advisory scan passed with local database-lock access; the sandbox-only lock failure is retained at `private historical evidence`. Fresh Sol review `private historical evidence` approves the native path, independently tests 103/five docs, and verifies isolated allowance-guard, spent-owner and hidden-card-leak mutations fail after strengthening the tests; retained sources/logs are under `private historical evidence`. Existing bank-trade preparation, scoring and RNG paths are preserved. Public reachable purchase-to-play histories are T17b.

**Description:** Implement the shared non-VP card eligibility path with playMonopoly as the first complete card effect.

**Modules:** All four modules. **Dependencies:** [T16](#t16).

**Rule trace:** BR026, BR028, BR031.

**Acceptance criteria:**

- [x] Only an owned earlier-turn card can play in PreRoll/Action; one shared allowance covers both phases, and new/foreign/spent cards reject safely.
- [x] The chosen resource moves in full from every opponent to the player, including a valid zero-result effect; bank stock remains unchanged.
- [x] The card is spent once, projected effect facts are public, the phase resumes unchanged, and invalid play does not consume the allowance.

**Verification:**

- [x] `cargo test --locked --test game_sequences monopoly_`
- [x] `cargo test --locked base_rules_br026_`
- [x] `cargo test --locked base_rules_br028_`

**Files likely touched:** `src/game_model/mod.rs`, `src/base_rules/mod.rs`, `src/turn_engine/mod.rs`, `src/engine_api/mod.rs`, `tests/game_sequences.rs`.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

**T17b evidence:** `private historical evidence,green-verified}.log` record missing-module RED and passing seeded public histories in both phases for 3/4 players. Independent pinned-rand/source oracle `private historical evidence,results.log}` derives fixed setup, full preceding hands/bank, ID16 purchase and literal Monopoly transfers of 4 and 6, plus next two dice; a separate T16 snapshot witnessed every prerequisite command. All viewers receive exact original purchase/play receipt revisions and ordered allowlisted events. Final `private historical evidence,fmt-verified,clippy-verified,coverage-verified}.log` pass 104 tests and 99.06% library lines (3577/3611); `{check,build,doc,doc-tests}.log` pass, with five doctests. BR026/BR028 focused logs run three/two tests; T17a's refreshed advisory verdict applies to unchanged dependencies. Fresh Terra review `private historical evidence` independently reruns oracle and source claims, catches a reduced-transfer mutation, and approves strengthened complete old-receipt assertions; retained control evidence remains in `private historical evidence`. T17 is complete; Invention and other card effects remain subsequent tasks.

<a id="t18"></a>
### T18: Play Invention with the approved bank-exhaustion policy

- [x] T18 complete

**Refined execution (10 November 2026):**
- [x] T18a: Complete the native Invention effect through the existing individual card eligibility, allowance, checked resources and original-fact projection. All four providers and the focused effect tests are necessarily connected; query only eligible card IDs using one available representative bundle, never enumerate all bundles. Verify exact two/same/mixed, short/one/empty bank, invalid choices, turn/phase/ownership and full rejection/RNG/revision conservation across viewers. Record RED, full gates and fresh independent review before the explicit-file commit.
- [x] T18b: Add independently derived seeded public purchase-to-Invention histories in both phases, with literal selections, bank/hands, original receipts and subsequent dice. Run nonzero focused filters, standing gates and independent review before completion/commit. C06 then records both complete tasks and checkpoint results.

**T18a evidence:** `private historical evidence,br029-final,cross}.log` record RED and focused GREEN for exact same/mixed two, exact-two exhaustion, sole-card, empty/short bank, generic card rejection and cross-kind allowance. `private historical evidence,fmt-final,clippy-final,coverage-final}.log` and `{check,build,doc,doc-tests}.log` pass 108 tests/five doctests and 99.08% library lines (3668/3702); T17a's refreshed advisory verdict applies to unchanged dependencies. Fresh Terra review `private historical evidence` approves and reports an independent 118,098-case small-bank oracle across both phases plus seven killed mutations for sole-card, shortage, empty bank, allowance, ownership and privacy. All-viewer original receipts and per-seat spent ownership are asserted, including after turn advance. Retained oracle/mutant evidence is in `private historical evidence`; public reachable histories are T18b.

**Description:** Add playInvention as one atomic choice/effect, reusing card eligibility and the resolved RQ2 policy.

**Modules:** All four modules. **Dependencies:** [T17](#t17).

**Rule trace:** BR026, BR029, BR031.

**Acceptance criteria:**

- [x] Take exactly two available cards, including two of one type, when total bank stock is at least two; take the sole card when total stock is one.
- [x] Empty-bank activation or an invalid selected bundle rejects without consuming the card, allowance, revision or resources.
- [x] The resolved effect/choice is public, the original PreRoll/Action phase resumes, and conservation holds at exact/short/zero stock.

**Verification:**

- [x] `cargo test --locked --test game_sequences invention_`
- [x] `cargo test --locked base_rules_br029_`
- [x] `cargo check --locked --all-targets`

**Files likely touched:** `src/game_model/mod.rs`, `src/base_rules/mod.rs`, `src/turn_engine/mod.rs`, `src/engine_api/mod.rs`, `tests/game_sequences.rs`.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

**T18b evidence:** `private historical evidence,green}.log` record missing-module RED and one focused integration test exercising eight public histories (3/4 seats, PreRoll/Action, same-kind/mixed). Seed8 independent pinned-rand/source oracle `private historical evidence,results.log}` provides literal deck, turn histories, exact hands/bank, both choices and next dice; a separately archived T16 snapshot verified every prerequisite public command. Tests compare complete original purchase and play receipt allowlists/revisions for each seat/spectator. Full `private historical evidence,check,tests,doc-tests,clippy,build,doc,coverage}.log` pass 109 tests/five doctests and 99.08% lines (3668/3702); BR029 focused filter runs three tests and C06 refreshed advisories pass. Fresh Terra review `private historical evidence` reruns oracle, witness and source cross-check, approves and kills same-kind and mixed-credit mutations; retained evidence is `private historical evidence`. T18 is complete.

<a id="c06"></a>
### C06: Card ownership and resource effects

- [x] T16–T18 acceptance and focused checks pass.
- [x] Private purchases, acquisition timing, Monopoly and the approved Invention exhaustion cases pass.
- [x] Run all applicable standing checkpoint commands in plan.md; record actual results and limitations.
- [x] Present checkpoint evidence for human review before proceeding.

**C06 evidence (10 November 2026):** [Retained checkpoint report](c06-checkpoint.md) records all six reviewed card slice commits through `0dcfbb5`, 109 tests/five doctests, 99.08% library line coverage (3668/3702), all standing build/lint/docs checks, freshly passing advisory scan and nonzero T16–T18 focused filters. Independent source/RNG, small-bank oracle, privacy and targeted mutation reviews approve; `private historical evidence` independently approves checkpoint evidence and the unchanged quality contract. Existing autonomous authorization permits Phase 7 to proceed. Knight, Road Building, complete-game histories, wire/save/replay, performance/memory, Wasm and hosted CI remain unfinished; nothing was published.

## Phase 7: Remaining cards and victory timing

<a id="t19"></a>
### T19: Play Knight with Largest Army and robber continuation

- [x] T19 complete

- [x] T19a: Complete native Knight reveal/played identity, Army reconciliation/public score and PreRoll/Action robber continuation using existing eligibility, theft and finish paths. Scope necessarily includes model, creation/invariant fixtures, shared rules/export, turn dispatcher/development/robber, API commands/views and focused tests. RED/GREEN, standing gates and fresh review/control precede an explicit-file commit.
- [x] T19b: Retain independent seeded public purchase/Knight/seven/Army histories with source/RNG literals; run nonzero focused tests, standing gates and fresh review before completion/commit.

**T19a evidence:** `private historical evidence` records missing Knight/Army/continuation contracts. The final `--lib knight_` filter passes six tests and `base_rules_br036_` one. Distinct `PlayedKnight` locations preserve progress-only spent views and the drawn-prefix invariant; played counts derive from those locations. Shared per-ID eligibility/allowance, actual first-third acquisition, native equal-third retention/fourth transfer, both origins through zero/automatic/chosen/empty victims, public original facts, all viewer identities, setup/discard/robber/stale/counter rejection and a genuine pre-roll Knight followed by a seven's fixed discard are asserted. A distance-valid eight-point position wins on the third reveal before destination/theft with unchanged RNG; loser VP remains hidden. `{fmt,check,tests,doc-tests,build,clippy,doc,coverage}.log` pass 115 tests (85 library/18 sequences/5 model/7 rules), five doctests and 99.02% library lines (3749/3786). The ordinary advisory lock failure is retained; authorized refreshed `private historical evidence` passes. Fresh Sol/high `private historical evidence` approves all connected imports/fields/tests and independently verifies fixture graph/resource/card facts and 115 tests; isolated `>=` Army transfer and omitted Knight victory-check mutations fail native tie/terminal assertions. Source/control logs remain in its recorded review directory; completed target removed. Public histories remain T19b; no full-game, trade, import/wire or replay claim.

**T19b evidence:** `private historical evidence` records missing public module RED. Two public Knight histories cover 3/4-seat pre-roll-to-seven and first-third-Army paths with 48 frozen source/RNG resource rows; all purchases, acquisition turns, public progress/played counts, score, original receipt projection and exact next dice are asserted. Full `{fmt,check,tests,doc-tests,build,clippy,doc,coverage}.log` gates pass 117 tests and five doctests, 99.02% library lines (3749/3786); Clippy type complexity was corrected with a plain table alias, without suppression. Nonzero `knight_` integration and BR036 filters pass. The ordinary advisory lock failure is retained; approved refreshed advisory gate passes. `private historical evidence` independently approves all 48 literal rows and both tests; wrong pre-roll resume and omitted public Army score mutations fail, sources restored and isolated target removed. Complete public wins remain T21; no performance/import/replay claim.

**Description:** Add playKnight, reconcile Largest Army on reveal, and reuse the existing robber flow without a seven's discard.

**Modules:** All four modules. **Dependencies:** [T10](#t10), [T18](#t18).

**Rule trace:** BR015, BR026, BR027, BR031, BR033, BR036, BR037, BR038.

**Acceptance criteria:**

- [x] An eligible Knight becomes public/played once; the first third Knight wins Largest Army and only a strict higher count transfers it.
- [x] PreRoll/Action origin is preserved across movement/theft; a pre-roll Knight followed by a seven correctly requires a second robber move but no second card play.
- [x] If Knight reveal/army points make the active player win, finish immediately before requesting movement; otherwise reuse normal robber rules and privacy.

**Verification:**

- [x] `cargo test --locked --test game_sequences knight_`
- [x] `cargo test --locked base_rules_br036_`
- [x] `cargo check --locked --all-targets`

**Files likely touched:** `src/game_model/mod.rs`, `src/base_rules/mod.rs`, `src/turn_engine/mod.rs`, `src/engine_api/mod.rs`, `tests/game_sequences.rs`.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

<a id="t20"></a>
### T20: Play Road Building with resumable free placements

- [x] T20 complete

- [x] T20a: Complete native Road Building activation and both placements with stored continuation, shared road legality/install/award/finish paths and meaningful RQ1 edge tests. Verify RED/GREEN, all gates and fresh review before an explicit-file commit.
- [x] T20b: Prove frozen public 3/4-seat sequential placement and next-dice histories, then verify gates/review and commit separately.

**T20a evidence:** `private historical evidence` records missing card/free-road contracts. Three `free_roads_` tests plus nonzero BR030/BR038 tests cover old/new/foreign/wrong-kind IDs, both continuations, all 72 candidate agreements, unchanged hands/bank/RNG, progress-only spent identity, shared allowance, wrong actor/phase/stale/counter rejection, one legal first with no possible second, no-site/no-piece activation, last-piece automatic completion and exact first-free victory. Paid/free roads use one install/award path and existing placement predicate; the shared finish hook precedes continuation. Full `{fmt,check,tests,doc-tests,build,clippy,doc,coverage}.log` gates pass 122 tests, five doctests and 99.01% library lines (3893/3932). Refreshed advisory gate passes after the retained ordinary lock failure. `private historical evidence` independently approves 122 tests, source fixture geometry/stocks/card/resource invariants and exact route witnesses; forced impossible second and omitted first-free victory mutations fail, restored sources and removed isolated target recorded. Public histories remain T20b; offers integrate in T23 and import/resume later.

**T20b evidence:** `private historical evidence` records missing integration wiring RED; public `free_roads_` runs one test with both source-frozen 3/4-seat histories and seven exact hands/bank rows. Eligible ID14 is purchased on turn1 and played pre-roll turn4/5; E44 rejects while disconnected, E45 makes it legal, the mandatory second placement resumes before rolling without payment, and next dice are exactly [3,4]/[4,1]. Candidates, revisions, allowance and original receipt privacy are asserted for all viewers. Full standing gates pass 123 tests, five doctests, 99.01% library lines (3893/3932). Unchanged dependencies retain the fresh passing T20a advisory verdict. `private historical evidence` independently matches all seven rows and endpoint witness; premature-resume, free-payment and hidden-purchase-identity mutations fail, restored sources and removed target recorded. Complete public first-free victory remains T21.

**Description:** Add playRoadBuilding and placeFreeRoad with stored continuation and the approved RQ1 policy.

**Modules:** All four modules. **Dependencies:** [T12](#t12), [T19](#t19).

**Rule trace:** BR017, BR026, BR030, BR031, BR035, BR037, BR038.

**Acceptance criteria:**

- [x] No legal road/piece at activation rejects without card/allowance use; otherwise up to two free roads use normal connectivity/occupancy/stock rules and the second sees the first.
- [x] A legal second road is mandatory; exhausted stock/no legal second site ends the effect automatically after the first. No lookahead invalidates a legal first placement and no unrelated action/forfeit interrupts it.
- [x] Each placement updates awards/checks victory; first-road victory ends immediately, and PreRoll/Action continuation remains explicit for later save/resume.

**Verification:**

- [x] `cargo test --locked --test game_sequences free_roads_`
- [x] `cargo test --locked base_rules_br030_`
- [x] `cargo test --locked base_rules_br038_`

**Files likely touched:** `src/game_model/mod.rs`, `src/base_rules/mod.rs`, `src/turn_engine/mod.rs`, `src/engine_api/mod.rs`, `tests/game_sequences.rs`.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

<a id="t21"></a>
### T21: Verify the exact first winning moment

- [x] T21 complete

- [x] T21a: Audit existing score-changing finish callers and verify exact score parts, active-owner timing, competing off-turn victory, above-ten start victory, winner-only privacy and all current terminal-command rejections. Full gates and fresh independent controls precede an explicit-file commit.
- [x] T21b: Execute independent complete public purchase-VP, third-Knight and first-free-road winning histories with frozen inputs/balances and next-dice controls; verify full gates and fresh review before completion/commit.

**T21a evidence:** `private historical evidence` records absent verification-module wiring, not a new product behavior claim. `base_rules_br03` runs eight tests, `privacy_victory_` two. New tests directly assert building/VP/Route/Army components once, hidden/revealed VP invariance, a valid eleven-point owner turn start before rolling, all 17 current command kinds rejected by all four seats without position/RNG/revision mutation, loser VP privacy and host final scores. Existing distance-valid award fixture gains ten off turn for seat2 while the active seat0 has nine; the active owner buys the fifth VP and wins first with final scores [10,2,10,0]. Existing score/finish callers are audited in paid road/settlement/city/purchase, Knight reveal, free road and turn start; no replacement calculator was added. Full standing gates pass 126 tests, five doctests and 99.01% library lines (3893/3932), retaining the unchanged-dependency T20a refreshed advisory verdict. Fresh-context `private historical evidence` independently approves 126 tests/five doctests and fixture graph/card/resource/stock facts; equality-only victory and all-owner scoring mutations fail meaningfully, restored sources and removed target recorded. Offers do not exist yet; T23 owns their cleanup integration. Complete public wins remain T21b.

**T21b evidence:** `tests/game_sequences/victory.rs` replays three complete fixed three-seat histories from all 235 checked-in frozen records in `tests/support/victory-prefixes.json`; `tests/support/victory-source.md` preserves independent sampler/arithmetic/endpoint provenance. VP24 purchase wins 9→10 on turn68, third Knight0 wins 8→10 before robber/dice on turn71, and first free E51 wins 8→10 on turn97 while connected empty E47 and ten road pieces permit a second placement. Every recorded roll, chosen seven discard, production, economy action, balance, score step, revision and final freeze is asserted through public APIs; alternative branches retain next dice [1,5], [4,3], [4,1]. RED records absent module wiring, not a new behavior claim. `private historical evidence,green,fmt,check,tests,doc-tests,build,clippy,doc,coverage}.log` retain full passing gates: 129 tests (93 library,24 sequences,5 model,7 rules), five doctests and 99.01% library lines (3893/3932). The refreshed C07 advisory scan passes in `private historical evidence`; its ordinary database-lock failure remains separately recorded. `private historical evidence` independently approves all 235 literals, source arithmetic, endpoint trails and isolated 129 tests/five doctests; omitted purchase/Knight/free-road victory each fails its corresponding test. The reviewer restored all 56 copied source/test files and frozen input hashes and removed its generated target, retaining controls in `private historical evidence`. T29 still owns complete histories for all four setup/player-count configurations; T23 owns offer closure once negotiation exists.

**Description:** Audit the victory hook already used by scoring actions and add the complete tie/off-turn/pending-effect conformance suite.

**Modules:** Scoring and complete native gameplay. **Dependencies:** [T20](#t20).

**Rule trace:** BR032, BR033, BR034, BR035, BR036, BR037, BR038.

**Acceptance criteria:**

- [x] True/public scores count settlement/city/VP/awards once; an active seat wins at at least ten after every score-changing step or at turn start before rolling.
- [x] Off-turn ten-point seats wait for their own turn and can be beaten meanwhile; a newly bought VP, third Knight and first free road can each finish immediately.
- [x] Completion reveals only winner VP cards, records host-private final scores, clears pending choices/offers, and rejects every subsequent gameplay command. No offer model exists yet; T23 verifies offer cleanup integration when introduced.

**Verification:**

- [x] `cargo test --locked --test game_sequences victory_`
- [x] `cargo test --locked base_rules_br03`
- [x] `cargo test --locked --lib privacy_victory_`

**Files likely touched:** `src/base_rules/mod.rs`, `src/turn_engine/mod.rs`, `src/engine_api/mod.rs`, `tests/rules_conformance.rs`, `tests/game_sequences.rs`.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

<a id="c07"></a>
### C07: All development cards and immediate victory

- [x] T19–T21 acceptance and focused checks pass.
- [x] Knight/army/robber continuations and free roads work; no score-changing step permits play past the first active-player win.
- [x] Run all applicable standing checkpoint commands in plan.md; record actual results and limitations.
- [x] Present checkpoint evidence for human review before proceeding.

**C07 evidence (10 November 2026):** [Retained checkpoint report](c07-checkpoint.md) records all six reviewed T19–T21 slice commits through `1190fbe`, 129 tests/five doctests, 99.01% library line coverage (3893/3932), all standing build/lint/docs checks, refreshed passing advisory scan and eight nonzero focused filters. Independent source/RNG/arithmetic/endpoint, privacy and targeted mutation reviews approve the task slices; `private historical evidence` retains the checkpoint evidence audit and final mapping. Three complete fixed three-seat winning histories end on a winning VP purchase, third Knight or first free road; T29 still requires all four setup/player-count configurations. Existing autonomous authorization permits Phase 8 to proceed. Domestic offers/closure, wire/save/replay, final conformance, measured performance/memory, Wasm and hosted CI remain unfinished; nothing was published.

## Phase 8: Domestic trade and native privacy audit

<a id="t22"></a>
### T22: Negotiate one public bilateral trade offer

- [x] T22 complete

**Refined execution (10 November 2026):**
- [x] T22a: Complete proposal/replacement/reject/cancel through the bounded model, shared rules and turn validator, actor exception, public choices/view/original receipts and creation initializers. Integrate the single successful-transition offer-lifetime hook now so all supported actions preserve the offer contract from the first commit, including hand changes, zero-result effects, Action exit and victory ordering. No accept stub, escrow or trade-bundle enumeration. Focused tests cover valid and rejected actors/terms, offer IDs/revisions, privacy and conservation; RED, gates and fresh independent review precede explicit-file commit.
- [x] T22b: Verify both active/inactive proposal orientations and replacement/counteroffer/reject/cancel through independent seeded public histories and original receipts; run nonzero focused filters, standing gates and independent review before completion/commit.

**T22a connected scope:** `game_model/state.rs` and exports, `turn_engine/creation.rs`/`base_rules/economy.rs` initializers and initial-state rejection, `base_rules/trade.rs`/export, `turn_engine/negotiation.rs`/dispatcher and one post-transition closure hook, `engine_api/command.rs`/`view.rs`, public facade, focused `trade_offer_tests.rs` and task status. Existing exact legal-action expectations in bank/road/end-turn tests and `tests/game_sequences/road.rs` gain the now-valid proposal choice without losing checks; `victory_tests.rs` adds all three new commands to terminal rejection. Atomic acceptance is T23.

**T22a evidence:** `private historical evidence` records missing-offer RED; `br021`, `precedence` and `lifetime-verified` focused logs cover actor/recipient/term/ID/phase errors, no recipient-hand inference, public offer/old receipts, replacement order, no escrow, unchanged zero-result Monopoly, real hand-change closure and Knight/Road Building/turn/paid-award/victory ordering. `private historical evidence,fmt-verified,clippy-verified,coverage-verified}.log` plus `{check-final,build-final,doc-final,doc-tests-final}.log` pass 139 tests/five doctests and 98.86% library lines (4093/4140); C07's refreshed advisory verdict applies to unchanged dependencies. Fresh Terra `private historical evidence` independently approves, caught and verified fixed recipient/phase precedence, and killed targeted privacy, busy/role/ID, zero/nonzero-effect and road-award-order mutations after strengthened tests. Public seeded histories remain T22b; acceptance is T23.

**Description:** Add proposeTrade/rejectTrade/cancelTrade, with explicit recipient, public terms, replacement/counteroffer rules, and no escrow.

**Modules:** All four modules. **Dependencies:** [T21](#t21).

**Rule trace:** BR021, BR022.

**Acceptance criteria:**

- [x] The active player is one party, both bundles are positive with disjoint kinds, and the proposer can fund their give bundle; proposals do not test/reveal the recipient's private hand.
- [x] Offer ID is creation revision; replacement/counteroffer/offerBusy behavior matches the spec, with closure before creation, and only authorized parties reject/cancel.
- [x] Each accepted negotiation increments revision without changing resources/RNG; stale IDs/revisions and invalid gifts/credit/development-card exchanges reject atomically.

**Verification:**

- [x] `cargo test --locked --test game_sequences trade_offer_`
- [x] `cargo test --locked base_rules_br021_`
- [x] `cargo test --locked --lib privacy_trade_`

**Files likely touched:** `src/game_model/mod.rs`, `src/base_rules/mod.rs`, `src/turn_engine/mod.rs`, `src/engine_api/mod.rs`, `tests/game_sequences.rs`.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

**T22b evidence:** `private historical evidence,green-verified}.log` record missing-module RED and one passing public test covering both 3/4-seat seed8 histories. Existing independently checked pinned-rand/source/witness inputs `private historical evidence` provide literal hands/bank/purchase/dice. Tests exercise funded active/inactive proposals, busy third party, counteroffer, active replacement to a different recipient, reject/cancel, exact ordered original receipts for every seat/spectator and unchanged following dice. Final `private historical evidence,fmt-verified,clippy-verified,coverage-verified}.log` and `{check,build,doc,doc-tests}.log` pass 140 tests/five doctests and 98.86% library lines (4093/4140); BR021 and privacy focused filters run two each. Fresh Terra `private historical evidence` independently verifies fixtures and all-viewer projections, and kills busy and different-recipient replacement mutations after its coverage findings were fixed. T22 is complete; explicit acceptance and further lifecycle proof are T23.

<a id="t23"></a>
### T23: Accept a trade and close offers consistently

- [x] T23 complete

**Refined execution (10 November 2026):**
- [x] T23a: Add explicit accept to the bounded model, shared trade validation and checked exchange, turn dispatcher, original-fact public receipts and bounded response queries; preserve T22's single existing offer-lifetime hook and expand terminal coverage. Recipient validation checks only their own hand; execution privately rechecks both parties and rejects generically without partial transfer. Focused tests cover both orientations, stale/competing/lost responses, shortage/privacy and complete position/RNG/revision atomicity. RED, full gates and independent review precede the explicit-file commit.
- [x] T23b: Prove both acceptance orientations and participant-hand-change versus zero-result Monopoly/Invention lifetime using independently derived public histories, exact event order/receipts, bank/hands and following dice. Run nonzero task filters, standing gates and independent review before completion/commit.

**T23a connected scope:** `game_model/state.rs` command/error/event/choice types, `base_rules/trade.rs` and export for both checked payments, `turn_engine/negotiation.rs` and dispatcher, `engine_api/command.rs` and `trade_accept_tests.rs`, and terminal command coverage in `victory_tests.rs`. Existing successful-transition expiry stays single and unchanged.

**T23a evidence:** `private historical evidence` records missing-accept RED; BR022 and privacy tests cover both orientations, no partial transfer, original all-viewer facts, insufficient recipient and privately rechecked proposer, stale ID/turn/phase/revision and competing retries, malformed public terms and u64 revision limit. `private historical evidence,fmt-verified,clippy-verified,coverage-verified}.log` plus `{check,build,doc,doc-tests}.log` pass 145 tests/five doctests and 98.95% library lines (4146/4190); C07 advisory verdict applies to unchanged dependencies. Fresh Terra `private historical evidence` independently passes all 145 tests and kills ten targeted acceptance, privacy and ordering mutants after stale-turn, precedence and malformed-term assertions were added. The deliberately malformed retained-offer fixture tests private recheck only; legal positions close on party hand change. Public exchange and effect histories remain T23b.

**Description:** Add acceptTrade and integrate offer closure into existing resource changes, card effects, turn ending and victory.

**Modules:** All four modules. **Dependencies:** [T22](#t22).

**Rule trace:** BR021, BR022.

**Acceptance criteria:**

- [x] Only the current recipient accepts; current phase/turn/ID/revision and both holdings validate, and the complete exchange occurs atomically without partial payment.
- [x] Close offers on the specified successful actions, any party hand change and Action exit; reject/failure leaves the offer intact, and generic closure/unavailability facts reveal no hidden affordability.
- [x] Competing/stale accepts, inactive third-party trading, replaced offers and lost-response resubmissions cannot transfer twice; no automatic acceptance exists.

**Verification:**

- [x] `cargo test --locked --test game_sequences trade_accept_`
- [x] `cargo test --locked --test game_sequences offer_lifetime_`
- [x] `cargo test --locked --lib privacy_trade_`

**Files likely touched:** `src/game_model/mod.rs`, `src/base_rules/mod.rs`, `src/turn_engine/mod.rs`, `src/engine_api/mod.rs`, `tests/game_sequences.rs`.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

**T23b evidence:** `private historical evidence,green-final,lifetime-final}.log` record missing-module RED and three passing public tests. Independent seed8 3/4-seat histories prove both acceptance orientations with exact conserved hands/bank, all-viewer original receipts and following dice. Seed5 reaches nonzero Monopoly closure and, after two real accepted trades, zero-result Monopoly with the offer retained until EndTurn; seed8 Invention closes an offer even though it remains affordable. Purchase/card-effect receipts are projected for every seat/spectator before and after later rolls. Final `private historical evidence,fmt-final,clippy-final,coverage-final}.log` and `{check,build,doc,doc-tests}.log` pass 148 tests/five doctests and 98.95% library lines (4146/4190); privacy filter runs three tests and existing C07 advisory applies to unchanged dependencies. Fresh Terra `private historical evidence` verifies source/RNG arithmetic, fixes an effect-receipt test gap and kills five transfer/lifetime/projection mutations. T23 is complete; full native privacy audit is T24.

<a id="t24"></a>
### T24: Audit complete viewer, query and receipt privacy

- [x] T24 complete

**Refined execution (10 November 2026):**
- [x] T24a: Consolidate native receipt types and original-fact projection into a focused `engine_api/receipt.rs`, retain the existing public API and all command callers/tests, and add all-viewer, all-event privacy/old-receipt tests plus public `tests/privacy.rs`. Verify hidden purchase/discard/theft and winner-only VP allowlists without exposing raw command or final scores. RED/green, full gates and fresh Sol privacy review precede an explicit-file commit.
- [x] T24b: Add native query/validate/execute contract checks in `tests/api_contract.rs` and focused internal counter-endpoint tests, including u64 revision and turn exhaustion, offer responses and all current gameplay actions. Preserve bounded candidates, error precedence, original offers and complete rejection/RNG/revision atomicity. Verify nonzero task filters, standing gates and independent review before completion/commit; C08 records the final evidence.

**T24a evidence:** `private historical evidence` records missing-module wiring RED; the extracted receipt implementation retains the native API and original-fact projection. Two `privacy_native_` tests check all 25 events for every viewer, paired hidden state and original receipts; the public privacy integration covers purchase and trade receipts across later commands. `private historical evidence,check,tests,doc-tests,build,clippy,doc,coverage}.log` record passing gates: 151 tests, seven doctests and 99.00% library lines (4148/4190); unchanged dependencies retain the C07 advisory verdict. Independent `private historical evidence` approves all viewer allowlists and connected validators, seven doctests, and six killed purchase/discard/theft/revision/VP privacy mutations; source copies and controls are retained in `private historical evidence`. T24b owns the full native endpoint and counter audit.

**T24b evidence:** `private historical evidence,public-red}.log` record absent test wiring/target RED, not a new behavior claim. The all-candidate endpoint test checks every current command variant at the observed revision, and at `u64::MAX` checks query emptiness plus complete position/RNG/revision preservation; the separate turn limit retains the offer and recipient response. The first 21-kind assertion exposed a missing settlement fixture (`private historical evidence`); the source-traced V12–E12–V8–E7–V4 fixture closes it. Two public tests cover setup candidates, version/seat/revision/actor/phase precedence, stale retry, private/public view preservation and unchanged next dice. `private historical evidence,check,tests,doc-tests,build,clippy,doc,coverage,privacy,native}.log` pass 155 tests, seven doctests and 99.00% library lines (4148/4190); refreshed `private historical evidence` passes after the separately retained read-only lock failure. Independent `private historical evidence` approves all 21 variants and kills stale-revision, rejection-RNG and discard-counter mutations. Synthetic endpoint positions test local contracts, not historical reachability or snapshot imports. T24 is complete; C08 records the checkpoint.

**Description:** Consolidate the now-complete native projections into focused files and verify every seat/spectator/event/error path against the allowlists.

**Modules:** engine-api. **Dependencies:** [T23](#t23).

**Rule trace:** BR012, BR014, BR022, BR025, BR032.

**Acceptance criteria:**

- [x] Every view/event/error respects own hands/cards/score, public bank/board/offer facts, private discards/thefts and host-only seed/snapshot/replay; old receipts project their original facts.
- [x] Seat-scoped validate/legal_actions share gameplay validators, expose bounded current candidates/obligations, and never query another seat's hand or enumerate all bundles.
- [x] All rejection codes/precedence and previous/new revision behavior are deterministic; receipt projection is opt-in and ordinary commands do not automatically allocate views or JSON.

**Verification:**

- [x] `cargo test --locked --test privacy`
- [x] `cargo test --locked --test api_contract native_`
- [x] `cargo check --locked --all-targets`

**Files likely touched:** `src/engine_api/mod.rs`, `src/engine_api/view.rs`, `src/engine_api/error.rs`, `tests/privacy.rs`, `tests/api_contract.rs`.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

<a id="c08"></a>
### C08: Negotiation and complete privacy

- [x] T22–T24 acceptance and focused checks pass.
- [x] Explicit bilateral offers/acceptance are atomic; all seats/spectators/queries/events/errors pass the final native privacy allowlists.
- [x] Run all applicable standing checkpoint commands in plan.md; record actual results and limitations.
- [x] Present checkpoint evidence for human review before proceeding.

**C08 evidence (10 November 2026):** [Retained checkpoint report](c08-checkpoint.md) records all six independently reviewed T22–T24 slice commits through `9cb29a3`, 155 tests/seven doctests, 99.00% library line coverage (4148/4190), passing standing build/lint/docs checks and refreshed advisory scan, plus eight nonzero focused filters. Negotiation and acceptance histories prove both orientations, atomic exchange and specified lifetime; complete native event/view/query/receipt allowlists and all 21 command counter endpoints are covered. Independent `private historical evidence` verifies commit/count/log mapping and evidence, and approves after correcting the Action-exit description. Existing autonomous authorization permits Phase 9 to proceed. Strict wire/save/replay, end-to-end conformance/properties, measured performance/memory, Wasm/bindings and hosted CI remain unfinished; nothing was published.

## Phase 9: Strict wire boundary and save/resume

<a id="t25"></a>
### T25: Expose the versioned strict JSON command contract

- [x] T25 complete

- [x] T25a: Strict complete command JSON conversion and native execution.
- [x] T25b: Borrowed viewer-safe view/query/projected-result encoding and privacy contracts.

**T25a evidence:** Complete strict decoder/borrowed command encoder and `execute_json` use all 21 native command variants and the ordinary atomic facade. `private historical evidence` records missing boundary wiring before implementation. Three public wire tests assert literal-to-native payloads, duplicate/escaped keys, unknown/unit-extra/sequence shapes, integer/ID/resource/counter limits, version precedence and accepted-envelope stale retry; the private parser test asserts actual 65,536/65,537 bytes and depth16/17. `private historical evidence,fmt,check,clippy,tests,doc-tests,build,doc,coverage}.log` pass **159 tests, seven doctests and 98.22% library lines (4479/4560)**. Refreshed advisory scan passes; the ordinary read-only lock failure is retained separately. Independent `private historical evidence` approves, checks final live byte equality and kills root-unknown-field and byte-limit mutations in a disposable archive. No core Serde derives, dependency, policy or gate changes. T25b closes parent completion below.

**T25b evidence:** Borrowed allowlist encoders expose views, legal actions, projected receipts/creation and bounded errors without raw state/receipt Serde or dependency changes. Exact fixtures cover all nine phases, 21 choices, 25 projected tags and seven creation variants; all wire counter sites include zero/u64::MAX, and every viewer retains owner-only purchase/discard/theft facts. Initial missing-operation red is retained in `private historical evidence`; final `private historical evidence,check,clippy,tests,doc-tests,build,doc,coverage}.log` pass **163 tests, seven doctests and 98.80% library lines (4845/4904)**. Same resolved graph retains T25a refreshed advisory evidence; C09 refreshes it again. Independent `private historical evidence` approves the final five files after live byte comparison; previously surviving decimal-turn and creation-tag mutations now fail the exact fixture, and Option-payload omission also fails. No remaining required findings.

**Description:** Implement external wire conversion for the completed native command/query/view/error types without duplicating rule logic or adding transport.

**Modules:** engine-api. **Dependencies:** [T24](#t24).

**Rule trace:** Model/API/quality contract; no additional tabletop rule.

**Acceptance criteria:**

- [x] Every command tag/field round-trips; resource keys, ID/count bounds, camelCase tags, decimal u64 counters/offer IDs and version checks match the spec.
- [x] Unknown/duplicate fields, unsupported tags/versions, malformed integers and overflow reject; acting seat remains host-derived and errors never echo private command data.
- [x] Wire tests drive the same native facade/validators and cover stale envelopes and lost-response retry semantics; public exports/rustdoc are deliberate.

**Verification:**

- [x] `cargo test --locked --test api_contract wire_`
- [x] `cargo test --locked --test privacy`
- [x] `cargo doc --locked --no-deps`

**Files likely touched:** `src/engine_api/mod.rs`, `src/engine_api/wire.rs`, `src/lib.rs`, `tests/api_contract.rs`, `tests/support/contracts.json`.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

<a id="t26"></a>
### T26: Export complete host-private snapshots

- [x] T26 complete

**Evidence:** Complete borrowed schema-1 export includes the source frame identity, config, every stored position fact/offer and private seed/stream/word position; topology and history are excluded. The frozen synthetic fixed3 fixture is 2,925 bytes. Literal all-nine-phase/native private-fact tests include u64/MAX and RNG position 2^68−1; a nonuniform seed freezes byte order and independent T06 variable seed12 goldens freeze frame/terrain/tokens/port IDs. Public tests assert repeated-byte equality, bounds/privacy and rejected-command snapshot equality. `private historical evidence` precedes the exporter; `private historical evidence,check,clippy,tests,doc-tests,build,doc,coverage}.log` pass **166 tests, eight doctests and 98.86% library lines (5039/5097)**. Independent `private historical evidence` approves after live comparison and phase/seed/frame/uppercase mutations all fail. The unchanged dependency graph retains refreshed T25a advisory evidence until C09. Import remains T27.

**Description:** Add byte-stable versioned export for the board/configuration, complete position, pending continuations, offer, revision and private generator state.

**Modules:** engine-api. **Dependencies:** [T25](#t25).

**Rule trace:** Model/API/quality contract; no additional tabletop rule.

**Acceptance criteria:**

- [x] Export contains every resume-relevant fact and schema/rules/RNG version, with canonical seed/counters/word position; no command history is stored in position.
- [x] Every phase and finished game exports deterministically within the 64 KiB/depth-16 contract; generic Serde boundary values handle fixed-array lengths without a second core model.
- [x] Snapshots are a visibly host-private operation, never a player view/event, and return values/bytes without filesystem or storage calls.

**Verification:**

- [x] `cargo test --locked --test snapshots export_`
- [x] `cargo test --locked --test privacy`
- [x] `cargo check --locked --all-targets`

**Files likely touched:** `src/engine_api/mod.rs`, `src/engine_api/snapshot.rs`, `tests/snapshots.rs`, `tests/support/snapshot-v1.json`.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

<a id="t27"></a>
### T27: Import snapshots through full validation

- [x] T27 complete

- [x] T27a: Full native invariants reused by creation.
- [x] T27b: Strict separate-engine snapshot import.
- [x] T27c: Public all-phase/RNG/corruption round-trip proof.

**T27a evidence:** One pure full current-state validator now checks sourced layouts, conservation/pieces/owners, card prefix/location/acquisition, setup pairing/order/counts, phase/continuations, offers, route caches/incumbent ties, Army qualification and active/off-turn completion. Creation reuses it plus meaningful initial-only restrictions. `private historical evidence` proves the prior creation validator accepted a valid-count wrong fixed token layout. Six native tests include ordinary creation/progressed setup/turns,22 corrupt facts, canonical spent kinds, acquisition, discard/offer/effects, Army/Route ties, overflow-safe counters and finished/revealed VP. Independent review found and fixed two roads paired at one setup settlement; its ordinary-transition regression and all four adversarial mutations now fail. `private historical evidence` approves and live native files match its manifest. The separate HEAD+four-native-files archive `private historical evidence` excludes later import work; `private historical evidence,check,clippy,tests,doc-tests,build,doc,coverage}.log` pass **173 tests, eight doctests and 98.76% library lines (5343/5410)**. Failed earlier Clippy output is retained; the tuple has a private type alias, with no suppression. No rules, dependencies or gates changed.

**T27b evidence:** Strict required/nullable native conversion rejects duplicate/unknown fields, sequence objects, unsupported versions, canonical decimal/seed failures, IDs/ranges and exact array lengths before candidate activation. Rebuilt topology and the shared full native validator precede returning a separate Engine. Public fixtures compare complete/nonuniform-seed export→import bytes and next receipts; inner literal enum tests freeze all nine phase/five location/offer/building payloads. `private historical evidence` precedes import. A pinned independent endpoint sampler exposed buffered RNG cursor overflow after a valid 2^68−1 draw; `private historical evidence,green}.log` retain its fix. Export now canonicalizes the equivalent cursor modulo2^68, and0/15/16/max fixtures reproduce exact dice, reimport, resolution and subsequent random receipts. The independent reviewer verifies100,000 future RNG words and kills all seven boundary mutations; `private historical evidence` approves final live bytes. Separate HEAD+four-import-files archive `private historical evidence` excludes T27c; `private historical evidence,check,clippy,tests,doc-tests,build,doc,coverage}.log` pass **179 tests, nine doctests and 98.69% library lines (5739/5815)**. No runtime dependency, storage, history, raw core Serde, rule or gate changes.

**T27c evidence:** Three public integration tests resume before/after every command across all nine phases, variable three-/four-seat setup, both free-road continuations at remaining two/one, offers, three weighted-theft cases and three frozen complete fixed-three-seat winning histories. Exact snapshots, every viewer JSON, all seats' legal choices, validation/private/projected receipts and next random results agree; queries and stale retries preserve bytes. The complete histories cover pre-roll Knight→seven and partially fulfilled multi-seat discard with a fulfilled seat still above seven. Public synthetic valid/corrupt candidates additionally distinguish Route/Army ties, card ownership/acquisition/deck cursor, free-road allowance, offer parties/terms, variable source layouts and exact completion/reveals. Independent `private historical evidence` approves final live bytes and kills all six isolated controls: activation-validator bypass, Route no-holder tie, incorrect zero-obligation rejection, lost RNG cursor, exact winning-score equality and winner-only revelation. The last two initially survived masked assertions; score11 and an otherwise-valid opponent Held→Revealed VP candidate now directly kill them. Earlier logs remain retained. `private historical evidence,check,clippy,tests,doc-tests,build,doc,coverage}.log` pass **182 tests, nine doctests and 98.74% library lines (5742/5815)**. Refreshed C09 advisory scan passes; all six final focused targets run nonzero. No source/rule/dependency/quality change in this proof leaf; replay remains T28.

**Description:** Add Engine::from_snapshot as a separate-engine constructor, validating format, distributions, phases, derived rules and RNG reconstruction before activation.

**Modules:** game-model/base-rules → engine-api. **Dependencies:** [T26](#t26).

**Rule trace:** Model/API/quality contract; no additional tabletop rule.

**Acceptance criteria:**

- [x] Preparse size/structural limits, strict keys/versions/IDs, exact array lengths and RNG word position below 2^68 reject corrupt inputs; no partial replacement of an existing engine occurs.
- [x] Rebuilt graph/source layout, resources/pieces/cards, ownership, phase payloads and recomputed scores/awards/caches satisfy full invariants; valid-looking arbitrary ports/tokens reject for the selected setup mode.
- [x] Round-trip every pending phase/offer/card acquisition/deck cursor/award/finished state and reproduce the next dice/theft result exactly.

**Verification:**

- [x] `cargo test --locked --test snapshots`
- [x] `cargo test --locked --test model_invariants`
- [x] `cargo test --locked --test privacy`

**Files likely touched:** `src/game_model/mod.rs`, `src/base_rules/mod.rs`, `src/engine_api/mod.rs`, `src/engine_api/snapshot.rs`, `tests/snapshots.rs`.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

<a id="c09"></a>
### C09: Wire contracts and save/resume

- [x] T25–T27 acceptance and focused checks pass.
- [x] Strict JSON contracts and corrupt-import rejection pass; every pending phase/offer resumes with exact next random output.
- [x] Run all applicable standing checkpoint commands in plan.md; record actual results and limitations.
- [x] Present checkpoint evidence for human review before proceeding.

**C09 evidence (10 November 2026):** [Retained checkpoint report](c09-checkpoint.md) records all six reviewed T25–T27 leaf commits through `652e873`, strict command/view/result contracts, complete host-private schema-1 export/import and shared full native activation. Final gates pass **182 tests, nine doctests and 98.74% library lines (5742/5815)**, including release/rustdoc and a refreshed passing advisory scan; six nonzero focused targets total 21 tests. Independent slice reviews kill selected parsing/privacy/preservation/native/activation/RNG/terminal controls; `private historical evidence` independently approves exact commit/source/count/log mapping and remaining scope. The setup-road pairing defect and buffered RNG period-wrap export defect are fixed with retained failing/passing evidence and independent oracles. Every pending phase, both card continuations, offers, partial multi-seat discard, awards, three complete fixed-three-seat wins and exact next weighted theft/dice resume through public commands. Validation establishes coherent current facts, not honest history. Existing autonomous authorization permits Phase 10; accepted-command replay remains T28, final complete configurations/BR/properties remain T29–T30 and measured performance/memory remain T31–T32. No rules, dependencies, quality gates, storage/transport, Wasm/bindings or hosted CI were added or weakened; nothing was published or deployed.

## Phase 10: Replay and complete conformance

<a id="t28"></a>
### T28: Verify deterministic accepted-command replay

- [x] T28 complete

**T28 evidence:** Host-private format-1 header/accepted records reuse strict C09 parsing, snapshot activation, ordinary execute and exhaustive event projection. The verifier retains one current game/index, checks exact parsed record equality/revisions, and restores complete state/RNG/index on mismatch. Six public tests freeze source seed63 weighted theft/next dice and literal seed8 private purchase, strict versions/keys/counters/byte/depth bounds, all-viewer private purchase/discard/theft facts, max RNG/revision headers and valid changed-command rollback. A focused private writer test proves rejected crossing writes cannot grow beyond65,536 bytes. `private historical evidence` precedes the API; final tests/Clippy/coverage logs pass **189 tests, ten doctests and 98.70% library lines (5922/6000)**, with original production fmt/check/release/rustdoc gates retained. Independent `private historical evidence` approves and kills eight meaningful controls: equality, full/RNG-only rollback, revision linkage, rules compatibility, public privacy, omitted private facts and early writer bound. The originally surviving memory control is closed by the new direct assertion; prior evidence remains. Dependencies/rules/gates unchanged; the fresh C09 advisory scan still applies, with refresh required at C10.

**Description:** Implement host-private replay format 1 and record-by-record verification through ordinary command execution.

**Modules:** engine-api → ordinary facade. **Dependencies:** [T27](#t27).

**Rule trace:** Model/API/quality contract; no additional tabletop rule.

**Acceptance criteria:**

- [x] Initial snapshot plus accepted envelopes/expected events replay with exact previous/new revisions, ordered private events and final snapshot/next random result.
- [x] A tampered command/event/revision or incompatible rules/RNG version reports the first mismatch index; rejected commands are not accepted history.
- [x] Per-record bounds and host-controlled stream length prevent an unbounded parser; player activity history uses projections and never exposes a full replay/seed.

**Verification:**

- [x] `cargo test --locked --test replay`
- [x] `cargo test --locked --test privacy`
- [x] `cargo doc --locked --no-deps`

**Files likely touched:** `src/engine_api/mod.rs`, `src/engine_api/replay.rs`, `src/lib.rs`, `tests/replay.rs`, `tests/support/replay-v1.json`.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

<a id="t29"></a>
### T29: Complete the source-traced end-to-end conformance suite

- [x] T29 complete

**Execution refinement (10 November 2026):** Preserve the original acceptance in three reviewed leaves: T29a closes the four confirmed BR012/014/015/030 native assertion gaps; T29b adds publicly reached complete configurations plus replay/resume and frozen source literals; T29c records the full clause-level BR map and closes any remaining direct assertion gaps. No approved rule or quality requirement changes. Each leaf retains verification evidence and an explicit-file commit. Existing fixed-three-seat235 records are reused.

- [x] T29a: Four direct rare-rule assertions.
- [x] T29b: Four complete public configurations, source facts, replay/resume.
- [x] T29b1: Reuse all three fixed-three-seat wins with complete replay/resume.
- [x] T29b2: Add frozen fixed-four/variable-three/variable-four complete histories.
- [x] T29c: Complete BR001–BR038 clause assertion/source map.

**T29a evidence:** Successful chosen discard now preserves all25 development locations with Held/PlayedKnight/Spent/Undrawn representatives. Independent weighted automatic theft cases and a chosen nonempty victim retain the full locations/deck/cursor while transferring exactlyone resource. Two free roads were already legal in distinct E0/E46 networks; E1/E45 leave two exact length2 routes without resource/RNG use. Robber on H15 stays while constructing adjacent V42/E61, spending full costs, immediately using its2:1 brick port and completing domestic trade. Native130 and full **192 tests, ten doctests,98.70% lines(5922/6000)** pass all applicable gates in `private historical evidence`. `private historical evidence` independently approves exact restored source, verifies source geometry/resource arithmetic and detects all eight compile-valid controls in nine focused runs. Temporary quota failures are retained as tooling failures; completed build caches were cleaned without losing source/evidence. No production/rule/quality/dependency change; full configurations and final clause map remain next leaves.

**T29b1 evidence:** The existing235 fixed-three-seat purchase/third-Knight/first-free-road records remain byte-identical. Their three tests now use one complete creation-header ReplayVerifier and ordinary before/after snapshot resume for every accepted command, comparing exact private/projected receipts, full snapshots/RNG, every view, legal queries/validation, stale rejection and terminal byte freeze. Test names carry `full_game_fixed3_victory_` for the approved focused filter. The test-only consumer also independently requires unchanged serialized RNG for all nonrandom commands, including winning transitions; an initially surviving shared terminal-RNG control exposed and closed this comparison blind spot. Final focused3 and **192 tests, ten doctests,98.70% lines(5922/6000)** pass all applicable gates; `private historical evidence` independently approves final helper hashes and catches all five controls. Prior survivor/native-failure evidence is retained honestly. Other configurations and BR015 same-resource production remain T29b2/T29c.

**T29b2 evidence:** Three complete fixed-four/variable-three/variable-four games add307 independently frozen rolls, source creation/layout/deck/start RNG facts, actual forward/reverse variable placements, every production/discard/payment/balance/card/score, exact route trail8→10, winner-only reveal, still-legal suppressed E47, terminal snapshot freeze and independently sampled next dice/cursor. Together with unchanged235 fixed-three records, six `full_game_` tests cover all four configurations and all development kinds. Every accepted command uses the B1 persistent replay/per-command resume consumer. [Fixture provenance](../tests/support/full-games-source.md) records source hashes and synthetic fixture provenance; tests do not read `/tmp`. Final six focused and **195 tests, ten doctests,98.70% lines(5922/6000)** pass all applicable gates. `private historical evidence` independently recomputes every row/command revision without product imports, reproduces all pinned sampler bytes and detects eight compile-valid controls including conserved partial-shortage payout and valid deck permutation. Exact live hashes match; intermediate stale isolated Cargo mtime evidence is retained and resolved by fresh compilation. No engine/rule/quality/dependency change; final clause map/same-kind production remains T29c.

**T29c evidence:** [Clause-level map](rules-conformance.md) records all38 ordered BR clauses,106 resolved named assertions/source pages and honest closed-command nonactions; RQ1/RQ2 stay explicitly approved policies. A source variable H2/H13 pasture5 fixture proves the robber blocks only H2 while H13's city still produces2wool. The existing public checkpoint helper additionally verifies a bounded header/record from every reached phase, including offers, victim choices and free-road two/one in both continuations, preserving its previous resume/privacy/purity assertions. Native1/checkpoint3 and **196 tests, ten doctests,98.71% lines(5954/6032)** pass all applicable gates. `private historical evidence` independently approves final source/map hashes, all38/106 references and21native/wire variants; overbroad resource-kind blocking survives the old different-resource fixture but fails the new direct assertion, and matching receipt with dropped checkpoint state fails exact snapshot equality. Both controls restore and pass; prior tooling mtime logs remain retained. No approved rule/quality, dependency or production behavior change. T29 acceptance is complete; shrinking properties remain T30 and final advisory/checkpoint evidence remains C10.

**Description:** Verify legal fixed/variable three/four-player games from creation to victory through public commands, and close every BR catalogue coverage gap.

**Modules:** Public engine integration. **Dependencies:** [T28](#t28).

**Rule trace:** BR001, BR002, BR003, BR004, BR005, BR006, BR007, BR008, BR009, BR010, BR011, BR012, BR013, BR014, BR015, BR016, BR017, BR018, BR019, BR020, BR021, BR022, BR023, BR024, BR025, BR026, BR027, BR028, BR029, BR030, BR031, BR032, BR033, BR034, BR035, BR036, BR037, BR038.

**Acceptance criteria:**

- [x] Four setup/player-count combinations reach exact victory without private-state mutation or UI-side rules; deterministic public command fixtures exercise all development types and mandatory decisions.
- [x] Every BR001–BR038 maps to direct positive/negative assertions with independent expected results; RQ1/RQ2 are labeled approved policies, not official quotations.
- [x] Complete histories replay and phase checkpoints resume with matching state/events/RNG; contract/privacy checks still pass on those complete games.

**Verification:**

- [x] `cargo test --locked --test game_sequences full_game_`
- [x] `cargo test --locked --test rules_conformance`
- [x] `cargo test --locked --test replay`

**Files likely touched:** `tests/game_sequences.rs`, `tests/rules_conformance.rs`, `tests/privacy.rs`, `tests/replay.rs`, `tests/support/full-games.json`.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

<a id="t30"></a>
### T30: Exercise invariants and malformed boundaries with properties

- [x] T30 complete

**Execution refinement (10 November 2026):** Preserve the original acceptance in three verified leaves: T30a adds reproducible synthetic creation/state fact properties; T30b extends them through bounded shrinkable legal/rejected decision vectors, candidate/query/atomicity and resume/replay checks; T30c adds bounded malformed snapshot/wire properties and retains any genuine minimal synthetic regression. No production hooks, generic test subsystem, dependencies or quality changes.

- [x] T30a: Reproducible synthetic configuration and independent state facts.
- [x] T30b: Bounded reached-state decision vectors with shrinking.
- [x] T30c: Bounded malformed imports/wire and regression evidence.

**T30a evidence:** [Durable property evidence](property-evidence.md) records 64 pinned synthetic creation cases, independent resource/piece/card/score/award/phase facts and query/import byte purity. Actual harness decoding failure shrank to seed0 before correction; it is retained honestly as a harness defect. Fresh `private historical evidence` approves exact final hash and kills six compile-valid controls; an initially surviving spectator hand leak is closed by an explicit absence assertion. All applicable standing gates pass **197 tests, ten doctests and98.71% lines(5954/6032)**. No product/rule/dependency/quality change; action vectors and malformed boundaries remain T30b/T30c.

**T30b evidence:** [Durable property evidence](property-evidence.md) records32 bounded decision-vector cases, publicly completed setup, all21 complete action assemblers, exhaustive placement membership/validation agreement, independent facts, full query/rejection bytes and the existing complete resume/replay consumer. Actual pinned cases reach1,274 accepted commands across16 families and all21 rejection families; omitted rare/terminal generation is linked to complete direct fixtures. Fresh `private historical evidence` approves exact final hashes and kills/restores eight compile-valid controls. Original accepted EndTurn RNG failure shrinks to `[0,0,16758,4083]`, explicitly a local minimum; optional ninth hidden-score experiment is retained as partial. Final gates pass **198 tests, ten doctests and98.71% lines(5954/6032)**. No production/rule/dependency/quality change; malformed boundaries remain T30c.

**T30c evidence:** [Durable property evidence](property-evidence.md) records64 pinned configurations, publicly reachedrevision1, 2,048 bounded malformed input instances/3,072 rejection calls, sanitized errors, complete byte preservation and exact next public continuation. Fresh `private historical evidence` approves exact hashes, allfour configuration counts and eight compile-valid killed/restored controls; actual duplicate-key failure shrinks to all-zero synthetic inputs and its persisted case passes restored production. Final gates pass **199 tests, ten doctests and98.72% lines(5955/6032)**. All original T30 acceptance is complete, with explicit bounded-generation gaps supported by complete direct fixtures; no genuine engine regression, production hook, dependency/rule/quality change. C10 final checkpoint evidence remains next.

**Description:** Add reproducible shrinking property tests for legal/rejected command sequences and malformed snapshot/wire cases, without turning test helpers into a product subsystem.

**Modules:** Model/API verification. **Dependencies:** [T29](#t29).

**Rule trace:** Model/API/quality contract; no additional tabletop rule.

**Acceptance criteria:**

- [x] Reached states conserve resources/pieces/cards and agree on phases/awards; every rejected command preserves byte-stable snapshot/RNG/revision/offer and every query is side-effect free.
- [x] Generated placement candidates and chosen-command validation agree at the same revision; repeated snapshots/replays and randomized valid sequences preserve exact results.
- [x] Malformed bounded imports reject without panic/data loss; shrinking prints only synthetic fixture data and retains minimal seed/command regressions.

**Verification:**

- [x] `cargo test --locked --test model_invariants`
- [x] `cargo test --locked --test property_sequences`
- [x] `cargo test --locked --test api_contract`

**Files likely touched:** `tests/model_invariants.rs`, `tests/property_sequences.rs`, `tests/api_contract.rs`, `tests/support/mod.rs`.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

<a id="c10"></a>
### C10: Complete rules and replay evidence

- [x] T28–T30 acceptance and focused checks pass.
- [x] All BR001–BR038, complete games, properties, malformed boundaries and replay fixtures pass without private-state shortcuts.
- [x] Run all applicable standing checkpoint commands in plan.md; record actual results and limitations.
- [x] Present checkpoint evidence for human review before proceeding.

**C10 evidence:** [Durable checkpoint report](c10-checkpoint.md) records all eight reviewed T28–T30 leaf commits through source/test `380ca5e`, all38/106 clause references, six complete public games across four configurations, bounded private replay and all-phase resume/replay, plus three pinned shrinking properties and2048 malformed input instances. Final checkpoint gates pass **199 tests, ten doctests and98.72% library lines(5955/6032)**, release/rustdoc and a fresh passing advisory scan. Eight focused commands pass36 nonzero tests; `private historical evidence` retains exactsource/count/exit/duration mapping. Independent `private historical evidence` checks complete acceptance, source hashes, controlled failures/restores, durable evidence and remaining scope. Initial privacy/RNG/writer survivors, harness/tooling failures and the optional partial ninth control remain honestly retained. Existing autonomous authorization permits measured-performance/final-handoff T31–T33; no rule, dependency, quality requirement, publishing, deployment, binding or hosted-CI change/claim.

## Phase 11: Measured performance and consumer handoff

<a id="t31"></a>
### T31: Measure release performance and retained memory

- [x] T31 complete

**T31 evidence:** [PERF.md](../PERF.md) records 20 optimized native/wire/query/view/snapshot cases, the validated public 77-turn hard-route fixture, three pinned raw Criterion runs and earlier unpinned runs. Same-binary timing varied beyond 5% for some cases; the approved 5% margin remains a manual review policy, not a noise bound. Safe external Massif phase captures at 0/1/16/64 games and receipts separate shared topology, inline size, useful heap, allocator extra and stack; corrected N=64 repeats agree. Nonrandom RNG and bounded history are checked by C10 and the fixture. Raw samples, profiler data, commands, hardware, hashes and the initial flawed cold control are retained in private original performance evidence; no absolute budget or globally worst-case claim is made. All 199 tests, 10 doctests, 98.72% line coverage, fmt/check/clippy/release/rustdoc and 20 benchmark smoke cases pass. The first advisory attempt failed on a read-only lock; the authorized refresh passed. [Independent review](../performance/review-t31.md) approves exact candidate and evidence hashes. No engine behavior, dependency, rule or quality threshold changed.

**Description:** Register the Criterion benchmark target and measure representative completed engine paths on documented native reference hardware.

**Modules:** Complete engine benchmarks. **Dependencies:** [T30](#t30).

**Rule trace:** Model/API/quality contract; no additional tabletop rule.

**Acceptance criteria:**

- [x] Benchmarks cover accepted/rejected commands, targeted legal queries, views, snapshots, hard Longest Route graphs and core-versus-wire cost, using fixed realistic fixtures with valid revisions and intact validation.
- [x] PERF.md records CPU/RAM/OS/target/toolchain/profile/commands, raw timing/variability and retained per-game/board/receipt memory; history length does not enlarge current position and nonrandom actions retain RNG.
- [x] Use an external safe profiler when heap measurements are needed, record its exact command/version, and distinguish size_of values from heap footprint. Establish a baseline without inventing an absolute budget or claiming a proven speed advantage.

**Verification:**

- [x] `cargo bench --locked --bench engine -- --save-baseline approved`
- [x] `cargo bench --locked --bench engine -- --baseline approved`
- [x] Inspect memory/profile evidence and fixture setup so benchmark repetitions perform the intended accepted/rejected work.

**Files likely touched:** `Cargo.toml`, `benches/engine.rs`, `benches/fixtures.json`, `PERF.md`.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

<a id="t32"></a>
### T32: Review architecture and performance evidence

- [x] T32 complete

**Evidence:** [Independent final source/performance review](t32-review.md) approves clean T31 `698e6ce` with no Critical or Required findings and no necessary optimization. All 48 Rust source-tree files and tests match verified C10; imports, shared validation, native/wire/save contracts, privacy, atomicity, bounded position, optional RNG and exact route invalidation were reviewed against every provider specification. All 60 timing medians, 56 memory phase totals and the committed evidence checksums reconcile. The review records the unchanged 5% manual margin and same-binary noise; no absolute budget is justified without a concrete consumer workload/hardware/percentile objective. Exact retained T31 gates verify 199 tests, ten doctests, 98.72% lines, all-target Clippy/build/rustdoc, 20 benchmark smoke cases and refreshed advisories. Those successful unchanged-source checks were reused rather than rerun. This documentation-only closure changes no executable behavior or approved requirement; current-status prose and the compiling consumer handoff remain T33/C11.

**Description:** Inspect the completed implementation for maintainability, security, duplicated checks and measured bottlenecks. Prefer an unchanged implementation when extra complexity has no demonstrated benefit.

**Modules:** Engine quality review. **Dependencies:** [T31](#t31).

**Rule trace:** Model/API/quality contract; no additional tabletop rule.

**Acceptance criteria:**

- [x] Imports match the four-module direction; public native/wire/save contracts agree; shared rule checks are not duplicated, and no oversized module, unsafe shortcut, silent error, skipped assertion or speculative subsystem is accepted.
- [x] Review the measured hot path/position copies/RNG/road invalidation and the 5% repeatable-regression policy; record current absolute budget proposals for human review instead of silently choosing hardware-independent numbers.
- [x] Any necessary correction/optimization becomes a narrowly scoped follow-up task with before/after evidence and unchanged conformance/privacy/replay tests; blocking findings must be resolved before final handoff, without an unbounded refactor.

**Verification:**

- [x] `cargo clippy --locked --all-targets -- -D warnings`
- [x] `cargo test --locked --lib --tests`
- [x] Review the source/imports, public rustdoc, CONSTRAINTS.md and PERF.md against all provider specs.

**Files likely touched:** `tasks/todo.md`, `CONSTRAINTS.md`, `PERF.md`.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

<a id="t33"></a>
### T33: Deliver the documented, verified engine library

- [x] T33 complete

**Evidence (10 November 2026):** [C11 checkpoint](c11-checkpoint.md) records all standing local gates and the compiling README/rustdoc consumer example: 199 tests, 11 doctests, 98.72% library lines (5,955/6,032), and 20 benchmark smoke cases. The sandbox advisory database lock failed; an authorized refreshed scan passed, with both outcomes retained. [Independent final review](t33-review.md) approves the exact technical candidate and all root success criteria without a required change. The gate archive (privately preserved original) retains 103 input hashes, command output and initial failures; its SHA-256 is `2740fcc4e6e42fea50f78e0d8ff945d60bbe05288ae6e62b53c255870dd27903`. Native measurement noise, manual architecture/5% review, unapproved absolute budgets and absent hosted CI/browser binding are explicit. No rule, quality threshold, executable behavior, dependency, publication or deployment changed.

**Description:** Finish a concise native consumer guide and run the complete agreed verification before calling the base-game engine complete. No deployment, published package, UI, or remote side effect is part of this task.

**Modules:** Public consumer documentation and final validation. **Dependencies:** [T32](#t32).

**Rule trace:** BR001, BR002, BR003, BR004, BR005, BR006, BR007, BR008, BR009, BR010, BR011, BR012, BR013, BR014, BR015, BR016, BR017, BR018, BR019, BR020, BR021, BR022, BR023, BR024, BR025, BR026, BR027, BR028, BR029, BR030, BR031, BR032, BR033, BR034, BR035, BR036, BR037, BR038.

**Acceptance criteria:**

- [x] README/rustdoc demonstrate create → view/legal choice → execute → projected receipt, host seat/seed ownership, save/resume and replay using examples that compile and require no duplicated rules.
- [x] All preceding tasks/checkpoints and required checks are complete, constraints accurately report enforced/manual status, and final-review benchmark/source-fixture evidence is linked with material limitations.
- [x] Final review confirms every root success criterion; no incomplete interpreter/binding/CI claim is hidden behind docs, and the library is ready for a consumer to integrate.

**Verification:**

- [x] `cargo test --locked --lib --tests && cargo test --locked --doc`
- [x] `cargo llvm-cov --locked --lib --tests --ignore-filename-regex '(^|/)(tests|benches)/' --fail-under-lines 90 && cargo deny --locked check advisories`
- [x] `cargo fmt --all -- --check && cargo clippy --locked --all-targets -- -D warnings && cargo build --locked --release && cargo doc --locked --no-deps`

**Files likely touched:** `README.md`, `src/lib.rs`, `CONSTRAINTS.md`, `PERF.md`, `tasks/todo.md`.

**Estimated scope:** Medium; one focused session. Split if actual scope exceeds five files or approximately two hours.

<a id="c11"></a>
### C11: Verified engine handoff

- [x] T31–T33 acceptance and focused checks pass.
- [x] Full quality commands and compiling consumer examples pass; native timing/memory and limitations are documented, with no unapproved publishing.
- [x] Run all applicable standing checkpoint commands in plan.md; record actual results and limitations.
- [x] Present checkpoint evidence for human review before proceeding or declaring the engine complete.

<a id="t34"></a>
### T34: Preserve checkpoint and fixture-oracle evidence

- [x] T34 complete

**Evidence (10 November 2026):** The privately preserved original evidence index maps
historical temporary labels to 991 checksummed archive members and 479 verified
Git-backed copies. All direct references resolve except documented intentional
tool-error paths, build output and reconstructed source trees. C10 reports,
failure/control logs and independent full-game and victory oracle inputs,
scripts and results are retained byte-for-byte. Extraction, source-freeze and
three-case independent arithmetic reproduction pass; the original verification record (privately preserved) and independent review (privately preserved original)
retain their original audit outputs. No engine, rule, dependency, fixture data
or quality-policy change; existing full gates were not repeated.

**Description:** Resolve the review finding that checkpoint reports, logs and
independent full-game oracle preparation are retained only in temporary paths.
Preserve referenced historical evidence with byte hashes, portable extraction
instructions and honest provenance; do not regenerate historical proof.

**Dependencies:** [C11](#c11). No engine, rule, dependency or quality-policy change.

**Acceptance and verification:**

- [x] Inventory checkpoint and fixture evidence references and report omissions.
- [x] Preserve relevant reports, gate/control results and independent oracle
  sources, inputs and outputs without caches, binaries or real private data.
- [x] Verify archive members, byte hashes, extraction, references and unchanged
  executable inputs; obtain independent review before recording completion.

## Execution notes

Task and checkpoint evidence is recorded at each task or linked retained report. New findings must become focused tasks with
explicit dependencies; they cannot be hidden by checking off failed acceptance.
