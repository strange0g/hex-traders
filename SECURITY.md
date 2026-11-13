# Security policy

The supported source release is Hex Traders 0.1.0. Report vulnerabilities privately
using the repository's [GitHub security reporting page](https://github.com/strange0g/hex-traders/security/advisories/new)
when private vulnerability reporting is enabled. This route is planned for repository publication. If GitHub does not offer private reporting, wait for the
maintainer to enable it; do not disclose a vulnerability in a public issue.

Describe the affected version, impact and a minimal reproduction using synthetic
fixtures. Keep credentials, real private seeds, snapshots and replay data out of
reports and logs. No response deadline or support SLA is promised.

The host must authenticate actor/viewer identity, protect seeds and raw
receipts/snapshots/replays, serialize access, own retry deduplication and store
snapshots atomically. The engine's typed boundaries and player projections do not
replace host access controls. See the [integration guide](README.md) and
[API contract](SPEC-engine-api.md). Use public issues for ordinary bugs that do not
expose private data or create a security vulnerability.
