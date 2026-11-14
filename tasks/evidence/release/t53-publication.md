# T53: Verified v0.1.0 publication

**Published 14 November 2026:** [Hex Traders v0.1.0](https://github.com/strange0g/hex-traders/releases/tag/v0.1.0)
is a public, non-draft, non-prerelease source release in
[`strange0g/hex-traders`](https://github.com/strange0g/hex-traders).
T52 and T53 are complete for this initial release under
[R01](../../../CONSTRAINTS.md#r01-initial-source-release-with-unresolved-native-timing-acceptance).
**T51 remains incomplete:** no matching-condition timing pass or confirmed
repeatable regression is established. The unchanged 5% policy applies to future
work; R01 expires with this publication and grants no future-release waiver.

## Immutable source and independent assessment

| Identity | Verified value |
|---|---|
| Released commit | `6f2d992581ab13bb782b9ffde9dd1ca982f85b06` |
| Released tree | `6a821cac3bba0bfab645c047b7f3c9770e2e7878` |
| Annotated `v0.1.0` tag object | `8ec6847cbe1832f8ba29471395493d807c9663ac` |
| Independently reviewed PR #9 head | `1b1a7e195bbde0c52725eaf0d220738fdcde02cd` |
| Release publication time | `2026-11-14T12:39:24Z` |

The PR head and released commit have identical trees; independent assessments
were rebound to the merged release identity before tagging. Full fresh reports
are copied byte-for-byte from retained originals:
[code review](../../ship-reports/code-review.md),
[security review](../../ship-reports/security-review.md), and
[test review](../../ship-reports/test-review.md).
Their [manifest](../../ship-reports/review-manifest.json) records sizes and SHA-256
hashes. Each report states its scope and then-pending checks; completed
post-publication verification is recorded here, without rewriting those reports.

The annotated tag was pushed explicitly and the release created with
`gh release create v0.1.0 --repo strange0g/hex-traders --verify-tag --title "Hex Traders v0.1.0" --notes-file docs/releases/v0.1.0.md`.
Release readback contains the prepared notes plus an explicit released-SHA and
R01 appendix. Cargo, changelog and tag agree on version 0.1.0. Remote tag/API
readback and a new remote tag clone confirm the tag object, peeled commit and
tree above. Later completion documentation on main does not change this version.

## Actual checks and protections

All three required checks, `quality`, `coverage` and `advisories`, completed
successfully for each exact assessed revision, under GitHub Actions app 15368:

| Context | Successful run | Commit |
|---|---|---|
| [PR #9](https://github.com/strange0g/hex-traders/pull/9) | [43595368109](https://github.com/strange0g/hex-traders/actions/runs/43595368109) | `1b1a7e195bbde0c52725eaf0d220738fdcde02cd` |
| Protected main | [43595653990](https://github.com/strange0g/hex-traders/actions/runs/43595653990) | `6f2d992581ab13bb782b9ffde9dd1ca982f85b06` |
| Release tag | [43595890969](https://github.com/strange0g/hex-traders/actions/runs/43595890969) | `6f2d992581ab13bb782b9ffde9dd1ca982f85b06` |

Live readbacks retain required PRs, conversation resolution, strict checks bound
to that app, linear history, and branch deletion/force-push prohibitions. Version
tag update/deletion prohibitions are active, with no bypass actors. API-added PR
defaults and omitted false tag parameters were compared semantically with the
authored requirements. Provider secret scanning, push protection, private
vulnerability reporting and dependency alerts remain enabled. Non-provider
secret patterns and validity checking are disabled; those checks are not claimed.
[T50](t50-hosted.md) retains actual enforcement
controls; [T50a](t50a-ci-every-pr.md) covers every-PR triggering.

## Native clone and external consumer

The new remote tagged clone remained clean, with all 239 tracked content hashes
and Git modes unchanged before, after and after the advisory retry. Rust 1.98.1
on `x86_64-unknown-linux-gnu` produced these results:

| Check | Result |
|---|---|
| All-target check, release build, format, warnings-denied Clippy, rustdoc | Pass |
| Library/integration tests | 200 passed, zero failed or ignored |
| Documentation tests | 11 passed, zero failed or ignored |
| Library line coverage, unchanged 90% floor | 5,955/6,032 lines, 98.72% |
| Benchmark smoke | 20/20 passed; no timing comparison |
| Branding self-tests, tree and reachable history | Pass; 53 controls |
| Cargo package list and diff whitespace | Pass; 99 package files |
| Fresh advisory check | Pass on identical authorized retry |

Of 22 initial clone commands, 21 exited zero; the advisory command and initial
pipeline exited 1 because the sandbox could not lock a read-only database path.
The identical `cargo deny --locked check advisories` retry passed. Both outcomes
are retained. RustSec database HEAD was
`4ef2df8e806a65e2f1ecc14f56f51249062b23d6`. The 31-file clone-evidence manifest
was independently verified; its SHA-256 is
`309c923f467a10e938fd5b795246ec141e38e42076f9450fa7aab07952c8573b`.
The earlier 22-command candidate run passed and its tree matches the release.

A separate Cargo consumer used the documented Git dependency and exact README
Rust example. Lock generation, `cargo run --locked` and locked metadata passed.
The lockfile pins
`git+https://github.com/strange0g/hex-traders?tag=v0.1.0#6f2d992581ab13bb782b9ffde9dd1ca982f85b06`.
The example exercised create/query/execute, safe projection, snapshot roundtrip
and replay equivalence assertions with an explicitly synthetic fixture. The
[earlier path consumer](t47-guide.md) also passed.

## Generated source archives and disclosure

| GitHub archive | Bytes | SHA-256 |
|---|---:|---|
| ZIP | 1,259,670 | `e4d1efd8d09dd56618d26adb2afac52e8d2a2a311174b0dac976bce8ccd8cf52` |
| tar.gz | 1,090,713 | `b5e34df3284478c78315a833e84a1183b614d4e33ae9dab4b182370a525ca46d` |

After stripping GitHub's prefix, each archive has exactly the tagged tree's 239
regular files and 78 ancestor directories. Every file's bytes, size and hash
match its tagged Git blob. Inspection rejected unsafe/duplicate names and special
members without executing archived content. Project and third-party notices
are present; branding scans include nested public archives. No tracked `.git`,
`.env`, `__pycache__` or `.pyc` path is included.

Existing public T50 nested logs contain eight current local-checkout-path
occurrences and 631 hosted-runner paths. The source archives also contain three
ordinary temporary paths: one in T50 evidence and two example commands in agent
tooling instructions. These are previously audited metadata, not known private-preservation/original paths
or identified credentials. Limited high-confidence credential/private-path scans
found no additional match; they cannot prove the absence of every disclosure.
The release therefore makes no claim that all absolute paths are absent.

## Corrected operation and completion evidence

A pre-publication clean-worktree assertion detected an untracked scanner-created
Python cache. The shell lacked fail-fast handling and proceeded with the approved
publication. Only the coordinator-created cache was removed; the tracked tree
was unchanged. The clean remote clone and matching source archives confirm it
was never tagged or published. Subsequent mutation commands use fail-fast handling
and Python bytecode generation is disabled. The tag was never moved.

The active `Hex-Traders` main checkout was verified clean and fast-forwarded from
`01296e0fee8c89a5efbbf08e215c2673f9e1f8c9` to the released commit, with matching
tree and tag. Exact original reports, command outputs/producers, source manifests,
readbacks, consumer inputs/results and raw source archives are privately retained
with verified sizes/hashes and a portable archive; build caches are excluded.
This documentation-only completion follows through a reviewed PR with all three
required checks. T53/C21 documentation completion entries take effect only after
those checks pass and the PR merges through protected main; until then, final
documentation closure is pending. Its later main revision is separate from the
immutable tag.

Recovery remains the [T52 procedure](t52-decision.md#recovery): pause adoption,
retain the immutable version and reproduction, review a focused correction,
merge through protected main and validate a new release version. Never move or
delete `v0.1.0`, force-push main or bypass checks. No recovery-time promise exists.
No CPU/power policy, service, global affinity or unrelated process was changed.
Browser/Wasm, hosted gameplay and registry publication remain outside scope.
