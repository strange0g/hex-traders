# T45: Fresh export verification

Input preparation revision: `ff1c7050a864f38ec025fa686137c380066b60bf`.
This is fresh release validation, not reused historical gate output.

The export manifest verifies 213 included files and 72 intentional original
exclusions. Included source/tooling bytes and modes matched the preparation input;
all paths are relative regular files. The evidence README is a newly generated
public derivative at the original index's path. The original index is excluded,
and its bytes remain privately preserved. The public historical performance
review is the previously reviewed derivative, not the private original report.

All 418 local Markdown file links resolved. Fresh Git configuration has only local
core settings, no remotes/alternates/grafts/shallow boundary or preparation refs;
the unborn branch is main. Staged blobs are generated from the explicit export
file set. Original source/history and preparation-through-T44 bundle remain
privately recoverable; no preparation ref or old Git metadata was transferred.

All ten native commands passed: compiler check, release build, formatting,
all-target warnings-denied Clippy, 200 library/integration tests, 11 rustdoc tests,
public documentation, 98.72% library lines (5,955/6,032), fresh advisories and
20 benchmark smoke cases. Exact commands/exits/durations and fresh output are
retained in [the native gate archive](t45-native.tar.gz), SHA-256 `5cb4161755f986b2645c99d1927ef085dece7738e88fc35e439ec574143e21d1`.
Only the release-checkout absolute prefix is normalized in these public logs.

Branding self-tests passed 53 controls. Tree/archive and unborn-history scans
passed before the initial commit. Initial staged diff checking found one extra
blank line at EOF in each task plan/status document; those two public derivatives
were trimmed after export validation. Export hashes identify input bytes; later
commits identify changes. No engine, fixture, tooling notice or quality policy
was changed.

The ordinary cargo package --locked --list invocation rejected the unborn staged
repository, as expected for uncommitted input, and warned that metadata is absent.
Repeat it after the initial commit; T46 supplies licensing/package metadata.

## Workspace handoff

All commands use the verified Hex-Traders destination explicitly. The user completed the manual project-folder handoff. App project readback
confirmed the existing local project now points to the verified Hex-Traders
destination. The original checkout/private preservation remains intact.

The initial commit passed the complete reachable-history scan. After that commit,
cargo package --locked --list passed; its temporary missing-metadata warning is
addressed by T46. Scoped diff checking passed. Workspace location verification passed after the user handoff. No publication has occurred.
