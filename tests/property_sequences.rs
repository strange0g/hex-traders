//! Bounded synthetic data only; pinned proptest seeds and decision vectors can shrink.
use hex_traders_engine::{Engine, GameConfig, Phase, VertexId, Viewer};
use proptest::{
    prelude::*,
    test_runner::{FileFailurePersistence, RngSeed},
};
use serde_json::Value;
use std::collections::BTreeSet;

fn create(four: bool, variable: bool, seed_word: u32, rotation: u8) -> Engine {
    let n = if four { 4 } else { 3 };
    let mut seats = (0..n).collect::<Vec<_>>();
    seats.rotate_left(usize::from(rotation % n));
    let config = if variable {
        GameConfig::variable(&seats)
    } else {
        GameConfig::fixed(&seats)
    }
    .unwrap();
    let mut seed = [0; 32];
    seed[..4].copy_from_slice(&seed_word.to_le_bytes());
    Engine::create(config, seed).unwrap().0
}
fn count(v: &Value) -> usize {
    v.as_u64().unwrap().try_into().unwrap()
}
fn resources(v: &Value) -> [u8; 5] {
    ["brick", "wood", "wool", "wheat", "ore"].map(|r| count(&v[r]).try_into().unwrap())
}
fn facts(e: &Engine) {
    let before = e.export_snapshot().unwrap();
    let snapshot: Value = serde_json::from_slice(&before).unwrap();
    let p = &snapshot["position"];
    let view = e.view(Viewer::Spectator).unwrap();
    assert!(view.own_player.is_none());
    let hands: [[u8; 5]; 4] = std::array::from_fn(|i| resources(&p["hands"][i]));
    let bank: [u8; 5] = resources(&p["bank"]);
    for r in 0..5 {
        assert_eq!(
            u16::from(bank[r]) + hands.iter().map(|h| u16::from(h[r])).sum::<u16>(),
            19
        );
    }
    for id in 0..54 {
        if view.board.buildings[id].is_some() {
            assert!(
                view.board
                    .graph
                    .vertex_neighbors(VertexId::try_from(id as u8).unwrap())
                    .iter()
                    .all(|v| view.board.buildings[v.index()].is_none())
            );
        }
    }
    let deck = p["deck"].as_array().unwrap();
    let locations = p["cardLocations"].as_array().unwrap();
    let cursor = count(&p["drawCursor"]);
    assert_eq!(
        deck.iter().map(count).collect::<BTreeSet<_>>(),
        (0..25).collect()
    );
    assert_eq!(locations.len(), 25);
    assert_eq!(usize::from(view.development_deck_remaining) + cursor, 25);
    for (draw, id) in deck.iter().enumerate() {
        assert_eq!(locations[count(id)]["type"] == "undrawn", draw >= cursor);
    }
    let mut lengths = [0; 4];
    let mut knights = [0; 4];
    for player in &view.players {
        let owner = player.seat.index();
        let own = e
            .view(Viewer::Seat(player.seat))
            .unwrap()
            .own_player
            .unwrap();
        assert_eq!(own.resources.counts(), &hands[owner]);
        assert_eq!(
            usize::from(player.resource_count),
            hands[owner].iter().map(|&c| usize::from(c)).sum::<usize>()
        );
        let roads = view
            .board
            .roads
            .iter()
            .filter(|&&r| r == Some(player.seat))
            .count();
        let settlements = p["buildings"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|b| b["owner"] == owner && b["kind"] == "settlement")
            .count();
        let cities = p["buildings"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|b| b["owner"] == owner && b["kind"] == "city")
            .count();
        assert_eq!(roads + usize::from(player.roads_remaining), 15);
        assert_eq!(settlements + usize::from(player.settlements_remaining), 5);
        assert_eq!(cities + usize::from(player.cities_remaining), 4);
        let held = locations
            .iter()
            .enumerate()
            .filter(|(_, l)| l["type"] == "held" && l["owner"] == owner)
            .map(|(id, _)| id)
            .collect::<Vec<_>>();
        assert_eq!(
            held,
            own.development_cards
                .iter()
                .map(|c| c.id.index())
                .collect::<Vec<_>>()
        );
        assert_eq!(held.len(), usize::from(player.development_card_count));
        let played = locations
            .iter()
            .filter(|l| l["type"] == "playedKnight" && l["owner"] == owner)
            .count();
        let spent = locations
            .iter()
            .filter(|l| l["type"] == "spent" && l["owner"] == owner)
            .count();
        assert_eq!(played, usize::from(player.played_knights));
        assert_eq!(spent, player.spent_progress.len());
        let vp = locations
            .iter()
            .enumerate()
            .filter(|(id, l)| {
                *id >= 20
                    && l["owner"] == owner
                    && (l["type"] == "held" || l["type"] == "revealedVictory")
            })
            .count();
        let visible_vp = locations
            .iter()
            .filter(|l| l["owner"] == owner && l["type"] == "revealedVictory")
            .count();
        let awards = 2 * usize::from(view.longest_route_holder == Some(player.seat))
            + 2 * usize::from(view.largest_army_holder == Some(player.seat));
        assert_eq!(
            usize::from(own.true_score),
            settlements + 2 * cities + vp + awards
        );
        assert_eq!(
            usize::from(player.public_score),
            settlements + 2 * cities + visible_vp + awards
        );
        lengths[owner] = player.longest_route_length;
        knights[owner] = player.played_knights;
        assert!(usize::from(player.longest_route_length) <= roads);
        e.legal_actions(player.seat).unwrap();
    }
    for (values, holder, threshold) in [
        (lengths, view.longest_route_holder, 5),
        (knights, view.largest_army_holder, 3),
    ] {
        let max = values.into_iter().max().unwrap();
        if max < threshold {
            assert!(holder.is_none());
        } else if values.iter().filter(|&&v| v == max).count() == 1 {
            assert_eq!(
                holder.unwrap().index(),
                values.iter().position(|&v| v == max).unwrap()
            );
        } else if let Some(holder) = holder {
            assert_eq!(values[holder.index()], max);
        }
    }
    assert!(view.seat_order.contains(&view.active_player));
    if let Phase::Discard { obligations } = view.phase {
        assert!(obligations.iter().any(|&q| q > 0));
        for (owner, &q) in obligations.iter().enumerate() {
            if q > 0 {
                assert_eq!(
                    usize::from(q),
                    hands[owner].iter().map(|&c| usize::from(c)).sum::<usize>() / 2
                );
            }
        }
    }
    if let Phase::Finished { winner, score } = view.phase {
        assert!(score >= 10);
        assert_eq!(winner, view.active_player);
        for &seat in &view.seat_order {
            assert!(e.legal_actions(seat).unwrap().choices.is_empty());
        }
    }
    assert_eq!(snapshot["random"]["stream"], "0");
    assert!(
        snapshot["random"]["wordPosition"]
            .as_str()
            .unwrap()
            .parse::<u128>()
            .unwrap()
            < 1_u128 << 68
    );
    assert_eq!(e.export_snapshot().unwrap(), before);
    assert_eq!(
        Engine::from_snapshot(&before)
            .unwrap()
            .export_snapshot()
            .unwrap(),
        before
    );
}
proptest! {
    #![proptest_config(ProptestConfig { failure_persistence: Some(Box::new(FileFailurePersistence::Direct("tests/property_sequences.proptest-regressions"))), cases: 64, max_shrink_iters: 2048, rng_seed: RngSeed::Fixed(0xca7a0030), .. ProptestConfig::default() })]
    #[test]
    fn property_created_configurations_have_independent_conservation_and_scores(
        four in any::<bool>(), variable in any::<bool>(), seed in any::<u32>(), rotation in 0_u8..4
    ) {
        facts(&create(four, variable, seed, rotation));
    }
}

#[path = "property_sequences/commands.rs"]
mod commands;

#[path = "property_sequences/malformed.rs"]
mod malformed;
