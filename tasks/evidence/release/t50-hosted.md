# T50 / C19: Verified hosted enforcement

Verified on 13 November 2026 after direct publication approval. Public repository:
[strange0g/hex-traders](https://github.com/strange0g/hex-traders).
Bootstrap assessed commit: `70faaf534a1ee1ee7c09bf55aaf931bb7daeb11c`.
Only the fresh release history was pushed; original preparation/private refs were
never exported. No version tag or release has been published at this checkpoint.

## Successful checks and active settings

[Bootstrap run 43494093519](https://github.com/strange0g/hex-traders/actions/runs/43494093519)
passed `quality`, `coverage` and `advisories` on the exact assessed SHA. Actual
check-run names and GitHub Actions app ID `15368` match the required ruleset
contexts; two separate successful Dependabot setup checks are additional checks.
The retained full logs record actual runner image/version, Rust/checker versions,
commands and results. Hosted smoke checks do not establish native performance.

Main ruleset `27227191` and version-tag ruleset `27227193` are active, with empty
bypass lists. Main requires current passing checks, PRs, resolved conversations,
linear history, and prohibits deletion/force updates. Squash is the sole merge
method, zero human approvals permits the sole maintainer, and merged branches are
deleted. Version tags permit creation and prohibit updates/deletion. Readback
verifies every specified policy and the effective main rules. GitHub adds an empty
reviewer list and a Copilot option applying only to nonzero approval counts, and
omits a branch-only fetch/merge parameter from tag updates; these canonical fields
do not change the zero-approval or immutable-tag policies.

Default branch is `main`; description/topics, Issues and committed issue forms
are present, with Wiki/Projects/Discussions disabled. Weekly Cargo/Actions
Dependabot configurations retain five-PR limits and reviewed updates. Dependency
alerts/security updates, private vulnerability reporting, secret scanning and
push protection are enabled and read back. Workflow token defaults are read-only
and cannot approve PRs. No CodeQL, generic-secret detection or secret-validity
assessment is claimed; the required dependency gate is the refreshed RustSec scan.
Automatic merge is disabled.

## Deliberate failure and owner-update proof

Every final control PR was observed `BLOCKED`, closed without merging, and its
remote branch deleted after retaining full logs and exact run/head identities.
No production threshold, scanner or assertion changed.

| Runtime synthetic control | Run | Required failed check | Observed gate exit |
|---|---|---|---|
| Malformed Rust formatting | [43494425893](https://github.com/strange0g/hex-traders/actions/runs/43494425893) | quality | 1 |
| Failing assertion after successful Clippy | [43494440967](https://github.com/strange0g/hex-traders/actions/runs/43494440967) | quality | 101 |
| Uncovered statements: 84.65% library lines, unchanged 90% floor | [43494459953](https://github.com/strange0g/hex-traders/actions/runs/43494459953) | coverage | 1 |
| Runtime `time = "=0.1.44"`: RUSTSEC-2020-0071 | [43494478806](https://github.com/strange0g/hex-traders/actions/runs/43494478806) | advisories | 1 |
| Byte-generated staged prohibited probe, neutral diagnostics | [43495154447](https://github.com/strange0g/hex-traders/actions/runs/43495154447) | quality | 1 |
| Git tool exits 127; scanner fails closed | [43494517770](https://github.com/strange0g/hex-traders/actions/runs/43494517770) | quality | 2 |

The first branding attempt supplied an untracked file, outside the defined release
index, and passed. Its complete initial run is retained. The corrected probe is
staged only in the runner index; its prohibited bytes are never committed or
printed. The production scanner is unchanged.

Owner `strange0g` attempted a direct failing update to a disposable branch under
an exact copy of main's policies. GitHub rejected it with exit 1 / `GH013`, naming
the PR requirement and failing `quality` check. The branch SHA and main stayed at
the bootstrap SHA. The temporary branch/ruleset were then removed; main/tag
protections were never weakened. The six independent blocked PR states establish
merge rejection without attempting to merge a negative control.

## Local verification and retention

All ten standing native commands passed against the current documentation
candidate: **200 tests, 11 doctests, 98.72% library lines (5,955/6,032)**,
compiler/fmt/Clippy/release/rustdoc/advisories and 20 benchmark smoke cases.
The recorded commands took approximately 120 seconds; the routine timing target
did not truncate a check. Current source/tests/fixtures/benchmarks, dependencies,
workflow and ruleset files are unchanged from the assessed bootstrap commit.
Current-status prose and CI/licensing badges now reflect actual enforcement;
all normative quality requirements and historical checkpoint facts remain intact.

Raw API readbacks, runner/job logs, PR states, owner rejection, cleanup, local
commands/input hashes and execution corrections are retained in
[the checksummed archive](t50-hosted.tar.gz). SHA-256:
`980aa5404f62cb73d0aade9f5b4d2381eaae3c3dc7d5294533540502ae4914fa`.
Its 59 members contain 2,241,099 source bytes; extraction hashes and recursive
branding/archive validation pass. `MANIFEST.json` identifies every other member.
No private original paths, source history or real game data are included.

T51 still needs the same-condition native performance verdict. T52 requires fresh
independent frozen-candidate reviews and GO; T53 owns the immutable release and
clean remote/tag/consumer/source-archive verification. This evidence PR must pass
the active required checks and independent review before merging.
