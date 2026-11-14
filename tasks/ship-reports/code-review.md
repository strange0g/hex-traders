# Hex Traders v0.1.0 release code review

## Review Summary

**Verdict: APPROVE documentation candidate `1b1a7e195bbde0c52725eaf0d220738fdcde02cd`, tree `6a821cac3bba0bfab645c047b7f3c9770e2e7878`.** No unresolved Critical or Required finding remains in the scoped delta. The informed release authorization, disclosed limitation and mandatory recovery procedure suffice for a release-only GO once the outstanding exact-revision publication checks pass; this is not a claim that those checks or T51 have passed.

**Overview:** This candidate adds four documentation changes recording the maintainer's informed instruction to proceed with the initial source release while T51 remains unresolved. It does not change engine code, fixtures, dependencies, tests, workflows or quality thresholds.

### Critical Issues

None found in the scoped delta.

### Required Changes

None remain. The earlier contradiction in CONSTRAINTS.md's introductory summary was corrected to “No threshold has changed; the release-only exception is recorded below.” Direct comparison of the revised frozen candidate against the previous candidate shows this is the only additional change.

### Optional

None.

### Nits

None.

### What's Done Well

- R01 records the affected release path, concrete reason, informed maintainer approval, owner `strange0g`, and expiry at publication completion or 28 November 2026. It expressly includes T51's unresolved timing acceptance and incomplete current public evidence, while leaving T51 incomplete.
- The release notes expressly disclaim a timing pass. The new decision record preserves the distinction between valid captures and a defensible matching-condition result, and retains the initial advisory error alongside its successful identical retry.
- Recovery preserves the immutable version and evidence, requires a protected and independently reviewed correction, and publishes a new version. It expressly forbids moving/deleting `v0.1.0`, force-pushing main or bypassing checks.

### Five-axis assessment

| Axis | Assessment |
|---|---|
| Correctness | The public verification and timing summaries match the retained native, full-block and no-launch reports. R01 is release-only authorization, not a T51 pass or acceptance of a demonstrated regression. The corrected introduction is consistent with the explicit exception record. |
| Readability | The four documents clearly distinguish release approval, historical evidence, current uncertainty, pending publication checks and future requirements. |
| Architecture | Direct tree comparison shows no changes in `src`, `tests`, `benches`, Cargo manifests/lockfile, toolchain, dependency-deny configuration, workflows or scripts. Four-module contracts and public interfaces remain unchanged. |
| Security | The delta adds no execution or trust-boundary behavior and exposes no private machine paths, raw benchmark records, seeds or credentials. R01 explicitly excludes private evidence publication and missing security checks. Source/package/archive and live protection checks remain mandatory. |
| Performance | T51 remains unresolved. The seven earlier runs and later four-run block do not prove a matching-condition repeatable regression or a passing comparison; later read-only screening launched no additional benchmark. R01 does not alter the 5% rule or waive future requirements. |

### Authorization and release disposition

The coordinator supplied trusted context that the maintainer explicitly instructed “proceed with the release” after repeated disclosure of unresolved T51. CONSTRAINTS.md permits human-approved exceptions with a rule, affected paths, reason, owner and bounded expiry. R01 supplies those fields and confines the authorization to `v0.1.0`. The prior NO-GO reports remain truthful historical assessments made before this informed instruction; they need no rewriting.

A release-only GO can acknowledge this performance uncertainty without pretending the performance requirement was met. No confirmed regression tradeoff, missing security check, private evidence publication, host setting change or future release waiver follows from that instruction. Outstanding exact-SHA local/hosted gates, protections, package/archive and external-consumer checks must be completed truthfully before declaring publication verified.

### Verification Story

- **Tests reviewed:** yes, through the retained complete-scope code assessment and fresh native verification records. The latest unchanged-engine record reports 200 tests and 11 doctests, zero failures or ignored cases, 98.72% library line coverage, and 20 benchmark smoke cases. The delta contains no test or executable behavior change; this reviewer performed no test/mutation run.
- **Build verified:** retained native build/check/lint/doc evidence reviewed, not rerun. Exact-candidate hosted/local gates remain the coordinator's publication prerequisite and are not represented here as newly passed.
- **Security checked:** yes within the documentation delta and retained source review scope. The independent security assessment and fresh live/advisory readbacks remain separate evidence.
- **Identity:** clean frozen checkout verified at the exact commit/tree above. Only the four stated documents differ from `01296e0fee8c89a5efbbf08e215c2673f9e1f8c9`; direct diff confirms source/test/fixture/dependency/tool/workflow inputs are unchanged. The base context's principal specs, constraints, plan/status, manifests and release notes match the retained earlier assessment byte-for-byte.
- **Recovery:** mandatory immutable-version procedure at `tasks/evidence/release/t52-decision.md:54` covers privacy/state/RNG/replay failure, serious consumer defects and confirmed unacceptable repeatable performance regressions. No recovery-time guarantee is claimed.

Not checked: new builds/tests/benchmarks, live hosted state, final tag clone or generated release archives; no source/host/remote writes occurred.
Risk: matching-condition performance acceptance and complete current public performance evidence remain unresolved; R01 permits only this informed initial release.

## Final merged release identity — 14 November 2026

**Verdict: APPROVE exact release candidate `6f2d992581ab13bb782b9ffde9dd1ca982f85b06`, tree `6a821cac3bba0bfab645c047b7f3c9770e2e7878`, with the same scoped R01 disposition and remaining publication prerequisites above.**

The clean final checkout's HEAD is the merged commit above. Its tree is exactly equal to the independently reviewed `1b1a7e195bbde0c52725eaf0d220738fdcde02cd` tree; direct commit-to-commit diff is empty and exits zero. This rebinds approval to the final immutable release identity without changing the reviewed content or repeating broad review.

The coordinator reports that protected PR #9 merged after the exact PR-head quality, coverage and advisories checks passed under GitHub Actions, with all 22 fresh native gates passing, and that current live main/tag protections retain their authored requirements. These additional checks/readbacks were not independently rerun in this identity-only follow-up. Main CI for `6f2d992581ab13bb782b9ffde9dd1ca982f85b06` must still pass before tagging; exact-tag clone, consumer and archive verification remain mandatory after publication.

Not checked: additional tests/builds/benchmarks, hosted checks or protection readbacks; this follow-up checked tree equality only.
Risk: T51 remains unresolved, and R01 permits only this initial release; no tag or publication-completion verification is implied by this approval.
