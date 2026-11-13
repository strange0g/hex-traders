# C09: Strict wire contracts and save/resume

**Public derivative:** This is a historical summary of the original checkpoint
and its privately preserved evidence. Results below were recorded on 10
November 2026; remaining scope refers to that checkpoint, and no runtime checks
were rerun for this summary.

Verified on 10 November 2026 on `codex/build-engine`; source/test HEAD is
`652e873`. T25–T27 deliver the
approved wire and host-private snapshot boundaries. Each leaf has independent
review and a verified commit; existing autonomous authorization permits the next
phase after this checkpoint evidence is recorded.

## Delivered capability

All 21 commands decode from strict API-version-1 JSON and execute through the
same native facade, validators and atomic transitions. Borrowed encoders expose
the safe native views, legal actions, projected gameplay/creation receipts and
bounded errors. Literal contracts cover nine phases, 21 legal choices,
25 projected gameplay events and seven creation variants. Counters use canonical
decimal strings, including zero and u64::MAX. Owner-only purchase, discard and
theft facts follow the original native receipt projection for every viewer.

Both command and snapshot input reject more than 65,536 bytes, container depth
beyond 16, duplicate decoded keys (including escaped spellings), trailing data,
unknown keys, incorrect object/array shapes and invalid numeric/ID values.
Required nullable fields remain required. Errors contain static codes/public
fields and never echo input or private state.

The host-private schema-1 snapshot contains config, source layout/frame
identity, all stored position facts and RNG seed/stream/word position. It
excludes topology and history. The frozen synthetic fixed-three-seat fixture is
2,925 bytes; a nonuniform seed and independent variable seed12 goldens protect
byte order and source layout. The historical rules identifier has since been
superseded by the release identifier; the random algorithm identifier was
`chacha12-rand010-v1`. Export is explicit, borrowed and performs no I/O.

Import builds a separate candidate Engine, rebuilds topology and performs the
shared full native validation before returning it. Creation reuses that validator
with additional initial-state restrictions. Validation checks these facts:

| Area | Required current-state facts |
|---|---|
| Layout and ownership | Configured seats/empty unused slots; exact fixed or variable source terrain, tokens, ports and frame permutation; valid owners and IDs before indexing |
| Resources and pieces | Bank plus hands equals 19 per resource; distance rule and 15-road/5-settlement/4-city stock; retained setup pieces and markers |
| Cards | Canonical kinds and shuffled deck prefix; exact locations/cursor; owner and acquisition-turn congruence; spent/played/revealed kinds and allowances |
| Setup and counters | Snake order, actor, counts and revision; each completed setup settlement paired with its own road; checked normal turn/revision minima and bounded endpoints |
| Pending decisions | Dice/continuations, partial discard obligations, robber victim sets including empty hands, free-road remaining count and legal next placement |
| Offers | Permitted parties/phase/turn, positive disjoint bundles, proposer affordability, creation revision; recipient affordability is checked on acceptance |
| Awards and completion | Exact road caches, qualifying Route incumbents/ties and Army holder, active score, exact winner/stored score/final scores and winner-only VP reveals |
| RNG | 32-byte lowercase seed, stream zero, canonical decimal word position strictly below 2^68 before restoration |

Public tests save and resume before and after every command in variable
three-/four-seat setup, fixed free-road prefixes, offer histories, weighted theft
cases and three complete fixed-three-seat winning histories. They compare exact
snapshot bytes, every viewer JSON, all seats' legal choices, validation outcomes,
private/projected receipts and following dice/theft. Queries and stale retries
preserve the full snapshot. Coverage includes all nine phases, both free-road
continuations at remaining two/one, pre-roll Knight followed by seven,
partially fulfilled multi-seat discard (a fulfilled seat still above seven),
card acquisition/deck cursor, awards and final reveals. Public synthetic current
states additionally distinguish qualifying award ties and corrupt cards,
pending effects, offers, layouts and final outcomes.

## Verified commits

| Leaf | Commit |
|---|---|
| T25a: strict command JSON | `c2be4bb` |
| T25b: viewer-safe JSON output | `d6c19cc` |
| T26: complete snapshot export | `b0a8a7c` |
| T27a: shared full native invariants | `fa58e99` |
| T27b: strict snapshot import | `e3dd1cc` |
| T27c: public resume/corruption proof | `652e873` |

Each leaf records its focused checks, gates and independent review in
[todo.md](todo.md). T27a and T27b were independently verified in separate archives
that exclude the next leaf's draft. Earlier failures and surviving mutation
controls remain retained as evidence of defects or assertion gaps, rather than
being replaced by successful logs.

## Findings resolved

The native review reproduced a setup state with both roads paired to the first
settlement and none at the second. The setup-only validator now checks each
completed settlement's incident own road, excluding the pending road vertex;
an ordinary-transition regression catches the defect.

At the maximum valid RNG word position, rand_chacha's buffered cursor reports
2^68+1 after rolling [4,3]. An uncanonicalized export could therefore fail its
own import. Export now writes the equivalent cursor modulo 2^68; import retains
its strict bound and native sampling remains unchanged. Independent pinned
samplers freeze positions 0/15/16/max, exact dice and subsequent receipts, and
compare 100,000 future words across block/period boundaries. The retained
failing and passing controls record the failure and repair.

Independent controls also exposed missing literal tags/counter sites, seed/frame
preservation, strict inner-object/offer/array checks and exact winning-score
assertions. Their final corrected tests kill the same isolated mutations.
The score corruption includes 11, ensuring the exact equality check cannot be
masked by the minimum winning threshold. A valid finished candidate gives an
opponent one held VP and adjusts its final score; changing only that location to
revealed keeps scores unchanged and directly proves winner-only revelation.
No production rules or quality gates were relaxed.

## Final verification

The original source/test gate logs record the results below. Checkpoint documentation changed no executable behavior.

| Command | Actual result |
|---|---|
| `cargo fmt --all -- --check` | Passed |
| `cargo check --locked --all-targets` | Passed |
| `cargo clippy --locked --all-targets -- -D warnings` | Passed |
| `cargo test --locked --lib --tests` | 182 passed: 126 unit and 56 integration; none ignored |
| `cargo test --locked --doc` | Nine passed |
| `cargo build --locked --release` | Passed |
| `cargo doc --locked --no-deps` | Passed |
| `cargo llvm-cov --locked --lib --tests --ignore-filename-regex '(^\|/)(tests\|benches)/' --fail-under-lines 90` | 98.74% library lines, 5742/5815 |
| `cargo deny --locked check advisories` | Refreshed scan passed |

The ordinary advisory attempt could not acquire its database lock on a
read-only path; the failure is retained separately. The authorized refresh
passed with `advisories ok`; no dependencies changed in this phase.

Six focused targets ran nonzero tests:

| `cargo test --locked` suffix | Passed |
|---|---:|
| `--test api_contract` | 5 |
| `--test snapshots` | 2 |
| `--test snapshot_import` | 5 |
| `--test snapshot_roundtrips` | 3 |
| `--test model_invariants` | 5 |
| `--test privacy` | 1 |

Six independent slice reviews inspect connected providers, interface/privacy
boundaries, pinned independent fixtures and isolated mutation sensitivity.
These selected controls are not a project-wide mutation score. An independent
checkpoint review approved the final commit/source, count/log mapping, review
evidence and remaining scope. The original reviews and controls are privately
preserved.

## Limits and next phase

Import establishes coherent current facts, not proof of an honest legal history;
there is no retained history to reconstruct. Synthetic valid candidates prove
this contract while ordinary public histories prove actual reached states.
Snapshot bytes contain host secrets and must be kept by the authoritative host;
viewer encoders remain separate allowlists. No storage or transport was added.

C09 evidence is available for human review under the existing autonomous plan.
Accepted-command replay remains T28; all four complete setup/player-count games
and final BR/property evidence remain T29–T30; measured performance/memory remain
T31–T32. Wasm/bindings and hosted CI remain outside this scope. No dependency,
specification, approved policy, constraint threshold or checker configuration was
changed. Nothing was published or deployed.
