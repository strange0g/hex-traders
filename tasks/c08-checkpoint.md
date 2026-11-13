# C08: Domestic trade and native privacy

**Public derivative:** This is a historical summary of the original checkpoint
and its privately preserved evidence. Results below were recorded on 10
November 2026; remaining scope refers to that checkpoint, and no runtime checks
were rerun for this summary.

Verified on 10 November 2026 on `codex/build-engine`; source/test HEAD is
`9cb29a3`. The six T22–T24 slices have independent reviews and verified commits.
The approved autonomous build continues under the existing plan.

## Delivered capability

The active seat and one other seat can make one public bilateral offer with
positive disjoint resource bundles. The proposer must afford the offered bundle;
creating an offer does not inspect or reserve the recipient's hand. A permitted
counteroffer or replacement closes the original before creating the new offer.
Only its recipient may accept or reject and only its proposer may cancel. Offer
IDs are creation revisions. Acceptance rechecks both holdings, exchanges both
bundles atomically and uses a generic unavailable error if either cannot fund it.
Failed, stale or competing commands cannot transfer resources twice. A successful
specified hand change or Action exit, including a card effect that leaves Action,
closes the offer. A zero-result effect that stays in Action keeps it; generic
closure does not disclose private affordability.

Views expose public board, bank, offer and aggregate holdings plus only the
selected seat's own hand, cards and true score. Receipt projection is opt-in and
uses the original receipt facts: purchase identity goes only to its buyer,
discard types to the discarder and theft type to its parties; only the winner's
VP identities are revealed at completion. Every typed event is covered for all
four seats and spectator. Legal actions are bounded, seat-scoped and use shared
validators; supplied resource bundles are validated without enumeration.

All 21 native command variants are exercised at an observed revision and at
revision exhaustion; local endpoint tests compare the complete position, RNG and
revision after rejection. Turn exhaustion preserves an existing offer. Public
contract tests cover version/seat/revision/actor/phase precedence, all setup
settlement candidates, stale retry, unaffected views and unchanged next dice.
Synthetic endpoint fixtures prove local contracts rather than historic reachability.

## Verified commits

| Slice | Commit |
|---|---|
| Proposals, responses and offer lifetime | `3a3a01c` |
| Public negotiation histories | `16cc827` |
| Atomic acceptance | `e4b0619` |
| Public acceptance and lifetime histories | `b9938b4` |
| Viewer-safe receipt and privacy audit | `d478ead` |
| Native command and counter contract audit | `9cb29a3` |

Each slice records focused checks, standing gates and independent review in
[todo.md](todo.md). Test-only RED logs for missing modules or targets are wiring
controls, not claims of a behavioral regression. T24b's 21-kind assertion first
failed with 20 variants; a source-traced settlement fixture closed the gap.

## Final standing gates

The original logs record the following results on the committed T24b
source/tests. Checkpoint documentation changed no executable behavior.

| Command | Actual result |
|---|---|
| `cargo fmt --all -- --check` | Passed |
| `cargo check --locked --all-targets` | Passed |
| `cargo clippy --locked --all-targets -- -D warnings` | Passed |
| `cargo build --locked --release` | Passed |
| `cargo test --locked --lib --tests` | 155 passed: 112 unit, 43 integration; none ignored |
| `cargo test --locked --doc` | Seven passed |
| `cargo doc --locked --no-deps` | Passed |
| `cargo llvm-cov --locked --lib --tests --ignore-filename-regex '(^\|/)(tests\|benches)/' --fail-under-lines 90` | 99.00% library lines, 4148/4190 |
| `cargo deny --locked check advisories` | Refreshed scan passed |

The ordinary advisory scan failed to acquire its read-only database lock. The
authorized refreshed scan passed with `advisories ok`; the original failure
remains separately preserved.

Each final focused filter ran a nonzero number of tests:

| `cargo test --locked` suffix | Passed |
|---|---:|
| `--test game_sequences trade_offer_` | 1 |
| `base_rules_br021_` | 2 |
| `--test game_sequences trade_accept_` | 1 |
| `--test game_sequences offer_lifetime_` | 2 |
| `--lib privacy_trade_` | 3 |
| `--lib native_contract_` | 2 |
| `--test api_contract native_` | 2 |
| `--test privacy` | 1 |

## Verification and remaining scope

Public fixed three- and four-seat histories cover both proposal and acceptance
orientations, replacement, counteroffer, reject/cancel, exact hands and bank,
every-viewer original receipts and following dice. They include nonzero Monopoly
and Invention closing offers, and zero-result Monopoly retaining one until EndTurn.
Native acceptance tests cover an artificially stale or underfunded offer to prove
the generic private recheck; legal positions close on participant hand changes.

Six independent slice reviews are privately preserved. They checked connected
code, fixtures and privacy with targeted mutation controls for offer rules,
atomic transfers, closure, projection, stale revisions, rejection RNG and
exhausted discard counters. T24a’s six privacy mutations and T24b’s three
contract mutations were killed. These selected controls are not a project-wide
mutation score.

C08 evidence is available for human review; existing autonomous authorization
permits the next phase. Strict JSON, snapshot import/export and replay remain
T25–T28; complete source-traced games and properties remain T29–T30, and measured
performance/memory remain T31–T32. Wasm/bindings and hosted CI remain outside the
current release scope. No specification, rules, quality threshold, dependencies
or checker configuration were weakened; nothing was published or deployed.
