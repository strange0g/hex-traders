# Historical native handoff decision: GO

**PUBLIC DERIVATIVE — historical summary.** This preserves the original native
engine assessment and evidence; it is not verification of the new release tree.
Historical source IDs and candidate hashes refer to privately preserved original
revisions, not new public-history commits or current renamed-file hashes.
T43 verified the original report SHA-256
`00ad42b1d8f804ab325ac7809210437dcddc288519f5539b3bab2255fc0a198a` against the T35 preservation inventory and reconciled the relevant
original evidence hashes. Original specialist reports, raw logs, C11/ship
archives, manifests, oracle sources and control artifacts remain private and
byte-preserved; none has been renamed as fresh evidence. Private machine paths,
source URLs and links to excluded raw evidence are omitted from this derivative.
Previously excluded generated build trees and nonexistent tool-error controls
remain exclusions. No engine tests, benchmarks, profilers or rollback procedures
were rerun for T43. T52 owns the fresh release assessment; historical approval
here grants no new publication or deployment authorization.

Public context: [approved rules](../SPEC-base-rules.md), BR001–BR038 and RQ1/RQ2;
[constraints](../CONSTRAINTS.md); [historical plan](plan.md) and
[task status](todo.md). The rule baseline is the 2025 English sixth edition,
document `CN3081`, revision `v6.250401`, with pages and compatible FAQ
qualifications recorded in the paraphrased catalogue. No publisher endorsement
is claimed.

GO for the approved native Rust library and local consumer handoff, assessed
on 10 November 2026 at `be9a641bc1897d9c4a4ab674406e8447c79dc314`
on `codex/build-engine`. All three independent specialist reviews and the
coordinator's then-current required local gates pass. This decision records readiness
for integration; publication, deployment and hosted operation require their
own authorization and verification.

### Blockers (must fix before ship)

None for the approved scope. Code review reports zero Critical/Required
findings; security reports zero Critical/High/Medium/Low findings; test review
finds no launch blocker. T34 resolved the prior required evidence-retention gap.

### Recommended fixes (should fix before ship)

No required pre-handoff remediation. The specialists recommend these optional
follow-ups at the relevant next change:

- Security documentation: the historical specialist recommended inspecting
  the independent arithmetic verification script and its imported source helper
  before a copied rerun. Other historical mutation/packaging scripts contain
  machine-specific paths and remain provenance rather than generic executables.
  These original scripts and the specialist report remain private.
- Test breadth: when changing card/phase logic, broaden generated accepted
  chains to settlements, Road Building/free roads, Monopoly, Invention and
  Finished. Direct fixtures and complete games already cover these paths.
  When changing import validators, generate malformed cases at more reached
  pending phases. Neither gap demonstrates a current defect.

### Acknowledged risks (shipping anyway)

- The consumer must authenticate actors/viewers, supply unpredictable private
  seeds, protect raw receipts/snapshots/replays, serialize per-game access and
  deduplicate lost responses. Use the [consumer guide](../README.md) and test
  those duties in the actual host before hosting real games.
- Synthetic examples and finite properties cannot prove every history. Replay
  and resume share engine code; independent fixture facts, direct assertions
  and retained failure controls supplement their comparisons.
- Native timings are machine-specific and some same-binary drift exceeds 5%.
  The unchanged 5% margin is a manual review policy; no absolute latency budget
  or cross-target performance guarantee is approved. See [PERF.md](../PERF.md).
- The 98.72% line report includes inline source test functions. It is a useful
  backstop against the unchanged 90% gate, not production-only coverage or a
  proof of correctness. Dependency scans are a current advisory opinion,
  not proof against undisclosed vulnerabilities.

### Rollback plan

**Trigger conditions:** suspend adoption on a confirmed rules/invariant,
privacy, atomicity or replay defect; an unusable consumer example; or evidence
corruption that invalidates the handoff. Investigate a repeatable native
regression over the approved 5% margin under matching measurement conditions.

**Historical T34 evidence/documentation recovery:** the original procedure
preserved affected bytes, inspected a clean checkout and proposed reverting
`be9a641bc1897d9c4a4ab674406e8447c79dc314` to prior verified
`1293325bb50085d122f0093c6ecf6708554727d7`, checking source, tests, benchmarks,
manifests and checker/toolchain configuration. It required preserving local
changes and stopping for conflicts. Those source IDs belong to private original
history; they are not a rollback target for the new release tree. An exact
reverse patch was applied only in isolated scratch: all 35 changed paths matched
the prior revision. An actual Git revert was not executed. The original
rehearsal result remains privately retained.

**Procedure for an engine or host defect:** suspend local adoption/new games
and preserve any consumer-owned private snapshots and accepted replay records
under existing private access controls. T34 and its predecessor have identical
engine inputs, so the T34 revert cannot repair a functional engine defect.
There is no prior production release or verified host rollback target. Repair
the focused defect, reproduce it with a test, and repeat the affected and
required gates before resuming adoption. A concrete host must supply its own
data-preserving recovery procedure before production use.

**Recovery time objective:** proposed local source-revert target of five
minutes from the decision to revert, assuming a clean checkout and no
conflicts. This is not a measured Git-revert or host-recovery guarantee. The
scratch reverse-patch check and application each took 0.014 seconds; engine
repair and consumer data recovery have no established time objective.

### Historical specialist conclusions

The three personas ran concurrently without delegating. Original reports and
checksums remain privately retained byte for byte, distinct from the
coordinator's then-current gates. The code reviewer (Sol/high) approved with
Critical 0, Required 0, Optional 0 and Nit 0; the prior evidence-retention finding
was resolved. The security auditor (Sol/high) reported Critical/High/Medium/Low
all zero, with the optional script-inspection clarification above. The test
engineer (Terra/high) found no native-handoff blocker conditional on coordinator
gates, retaining the finite-property and later pending-phase breadth limits.
These are historical conclusions, not new release specialist reviews. See the
retained [architecture review](t32-review.md), [handoff review](t33-review.md)
and [property evidence](property-evidence.md); T52 will supply fresh release
reports.

### Historical coordinator verification

The coordinator ran the following against the clean assessed revision. Original argument arrays, exits, durations, log hashes and the refreshed
advisory execution-tool result remain privately retained.

| Check | Result |
| --- | --- |
| Formatting, all-target compiler check and Clippy with warnings denied | Pass |
| Library and integration tests | 199 pass: 131 unit + 68 integration; zero failures/ignored |
| Rustdoc examples | 11 pass: 6 ordinary + 5 compile-fail |
| Native release build and public rustdoc | Pass |
| Library line coverage, 90% floor | 98.72%: 5,955/6,032 |
| Criterion benchmark smoke | 20 cases pass |
| Refreshed locked dependency advisory scan | Exit 0, `advisories ok`; cargo-deny 0.20.2 |

Rustc is 1.98.1; cargo-llvm-cov is 0.9.1. Source/import/privacy and evidence
reviews pass; the unchanged performance baseline and manual architecture gate
are documented. README and public API examples compile; specifications,
architecture/performance reviews, rule traceability and task/checkpoint history
are current. No package release
was created, so this assessment adds no version tag or release changelog.

There is no UI, browser/Wasm binding, service, environment-variable interface,
migration, network/storage layer, hosted CI or feature-flag surface. Browser
accessibility/Web Vitals, service rollout metrics and production infrastructure
checks therefore do not apply to this library assessment. Host authentication,
monitoring and recovery need verification when a concrete host exists.

### Private evidence preservation

The private original ship archive retains all nine coordinator gate logs/results,
the advisory scan, rollback rehearsal and specialist audit JSON. Its scope
record binds them to the assessed commit, tools and original tracked-file hashes.
The reverse patch is recoverable from private original Git history using the
recorded parent/candidate; it was not duplicated in the archive. Successful
engine gates were not repeated for the original documentation/evidence recording
or for T43. Engine behavior, tests, fixtures, requirements and dependencies are
unchanged by this summary. The original GO applies only to that native handoff;
T52 must assess the new release and its publication readiness.
