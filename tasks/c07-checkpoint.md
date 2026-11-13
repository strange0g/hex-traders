# C07: All development cards and immediate victory

**Public derivative:** This is a historical summary of the original checkpoint
and its privately preserved evidence. Results below were recorded on 10
November 2026; remaining scope refers to that checkpoint, and no runtime checks
were rerun for this summary.

Verified on 10 November 2026 on `codex/build-engine`; source/test HEAD is
`1190fbe27307224e24ce02ddb452e230b64592f8`. All six T19–T21 slices have
independent approval and verified commits. The approved autonomous build continues
under the existing plan.

## Delivered capability

Knight reveals a distinct played identity and derives the public played count
from authoritative card locations. The first seat to play three Knights earns
Largest Army; only a strictly higher played count transfers it. Army contributes two points
through the existing score calculator. Reveal, award and immediate owner victory
precede any robber prompt. Otherwise the existing robber destination/victim/theft
path resumes the stored PreRoll or Action phase without seven discards.

Road Building spends an eligible earlier-turn progress card only when at least
one ordinary legal road and piece exists. It creates a stored two-placement
continuation. Each free road uses the paid-road placement predicate and the same
install/route-refresh/award path. The updated first road can connect the second;
a legal second is mandatory. Exhausted pieces or blocked continuation ends the
effect automatically. Victory is checked before further placement availability,
so a first-road win cancels the pending second. No resource payment or random
draw occurs. Rejections preserve complete position, RNG and revision.

All development effects share per-card acquisition eligibility and the one
non-VP allowance across both ordinary phases. Progress cards remain spent by
owner; Knights remain separately public and played. Queries and execution share
validators; all viewers receive the same public phase and award facts, while
purchase identity and held VP remain owner-only. Old receipts project original
facts rather than later state.

The shared score/finish path counts buildings, hidden/revealed VP and both awards
once. It checks only the active owner at at least ten, after score-changing steps
or before rolling on turn start. An off-turn ten-point player waits and can be
beaten by the current active player. Winning reveals only winner VP, records
host-private final scores, replaces all pending choices with Finished and rejects
every current gameplay command for every seat.

## Verified commits

| Slice | Commit |
|---|---|
| Knight reveal, Army and robber continuation | `b9f51d8` |
| Public Knight and Army histories | `26e7307` |
| Road Building and immediate first-free victory | `93665ba` |
| Public sequential free-road histories | `19c99a1` |
| Exact native owner-turn score/terminal suite | `0d65235` |
| Complete public immediate-winning histories | `1190fbe` |

Each completed slice retains its RED/GREEN, appropriate standing gates and independent
mutation review in [todo.md](todo.md). Public verification modules first fail
on absent wiring; frozen source arithmetic and mutation controls provide their
behavioral evidence rather than claiming new behavior from a missing file.

## Final standing gates

The original logs record the following results on the committed T21b
source/tests. Checkpoint documentation changed no executable behavior.

| Command | Actual result |
|---|---|
| `cargo fmt --all -- --check` | Passed |
| `cargo check --locked --all-targets` | Passed |
| `cargo clippy --locked --all-targets -- -D warnings` | Passed |
| `cargo build --locked --release` | Passed |
| `cargo test --locked --lib --tests` | 129 passed:93 library,24 public sequences,5 model,7 rules; none ignored |
| `cargo test --locked --doc` | Five passed |
| `cargo doc --locked --no-deps` | Passed |
| `cargo llvm-cov --locked --lib --tests --ignore-filename-regex '(^\|/)(tests\|benches)/' --fail-under-lines 90` | 99.01% library lines,3893/3932 |
| `cargo deny --locked check advisories` | Refreshed scan passed |

The ordinary advisory scan could not acquire its read-only database lock. The
authorized refreshed scan passed with `advisories ok`; the failure remains
separately preserved. Unchanged-dependency test-only slices retained the
applicable preceding verdict.

Every final focused filter ran nonzero:

| `cargo test --locked` suffix | Passed |
|---|---:|
| `--test game_sequences knight_` | 3, including the third-Knight victory test |
| `base_rules_br036_` | 1 |
| `--test game_sequences free_roads_` | 1, covering both seat counts |
| `base_rules_br030_` | 1 |
| `base_rules_br038_` | 1 |
| `--test game_sequences victory_` | 3 |
| `base_rules_br03` | 8 |
| `--lib privacy_victory_` | 2 |

## Verification and evidence

Native tests prove old/new/foreign card timing, both robber and free-road
continuations, exact Army ties/transfers, zero/automatic/chosen/empty victims,
zero-site/piece activation rejection, a one-site first placement with no second,
last-piece completion, mandatory legal continuation, both immediate terminal
card effects, exact score components, above-ten start victory, competing winners,
all 17 current commands after completion, and winner-only private projection.

Public source/RNG literals cover both 3/4-seat Knight and Road Building prefixes.
Three complete fixed three-seat histories end on the exact first winning command:
new VP24 on turn68, third Knight0 on turn71 and first free E51 on turn97. Each
checks all recorded dice, seven choices, production, balances, purchases/trades/
cities, score steps, revisions, public projection and final freeze. The free-road
win suppresses still-empty/connected E47 with ten road pieces available. Alternative
public branches preserve the frozen next dice [1,5], [4,3] and [4,1]. Native
terminal tests assert actual RNG equality. Source arithmetic and pinned sampler
inputs were frozen before product execution; tests use checked-in literals and
never temporary files or the product calculator for expected balances.

Independent oracle provenance supports these histories. Targeted review
mutations verified Army transfer/score, robber PreRoll resume, free-road
mandatory/impossible continuation, no payment, purchase privacy, first-free
victory, victory threshold and owner-only timing. These were selected controls,
not a mutation-score claim. Reviewers used isolated copies and restored
sources.

Six independent slice reviews are privately preserved. The T21b reviewer
independently verified all 235 frozen records, source arithmetic and endpoint
trails; isolated 129 tests and five doctests passed. Omitting purchase, Knight
or free-road victory failed its corresponding public test. All 56 copied
source/test files and frozen-input hashes were restored. An independent
checkpoint review audited evidence and the final commit/count mapping.

## Scope and handoff

T19–T21 are complete with independent approval, standing gates and verified
commits. C07 evidence is available for human review; existing autonomous
authorization permits progression after the checkpoint.
Four setup/player-count complete histories remain required
by T29. Domestic offers and their shared victory/phase closure integration are
T22/T23; wire, snapshot/import/resume/replay and their phase-invariant validation
remain later tasks. Measured performance/memory, Wasm/bindings and hosted CI are
still later or outside this release. No approved specification, quality contract,
dependency, RNG sampler, checker configuration or base rule was weakened.
