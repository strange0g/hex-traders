# T46: Licensed source package

T45's verified fresh repository and completed app-folder handoff supply this task.
Four files add complete MIT and Apache-2.0 licenses, package metadata/selection,
and the initial 0.1.0 changelog. Original project code/docs use MIT OR Apache-2.0;
third-party tooling and dependencies retain their own licensing. The repository's
original tooling license/source attribution is unchanged and tooling is excluded
from the Cargo package.

Cargo check --locked --all-targets and cargo test --locked --doc passed (11
rustdoc tests). Cargo package --locked --allow-dirty --list yielded 96 intended
entries. Cargo package --locked --allow-dirty --offline built the source package
successfully. Source/tests/fixtures/benches, README/changelog/licenses and pinned
configuration are included; task/private evidence, agent tooling and build output
are excluded. Repository-only documentation links are explicitly scoped to the
complete repository. T47 adds contribution/security/release docs to the selected
package patterns.

An independent Terra high review caught a root README basename glob also matching
a task evidence README. All include entries are now rooted; the exact package
list was rechecked and no task/tooling/private entry remained. The review approved
T45 and repaired T46 content, verified all 110 non-root lock packages against the
retained original, exact source/fixture inclusion, full license text and private
exclusions. Cargo.lock remains byte-identical. The independent report is retained
privately; this record summarizes its new findings accurately.

Both branding gates and git diff --check passed. The committed package file list
and complete reachable-history scan are checked again before checkpoint closure.
No engine behavior, dependencies, RNG, assertions, rules or quality requirement
changed. No registry publication or GitHub resource is claimed yet.
