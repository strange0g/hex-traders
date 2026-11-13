# T49: Pinned CI and reviewable rulesets

Three configuration files implement quality, coverage and advisories on every
PR/main/version-tag/manual/weekly event. Jobs use Ubuntu 24.04, Rust 1.98.1,
locked dependencies, cargo-llvm-cov 0.9.1 and cargo-deny 0.20.2. Full action
commits were resolved and inspected from official upstream v7.0.1 checkout,
v6.1.0 cache and v7.0.2 upload-artifact. The actual GitHub Actions app readback
confirms integration 15368; actual hosted check names/conclusions remain T50.

Each job has read-only contents permission, full history/tags, no persisted
checkout credential, no privileged PR trigger/secrets or untrusted shell event
interpolation. Only Cargo registry cache is retained, keyed by runner/architecture,
toolchain and lockfile. Coverage is cleaned and retains an explicit bounded LCOV
artifact. Fresh advisory fetch/network/tool failures fail. No paths filters,
continue-on-error or threshold changes are configured. Non-PR concurrency uses
a unique run ID; only superseded PR runs share a cancellable group.

Main rules require PR/squash, resolved conversations, linear history and current
GitHub Actions-bound quality/coverage/advisories. Zero mandatory human approvals
supports the sole maintainer; no bypass actor is configured. Version tags allow
creation and reject updates/deletion. Activation and actor-aware controls remain
T50, not a local enforcement claim.

Actionlint 1.7.12, YAML/JSON parsing, focused event/security/check/ruleset assertions
and independent Terra high review pass. The review found the non-PR pending-run
replacement risk; it was repaired and reviewed. JSONschema was unavailable; no
full JSON-schema validator execution is claimed. Field/schema inspection uses
the retained official OpenAPI and independent review instead.

All ten native gates pass: 200 tests, 11 doctests, 98.72% library lines, fresh
advisories and 20 benchmark smoke cases plus check/build/fmt/Clippy/doc. Exact
pinned installation commands, fresh advisory fetch, clean coverage and LCOV
report also pass. [Fresh commands/output/report](t49-native.tar.gz) SHA-256
`ce02eedfe1d42e44ec5335b8723e778cd0b71e0870ede720053c768b534daf88`. Logs normalize only the release-checkout absolute prefix.
The helper's cross-filesystem rename cleanup failed after all seven extra gate
commands succeeded; explicit copy/unlink preserved the report and resolved that
cleanup without rerunning successful gates.

Branding self-tests pass 53 deliberate controls; tree/archive/history and diff
checks pass. The unchanged local gate failure/tool-error controls are retained in
the original approved quality evidence; actual hosted violations/tool errors and
blocked updates remain T50. Architecture/performance stay manual gates with the
unchanged 5% native policy. No engine/fixture/dependency/rule/policy change or
hosted protection claim.
