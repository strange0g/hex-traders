# T50a: CI for every PR base branch

The requested T49/T50 review found that the workflow's PR branch filter excluded
PRs targeting feature/release branches. The publication specification requires CI
on every PR. The correction deletes that filter from the ordinary `pull_request`
event; main/version-tag push filters and all job/security settings remain intact.

## Fresh local verification (14 November 2026)

A disposable YAML contract check failed against the original filter, passed with
the correction, and failed when the filter was restored. Whole-document comparison
confirmed that only the PR filter changed. No test framework or dependency was added.

All ten standing native gates passed: compiler check, release build, formatting,
warnings-denied Clippy, library/integration tests, doctests, documentation build,
coverage, advisory check and benchmark smoke. Results: 200 tests (zero failed or
ignored), 11 doctests, 98.72% library line coverage (5,955/6,032; unchanged 90% floor),
and 20 benchmark smoke cases. Branding self-test/tree/history and diff checks are
passed before the task commit. Fresh Terra review approved the focused change
with no Critical or Required findings.

Original review, fresh logs, RED/GREEN/restored-filter controls and independent
review are privately preserved. All subsequent main updates use the protected PR
path; hosted run and merge readbacks are retained separately after publication.

No engine rules, fixture facts, dependency graph or quality requirement changed.
This focused fix does not resolve T51 timing or supply a T52 release decision.
