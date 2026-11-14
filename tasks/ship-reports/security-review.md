# Security Audit Report — initial release preparation

## Summary

- Critical: 0
- High: 0
- Medium: 0
- Low: 0
- **Security recommendation: GO within the reviewed scope**, subject to the remaining exact-release hosted checks, repository protection readback, package/source-archive privacy verification and consumer verification. This security verdict is not a T51 timing pass or overall publication verification.

## Identity and scope

Reviewed clean candidate `e6556a6401178585cea1c1d9031e5a3e512659a9`, tree `c5f126a2a8ed9c2e49423ff1c31bcc394ca3fef6`, on 14 November 2026. Its only parent is `01296e0fee8c89a5efbbf08e215c2673f9e1f8c9`, tree `6e6aab255c7090c77a6a1f1f0073661004c9cf78`. The entire delta consists of `CONSTRAINTS.md`, `docs/releases/v0.1.0.md`, `tasks/todo.md`, and new `tasks/evidence/release/t52-decision.md`. The source, tests, lockfile, manifest/package selection, CI, scanner, ruleset files and license/agent-tooling notices have no delta. The checkout was clean before and after inspection; `git diff --check` passed.

Read the project guidance, constraints and both release specifications, the complete four-file delta, current Cargo manifest, advisory configuration, workflow, rulesets, security policy, license and attribution documents, and the prior independent security, ship and test evidence. Reused the prior broad security review rather than repeating it. Assessed new public claims against retained private originals, including their raw native test/coverage/smoke/advisory logs, capture-control log, four-run record and 20-second observation. Both relevant private archive hashes match their preservation records. No original or raw private evidence was copied into the public tree.

## Trust boundaries and STRIDE

Assets are private game state and randomness, release and check integrity, and private evidence. The host-selected seat/viewer boundary has spoofing and elevation risks; the existing host authentication requirement and scoped projections remain unchanged. Command, snapshot and replay inputs carry tampering, repudiation, disclosure and denial-of-service risks; the prior audit reviewed bounded parsing, validation, atomicity and rollback on the same unchanged source. Untrusted PRs and dependencies cross an execution boundary: current workflow uses ordinary pull requests, read-only token permissions, nonpersistent checkout credentials, locked dependencies and SHA-pinned actions. The documentation/publication boundary can disclose private paths or evidence, or misrepresent acceptance: all four full changed files were reviewed and scanned for high-confidence credential and private absolute-path patterns; no matches were found, and R01 expressly limits the exception to unresolved timing. A pattern scan cannot exclude every secret or semantic disclosure.

## Findings

No Critical, High, Medium or Low exploitable security finding was identified in the changed documentation or in the unchanged package/workflow controls examined for this release. The earlier independent security audit reported no such finding for the engine and repository at the parent revision; this review does not claim to have independently repeated its broad source and reachable-history examination.

## Evidence and limitations

- The raw retained native results substantiate 200 passing tests and 11 passing doctests, with zero failed or ignored; the coverage total reports 6,032 lines and 77 missed, or 5,955 covered and 98.72%. The raw smoke log has 20 cases and 20 successes. The retained advisory log records an initial database-lock error and an identical retry that returned `advisories ok`; source and lockfile are unchanged. The retry was not rerun by this reviewer.
- The retained green control log has 31 passing entries; the original block record contains four valid O1/C1/C2/O2 captures. The archived analysis records 20 cases and concludes the timing gate is unresolved. The subsequent read-only observation has 21 samples over approximately 20 seconds and reports changing cap/turbo policy. The public summary does not claim a pass or a confirmed repeatable regression. These checks corroborate the reported figures; they do not independently remeasure performance.
- `CONSTRAINTS.md` records R01's scope, named owner, approval date and expiry while retaining security and other gates. `tasks/todo.md` leaves T51 incomplete. Release notes explicitly disclose the timing uncertainty and host obligations. The public decision calls subsequent hosted, protections, tag, archive and consumer checks pending; it does not report them as completed.
- The Cargo include list continues to exclude task/evidence archives and agent tooling; the prior exact-source package listing reported 99 intended files. The added task decision is outside that include list, while the changed release notes are included. Full repository source archives include tracked material, so generated GitHub ZIP/tar contents must still be checked against the actual tagged tree.
- MIT and Apache project license files and the separately attributed third-party tooling notice are unchanged. The prior audit reviewed the original current tree, reachable history, archive members and security settings; its limited-pattern scan found no credential match and it verified the archive derivations then in scope. New source archives and live settings were not independently checked here.
- The fresh exact-parent Cargo-deny result is a known-advisory opinion, not protection against unknown or malicious dependencies. Do not represent it as a fresh scan of a different graph. PR and release checks must remain bound to the actual final commit. If a squash merge changes the commit identity, verify that its tree equals the reviewed tree and that required checks pass on the merged commit before tagging.

## Positive observations

Release-only wording clearly separates performance uncertainty from a pass; host authentication and private-data responsibilities are retained. Existing CI constrains PR privileges and the package include list excludes private evidence and agent tooling. Recovery preserves immutable version tags and requires reviewed corrections through protected main.

## Recommendations

Complete the pending live repository settings and exact-commit CI checks, and verify clean tagged-clone consumer and generated archive/package contents before representing publication as verified. Preserve raw evidence privately and review future dependency updates for provenance and reachability; retain the advisory gate. Do not carry R01 into later releases.

Skipped duplicate broad source/security scans, new tests/builds/benchmarks and remote or host mutations. Risk: exact-release hosted checks and generated archives remain outside this review, and finite pattern scans and known-advisory checks cannot rule out unknown disclosures or vulnerabilities.

## Final amended-candidate recheck

The clean amended candidate is `1b1a7e195bbde0c52725eaf0d220738fdcde02cd`, tree `6a821cac3bba0bfab645c047b7f3c9770e2e7878`. Compared with the previously reviewed `e6556a6401178585cea1c1d9031e5a3e512659a9`, only the `CONSTRAINTS.md` introduction changed: it now accurately states that thresholds are unchanged and the release-only exception is recorded below. The sentence introduces no credential or private path, and `git diff --check` passed. The overall delta from `01296e0` remains the same four documentation files; source, dependency, workflow, package selection and notices remain unchanged. The scoped security GO and outstanding checks above apply to this amended candidate. Final merged-tree equality and hosted CI remain the coordinator's responsibility.

## Merged-candidate identity recheck

Clean merged main is `6f2d992581ab13bb782b9ffde9dd1ca982f85b06`, parent `01296e0fee8c89a5efbbf08e215c2673f9e1f8c9`, tree `6a821cac3bba0bfab645c047b7f3c9770e2e7878`. Its tree equals the reviewed `1b1a7e195bbde0c52725eaf0d220738fdcde02cd` tree exactly; `git diff` between them is empty. The scoped security GO therefore applies to that merged tree. Final main CI and generated release-archive checks remain pending; this identity check did not independently query the repository settings or run CI.
