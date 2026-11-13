# Setup source provenance

Historical source capture and visual verification: 9 November 2026.

The original provenance document is privately preserved under its SHA-256 ID
`5c991e2a7a868273a3895a178a313a7c8d27df72270224f57f7b79d4d1b92e93`.
T35 verified the inventoried original documents, archived members and recorded
Git blob copies byte-for-byte. Only those records are claimed recoverable;
standalone scratch scripts and diagnostic images outside that inventory are
not established as retained. Original source URLs remain private. The portable
[baseline specification](../../SPEC.md) and
[rule catalogue](../../SPEC-base-rules.md) define the approved requirements.

Source: 2025 English sixth-edition rulebook CN3081 v6.250401, downloaded from the original privately preserved source URL. PDF SHA-256: `0d81a40c1d4eb5eb438a62f80d71cdaab5f00404722c3cfa0b106d79b635ebc6`. Its metadata lists 12 pages, 594 x 774 pt pages, and print permission. Complete relevant pages 3, 4, 5 and 11 were rendered and visually inspected. Higher-resolution renders used 220 DPI for the fixed spread, 400 DPI for the component templates and 320 DPI for the number-placement diagrams.

`setup.json` contains the captured fixture. Diagnostic images and the capture script were temporary references; their standalone bytes are not established as retained. Image names below identify historical diagnostics, not available product assets. The diagnostic overlay was derived independently from the approved logical geometry. Historically, `fixed-spread.png` joined pages 4 and 5; `fixed-graph-overlay.png` marked all vertex IDs in red and edge IDs in black. Pixel references below use that 1536 x 1001 diagnostic spread, not PDF coordinates. The approved geometry was approximately fitted as x = 737 + 78 * logical_x; y = 503 + 44.5 * logical_y. This mapping is a visual diagnostic, not an exact measurement of publisher art.

## Fixed pieces

The second settlements have black outlines in the source. First/second order is therefore observable, not inferred from resource payouts.

| Role | First vertex / road | Second vertex / road | Second hand (brick, wood, wool, wheat, ore) |
|---|---|---|---|
| Blue | 31 / 46 | 40 / 57 | 1, 0, 0, 1, 1 |
| Red | 39 / 55 | 9 / 14 | 0, 1, 1, 1, 0 |
| Orange | 19 / 21 | 17 / 25 | 1, 0, 1, 1, 0 |
| White | 41 / 59 | 29 / 42 | 2, 0, 0, 1, 0 |

Source pages 4-5 visibly show all eight placements and road directions. Observe the Blue second road near spread pixel (698,703), Red second road near (776,303), Orange first road near (893,370), and White second road near (620,570); exact locations can be found using the overlay's IDs. The full terrain/token arrays and settlement triples in SPEC-base-rules.md match the source. Page 5 expressly omits White with three players and shows the four starting-hand bundles. No fixture discrepancy found.

## Fixed ports and frame templates

Port endpoints are the two land vertices where the printed pair of piers meets the coast. Reading the icon alone would not determine a port edge. Starting at the northwest port and moving clockwise, observed ports are:

| Edge | Endpoints | Kind | Diagnostic vicinity |
|---|---|---|---|
| 0 | 0, 3 | generic 3:1 | (542,169), page 4 north |
| 3 | 1, 5 | wool 2:1 | (776,169), spread north seam |
| 17 | 10, 15 | generic 3:1 | (1010,303), page 5 northeast |
| 38 | 26, 32 | generic 3:1 | (1127,503), page 5 east |
| 61 | 42, 46 | brick 2:1 | (1010,703), page 5 southeast |
| 69 | 49, 52 | wood 2:1 | spread south; see south-id.png |
| 66 | 47, 51 | generic 3:1 | page 4 southwest; see south-id.png |
| 49 | 33, 38 | wheat 2:1 | page 4 west; see west-id.png |
| 18 | 11, 16 | ore 2:1 | page 4 northwest; see west-id.png |

The six printed frame pieces are pictured separately on page 3. Left-to-right protruding-tab labels are 2, 4, 1, 3, 6, 5; their other notches are 1, 3, 6, 2, 5, 4 respectively. Historical source images `frame-components.png` and `frame-ends-*.png` recorded the icons and labels. Template IDs in `setup.json` use the protruding-tab label. This is a capture convention, not an additional public rules option.

Partition the clockwise coast from VertexId 3 into six five-edge segments. Canonical north-to-northwest slots have edges:

```text
north:     0, 1, 2, 3, 4
northeast: 5, 9, 17, 22, 32
southeast: 38, 48, 53, 61, 65
south:     71, 70, 69, 68, 67
southwest: 66, 62, 54, 49, 39
northwest: 33, 23, 18, 10, 6
```

Offsets are zero-based in the destination segment's clockwise order. Template 1 has generic@0 and wool@3; 2 generic@2; 3 generic@0 and brick@3; 4 wood@2; 5 generic@0 and wheat@3; 6 ore@2. The fixed permutation [1,2,3,4,5,6] reproduces all nine observed ports exactly. Shuffling these templates preserves their printed spacing and individual port kinds. Source connectors reverse order when reading each separated piece left-to-right; the stored offsets deliberately follow the island's clockwise traversal instead.

## Number sequence

Page 11 contains paired letter-face/number-face diagrams. Each corresponding disc was visually checked, yielding A-R = 5,2,6,3,8,10,9,12,11,4,8,10,9,4,5,6,3,11. Historical `number-boards.png` recorded both diagrams. In the source example the desert is HexId 14, so the observed letter positions are 2,1,0,3,7,12,16,17,18,15,11,6,5,4,8,13,10,9. The full approved top-right spiral inserts the skipped desert position: 2,1,0,3,7,12,16,17,18,15,11,6,5,4,8,13,14,10,9. The source text permits any starting corner; using top-right remains the already-approved digital policy.

## Confidence and verification limits

High confidence for terrain, tokens, all piece directions, port endpoints/types, six frame templates/spacing, and A-R/spiral. No unresolved diagram ambiguity found. The original capture report records that `capture_data.py` passed assert-based checks for road/building incidence, starting-hand calculation from observed hexes, coastal membership, fixed/template port equivalence and corresponding source letters/numbers. The repository tests check the capture against literal values and graph topology; creation tests compare actual setup results against this fixture (see the [assertion map](../../tasks/rules-conformance.md)). The source art must remain a reference, not a product asset.
