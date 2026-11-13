# T47: Consumer and security guide

Four documents extend the existing compiling integration guide with versioned Git
and local path dependencies, required build/check commands, contribution/privacy
responsibilities, a provisionable private GitHub vulnerability-reporting route and
accurate initial release notes. Existing create/query/execute/project/save/replay
Rust code is unchanged. Raw values remain host-private, actor/viewer are host
authenticated, queries do not reserve actions and retry deduplication/storage are
host responsibilities. No browser, registry, SLA or unverified badge is claimed.

An external path consumer built and ran all guide assertions using the actual
library. All 11 rustdoc tests and public documentation build passed. Markdown
file links, both naming gates and diff checking passed. Repository evidence,
contracts and fixture provenance are explicitly repository-only when omitted
from the Cargo source package.

Independent Terra review identified GitHub release-page relative links, a packaged
README contract link, and premature protection/provisioning wording. Release
notes now use version-tagged absolute repository URLs; the API-contract link does
also. Protection/report provisioning claims are conditional until T50 readback.
The Rust sample and its external consumer remain byte-identical after prose fixes.

The Git dependency/tagged URLs require T53's published tag, and private reporting
requires T50 provisioning/readback. These planned later checks are retained; they
are not called passed here. No engine, dependency, rule or quality-policy change.
