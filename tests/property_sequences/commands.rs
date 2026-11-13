use super::*;
#[path = "../game_sequences/verified_game.rs"]
mod verified_game;
use hex_traders_engine::{
    CardId, Command, CommandEnvelope, EdgeId, HexId, LegalAction, PlayerId, Resource, Resources,
};
use verified_game::VerifiedGame;

fn bundle(available: &[u8; 5], quantity: u8, offset: usize) -> Resources {
    let mut left = quantity;
    let mut chosen = [0; 5];
    for i in 0..5 {
        let r = (i + offset) % 5;
        chosen[r] = left.min(available[r]);
        left -= chosen[r];
    }
    assert_eq!(left, 0);
    Resources::try_from(chosen).unwrap()
}
fn pick<T: Copy>(values: &[T], decision: usize) -> T {
    values[decision % values.len()]
}
fn resource(index: usize) -> Resource {
    [
        Resource::Brick,
        Resource::Wood,
        Resource::Wool,
        Resource::Wheat,
        Resource::Ore,
    ][index % 5]
}
// Only assemble public choices; shared validation still decides eligibility.
pub(super) fn supplied(e: &Engine, seat: PlayerId, action: &LegalAction, d: usize) -> Command {
    match action {
        LegalAction::PlaceSetupSettlement { vertices } => Command::PlaceSetupSettlement {
            vertex: pick(vertices, d),
        },
        LegalAction::PlaceSetupRoad { edges } => Command::PlaceSetupRoad {
            edge: pick(edges, d),
        },
        LegalAction::BuildSettlement { vertices } => Command::BuildSettlement {
            vertex: pick(vertices, d),
        },
        LegalAction::UpgradeCity { vertices } => Command::UpgradeCity {
            vertex: pick(vertices, d),
        },
        LegalAction::BuildRoad { edges } => Command::BuildRoad {
            edge: pick(edges, d),
        },
        LegalAction::PlaceFreeRoad { edges } => Command::PlaceFreeRoad {
            edge: pick(edges, d),
        },
        LegalAction::MoveRobber { hexes } => Command::MoveRobber {
            hex: pick(hexes, d),
        },
        LegalAction::ChooseRobberVictim { victims } => Command::ChooseRobberVictim {
            victim: pick(victims, d),
        },
        LegalAction::RollDice => Command::RollDice,
        LegalAction::EndTurn => Command::EndTurn,
        LegalAction::BuyDevelopmentCard => Command::BuyDevelopmentCard,
        LegalAction::PlayKnight { cards } => Command::PlayKnight {
            card: pick(cards, d),
        },
        LegalAction::PlayRoadBuilding { cards } => Command::PlayRoadBuilding {
            card: pick(cards, d),
        },
        LegalAction::PlayMonopoly { cards } => Command::PlayMonopoly {
            card: pick(cards, d),
            resource: resource(d),
        },
        LegalAction::PlayInvention { cards } => {
            let bank = e.view(Viewer::Spectator).unwrap().bank;
            let quantity = bank.counts().iter().copied().sum::<u8>().min(2);
            Command::PlayInvention {
                card: pick(cards, d),
                resources: bundle(bank.counts(), quantity, d % 5),
            }
        }
        LegalAction::DiscardResources { quantity } => {
            let hand = e
                .view(Viewer::Seat(seat))
                .unwrap()
                .own_player
                .unwrap()
                .resources;
            Command::DiscardResources {
                resources: bundle(hand.counts(), *quantity, d % 5),
            }
        }
        LegalAction::BankTrade { pairs } => {
            let (give_resource, receive_resource) = pick(pairs, d);
            Command::BankTrade {
                give_resource,
                receive_resource,
            }
        }
        LegalAction::ProposeTrade { recipients } => {
            let hand = e
                .view(Viewer::Seat(seat))
                .unwrap()
                .own_player
                .unwrap()
                .resources;
            let give = bundle(hand.counts(), 1, d % 5);
            let given = give.counts().iter().position(|&q| q > 0).unwrap();
            let mut receive = [0; 5];
            receive[(given + 1 + d % 4) % 5] = 1;
            Command::ProposeTrade {
                recipient: pick(recipients, d),
                give,
                receive: Resources::try_from(receive).unwrap(),
            }
        }
        LegalAction::AcceptTrade { offer_id } => Command::AcceptTrade {
            offer_id: *offer_id,
        },
        LegalAction::RejectTrade { offer_id } => Command::RejectTrade {
            offer_id: *offer_id,
        },
        LegalAction::CancelTrade { offer_id } => Command::CancelTrade {
            offer_id: *offer_id,
        },
    }
}
fn placement_agreement(e: &Engine, seat: PlayerId, choices: &[LegalAction]) {
    for id in 0..72_u8 {
        let edge = EdgeId::try_from(id).unwrap();
        for (command, listed) in [
            (
                Command::BuildRoad { edge },
                choices
                    .iter()
                    .any(|a| matches!(a,LegalAction::BuildRoad {edges} if edges.contains(&edge))),
            ),
            (
                Command::PlaceFreeRoad { edge },
                choices.iter().any(
                    |a| matches!(a,LegalAction::PlaceFreeRoad {edges} if edges.contains(&edge)),
                ),
            ),
            (
                Command::PlaceSetupRoad { edge },
                choices.iter().any(
                    |a| matches!(a,LegalAction::PlaceSetupRoad {edges} if edges.contains(&edge)),
                ),
            ),
        ] {
            assert_eq!(e.validate(seat, &command).is_ok(), listed);
        }
    }
    for id in 0..54_u8 {
        let vertex = VertexId::try_from(id).unwrap();
        for (command,listed) in [
            (Command::BuildSettlement {vertex},choices.iter().any(|a|matches!(a,LegalAction::BuildSettlement {vertices} if vertices.contains(&vertex)))),
            (Command::UpgradeCity {vertex},choices.iter().any(|a|matches!(a,LegalAction::UpgradeCity {vertices} if vertices.contains(&vertex)))),
            (Command::PlaceSetupSettlement {vertex},choices.iter().any(|a|matches!(a,LegalAction::PlaceSetupSettlement {vertices} if vertices.contains(&vertex)))),
        ] { assert_eq!(e.validate(seat,&command).is_ok(),listed); }
    }
}
fn arbitrary_command(d: u16) -> Command {
    let n = usize::from(d / 21);
    let vertex = VertexId::try_from((n % 54) as u8).unwrap();
    let edge = EdgeId::try_from((n % 72) as u8).unwrap();
    let card = CardId::try_from((n % 25) as u8).unwrap();
    let resources = Resources::try_from([(n % 20) as u8; 5]).unwrap();
    match d % 21 {
        0 => Command::PlaceSetupSettlement { vertex },
        1 => Command::PlaceSetupRoad { edge },
        2 => Command::RollDice,
        3 => Command::DiscardResources { resources },
        4 => Command::MoveRobber {
            hex: HexId::try_from((n % 19) as u8).unwrap(),
        },
        5 => Command::ChooseRobberVictim {
            victim: PlayerId::try_from((n % 4) as u8).unwrap(),
        },
        6 => Command::PlaceFreeRoad { edge },
        7 => Command::BuildRoad { edge },
        8 => Command::BuildSettlement { vertex },
        9 => Command::UpgradeCity { vertex },
        10 => Command::BuyDevelopmentCard,
        11 => Command::PlayKnight { card },
        12 => Command::PlayRoadBuilding { card },
        13 => Command::PlayMonopoly {
            card,
            resource: resource(n),
        },
        14 => Command::PlayInvention { card, resources },
        15 => Command::BankTrade {
            give_resource: resource(n),
            receive_resource: resource(n + 1),
        },
        16 => Command::ProposeTrade {
            recipient: PlayerId::try_from((n % 4) as u8).unwrap(),
            give: resources,
            receive: resources,
        },
        17 => Command::AcceptTrade { offer_id: n as u64 },
        18 => Command::RejectTrade { offer_id: n as u64 },
        19 => Command::CancelTrade { offer_id: n as u64 },
        _ => Command::EndTurn,
    }
}
fn advance(game: &mut VerifiedGame, d: u16) -> bool {
    facts(game);
    let before = game.export_snapshot().unwrap();
    let view = game.view(Viewer::Spectator).unwrap();
    let mut actions = Vec::new();
    for &seat in &view.seat_order {
        let query = game.legal_actions(seat).unwrap();
        assert_eq!(query.revision, view.revision);
        placement_agreement(game, seat, &query.choices);
        for action in query.choices {
            let command = supplied(game, seat, &action, usize::from(d));
            assert!(game.validate(seat, &command).is_ok());
            actions.push((seat, command));
        }
        let command = arbitrary_command(d);
        if let Err(expected) = game.validate(seat, &command) {
            let rejected = game
                .execute(CommandEnvelope {
                    api_version: 1,
                    actor: seat,
                    expected_revision: view.revision,
                    command,
                })
                .unwrap_err();
            assert_eq!(rejected, expected);
            assert_eq!(game.export_snapshot().unwrap(), before);
        }
        let malformed = CommandEnvelope {
            api_version: if d.is_multiple_of(2) { 2 } else { 1 },
            actor: seat,
            expected_revision: view.revision + 1,
            command,
        };
        assert!(game.execute(malformed).is_err());
        assert_eq!(game.export_snapshot().unwrap(), before);
    }
    assert_eq!(game.export_snapshot().unwrap(), before);
    if actions.is_empty() {
        return false;
    }
    let (seat, command) = pick(&actions, usize::from(d));
    game.act(seat.index() as u8, command);
    facts(game);
    true
}
proptest! {
    #![proptest_config(ProptestConfig { failure_persistence: Some(Box::new(FileFailurePersistence::Direct("tests/property_sequences.proptest-regressions"))), cases: 32, max_shrink_iters: 2048, rng_seed: RngSeed::Fixed(0xca7a0031), .. ProptestConfig::default() })]
    #[test]
    fn property_reached_decision_vectors_preserve_facts_queries_rejections_and_replay(
        four in any::<bool>(), variable in any::<bool>(), seed in any::<u32>(), rotation in 0_u8..4,
        decisions in prop::collection::vec(any::<u16>(),0..65)
    ) {
        let mut game=VerifiedGame::new(create(four,variable,seed,rotation));
        // Finish actual setup through public candidates even when the decision vector shrinks empty.
        for i in 0..16 {
            if !matches!(game.view(Viewer::Spectator).unwrap().phase,Phase::SetupSettlement{..}|Phase::SetupRoad{..}) {break;}
            advance(&mut game,decisions.get(i).copied().unwrap_or(0));
        }
        for d in decisions {if !advance(&mut game,d){break;}}
    }
}
