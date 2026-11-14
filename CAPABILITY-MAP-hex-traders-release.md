# Capability Map: Hex Traders Rename and GitHub Publication

Status: approved by the user on 13 November 2026. The subsequent clarification
confirmed approval without additional scope changes. Publication settings, fresh
history, local rename and initial-release compatibility are approved.
Autonomous implementation was approved by `$build auto` on 13 November 2026.
Native identity, fresh history, relocation and initial publication are verified
in [the task record](tasks/todo.md#hex-traders-release) and
[T53 evidence](tasks/evidence/release/t53-publication.md). T51 remains open;
v0.1.0 publication uses the release-only R01 exception. The approved engine map in
`CAPABILITY-MAP.md`, engine
specifications, completed task evidence, and quality contract remain authoritative.

## Objective and assumptions

Make Hex Traders the sole project identity and publish a well maintained Rust
engine repository on GitHub. The immediate consumers remain developers embedding
the native library. This initiative adds repository infrastructure, not a playable
website, new game rules, or a different language.

Approved names: product **Hex Traders**, repository `hex-traders`, package
`hex-traders-engine`, Rust import `hex_traders_engine`. The existing four engine
modules and approved rule behavior remain in place.

## Approved capabilities

| Module id | Responsibility | Depends on |
|---|---|---|
| `hex-traders-branding` | Rename product and package identity throughout executable source, callers, exports, rustdoc, README, specifications, fixtures, benchmark environment variables, scripts and documentation. Define the new rules identifier and snapshot/replay compatibility policy. Produce a release tree and evidence that contain no former-name mentions, including archive contents and filenames. Account for the local checkout folder and historical evidence explicitly. | Existing approved engine contracts |
| `github-publication` | Publish the verified renamed tree under the chosen GitHub owner and visibility. Add appropriate licensing and package metadata, contributor/security guidance, issue and PR templates, enforced CI, dependency-update PRs, repository protections and an initial tagged release. Verify hosted checks, repository settings, release contents and a fresh consumer checkout. | `hex-traders-branding` |

Dependency direction and build order:
`hex-traders-branding` → `github-publication`.

These are initiative capabilities, not new Rust source modules or separate
services. Module specifications are written in dependency order and indexed from
`SPEC.md` without replacing the approved engine contract. The user's `$plan`
invocation accepted them for planning.

| Module id | Specification |
|---|---|
| `hex-traders-branding` | [SPEC-hex-traders-branding.md](SPEC-hex-traders-branding.md) |
| `github-publication` | [SPEC-github-publication.md](SPEC-github-publication.md) |

## Confirmed decisions

- **History and evidence:** the former identity occurs in current documents,
  saved-game metadata, archived logs and historical Git objects. A renamed latest
  commit alone cannot satisfy a zero-mention publication. Approved solution:
  prepare a fresh publication history and new verification evidence, while
  retaining original history and byte-for-byte historical evidence privately.
  Do not rewrite historical proof or present renamed logs as original results.
- **Sources:** existing source links also contain the former identity. Approved
  public documentation cites edition, document identifiers, rule IDs and
  paraphrased requirements; exact source links remain in the preserved private
  provenance. Approved BR001–BR038 and RQ1/RQ2 behavior is unchanged. The new
  product must not be described as an official publisher release.
- **Persistence:** this is a fresh initial release. No existing saves or replay
  files need migration. Define and test the new rules identifier; no legacy
  aliases or conversion tools are needed.
- **Location:** rename the local checkout folder to `Hex-Traders`.
  Moving the active workspace needs its path and attachment
  updated; preserve the original checkout until the destination is verified.
- **GitHub:** publish `strange0g/hex-traders` publicly. Original project code
  uses dual MIT OR Apache-2.0 licensing, with third-party provenance and existing
  notices checked before publication.

## Publication quality baseline

Carry forward every applicable command and threshold in `CONSTRAINTS.md`.
GitHub checks should include build, formatting, warnings-denied Clippy, unit and
integration tests, rustdoc tests, the existing 90% coverage floor and a fresh
dependency-advisory scan. Preserve manual architecture/performance review and
the recorded hardware limitations rather than claiming hosted timings establish
a comparable native baseline.

Use pinned tooling and verified action revisions, least-privilege workflow
permissions, safe pull-request execution, required passing checks, and protection
against branch deletion and force pushes where the account supports enforcement.
Dependency updates remain reviewed PRs, including replay-sensitive RNG changes.
Avoid a self-review requirement that prevents a sole maintainer from merging.
Report any unavailable repository security feature rather than claiming it is
enabled. Hosted success and a clean clone are required publication evidence.

Package-registry publication, hosted gameplay, deployment and paid GitHub
features are outside this proposal. GitHub publication itself is requested;
the specification and scope approval gates still apply because `$spec` was
explicitly invoked.

## Inspection evidence and remaining inputs

The working tree was clean on `main` at `1713fd3` at inspection. The existing
native handoff and evidence-preservation tasks are marked complete. There is no
configured Git remote, hosted workflow or root project license. The tracked tree
contains 270 files; 89 text files and several retained archives contain the former
identity. Archive members were inspected read-only without executing scripts.

The initial planning-time GitHub CLI check reported an invalid credential.
Fresh network-approved build preflight successfully authenticated `strange0g`
and verified the destination absent; the earlier sandbox result was misleading.
The user confirmed `strange0g` as the publication owner;
authenticated access was verified during build preflight. Use a secure
sign-in flow if subsequent access expires; never request a token in chat.

No capability-map questions remain. The module specifications have advanced to
planning; the user subsequently approved autonomous implementation of the release plan.
Workspace relocation and verification of subsequent GitHub operations remain
implementation prerequisites, not unresolved scope choices.
