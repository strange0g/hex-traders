# T38: Portable source fixture provenance

Executable input revision: `db35ad4`; concurrent changes were documentation only.
Four source/traceability documents retain edition `2025 English sixth edition`,
`CN3081 v6.250401`, source pages, BR IDs and independent historical verification.
Original URLs/documents and indexed evidence remain privately preserved; missing
standalone scratch artifacts are not claimed recoverable.

Fresh verification on 13 November 2026: all 38 BR rows and 106 named assertion
mappings are preserved; seven test JSON fixture files compare equal after only
approved identity/source metadata normalization. Original doc hashes match T35.
Portable links resolve and two stale API test line references were corrected to
actual definitions. Scoped identity/provenance scan and `git diff --check` pass.
Sol medium worker independently reconciled the original facts and recorded:

- `cargo test --locked --test rules_conformance`: exit 0, 7 tests.
- `cargo test --locked --test game_sequences`: exit 0, 31 tests.
- `cargo test --locked --test snapshot_import`: exit 0, 5 tests.

No expected answer was regenerated from production code and no rule assertion
was weakened. These focused suites verify the renamed executable; they do not
claim historical scratch programs were rerun.
