# Complete three/four-seat fixed/variable game fixtures

The original provenance document is privately preserved under SHA-256 ID
`acf63d5ce02f30c37982b918b2717a5c52e6637f703bcfaa26a399a1e432aa9f`.
T35 verified the inventoried original documents, archived members and recorded
Git blob copies byte-for-byte. The full-game preparation described below is an
actual retained archive record. Only inventoried records are claimed recoverable;
additional standalone scratch oracles outside the inventory are not established
as retained. Original URLs and archive labels remain private. Portable public
references are [setup provenance](setup-source.md),
[victory provenance](victory-source.md), the
[rule catalogue](../../SPEC-base-rules.md) and the
[assertion map](../../tasks/rules-conformance.md).

`full-games.json` retains three synthetic publicly reached histories: fixed4
seed4935, variable3 seed7943 and variable4 seed7943. They supplement the235
existing fixed3 records in `victory-prefixes.json`; no existing frozen record
was replaced. Seeds are deterministic fixtures. Resource order is
brick/wood/wool/wheat/ore; seat order is0/1/2[/3]. Encode each seed as u32
little-endian in bytes0..3 and zeros in the remaining28bytes.

The source baseline is the approved 2025 English sixth edition, CN3081 v6.250401: pp4–5 fixed layout/grants, p11 printed
frame spacing and A–R spiral, p12 forward/reverse placements, pp6–10 economy,
progress cards, awards and first winning moment. PDF SHA256:
`0d81a40c1d4eb5eb438a62f80d71cdaab5f00404722c3cfa0b106d79b635ebc6`.
`setup-source.md` records independent visual capture; `setup.json` and the
hand-recorded `longest-route.json` endpoints are source inputs. Compact endpoint
list SHA256 is
`fab1cb317b2faefa851c273cf2bbfddbb235f195f7662020cd30f36dd4894b3d`.

The privately archived independent full-game preparation contains these original records:
`inputs.json` was frozen before product execution; `creation-literals.json`
contains source-derived layouts/deck/starters/setup; `source.py`/`verify.py`
recompute only these selected sequences using independent source incidence and
integer arithmetic, with no product imports or production rule solver. Its
separately linked pinned rand0.10.3 unbiased/ChaCha12 rand_chacha0.10.0 sampler
supplies literal draws, cursor positions and dice. `verified.json` adds every
roll/action's literal balances, payments/cards/acquisition/score/held IDs.
The checked-in fixture is the compact form plus three final receipt revisions.
Tests read only checked-in fixtures; the inventoried preparation scripts, logs
and checksums are retained privately. These statements describe historical
preparation, not a new execution of those scripts during the identity change.

| Configuration | Starter/source word | Exact win turn/revision | Final word | Next dice/word |
|---|---|---|---|---|
|fixed4 seed4935|White/12|110/301|232|Red3,6/234|
|variable3 seed7943|Red/13|93/249|199|Red1,2/201|
|variable4 seed7943|Red/15|104/286|223|Red1,5/225|

Variable setup commands actually place the legal human-selected preset pairs
forward then reverse; no private mutation installs the position. Fixed source
Blue31/E46+40/E57, Red39/E55+9/E14, Orange19/E21+17/E25 and White41/E59+29/E42
are also the chosen variable pairs. Creation assertions check full terrain,
number, port, hidden deck, source RNG word, starter rounds, empty variable
board/bank, phases, every setup grant, occupancy and stock.

Blue has no port at its two interior vertices, so all chosen ordinary bank
exchanges are4:1. H0/H7 robber moves have no adjacent chosen buildings and
consume no theft sample. Red's two variable settlements each produce wool on5;
the summed two-card credit is explicitly asserted. Variable3 turn78 and
variable4 turn75 record multi-seat wool shortages paying none. Every resource
sums to19 after every roll/discard/action; expected balances were not copied
from engine output. Fixed4 uses Invention18; both variable cases use Monopoly16;
existing fixed3 Knight wins supply the remaining development kind.

Each final score is two cities4 plus four held VP4, then first freeE51 joins
`V31–E46–V36–E45–V30–E44–V35–E51–V40–E57–V44` into exact route5 worth2.
Before that placement true score8/public4; after it true/public10. Empty E47
[31,37] remains connected with road stock11→10, so a second free road would
be legal absent immediate victory. RQ1/RQ2 are approved engine policies,
not publisher quotations; these fixture plays have ample legal stock/bank.

All accepted commands, including setup, are fed in order to one ReplayVerifier
initialized at creation. Every command also resumes from the full preceding
snapshot and compares exact receipt, subsequent snapshot/RNG, every authorized
view/projection and query results; stale retries preserve bytes. Source terminal
cursor and independently sampled next dice are checked in a nonwinning control
branch. Finished legal choices are empty, revelations/final scores are exact,
and rejected terminal commands preserve the complete private snapshot.
