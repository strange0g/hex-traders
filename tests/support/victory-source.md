# Immediate-victory source fixtures

The original provenance document is privately preserved under SHA-256 ID
`af2ee2c2214795ded9d7ebd57ed4687d055c69061b2f7b3f31ec3f2effa2ad92`.
T35 verified the inventoried original documents, archived members and recorded
Git blob copies byte-for-byte. The archived immediate-victory and winning-history
preparations include independent inputs, arithmetic/sampler sources, locks,
frozen results and logs; the fixture-formatting script is also archived.
Only inventoried records are claimed recoverable; additional standalone scratch
oracles outside the inventory are not established as retained. Original URLs
and archive labels remain private. See [setup provenance](setup-source.md), the
[rule catalogue](../../SPEC-base-rules.md) and the
[assertion map](../../tasks/rules-conformance.md) for portable public references.

These deterministic fixtures were frozen on 10 November 2026 before the final
commands were run through the product. Arrays use brick, wood, wool, wheat, ore.
They are explicit test seeds, not private real-game data. Tests read only checked-in
literals; no temporary file or production calculator supplies expected values.

`victory-prefixes.json` contains three complete fixed three-seat histories.
Each record freezes starting balances, dice, seven choices, resolved roll balances,
actions and their resulting balances/scores, and final balances/robber location.
All seats use the source presets in `setup.json`: approved 2025 English
sixth edition, CN3081 v6.250401 pp.4–6. The source PDF SHA-256 is
`0d81a40c1d4eb5eb438a62f80d71cdaab5f00404722c3cfa0b106d79b635ebc6`.
Both cities replace Blue V40/V31. Robber destinations H0/H7 touch no preset
building, so no theft consumes extra randomness or changes source production.
Every listed maritime exchange has the source 4:1 rate.

The independent pinned rand 0.10.3 unbiased / rand_chacha 0.10.0 oracle shuffles
canonical card IDs, samples starting dice with tied-highest rerolls, then samples
each gameplay dice pair first die before second. Seeds are u32 little endian in
bytes 0–3 with the remaining 28 bytes zero, stream zero.

- Seed 2441 starts Orange and draws [22,12,3,0,21,23,24]. Purchase mode wins
  on the second turn-68 purchase: two cities + Army + four VP = 10. Knight mode
  holds the third Knight until turn 71 and wins on reveal: two cities + four VP
  + Army = 10, without a robber prompt. Independent source arithmetic,
  sampler/lock, full literals and logs belong to the privately archived
  immediate-victory preparation.
- Seed 3912 starts Blue and draws [15,14,4,20,22,23,17,21]. It has two cities,
  four VP, E46/E45/E44 and separate E57 at turn 97. The first free E51 makes the
  exact five-edge trail V31–E46–V36–E45–V30–E44–V35–E51–V40–E57–V44,
  acquiring Route and winning 8→10. E47 [31,37] is still empty and connected
  through the owned city V31 with ten road pieces available, so victory overrides
  a possible second placement. Endpoint literals come from `longest-route.json`
  with SHA-256 fab1cb317b2faefa851c273cf2bbfddbb235f195f7662020cd30f36dd4894b3d.
  Independent inputs, source verifier and accepted C06 public prefix belong
  to the privately archived winning-history preparation.

The archived fixture-formatting script only rearranges these frozen facts into the common
fixture shape; it does not calculate rules, balances or dice. Public control
branches retain the next dice [1,5], [4,3] and [4,1] respectively. Native tests
check actual RNG equality around each immediate terminal transition. Four setup /
player-count complete wins are still required by T29; these three histories prove
the three exact card-related winning moments required by T21.
