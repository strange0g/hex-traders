//! Literal CN3081 preset production and pinned rand/chacha traces; no runtime temporary inputs.
use super::seat;
use hex_traders_engine::{
    CardId, Command, CommandEnvelope, Engine, Event, GameConfig, HexId, LegalAction, Phase,
    ProjectedEvent, Receipt, ResumePhase, RobberOrigin, RuleError, Viewer,
};
fn card(id: u8) -> CardId {
    CardId::try_from(id).unwrap()
}
fn hex(id: u8) -> HexId {
    HexId::try_from(id).unwrap()
}
fn act(e: &mut Engine, actor: u8, command: Command) -> Receipt {
    let revision = e.view(Viewer::Spectator).unwrap().revision;
    assert!(e.validate(seat(actor), &command).is_ok());
    let receipt = e
        .execute(CommandEnvelope {
            api_version: 1,
            actor: seat(actor),
            expected_revision: revision,
            command,
        })
        .unwrap();
    assert_eq!(
        (receipt.previous_revision, receipt.revision),
        (revision, revision + 1)
    );
    receipt
}
type TurnFacts = (u8, [u8; 2], [[u8; 5]; 4], [u8; 5]);

struct Case {
    seed: u32,
    seats: u8,
    turns: &'static [TurnFacts],
    buys: &'static [(u64, u8)],
    plays: &'static [(u64, u8, u8)],
    next: [u8; 2],
}
fn cases() -> Vec<Case> {
    vec![
        Case {
            seed: 511,
            seats: 3,
            turns: &[
                (
                    0,
                    [4, 1],
                    [
                        [1, 0, 0, 0, 0],
                        [0, 1, 1, 1, 0],
                        [1, 0, 1, 2, 0],
                        [0, 0, 0, 0, 0],
                    ],
                    [17, 18, 17, 16, 19],
                ),
                (
                    1,
                    [3, 6],
                    [
                        [2, 0, 0, 0, 0],
                        [1, 1, 1, 1, 0],
                        [1, 0, 1, 2, 1],
                        [0, 0, 0, 0, 0],
                    ],
                    [15, 18, 17, 16, 18],
                ),
                (
                    2,
                    [3, 5],
                    [
                        [2, 1, 0, 0, 0],
                        [1, 1, 1, 2, 0],
                        [1, 0, 1, 2, 1],
                        [0, 0, 0, 0, 0],
                    ],
                    [15, 17, 17, 15, 18],
                ),
            ],
            buys: &[(1, 7)],
            plays: &[(4, 7, 7)],
            next: [6, 1],
        },
        Case {
            seed: 785,
            seats: 3,
            turns: &[
                (
                    0,
                    [1, 4],
                    [
                        [1, 0, 0, 0, 0],
                        [0, 1, 1, 1, 0],
                        [1, 0, 1, 2, 0],
                        [0, 0, 0, 0, 0],
                    ],
                    [17, 18, 17, 16, 19],
                ),
                (
                    1,
                    [5, 6],
                    [
                        [1, 1, 0, 0, 0],
                        [0, 1, 2, 1, 0],
                        [1, 1, 1, 2, 0],
                        [0, 0, 0, 0, 0],
                    ],
                    [17, 16, 16, 16, 19],
                ),
                (
                    2,
                    [1, 1],
                    [
                        [1, 1, 0, 0, 0],
                        [0, 1, 2, 2, 0],
                        [1, 1, 1, 2, 0],
                        [0, 0, 0, 0, 0],
                    ],
                    [17, 16, 16, 15, 19],
                ),
                (
                    0,
                    [3, 2],
                    [
                        [1, 1, 1, 0, 0],
                        [0, 1, 2, 2, 0],
                        [1, 1, 1, 3, 0],
                        [0, 0, 0, 0, 0],
                    ],
                    [17, 16, 15, 14, 19],
                ),
                (
                    1,
                    [4, 6],
                    [
                        [1, 1, 1, 0, 0],
                        [0, 1, 2, 2, 0],
                        [1, 1, 1, 3, 0],
                        [0, 0, 0, 0, 0],
                    ],
                    [17, 16, 15, 14, 19],
                ),
                (
                    2,
                    [2, 1],
                    [
                        [1, 1, 1, 1, 0],
                        [0, 1, 2, 2, 0],
                        [1, 1, 2, 3, 0],
                        [0, 0, 0, 0, 0],
                    ],
                    [17, 16, 14, 13, 19],
                ),
                (
                    0,
                    [3, 1],
                    [
                        [1, 1, 0, 0, 0],
                        [0, 2, 2, 2, 0],
                        [1, 2, 2, 3, 0],
                        [0, 0, 0, 0, 0],
                    ],
                    [17, 14, 15, 14, 19],
                ),
                (
                    1,
                    [1, 1],
                    [
                        [1, 1, 0, 0, 0],
                        [0, 2, 2, 3, 0],
                        [1, 2, 2, 3, 0],
                        [0, 0, 0, 0, 0],
                    ],
                    [17, 14, 15, 13, 19],
                ),
                (
                    2,
                    [2, 3],
                    [
                        [1, 1, 1, 0, 0],
                        [0, 2, 2, 3, 0],
                        [1, 2, 2, 4, 0],
                        [0, 0, 0, 0, 0],
                    ],
                    [17, 14, 14, 12, 19],
                ),
                (
                    0,
                    [6, 2],
                    [
                        [1, 2, 1, 0, 0],
                        [0, 2, 2, 4, 0],
                        [1, 2, 2, 4, 0],
                        [0, 0, 0, 0, 0],
                    ],
                    [17, 13, 14, 11, 19],
                ),
                (
                    1,
                    [5, 5],
                    [
                        [1, 2, 1, 0, 0],
                        [0, 2, 2, 4, 0],
                        [1, 2, 2, 4, 0],
                        [0, 0, 0, 0, 0],
                    ],
                    [17, 13, 14, 11, 19],
                ),
                (
                    2,
                    [1, 1],
                    [
                        [1, 2, 1, 0, 0],
                        [0, 2, 2, 5, 0],
                        [1, 2, 2, 4, 0],
                        [0, 0, 0, 0, 0],
                    ],
                    [17, 13, 14, 10, 19],
                ),
                (
                    0,
                    [5, 3],
                    [
                        [1, 3, 1, 0, 0],
                        [0, 2, 2, 6, 0],
                        [1, 2, 2, 4, 0],
                        [0, 0, 0, 0, 0],
                    ],
                    [17, 12, 14, 9, 19],
                ),
                (
                    1,
                    [3, 1],
                    [
                        [1, 3, 1, 0, 1],
                        [0, 3, 2, 6, 0],
                        [1, 3, 2, 4, 0],
                        [0, 0, 0, 0, 0],
                    ],
                    [17, 10, 14, 9, 18],
                ),
                (
                    2,
                    [2, 1],
                    [
                        [1, 3, 1, 1, 1],
                        [0, 3, 2, 6, 0],
                        [1, 3, 3, 4, 0],
                        [0, 0, 0, 0, 0],
                    ],
                    [17, 10, 13, 8, 18],
                ),
                (
                    0,
                    [4, 6],
                    [
                        [1, 3, 0, 0, 0],
                        [0, 3, 2, 6, 0],
                        [1, 3, 3, 4, 0],
                        [0, 0, 0, 0, 0],
                    ],
                    [17, 10, 14, 9, 19],
                ),
                (
                    1,
                    [1, 1],
                    [
                        [1, 3, 0, 0, 0],
                        [0, 3, 2, 7, 0],
                        [1, 3, 3, 4, 0],
                        [0, 0, 0, 0, 0],
                    ],
                    [17, 10, 14, 8, 19],
                ),
                (
                    2,
                    [6, 2],
                    [
                        [1, 4, 0, 0, 0],
                        [0, 3, 2, 8, 0],
                        [1, 3, 3, 4, 0],
                        [0, 0, 0, 0, 0],
                    ],
                    [17, 9, 14, 7, 19],
                ),
            ],
            buys: &[(1, 12), (7, 1), (16, 13)],
            plays: &[(4, 12, 7), (10, 1, 0), (19, 13, 7)],
            next: [4, 1],
        },
        Case {
            seed: 136,
            seats: 4,
            turns: &[
                (
                    0,
                    [1, 4],
                    [
                        [1, 0, 0, 0, 0],
                        [0, 1, 1, 1, 0],
                        [1, 0, 1, 2, 0],
                        [2, 0, 0, 2, 0],
                    ],
                    [15, 18, 17, 14, 19],
                ),
                (
                    1,
                    [6, 5],
                    [
                        [1, 1, 0, 0, 0],
                        [0, 1, 2, 1, 0],
                        [1, 1, 1, 2, 0],
                        [2, 0, 0, 2, 0],
                    ],
                    [15, 16, 16, 14, 19],
                ),
                (
                    2,
                    [1, 2],
                    [
                        [1, 1, 0, 1, 0],
                        [0, 1, 2, 1, 0],
                        [1, 1, 2, 2, 0],
                        [2, 0, 0, 2, 0],
                    ],
                    [15, 16, 15, 13, 19],
                ),
                (
                    3,
                    [6, 2],
                    [
                        [1, 2, 0, 1, 0],
                        [0, 1, 2, 2, 0],
                        [1, 1, 2, 2, 0],
                        [2, 1, 0, 2, 0],
                    ],
                    [15, 14, 15, 12, 19],
                ),
            ],
            buys: &[(1, 9)],
            plays: &[(5, 9, 7)],
            next: [5, 2],
        },
        Case {
            seed: 3190,
            seats: 4,
            turns: &[
                (
                    1,
                    [3, 2],
                    [
                        [1, 0, 1, 1, 1],
                        [0, 1, 1, 1, 0],
                        [1, 0, 1, 2, 0],
                        [2, 0, 0, 2, 0],
                    ],
                    [15, 18, 16, 13, 18],
                ),
                (
                    2,
                    [6, 6],
                    [
                        [1, 0, 1, 1, 1],
                        [0, 1, 1, 1, 0],
                        [1, 0, 1, 2, 0],
                        [3, 0, 0, 2, 0],
                    ],
                    [14, 18, 16, 13, 18],
                ),
                (
                    3,
                    [2, 6],
                    [
                        [1, 1, 1, 1, 1],
                        [0, 1, 1, 2, 0],
                        [1, 0, 1, 2, 0],
                        [3, 1, 0, 2, 0],
                    ],
                    [14, 16, 16, 12, 18],
                ),
                (
                    0,
                    [4, 4],
                    [
                        [1, 2, 0, 0, 0],
                        [0, 1, 1, 3, 0],
                        [1, 0, 1, 2, 0],
                        [3, 2, 0, 2, 0],
                    ],
                    [14, 14, 17, 12, 19],
                ),
                (
                    1,
                    [4, 2],
                    [
                        [1, 2, 0, 0, 0],
                        [0, 1, 1, 3, 1],
                        [2, 0, 1, 2, 0],
                        [3, 2, 0, 2, 0],
                    ],
                    [13, 14, 17, 12, 18],
                ),
                (
                    2,
                    [5, 3],
                    [
                        [1, 3, 0, 0, 0],
                        [0, 1, 1, 4, 1],
                        [2, 0, 1, 2, 0],
                        [3, 3, 0, 2, 0],
                    ],
                    [13, 12, 17, 11, 18],
                ),
                (
                    3,
                    [3, 1],
                    [
                        [1, 3, 0, 0, 1],
                        [0, 2, 1, 4, 1],
                        [2, 1, 1, 2, 0],
                        [3, 3, 0, 2, 1],
                    ],
                    [13, 10, 17, 11, 16],
                ),
                (
                    0,
                    [1, 1],
                    [
                        [1, 3, 0, 0, 1],
                        [0, 2, 1, 5, 1],
                        [2, 1, 1, 2, 0],
                        [3, 3, 0, 2, 1],
                    ],
                    [13, 10, 17, 10, 16],
                ),
                (
                    1,
                    [3, 2],
                    [
                        [1, 3, 1, 0, 1],
                        [0, 2, 1, 5, 1],
                        [2, 1, 1, 3, 0],
                        [3, 3, 0, 3, 1],
                    ],
                    [13, 10, 16, 8, 16],
                ),
                (
                    2,
                    [5, 4],
                    [
                        [2, 3, 1, 0, 1],
                        [1, 2, 1, 5, 1],
                        [2, 1, 1, 3, 1],
                        [4, 3, 0, 3, 1],
                    ],
                    [10, 10, 16, 8, 15],
                ),
                (
                    3,
                    [4, 5],
                    [
                        [3, 3, 1, 0, 1],
                        [2, 2, 1, 5, 1],
                        [2, 1, 1, 3, 2],
                        [5, 3, 0, 3, 1],
                    ],
                    [7, 10, 16, 8, 14],
                ),
                (
                    0,
                    [1, 5],
                    [
                        [3, 3, 1, 0, 1],
                        [2, 2, 1, 5, 2],
                        [3, 1, 1, 3, 2],
                        [5, 3, 0, 3, 1],
                    ],
                    [6, 10, 16, 8, 13],
                ),
                (
                    1,
                    [2, 2],
                    [
                        [3, 3, 1, 0, 2],
                        [2, 3, 1, 5, 2],
                        [3, 2, 1, 3, 2],
                        [5, 3, 0, 3, 2],
                    ],
                    [6, 8, 16, 8, 11],
                ),
                (
                    2,
                    [1, 2],
                    [
                        [3, 3, 1, 1, 2],
                        [2, 3, 1, 5, 2],
                        [3, 2, 2, 3, 2],
                        [5, 3, 0, 3, 2],
                    ],
                    [6, 8, 15, 7, 11],
                ),
                (
                    3,
                    [3, 2],
                    [
                        [3, 3, 2, 1, 2],
                        [2, 3, 1, 5, 2],
                        [3, 2, 2, 4, 2],
                        [5, 3, 0, 4, 2],
                    ],
                    [6, 8, 14, 5, 11],
                ),
                (
                    0,
                    [2, 6],
                    [
                        [3, 4, 1, 0, 1],
                        [2, 3, 1, 6, 2],
                        [3, 2, 2, 4, 2],
                        [5, 4, 0, 4, 2],
                    ],
                    [6, 6, 15, 5, 12],
                ),
                (
                    1,
                    [2, 4],
                    [
                        [3, 4, 1, 0, 1],
                        [2, 3, 1, 6, 3],
                        [4, 2, 2, 4, 2],
                        [5, 4, 0, 4, 2],
                    ],
                    [5, 6, 15, 5, 11],
                ),
                (
                    2,
                    [2, 1],
                    [
                        [3, 4, 1, 1, 1],
                        [2, 3, 1, 6, 3],
                        [4, 2, 3, 4, 2],
                        [5, 4, 0, 4, 2],
                    ],
                    [5, 6, 14, 4, 11],
                ),
                (
                    3,
                    [5, 1],
                    [
                        [3, 4, 1, 1, 1],
                        [2, 3, 1, 6, 4],
                        [5, 2, 3, 4, 2],
                        [5, 4, 0, 4, 2],
                    ],
                    [4, 6, 14, 4, 10],
                ),
                (
                    0,
                    [5, 1],
                    [
                        [3, 4, 0, 0, 0],
                        [2, 3, 1, 6, 5],
                        [6, 2, 3, 4, 2],
                        [5, 4, 0, 4, 2],
                    ],
                    [3, 6, 15, 5, 10],
                ),
                (
                    1,
                    [3, 6],
                    [
                        [4, 4, 0, 0, 0],
                        [3, 3, 1, 6, 5],
                        [6, 2, 3, 4, 3],
                        [6, 4, 0, 4, 2],
                    ],
                    [0, 6, 15, 5, 9],
                ),
                (
                    2,
                    [2, 3],
                    [
                        [4, 4, 1, 0, 0],
                        [3, 3, 1, 6, 5],
                        [6, 2, 3, 5, 3],
                        [6, 4, 0, 5, 2],
                    ],
                    [0, 6, 14, 3, 9],
                ),
                (
                    3,
                    [6, 5],
                    [
                        [4, 5, 1, 0, 0],
                        [3, 3, 2, 6, 5],
                        [6, 3, 3, 5, 3],
                        [6, 4, 0, 5, 2],
                    ],
                    [0, 4, 13, 3, 9],
                ),
            ],
            buys: &[(4, 2), (16, 10), (20, 8)],
            plays: &[(8, 2, 7), (20, 10, 0), (24, 8, 7)],
            next: [3, 5],
        },
    ]
}

fn play(e: &mut Engine, c: &Case, turn: u64, id: u8, destination: u8) {
    let before = e.view(Viewer::Seat(seat(0))).unwrap();
    let held = before.own_player.unwrap().development_cards;
    assert!(
        held.iter()
            .any(|own| own.id == card(id) && own.acquired_turn < turn)
    );
    for actor in 0..c.seats {
        assert_eq!(e.legal_actions(seat(actor)).unwrap().choices.iter().any(|choice|matches!(choice,LegalAction::PlayKnight{cards} if cards.contains(&card(id)))),actor==0);
    }
    let receipt = act(e, 0, Command::PlayKnight { card: card(id) });
    let count = c.plays.iter().filter(|&&(when, _, _)| when <= turn).count() as u8;
    let mut expected = vec![Event::KnightPlayed {
        seat: seat(0),
        card: card(id),
    }];
    if count == 3 {
        expected.push(Event::LargestArmyChanged {
            previous: None,
            holder: Some(seat(0)),
        });
    }
    expected.push(Event::PhaseChanged {
        phase: Phase::RobberMove {
            origin: RobberOrigin::Knight,
            continuation: ResumePhase::PreRoll,
        },
    });
    assert_eq!(receipt.events, expected);
    let moved = act(
        e,
        0,
        Command::MoveRobber {
            hex: hex(destination),
        },
    );
    assert_eq!(
        moved.events,
        vec![
            Event::RobberMoved {
                seat: seat(0),
                hex: hex(destination)
            },
            Event::PhaseChanged {
                phase: Phase::PreRoll
            }
        ]
    );
    for viewer in
        std::iter::once(Viewer::Spectator).chain((0..c.seats).map(|id| Viewer::Seat(seat(id))))
    {
        let view = e.view(viewer).unwrap();
        assert_eq!(view.phase, Phase::PreRoll);
        assert_eq!(view.players[0].played_knights, count);
        assert!(view.players[0].spent_progress.is_empty());
        assert_eq!(
            view.largest_army_holder,
            if count == 3 { Some(seat(0)) } else { None }
        );
        assert_eq!(view.players[0].public_score, if count == 3 { 4 } else { 2 });
        assert_eq!(
            e.project_receipt(&receipt, viewer).unwrap().events[0],
            ProjectedEvent::KnightPlayed {
                seat: seat(0),
                card: card(id)
            }
        );
        assert_eq!(
            view.own_player.as_ref().map(|own| own.seat),
            match viewer {
                Viewer::Seat(s) => Some(s),
                Viewer::Spectator => None,
            }
        );
        if let Some(own) = view.own_player {
            assert!(!own.development_cards.iter().any(|own| own.id == card(id)));
        }
    }
    let revision = e.view(Viewer::Spectator).unwrap().revision;
    assert_eq!(
        e.execute(CommandEnvelope {
            api_version: 1,
            actor: seat(0),
            expected_revision: revision,
            command: Command::PlayKnight { card: card(id) }
        })
        .unwrap_err()
        .code,
        RuleError::CardNotPlayable
    );
    assert_eq!(e.view(Viewer::Spectator).unwrap().revision, revision);
}
fn run(c: Case, army: bool) {
    let mut seed = [0; 32];
    seed[..4].copy_from_slice(&c.seed.to_le_bytes());
    let seats = (0..c.seats).collect::<Vec<_>>();
    let (mut e, _) = Engine::create(GameConfig::fixed(&seats).unwrap(), seed).unwrap();
    assert_eq!(
        e.view(Viewer::Spectator).unwrap().starting_player,
        if c.seed == 3190 { seat(1) } else { seat(0) }
    );
    let mut receipts = vec![];
    for (index, &(actor, dice, hands, bank)) in c.turns.iter().enumerate() {
        let turn = index as u64 + 1;
        for &(when, id, destination) in c.plays {
            if when == turn {
                play(&mut e, &c, turn, id, destination);
            }
        }
        let rolled = act(&mut e, actor, Command::RollDice);
        assert_eq!(
            rolled.events[0],
            Event::DiceRolled {
                seat: seat(actor),
                dice
            }
        );
        for &(when, id) in c.buys {
            if when == turn {
                let receipt = act(&mut e, actor, Command::BuyDevelopmentCard);
                assert!(
                    matches!(receipt.events[1],Event::DevelopmentPurchased{card:got,..} if got==card(id))
                );
                receipts.push(receipt);
            }
        }
        let public = e.view(Viewer::Spectator).unwrap();
        assert_eq!(public.bank.counts(), &bank);
        assert_eq!(public.phase, Phase::Action);
        for player in 0..c.seats {
            assert_eq!(
                e.view(Viewer::Seat(seat(player)))
                    .unwrap()
                    .own_player
                    .unwrap()
                    .resources
                    .counts(),
                &hands[player as usize]
            );
        }
        act(&mut e, actor, Command::EndTurn);
    }
    let turn = c.turns.len() as u64 + 1;
    let &(_, id, destination) = c.plays.last().unwrap();
    play(&mut e, &c, turn, id, destination);
    let rolled = act(&mut e, 0, Command::RollDice);
    assert_eq!(
        rolled.events[0],
        Event::DiceRolled {
            seat: seat(0),
            dice: c.next
        }
    );
    if !army {
        assert_eq!(
            e.view(Viewer::Spectator).unwrap().phase,
            Phase::RobberMove {
                origin: RobberOrigin::Seven,
                continuation: ResumePhase::Action
            }
        );
        act(&mut e, 0, Command::MoveRobber { hex: hex(0) });
        assert_eq!(e.view(Viewer::Spectator).unwrap().phase, Phase::Action);
        assert!(
            e.legal_actions(seat(0))
                .unwrap()
                .choices
                .iter()
                .all(|choice| !matches!(choice, LegalAction::PlayKnight { .. }))
        );
    }
    for receipt in receipts {
        for viewer in
            std::iter::once(Viewer::Spectator).chain((0..c.seats).map(|id| Viewer::Seat(seat(id))))
        {
            assert!(
                matches!(e.project_receipt(&receipt,viewer).unwrap().events[1],ProjectedEvent::DevelopmentPurchased{seat:buyer,card:identity,..} if identity.is_some()==(viewer==Viewer::Seat(buyer)))
            );
        }
    }
}
#[test]
fn knight_pre_roll_then_seven_public_histories() {
    for c in cases().into_iter().filter(|c| [511, 136].contains(&c.seed)) {
        run(c, false);
    }
}
#[test]
fn knight_first_army_public_histories() {
    for c in cases()
        .into_iter()
        .filter(|c| [785, 3190].contains(&c.seed))
    {
        run(c, true);
    }
}
