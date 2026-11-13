# Changelog

## 0.1.0

Initial Hex Traders source release, published as `hex-traders-engine` and imported
as `hex_traders_engine`.

- Complete three- and four-seat base-game play in fixed and variable setups,
  including trading, all development cards, exact Longest Route and Largest Army,
  and immediate victory.
- Atomic commands, legal choices, typed rejections and seat/spectator projections
  preserve hidden information and leave state and randomness unchanged on rejection.
- Strict JSON API, host-private save/resume and deterministic accepted-command
  replay. API, snapshot schema and replay format are version `1`; rules identity is
  `hex-traders-base-v1`, and RNG identity is `chacha12-rand010-v1`.
- Native Linux verification with Rust 1.98.1 and edition 2024. The host supplies
  authentication, private seeds, storage and transport; browser bindings, UI,
  networking and package-registry publication are outside this release.

Original project code and documentation are available under either
[MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option. Third-party
materials retain their own licenses: repository agent tooling retains Addy
Osmani's MIT notice in `.agents/LICENSE` and attribution in `.agents/SOURCE.json`;
it is excluded from the Cargo package. Dependencies retain their own licenses.
This grant covers original implementation and documentation; it grants no rights
to third-party artwork, trademarks or copied rulebook text.
