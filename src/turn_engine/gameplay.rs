//! Atomic dice production and seven decisions.
use crate::{
    base_rules::{discard_obligations, production},
    game_model::{
        Board, Event, GameState, Phase, PlayerId, Resources, ResumePhase, RobberOrigin, RuleError,
        Transition,
    },
};
pub(super) fn apply_validated(
    board: &Board,
    state: &GameState,
    actor: PlayerId,
) -> Result<Transition, RuleError> {
    let mut position = state.position.clone();
    let mut random = state.random.clone();
    let dice = super::creation::roll_dice(&mut random);
    let mut events = vec![Event::DiceRolled { seat: actor, dice }];
    position.last_dice = Some(dice);
    if dice[0] + dice[1] == 7 {
        let obligations = discard_obligations(&position.hands);
        position.phase = if obligations.iter().all(|&quantity| quantity == 0) {
            Phase::RobberMove {
                origin: RobberOrigin::Seven,
                continuation: ResumePhase::Action,
            }
        } else {
            Phase::Discard { obligations }
        };
    } else {
        let grants = production(board, &position, dice[0] + dice[1]);
        for &seat in position.config.seat_order() {
            let resources = grants[seat.index()];
            if resources == Resources::default() {
                continue;
            }
            position.hands[seat.index()] = position.hands[seat.index()]
                .checked_add(&resources)
                .expect("conserved hand bounds");
            position.bank = position
                .bank
                .checked_sub(&resources)
                .expect("production bounded by bank");
            events.push(Event::Production { seat, resources });
        }
        position.phase = Phase::Action;
    }
    events.push(Event::PhaseChanged {
        phase: position.phase,
    });
    Ok(Transition {
        position,
        revision: state.revision + 1,
        random: Some(random),
        events,
    })
}

pub(super) fn end_turn(state: &GameState) -> Result<Transition, RuleError> {
    let mut position = state.position.clone();
    let order = position.config.seat_order();
    let current = order
        .iter()
        .position(|&seat| seat == position.active_player)
        .expect("validated active seat");
    position.active_player = order[(current + 1) % order.len()];
    position.turn += 1;
    position.last_dice = None;
    position.played_non_vp = false;
    position.phase = Phase::PreRoll;
    let mut events = vec![Event::TurnStarted {
        seat: position.active_player,
        turn: position.turn,
    }];
    if !check_victory(&mut position, &mut events) {
        events.push(Event::PhaseChanged {
            phase: position.phase,
        });
    }
    Ok(Transition {
        position,
        revision: state.revision + 1,
        random: None,
        events,
    })
}

pub(super) fn check_victory(
    position: &mut crate::game_model::Position,
    events: &mut Vec<Event>,
) -> bool {
    let score = crate::base_rules::current_score(position, position.active_player);
    if score < 10 {
        return false;
    }
    position.final_scores = Some(std::array::from_fn(|id| {
        crate::base_rules::current_score(
            position,
            PlayerId::try_from(id as u8).expect("bounded seat"),
        )
    }));
    let winner = position.active_player;
    for (card, _) in position.held_cards(winner).collect::<Vec<_>>() {
        if card.kind() == crate::game_model::DevelopmentKind::VictoryPoint {
            position.card_locations[card.index()] =
                crate::game_model::CardLocation::RevealedVictory { owner: winner };
        }
    }
    let cards = position.revealed_victory_cards();
    if cards.iter().any(Option::is_some) {
        events.push(Event::VictoryPointsRevealed { winner, cards });
    }
    position.phase = Phase::Finished {
        winner: position.active_player,
        score,
    };
    events.push(Event::GameFinished {
        winner: position.active_player,
        score,
    });
    true
}

pub(super) fn discard(
    state: &GameState,
    actor: PlayerId,
    resources: Resources,
) -> Result<Transition, RuleError> {
    let mut position = state.position.clone();
    let Phase::Discard { mut obligations } = position.phase else {
        unreachable!("validated discard phase");
    };
    position.hands[actor.index()] = position.hands[actor.index()]
        .checked_sub(&resources)
        .expect("validated discard stock");
    position.bank = position
        .bank
        .checked_add(&resources)
        .expect("conserved bank bounds");
    obligations[actor.index()] = 0;
    position.phase = if obligations.iter().all(|&quantity| quantity == 0) {
        Phase::RobberMove {
            origin: RobberOrigin::Seven,
            continuation: ResumePhase::Action,
        }
    } else {
        Phase::Discard { obligations }
    };
    let events = vec![
        Event::Discarded {
            seat: actor,
            resources,
        },
        Event::PhaseChanged {
            phase: position.phase,
        },
    ];
    Ok(Transition {
        position,
        revision: state.revision + 1,
        random: None,
        events,
    })
}
