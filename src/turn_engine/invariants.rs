//! Pure complete current-state validation, shared by creation and snapshot import.
use crate::{
    base_rules::{
        CANONICAL_FRAMES, current_score, fixed_setup, frame_ports, longest_route_length,
        reconcile_longest_route, robber_victims, validate_offer_terms, validate_road_building,
        validate_setup_road, validate_setup_settlement, variable_numbers,
    },
    game_model::{
        Board, BoardGraph, BuildingKind, CardId, CardLocation, DevelopmentKind, EdgeId, GameState,
        Phase, PlayerId, Position, ResumePhase, RobberOrigin, SetupMode, Terrain, VertexId,
    },
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct InvalidState;
type Verdict = Result<(), InvalidState>;
type PieceCounts = ([u8; 4], [u8; 4], [u8; 4]);

fn require(condition: bool) -> Verdict {
    if condition { Ok(()) } else { Err(InvalidState) }
}
fn seat(id: usize) -> PlayerId {
    PlayerId::try_from(id as u8).expect("bounded seat")
}

pub(crate) fn validate_game(board: &Board, state: &GameState) -> Verdict {
    let p = &state.position;
    let seats = p.config.seat_order();
    require(
        seats.contains(&p.starting_player)
            && seats.contains(&p.active_player)
            && state.random.get_stream() == 0,
    )?;
    require(std::ptr::eq(board.graph, BoardGraph::base_game()))?;
    layout(board, p)?;
    for r in 0..5 {
        require(
            u16::from(p.bank.counts()[r])
                + p.hands
                    .iter()
                    .map(|h| u16::from(h.counts()[r]))
                    .sum::<u16>()
                == 19,
        )?;
    }
    let (roads, settlements, cities) = pieces(board, p)?;
    for id in seats.len()..4 {
        require(
            p.hands[id].total() == 0
                && p.road_lengths[id] == 0
                && p.second_settlements[id].is_none()
                && p.final_scores.is_none_or(|scores| scores[id] == 0),
        )?;
    }
    let start = seats
        .iter()
        .position(|&s| s == p.starting_player)
        .ok_or(InvalidState)?;
    let order: Vec<_> = (0..seats.len())
        .map(|i| seats[(start + i) % seats.len()])
        .collect();
    cards(p, &order)?;
    let lengths = std::array::from_fn(|id| {
        longest_route_length(board.graph, &p.roads, &p.buildings, seat(id))
    });
    require(
        p.road_lengths == lengths
            && p.longest_route_holder.is_none_or(|s| seats.contains(&s))
            && reconcile_longest_route(&lengths, p.longest_route_holder) == p.longest_route_holder,
    )?;
    let maximum = seats
        .iter()
        .map(|&s| p.played_knights(s))
        .max()
        .ok_or(InvalidState)?;
    match p.largest_army_holder {
        None => require(maximum < 3)?,
        Some(s) => require(
            seats.contains(&s) && p.played_knights(s) >= 3 && p.played_knights(s) == maximum,
        )?,
    }
    require(
        p.last_dice
            .is_none_or(|dice| dice.iter().all(|v| (1..=6).contains(v))),
    )?;
    if p.played_non_vp {
        require(p.card_locations.iter().any(|&v|matches!(v,CardLocation::Spent{owner}|CardLocation::PlayedKnight{owner} if owner==p.active_player)))?;
    }
    if matches!(
        p.phase,
        Phase::SetupSettlement { .. } | Phase::SetupRoad { .. }
    ) {
        setup(board, state, &order, &roads, &settlements, &cities)?;
    } else {
        require(p.turn >= 1)?;
        require(p.active_player == order[((p.turn - 1) % order.len() as u64) as usize])?;
        let setup_count = if p.config.setup() == SetupMode::Variable {
            4 * order.len() as u64
        } else {
            0
        };
        let minimum = (p.turn - 1)
            .checked_mul(2)
            .and_then(|v| v.checked_add(setup_count))
            .and_then(|v| v.checked_add(u64::from(p.last_dice.is_some())))
            .ok_or(InvalidState)?;
        require(state.revision >= minimum)?;
        for &s in seats {
            let id = s.index();
            require(roads[id] >= 2 && settlements[id] + cities[id] >= 2)?;
            let v = p.second_settlements[id].ok_or(InvalidState)?;
            require(p.buildings[v.index()].is_some_and(|b| b.owner == s))?;
        }
        normal_phase(board, p)?;
    }
    if let Some(offer) = p.offer {
        require(
            p.phase == Phase::Action
                && offer.turn == p.turn
                && offer.id > 0
                && offer.id <= state.revision
                && seats.contains(&offer.proposer)
                && seats.contains(&offer.recipient)
                && offer.proposer != offer.recipient
                && (p.active_player == offer.proposer || p.active_player == offer.recipient),
        )?;
        validate_offer_terms(
            &p.hands[offer.proposer.index()],
            &offer.give,
            &offer.receive,
        )
        .map_err(|_| InvalidState)?;
    }
    match p.phase {
        Phase::Finished { winner, score } => {
            require(
                winner == p.active_player
                    && score >= 10
                    && score == current_score(p, winner)
                    && p.offer.is_none(),
            )?;
            let scores = std::array::from_fn(|id| {
                if id < seats.len() {
                    current_score(p, seat(id))
                } else {
                    0
                }
            });
            require(p.final_scores == Some(scores))?;
            for (id, &location) in p.card_locations.iter().enumerate() {
                require(!matches!(location,CardLocation::RevealedVictory{owner} if owner!=winner))?;
                require(
                    !(CardId::try_from(id as u8).expect("bounded card").kind()
                        == DevelopmentKind::VictoryPoint
                        && matches!(location,CardLocation::Held{owner,..} if owner==winner)),
                )?;
            }
        }
        _ => require(
            p.final_scores.is_none()
                && !p
                    .card_locations
                    .iter()
                    .any(|v| matches!(v, CardLocation::RevealedVictory { .. }))
                && current_score(p, p.active_player) < 10,
        )?,
    }
    Ok(())
}

fn layout(board: &Board, p: &Position) -> Verdict {
    let mut terrains = [0; 6];
    let mut numbers = [0; 13];
    for (&t, &n) in board.terrain.iter().zip(&board.numbers) {
        terrains[t as usize] += 1;
        match n {
            None if t == Terrain::Desert => {}
            Some(v) if t != Terrain::Desert && (2..=12).contains(&v) && v != 7 => {
                numbers[usize::from(v)] += 1
            }
            _ => return Err(InvalidState),
        }
    }
    require(terrains == [3, 4, 4, 4, 3, 1] && numbers == [0, 0, 1, 2, 2, 2, 2, 0, 2, 2, 2, 2, 1])?;
    let mut frames = board.frame_permutation;
    frames.sort_unstable();
    // frame_ports indexes frame-1, so reject malformed IDs before calculating.
    require(frames == CANONICAL_FRAMES)?;
    require(board.ports == frame_ports(board.frame_permutation))?;
    match p.config.setup() {
        SetupMode::Fixed => {
            let fixed = fixed_setup(&p.config);
            require(
                board.terrain == fixed.board.terrain
                    && board.numbers == fixed.board.numbers
                    && board.frame_permutation == CANONICAL_FRAMES,
            )?;
            for (id, &owner) in fixed.roads.iter().enumerate() {
                if owner.is_some() {
                    require(p.roads[id] == owner)?;
                }
            }
            for (id, building) in fixed.buildings.iter().enumerate() {
                if let Some(b) = building {
                    require(p.buildings[id].is_some_and(|v| v.owner == b.owner))?;
                }
            }
            require(p.second_settlements == fixed.second_settlements)?;
        }
        SetupMode::Variable => require(board.numbers == variable_numbers(&board.terrain))?,
    }
    Ok(())
}
fn pieces(board: &Board, p: &Position) -> Result<PieceCounts, InvalidState> {
    let mut roads = [0; 4];
    let mut settlements = [0; 4];
    let mut cities = [0; 4];
    for (id, b) in p.buildings.iter().enumerate() {
        if let Some(b) = b {
            require(p.config.seat_order().contains(&b.owner))?;
            let v = VertexId::try_from(id as u8).expect("bounded vertex");
            require(
                !board
                    .graph
                    .vertex_neighbors(v)
                    .iter()
                    .any(|v| p.buildings[v.index()].is_some()),
            )?;
            match b.kind {
                BuildingKind::Settlement => settlements[b.owner.index()] += 1,
                BuildingKind::City => cities[b.owner.index()] += 1,
            }
        }
    }
    for &owner in p.roads.iter().flatten() {
        require(p.config.seat_order().contains(&owner))?;
        roads[owner.index()] += 1;
    }
    for id in 0..4 {
        require(roads[id] <= 15 && settlements[id] <= 5 && cities[id] <= 4)?;
    }
    Ok((roads, settlements, cities))
}
fn cards(p: &Position, order: &[PlayerId]) -> Verdict {
    require(p.draw_cursor <= 25)?;
    let mut seen = [false; 25];
    for (offset, card) in p.deck.iter().enumerate() {
        require(!seen[card.index()])?;
        seen[card.index()] = true;
        require(
            (p.card_locations[card.index()] != CardLocation::Undrawn)
                == (offset < usize::from(p.draw_cursor)),
        )?;
    }
    for (id, &location) in p.card_locations.iter().enumerate() {
        let kind = CardId::try_from(id as u8).expect("bounded card").kind();
        let owner = match location {
            CardLocation::Undrawn => continue,
            CardLocation::Held {
                owner,
                acquired_turn,
            } => {
                require(acquired_turn >= 1 && acquired_turn <= p.turn)?;
                require(order[((acquired_turn - 1) % order.len() as u64) as usize] == owner)?;
                require(
                    acquired_turn != p.turn || (owner == p.active_player && p.last_dice.is_some()),
                )?;
                owner
            }
            CardLocation::Spent { owner } => {
                require(matches!(
                    kind,
                    DevelopmentKind::RoadBuilding
                        | DevelopmentKind::Monopoly
                        | DevelopmentKind::Invention
                ))?;
                owner
            }
            CardLocation::PlayedKnight { owner } => {
                require(kind == DevelopmentKind::Knight)?;
                owner
            }
            CardLocation::RevealedVictory { owner } => {
                require(kind == DevelopmentKind::VictoryPoint)?;
                owner
            }
        };
        require(order.contains(&owner))?;
    }
    Ok(())
}
fn setup(
    board: &Board,
    state: &GameState,
    order: &[PlayerId],
    roads: &[u8; 4],
    settlements: &[u8; 4],
    cities: &[u8; 4],
) -> Verdict {
    let p = &state.position;
    let (actor, round, pending) = match p.phase {
        Phase::SetupSettlement { actor, round } => (actor, round, None),
        Phase::SetupRoad {
            actor,
            round,
            settlement,
        } => (actor, round, Some(settlement)),
        _ => return Err(InvalidState),
    };
    require(
        p.config.setup() == SetupMode::Variable
            && p.turn == 0
            && actor == p.active_player
            && (1..=2).contains(&round)
            && p.last_dice.is_none()
            && board.terrain[p.robber.index()] == Terrain::Desert
            && p.bank.counts() == &[19; 5]
            && p.hands.iter().all(|h| h.total() == 0)
            && p.draw_cursor == 0
            && !p.played_non_vp
            && p.longest_route_holder.is_none()
            && p.largest_army_holder.is_none()
            && p.offer.is_none()
            && p.final_scores.is_none(),
    )?;
    let k = order.iter().position(|&s| s == actor).ok_or(InvalidState)?;
    for (j, &s) in order.iter().enumerate() {
        let completed = if round == 1 {
            u8::from(j < k)
        } else {
            1 + u8::from(j > k)
        };
        let count = completed + u8::from(pending.is_some() && s == actor);
        require(
            roads[s.index()] == completed
                && settlements[s.index()] == count
                && cities[s.index()] == 0,
        )?;
        let marker = p.second_settlements[s.index()];
        let second = round == 2 && (j > k || (j == k && pending.is_some()));
        require(marker.is_some() == second)?;
        if let Some(v) = marker {
            require(
                p.buildings[v.index()]
                    .is_some_and(|b| b.owner == s && b.kind == BuildingKind::Settlement),
            )?;
            if s == actor {
                require(Some(v) == pending)?;
            }
        }
    }
    require(
        state.revision
            == u64::from(roads.iter().sum::<u8>()) + u64::from(settlements.iter().sum::<u8>()),
    )?;
    for (id, &owner) in p.roads.iter().enumerate() {
        if let Some(owner) = owner {
            let edge = EdgeId::try_from(id as u8).expect("bounded edge");
            require(
                board
                    .graph
                    .edge_vertices(edge)
                    .iter()
                    .any(|v| p.buildings[v.index()].is_some_and(|b| b.owner == owner)),
            )?;
        }
    }
    // Each completed setup pair has its own incident road. Counts alone could
    // otherwise put both roads at the first settlement and strand the second.
    for (id, building) in p.buildings.iter().enumerate() {
        if let Some(building) = building {
            let vertex = VertexId::try_from(id as u8).expect("bounded vertex");
            if Some(vertex) != pending {
                require(
                    board
                        .graph
                        .vertex_edges(vertex)
                        .iter()
                        .any(|edge| p.roads[edge.index()] == Some(building.owner)),
                )?;
            }
        }
    }
    if let Some(v) = pending {
        require(
            p.buildings[v.index()]
                .is_some_and(|b| b.owner == actor && b.kind == BuildingKind::Settlement),
        )?;
        require(
            board
                .graph
                .vertex_edges(v)
                .iter()
                .any(|&e| validate_setup_road(board, &p.roads, v, e).is_ok()),
        )?;
    } else {
        require((0..VertexId::COUNT).any(|id| {
            validate_setup_settlement(
                board,
                &p.buildings,
                VertexId::try_from(id as u8).expect("bounded vertex"),
            )
            .is_ok()
        }))?;
    }
    Ok(())
}
fn continuation(p: &Position, resume: ResumePhase) -> Verdict {
    require(match resume {
        ResumePhase::PreRoll => p.last_dice.is_none(),
        ResumePhase::Action => p.last_dice.is_some(),
    })
}
fn robber(p: &Position, origin: RobberOrigin, resume: ResumePhase) -> Verdict {
    continuation(p, resume)?;
    match origin {
        RobberOrigin::Seven => {
            require(resume == ResumePhase::Action && p.last_dice.is_some_and(|d| d[0] + d[1] == 7))
        }
        RobberOrigin::Knight => require(p.played_non_vp && p.played_knights(p.active_player) > 0),
    }
}
fn normal_phase(board: &Board, p: &Position) -> Verdict {
    match p.phase {
        Phase::PreRoll => require(p.last_dice.is_none()),
        Phase::Action => require(p.last_dice.is_some()),
        Phase::Discard { obligations } => {
            require(
                p.last_dice.is_some_and(|d| d[0] + d[1] == 7) && obligations.iter().any(|&n| n > 0),
            )?;
            for (id, &n) in obligations.iter().enumerate() {
                if n > 0 {
                    require(
                        id < p.config.seat_order().len()
                            && p.hands[id].total() > 7
                            && u16::from(n) == p.hands[id].total() / 2,
                    )?;
                }
            }
            Ok(())
        }
        Phase::RobberMove {
            origin,
            continuation,
        } => robber(p, origin, continuation),
        Phase::RobberVictim {
            origin,
            continuation,
            victims,
        } => {
            robber(p, origin, continuation)?;
            require(
                victims == robber_victims(board.graph, &p.buildings, p.active_player, p.robber)
                    && victims.iter().flatten().count() > 1,
            )
        }
        Phase::FreeRoads {
            remaining,
            continuation: resume,
        } => {
            continuation(p, resume)?;
            require(
                (1..=2).contains(&remaining)
                    && p.played_non_vp
                    && p.card_locations.iter().enumerate().any(|(id, &v)| {
                        matches!(v,CardLocation::Spent{owner} if owner==p.active_player)
                            && CardId::try_from(id as u8).expect("bounded card").kind()
                                == DevelopmentKind::RoadBuilding
                    }),
            )?;
            validate_road_building(board.graph, p, p.active_player).map_err(|_| InvalidState)
        }
        Phase::Finished { .. } => Ok(()),
        Phase::SetupSettlement { .. } | Phase::SetupRoad { .. } => Err(InvalidState),
    }
}

#[cfg(test)]
#[path = "invariants_tests.rs"]
mod tests;
