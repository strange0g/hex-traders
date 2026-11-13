# T48: Contribution and dependency-update intake

Four maintenance files add focused bug/feature issue forms, a PR template and
weekly Dependabot requests for root Cargo/GitHub Actions. Each ecosystem is
bounded to five open PRs. Forms request public/synthetic reproductions and
sanitized output; real private seeds/snapshots/replays/credentials are excluded.
Replay-sensitive changes require review/compatibility decisions; unattended merge
and automatic toolchain/RNG drift are not configured.

PyYAML parsed both forms and Dependabot. Asserted unique input IDs, allowed form
types, required relevant fields, exact ecosystem/root/weekly/five-PR policy.
Independent Terra review approved the form structure and update requirements.
Both branding gates and git diff --check pass. GitHub form/configuration
recognition remains a T50 hosted check, not a local claim. No engine/dependency,
rule, threshold or workflow subsystem was changed.
