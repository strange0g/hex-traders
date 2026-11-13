# T30: Historical property verification evidence

**PUBLIC DERIVATIVE — historical summary.** This preserves the original native
engine assessment and evidence; it is not verification of the new release tree.
Historical source IDs and candidate hashes refer to privately preserved original
revisions, not new public-history commits or current renamed-file hashes.
T43 verified the original report SHA-256
`0ed3940b78bf0aa23dc12658411a22cba44ac843dcf242d2846d416a5db82ce1` against the T35 preservation inventory and reconciled the relevant
original evidence hashes. Original specialist reports, raw logs, C11/ship
archives, manifests, oracle sources and control artifacts remain private and
byte-preserved; none has been renamed as fresh evidence. Private machine paths,
source URLs and links to excluded raw evidence are omitted from this derivative.
Previously excluded generated build trees and nonexistent tool-error controls
remain exclusions. No engine tests, benchmarks, profilers or rollback procedures
were rerun for T43. T52 owns the fresh release assessment; historical approval
here grants no new publication or deployment authorization.

Public context: [approved rules](../SPEC-base-rules.md), BR001–BR038 and RQ1/RQ2;
[constraints](../CONSTRAINTS.md); [historical plan](plan.md) and
[task status](todo.md). The rule baseline is the 2025 English sixth edition,
document `CN3081`, revision `v6.250401`, with pages and compatible FAQ
qualifications recorded in the paraphrased catalogue. No publisher endorsement
is claimed.

All generated inputs are synthetic. Properties use pinned Proptest 1.12.0,
fixed test-runner seeds, at most 2,048 shrink iterations and direct persistence
at `tests/property_sequences.proptest-regressions`. New failure seeds are
reported with synthetic inputs and replayed before new cases. No production
hook, private-state installation, dependency or quality change is required.

## T30a: Creation and independent state facts

`property_created_configurations_have_independent_conservation_and_scores`
uses 64 cases with runner seed `0xca7a0030`: fixed/variable, three/four seats,
32-bit creation seed word and rotated supplied seat order. The other 28 seed
bytes are zero. Creation uses the public constructor.

The read-only oracle counts literal resource totals of 19, complementary road/
settlement/city stock of 15/5/4, all 25 card IDs and draw/location correspondence,
public occupancy distance, public/private resource and card counts, building/
card/award score arithmetic, award maxima/thresholds, phase obligations and
terminal queries. RNG stream/canonical cursor and complete byte-stable export,
queries and validated import roundtrip are checked. Current facts do not prove
historically honest ownership or tied-award history. Creation alone does not
exercise held/spent cards or later award/phase branches; T30b reaches actions.
Spectator projection must contain no private player record.

Initial compiler type inference and test resource-map decoding mistakes were
corrected before acceptance. The decoder mistake actually shrank to
`four=false, variable=false, seed=0, rotation=0`, with zero successful cases.
This was a test-harness defect, not an engine defect: the obsolete persisted
case was byte-archived separately instead of represented as a product
regression. The original red log and obsolete harness regression artifact
retain the original shrink output privately.

The historical independent review exercises compile-valid
production controls for resource total 18 versus 19, piece complement 14 versus 15,
duplicate deck ID, wrong undrawn location, and public-score mismatch. An
active-seat hand leaked to a spectator initially survived this exact property;
the explicit private-record absence assertion closes it. The final six
controls fail, restore source hashes, and pass the unchanged production.
Deterministic instrumentation verifies the pinned case stream repeats and
covers all four setup/seat-count configurations and rotations.

Historical final applicable gates
pass 197 tests (131 unit/66 integration), ten doctests and 98.71% library lines
(5954/6032). Dependencies and production are unchanged; the C09 advisory
verdict applies, with database refresh required at C10.

## T30b: Reached decision vectors

`commands::property_reached_decision_vectors_preserve_facts_queries_rejections_and_replay`
uses 32 cases with runner seed `0xca7a0031` and shrinkable `Vec<u16>` of zero
through 64 decisions. Real public settlement/paired-road choices finish variable
setup (up to 16 commands), including when the vector shrinks empty. Each decision
selects an available actor/action family, then a bounded ID or complete public
resource choice. Assembly matches all 21 `LegalAction` variants exhaustively;
it supplies exact discard quantities, bank-available Invention bundles and
positive disjoint proposal bundles without copying rule eligibility.

At every reached state, the T30a independent facts apply. Every seat's query
revision and supplied action validate. All 72 edges for three placement kinds
and 54 vertices for three placement kinds compare list membership with the
ordinary validator, detecting both missing and spurious candidates. Arbitrary
complete commands span all 21 variants; rejected validation must agree with
execution and preserve the complete bytes. Unsupported/stale envelopes,
queries and accepted stale retries preserve snapshot/RNG/revision/offer.
The existing `VerifiedGame` consumer compares ordinary import continuation,
complete persistent replay, ordered private/projected receipts, every view and
exact snapshots, with independently unchanged RNG for nonrandom commands.

Independent instrumentation of the exact pinned 32 cases reaches 1,274 accepted
commands across 16 families and all 21 arbitrary rejection families. Acceptances
include 176 setup placements, 195 rolls, 37 robber moves/one victim/one discard,
459 proposals/34 accepts/73 rejects/76 cancellations, 169 turn endings, 24 paid
roads, 12 purchases, 10 bank trades, six Knights and one city. Nine Knight/three VP
cards are purchased; all four configurations occur. These bounded random cases
do not claim accepted settlement, Road Building/free-road, Monopoly, Invention,
FreeRoads or terminal exploration. The independently source-frozen complete
[T29 complete games](../tests/game_sequences/full_games.rs),
[fixed-three-seat wins](../tests/game_sequences/victory.rs) and
[all-phase replay checkpoints](../tests/snapshot_roundtrips.rs) retain those
direct cases.

The first run passed both properties in 17.13s. An initial Clippy parity warning
was corrected to `is_multiple_of` without suppression; its original failure log remains privately retained. Historical final applicable standing logs
pass 198 tests (131 unit/67 integration), ten doctests and 98.71% library lines
(5954/6032). The historical independent review checks final source hashes, actual
reach counts and isolated compile-valid controls for missing candidates, query
mutation, rejected revision/RNG/offer mutation, resume RNG, accepted EndTurn
RNG and production conservation. Each exact reached property fails and each
source restore passes. Original shrinking/persistence output remains retained;
control witnesses are evidence of test sensitivity, not discovered product bugs.

The accepted EndTurn RNG control actually shrinks to fixed-three-seat seed 0,
rotation 0, decisions `[0, 0, 16758, 4083]`; this is a retained local minimum,
not proof of global minimality. Original log and persistence artifacts remain privately retained.
A ninth optional public-score/true-score control reaches direct score facts
3 versus 2 while execution/resume/replay agree; its long shrink experiment was
stopped after that assertion. It remains explicitly partial sensitivity
evidence, not a ninth completed killed-control verdict. No further diagnostic
is needed for the eight completed controls and final approval.

## T30c: Bounded malformed boundaries

`malformed::property_bounded_malformed_snapshots_and_wire_preserve_original_continuation`
uses 64 cases with runner seed `0xca7a0032`, synthetic configuration/seed/order,
32-bit mutation values and a zero-through 256-byte noise vector. Before corruption,
one real public command reaches revision 1 (a setup settlement or gameplay roll).
Each case checks 16 snapshot and 16 wire inputs: 2,048 malformed input instances
and 3,072 rejection calls (wire decoding and execution are checked separately).
The mutation families remain invalid when inputs shrink.

Snapshot families alter one conserved bank count, duplicate a deck ID, make turn
noncanonical or revision numeric, exceed the 2^68 RNG position bound, select a
nonzero RNG stream, change owner/phase facts, change an undrawn card location,
shorten an exact road array, or add unknown fields. Literal duplicate/escaped
schema keys, invalid-UTF-8 noise and a valid snapshot padded to 65,537 bytes cover
the external parser boundary. Wire families vary API versions, seats, canonical
counters, tags/unknown fields, shapes, floats, IDs/resource bounds, required keys,
literal/escaped duplicates, invalid UTF-8 and a valid padded envelope at 65,537
bytes. A deeply nested array is a rejection family; the existing direct parser
unit test proves the actual depth 16 versus 17 guard independently.

Standalone imports/decodes must reject without panic or input-text disclosure.
Execution reports the unchanged public revision 1 and decoder code; every failure
preserves complete snapshot/RNG/revision/offer bytes. The next fully supplied
ordinary public choice then produces the exact receipt/snapshot of an untouched
imported control, followed by the independent state facts. Input allocation is
explicitly bounded to 65,537 bytes, including the one-byte-over-bound probes.
No genuine engine regression was discovered during T30; deliberate controls and
the corrected T30a harness decoder are distinguished from product bugs. The
existing direct persistence path will retain future genuine synthetic failures.

The first exact malformed property passed 64 cases in 0.80s. Historical final applicable
standing logs
pass 199 tests (131 unit/68 integration), ten doctests and 98.72% library lines
(5955/6032). Independent review/control hashes are recorded with the verified
task status.

The historical independent review approves final hashes and independently
measures fixed three/fixed four/variable three/variable four case counts 11/17/20/16, all at
revision 1, with original snapshots 2854–3018 bytes. Eight compile-valid controls
(conservation, deck uniqueness, literal/escaped duplicate keys, canonical
decimals, byte bound, error revision, rollback and diagnostic sanitization)
each fail the exact property with exit 101 and pass restored with exit 0. The
duplicate-key control actually shrinks to fixed three/seed 0/rotation 0/mutation 0/empty
noise, with persisted seed `caf2a89b975edec904ae4b97cc58d107401e8718039bb0019a71e50fcfa2ddda`;
the same persisted synthetic case passes restored production. This is local
shrink/persistence evidence, not global minimality or a product regression.
