//! Atomic relocation, explicit victim choice and pinned weighted theft.
use crate::{
    base_rules::{card_at_index, robber_victims},
    game_model::{
        Board, Command, Event, GameState, Phase, PlayerId, Position, Resources, ResumePhase,
        RuleError, Transition,
    },
};
use rand::distr::{Distribution, Uniform};

pub(super) fn validate(state: &GameState, command: &Command) -> Result<(), RuleError> {
    match (state.position.phase, command) {
        (Phase::RobberMove { .. }, Command::MoveRobber { hex }) => {
            if *hex == state.position.robber {
                return Err(RuleError::InvalidInput);
            }
        }
        (Phase::RobberVictim { victims, .. }, Command::ChooseRobberVictim { victim }) => {
            if victims[victim.index()] != Some(*victim) {
                return Err(RuleError::InvalidVictim);
            }
        }
        _ => return Err(RuleError::WrongPhase),
    }
    Ok(())
}

pub(super) fn apply_validated(
    board: &Board,
    state: &GameState,
    actor: PlayerId,
    command: &Command,
) -> Result<Transition, RuleError> {
    let mut position = state.position.clone();
    let mut events = Vec::new();
    let mut random = None;
    let (victim, continuation) = match (position.phase, command) {
        (
            Phase::RobberMove {
                origin,
                continuation,
            },
            Command::MoveRobber { hex },
        ) => {
            position.robber = *hex;
            events.push(Event::RobberMoved {
                seat: actor,
                hex: *hex,
            });
            let victims = robber_victims(board.graph, &position.buildings, actor, *hex);
            if victims.iter().flatten().count() > 1 {
                position.phase = Phase::RobberVictim {
                    origin,
                    continuation,
                    victims,
                };
                (None, None)
            } else {
                (victims.into_iter().flatten().next(), Some(continuation))
            }
        }
        (Phase::RobberVictim { continuation, .. }, Command::ChooseRobberVictim { victim }) => {
            (Some(*victim), Some(continuation))
        }
        _ => unreachable!("validated robber command and phase"),
    };
    if let Some(victim) = victim {
        random = steal(state, &mut position, actor, victim, &mut events);
    }
    if let Some(continuation) = continuation {
        position.phase = match continuation {
            ResumePhase::PreRoll => Phase::PreRoll,
            ResumePhase::Action => Phase::Action,
        };
    }
    events.push(Event::PhaseChanged {
        phase: position.phase,
    });
    Ok(Transition {
        position,
        revision: state.revision + 1,
        random,
        events,
    })
}

fn steal(
    state: &GameState,
    position: &mut Position,
    thief: PlayerId,
    victim: PlayerId,
    events: &mut Vec<Event>,
) -> Option<rand_chacha::ChaCha12Rng> {
    let total = position.hands[victim.index()].total();
    let mut random = None;
    let resource = if total == 0 {
        None
    } else {
        let mut next = state.random.clone();
        // Constructed u8 Uniform is part of chacha12-rand010-v1's pinned word stream.
        let cards = Uniform::new(0u8, u8::try_from(total).expect("at most ninety-five cards"))
            .expect("nonempty hand");
        let resource = card_at_index(&position.hands[victim.index()], cards.sample(&mut next))
            .expect("sampled actual card");
        let mut counts = [0; 5];
        counts[resource.index()] = 1;
        let card = Resources::try_from(counts).expect("one resource card");
        position.hands[victim.index()] = position.hands[victim.index()]
            .checked_sub(&card)
            .expect("sampled card belongs to victim");
        position.hands[thief.index()] = position.hands[thief.index()]
            .checked_add(&card)
            .expect("conserved card bounds");
        random = Some(next);
        Some(resource)
    };
    events.push(Event::RobberTheft {
        thief,
        victim,
        resource,
    });
    random
}
