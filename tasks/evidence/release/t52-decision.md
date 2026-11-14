# Initial release decision

**Disposition: proceed with `v0.1.0` only after all remaining source, security,
check, archive and consumer verification succeeds, with T51 explicitly unresolved.**

On 14 November 2026, the maintainer instructed “proceed with the release” after
being informed that native performance acceptance remained unresolved. The
release-specific authorization is recorded in [R01](../../../CONSTRAINTS.md#r01-initial-source-release-with-unresolved-native-timing-acceptance),
including its owner, scope and expiry. It permits this initial source publication;
it does not mark T51 complete or lower the standing performance requirement.

## Assessed source and evidence

The engine, fixtures, lockfile and toolchain are unchanged from reviewed source
commit `01296e0fee8c89a5efbbf08e215c2673f9e1f8c9`. This preparation changes only
release documentation and the explicit limitation record. Independent code,
security and test reviews assess the exact frozen documentation revision before
publication, and the annotated release tag records the final hosted-green commit.
Publication readback will identify that exact commit separately from any later
completion-documentation revision.

Fresh native verification on the unchanged engine passed 200 tests and 11
doctests with zero failures or ignored cases; line coverage was 5,955/6,032
(98.72%, against the unchanged 90% floor). Build, check, formatting, Clippy,
rustdoc, the 20-case benchmark smoke test, branding controls/tree/history and
package listing passed. The refreshed advisory scan passed; its initial
sandbox database-lock error and successful identical retry are both retained
privately. Hosted quality, coverage and advisories must pass on the actual
release revision before tagging.

The native timing collector repairs passed 31 controls and independent review.
Four full original/candidate runs produced valid samples, but changing CPU caps,
turbo and activity prevented a defensible matching-condition decision. A later
20-second read-only observation again found policy changes, so no additional
benchmark was launched. These facts support **unresolved**, not a repeatable
regression finding or a passing 5% comparison. All complete/partial historical
and new records stay private and recoverable; no private paths or raw evidence
are added to this release. The older baseline in `PERF.md` remains historical.

## Remaining publication checks

Before publication, independently approve the frozen candidate and verify its
three hosted checks, repository/tag protections, notices and source/package
selection. Publish an annotated `v0.1.0` only at that assessed commit, then read
back the non-draft release and exact peeled tag. A new remote tag clone must pass
the relevant full native/branding/advisory gates and compile an external Git
consumer. Validate GitHub's generated source ZIP/tar contents against the tagged
source and verify intended licenses, privacy and naming. Completion evidence
must identify actual commands, revisions and results; no unperformed check is
reported as passing.

## Recovery

If a privacy/state/RNG/replay failure, serious consumer defect or unacceptable
confirmed repeatable performance regression is found, pause adoption and retain
the affected immutable version and reproduction evidence. Prepare a focused
correction on a branch, independently review it, and merge through protected
main with required checks. Publish and validate the correction as a new version.
Never move/delete `v0.1.0`, force-push main or bypass checks. No approved recovery
time target exists, so no recovery-time promise is made.

No host setting, power policy, service or unrelated process is modified for this
release. Browser/Wasm, hosted gameplay and package-registry publication remain
outside scope. R01 acknowledges the performance uncertainty only; other release
blockers must still be resolved.

## Published decision and completed verification

**GO for the initial source release under R01; publication verified on
14 November 2026.** The released commit is
`6f2d992581ab13bb782b9ffde9dd1ca982f85b06`, tree
`6a821cac3bba0bfab645c047b7f3c9770e2e7878`; annotated `v0.1.0` tag object
`8ec6847cbe1832f8ba29471395493d807c9663ac` peels to that exact commit.
The reviewed PR #9 head `1b1a7e195bbde0c52725eaf0d220738fdcde02cd` has the
same tree, and all three independent assessments rebound approval to the merged
source. Full [code](../../ship-reports/code-review.md),
[security](../../ship-reports/security-review.md) and
[test](../../ship-reports/test-review.md) reports are byte-identical to retained
originals; their scopes and original outstanding-check statements remain intact.

[T53 publication evidence](t53-publication.md) records successful exact-SHA PR,
main and tag checks under Actions app 15368, current protections, the clean-tag
200-test/11-doctest run with 98.72% coverage and retained advisory retry, the
working locked Git consumer, matching generated source archives and metadata
disclosure. It also records the corrected scanner-cache guard incident. No
remaining source/security/test publication blocker was identified. T51 and
C20's combined performance criterion remain unchecked; no timing acceptance is
claimed. R01 expires with this publication and does not waive future requirements.

These completion records follow in a reviewed documentation-only PR. Any later
main HEAD is distinct from the immutable released commit above. The recovery
procedure remains unchanged.
