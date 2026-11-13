//! Synthetic, source-traced native and wire measurements; no private engine access.
#![forbid(unsafe_code)]

use criterion::{BatchSize, Criterion};
use hex_traders_engine::{
    BoardGraph, Command, CommandEnvelope, CreationReceipt, EdgeId, Engine, Event, GameConfig,
    LegalAction, Phase, PlayerId, Receipt, Resources, RuleError, Viewer, decode_command_envelope,
    encode_command_envelope,
};
use serde_json::Value;
use std::{hint::black_box, time::Duration};

fn seat(id: u8) -> PlayerId {
    PlayerId::try_from(id).unwrap()
}
fn edge(id: u8) -> EdgeId {
    EdgeId::try_from(id).unwrap()
}
fn envelope(actor: u8, revision: u64, command: Command) -> CommandEnvelope {
    CommandEnvelope {
        api_version: 1,
        actor: seat(actor),
        expected_revision: revision,
        command,
    }
}
fn seed() -> [u8; 32] {
    let mut seed = [0; 32];
    seed[..4].copy_from_slice(&15508_u32.to_le_bytes());
    seed
}
fn create() -> (Engine, CreationReceipt) {
    Engine::create(GameConfig::fixed(&[0, 1, 2]).unwrap(), seed()).unwrap()
}
fn snapshots() -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let data: Value = serde_json::from_str(include_str!("fixtures.json")).unwrap();
    let read = |key| data[key].as_str().unwrap().as_bytes().to_vec();
    (read("hard_pre"), read("hard_post"), read("award_pre"))
}

fn verify(hard_pre: &[u8], hard_post: &[u8], award_pre: &[u8]) -> (Receipt, CommandEnvelope) {
    let mut hard = Engine::from_snapshot(hard_pre).unwrap();
    assert_eq!(hard.export_snapshot().unwrap(), hard_pre);
    let view = hard.view(Viewer::Seat(seat(0))).unwrap();
    assert_eq!(
        (view.revision, view.turn, view.phase),
        (191, 77, Phase::Action)
    );
    assert_eq!(view.players[0].roads_remaining, 1);
    assert_eq!(view.players[0].longest_route_length, 7);
    assert_eq!(
        view.own_player.unwrap().resources.counts(),
        &[1, 1, 3, 2, 4]
    );
    assert!(hard.legal_actions(seat(0)).unwrap().choices.iter().any(
        |choice| matches!(choice, LegalAction::BuildRoad { edges } if edges.contains(&edge(58)))
    ));
    let command = Command::BuildRoad { edge: edge(58) };
    assert!(hard.validate(seat(0), &command).is_ok());
    let intent = envelope(0, 191, command);
    let wire = encode_command_envelope(&intent).unwrap();
    assert_eq!(decode_command_envelope(&wire).unwrap(), intent);
    let receipt = hard.execute(intent).unwrap();
    assert_eq!((receipt.previous_revision, receipt.revision), (191, 192));
    assert_eq!(
        receipt.events,
        vec![
            Event::ConstructionPaid {
                seat: seat(0),
                resources: Resources::try_from([1, 1, 0, 0, 0]).unwrap()
            },
            Event::RoadPlaced {
                seat: seat(0),
                edge: edge(58)
            },
        ]
    );
    assert_eq!(hard.export_snapshot().unwrap(), hard_post);
    let before: Value = serde_json::from_slice(hard_pre).unwrap();
    let after: Value = serde_json::from_slice(hard_post).unwrap();
    assert_eq!(before["random"], after["random"]);
    assert_eq!(before["random"]["wordPosition"], "164");
    let mut wire_game = Engine::from_snapshot(hard_pre).unwrap();
    assert_eq!(wire_game.execute_json(&wire).unwrap(), receipt);
    assert_eq!(wire_game.export_snapshot().unwrap(), hard_post);

    let mut award = Engine::from_snapshot(award_pre).unwrap();
    let awarded = award
        .execute(envelope(0, 59, Command::BuildRoad { edge: edge(44) }))
        .unwrap();
    assert!(awarded.events.contains(&Event::LongestRouteChanged {
        previous: None,
        holder: Some(seat(0))
    }));
    assert_eq!(
        award.view(Viewer::Spectator).unwrap().players[0].longest_route_length,
        6
    );
    let mut reject = Engine::from_snapshot(hard_pre).unwrap();
    assert_eq!(
        reject.execute(envelope(0, 190, command)).unwrap_err().code,
        RuleError::StaleRevision
    );
    assert_eq!(
        reject
            .execute(envelope(0, 191, Command::BuildRoad { edge: edge(46) }))
            .unwrap_err()
            .code,
        RuleError::Occupied
    );
    assert_eq!(reject.export_snapshot().unwrap(), hard_pre);
    (receipt, intent)
}

fn benchmark(c: &mut Criterion) {
    let (hard_pre, hard_post, award_pre) = snapshots();
    let (receipt, intent) = verify(&hard_pre, &hard_post, &award_pre);
    let hard = Engine::from_snapshot(&hard_pre).unwrap();
    let projected = hard.project_receipt(&receipt, Viewer::Spectator).unwrap();
    let view = hard.view(Viewer::Seat(seat(0))).unwrap();
    let wire = encode_command_envelope(&intent).unwrap();
    let (initial, _) = create();
    let initial_snapshot = initial.export_snapshot().unwrap();
    assert_eq!(
        initial.view(Viewer::Spectator).unwrap().active_player,
        seat(2)
    );
    let roll = envelope(2, 0, Command::RollDice);
    let mut roll_check = Engine::from_snapshot(&initial_snapshot).unwrap();
    assert_eq!(
        roll_check.execute(roll).unwrap().events[0],
        Event::DiceRolled {
            seat: seat(2),
            dice: [6, 2]
        }
    );
    let (setup, _) = Engine::create(GameConfig::variable(&[0, 1, 2]).unwrap(), seed()).unwrap();
    let setup_actor = match setup.view(Viewer::Spectator).unwrap().phase {
        Phase::SetupSettlement { actor, .. } => actor,
        _ => panic!("variable setup must await a settlement"),
    };
    let batch = BatchSize::NumIterations(16);
    let mut group = c.benchmark_group("engine");
    group
        .sample_size(40)
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_secs(1));

    for (name, bytes, command) in [
        ("native/roll", &initial_snapshot, roll),
        (
            "native/end_turn",
            &hard_pre,
            envelope(0, 191, Command::EndTurn),
        ),
        (
            "native/road_award",
            &award_pre,
            envelope(0, 59, Command::BuildRoad { edge: edge(44) }),
        ),
        ("native/road_hard", &hard_pre, intent),
    ] {
        group.bench_function(name, |b| {
            b.iter_batched_ref(
                || Engine::from_snapshot(bytes).unwrap(),
                |game| black_box(game.execute(black_box(command)).unwrap()),
                batch,
            )
        });
    }
    for (name, command) in [
        ("reject/stale", envelope(0, 190, intent.command)),
        (
            "reject/occupied",
            envelope(0, 191, Command::BuildRoad { edge: edge(46) }),
        ),
    ] {
        let mut game = Engine::from_snapshot(&hard_pre).unwrap();
        group.bench_function(name, |b| {
            b.iter(|| black_box(game.execute(black_box(command)).unwrap_err()))
        });
        assert_eq!(game.export_snapshot().unwrap(), hard_pre);
    }
    group.bench_function("validate/legal", |b| {
        b.iter(|| black_box(hard.validate(seat(0), black_box(&intent.command))).unwrap())
    });
    group.bench_function("validate/occupied", |b| {
        b.iter(|| {
            black_box(
                hard.validate(seat(0), black_box(&Command::BuildRoad { edge: edge(46) }))
                    .unwrap_err(),
            )
        })
    });
    group.bench_function("legal/setup", |b| {
        b.iter(|| black_box(setup.legal_actions(black_box(setup_actor)).unwrap()))
    });
    group.bench_function("legal/late", |b| {
        b.iter(|| black_box(hard.legal_actions(black_box(seat(0))).unwrap()))
    });
    group.bench_function("view/seat", |b| {
        b.iter(|| black_box(hard.view(black_box(Viewer::Seat(seat(0)))).unwrap()))
    });
    group.bench_function("view/spectator", |b| {
        b.iter(|| black_box(hard.view(black_box(Viewer::Spectator)).unwrap()))
    });
    group.bench_function("receipt/project", |b| {
        b.iter(|| {
            black_box(
                hard.project_receipt(black_box(&receipt), Viewer::Spectator)
                    .unwrap(),
            )
        })
    });
    group.bench_function("wire/encode_command", |b| {
        b.iter(|| black_box(encode_command_envelope(black_box(&intent)).unwrap()))
    });
    group.bench_function("wire/decode_command", |b| {
        b.iter(|| black_box(decode_command_envelope(black_box(&wire)).unwrap()))
    });
    group.bench_function("wire/decode_execute_native_receipt", |b| {
        b.iter_batched_ref(
            || Engine::from_snapshot(&hard_pre).unwrap(),
            |game| black_box(game.execute_json(black_box(&wire)).unwrap()),
            batch,
        )
    });
    group.bench_function("wire/encode_view", |b| {
        b.iter(|| black_box(black_box(&view).to_json().unwrap()))
    });
    group.bench_function("wire/encode_projected_receipt", |b| {
        b.iter(|| black_box(black_box(&projected).to_json().unwrap()))
    });
    group.bench_function("snapshot/export", |b| {
        b.iter(|| black_box(hard.export_snapshot().unwrap()))
    });
    group.bench_function("snapshot/import", |b| {
        b.iter_batched_ref(
            || (),
            |_| black_box(Engine::from_snapshot(black_box(&hard_pre)).unwrap()),
            batch,
        )
    });
    group.finish();
}

// An opt-in external-profiler workload: phase checkpoints keep retained values alive.
fn memory() {
    use std::{
        io::{self, BufRead, Write},
        mem::size_of,
        sync::mpsc,
        thread,
    };
    let count: usize = std::env::var("HEX_TRADERS_MEMORY_COUNT")
        .unwrap_or_else(|_| "32".into())
        .parse()
        .unwrap();
    assert!(count >= 2);
    let (send, receive) = mpsc::channel();
    thread::spawn(move || {
        for line in io::stdin().lock().lines() {
            if line.is_err() || send.send(()).is_err() {
                break;
            }
        }
    });
    let phase = |name: &str| {
        println!("READY {name} pid={} count={count}", std::process::id());
        io::stdout().flush().unwrap();
        loop {
            match receive.try_recv() {
                Ok(()) => break,
                Err(mpsc::TryRecvError::Empty) => std::hint::spin_loop(),
                Err(mpsc::TryRecvError::Disconnected) => {
                    panic!("profiling controller disconnected")
                }
            }
        }
    };
    println!(
        "INLINE Engine={} BoardGraph={} Receipt={} CreationReceipt={} Event={}",
        size_of::<Engine>(),
        size_of::<BoardGraph>(),
        size_of::<Receipt>(),
        size_of::<CreationReceipt>(),
        size_of::<Event>()
    );
    let mut games: Vec<Engine> = Vec::with_capacity(count);
    let mut receipts: Vec<Receipt> = Vec::with_capacity(count);
    let mut creations: Vec<CreationReceipt> = Vec::with_capacity(count);
    phase("cold");
    black_box(BoardGraph::base_game());
    phase("graph");
    for index in 0..count {
        let (game, creation) = create();
        drop(creation);
        games.push(game);
        if index == 0 {
            phase("early_1");
        }
    }
    black_box(&games);
    phase("early_n");
    games.clear();
    phase("empty_after_early");
    let (hard_pre, _, _) = snapshots();
    // Snapshot bytes are owned by the workload and remain present in both late controls.
    phase("late_0");
    for index in 0..count {
        games.push(Engine::from_snapshot(&hard_pre).unwrap());
        if index == 0 {
            phase("late_1");
        }
    }
    black_box(&games);
    phase("late_n");
    games.clear();
    phase("receipt_0");
    for index in 0..count {
        let mut game = Engine::from_snapshot(&hard_pre).unwrap();
        let receipt = game
            .execute(envelope(0, 191, Command::BuildRoad { edge: edge(58) }))
            .unwrap();
        drop(game);
        receipts.push(receipt);
        if index == 0 {
            phase("receipt_1");
        }
    }
    black_box(&receipts);
    phase("receipt_n");
    receipts.clear();
    phase("creation_0");
    for index in 0..count {
        let (game, receipt) = create();
        drop(game);
        creations.push(receipt);
        if index == 0 {
            phase("creation_1");
        }
    }
    black_box(&creations);
    phase("creation_n");
}

fn main() {
    if std::env::var_os("HEX_TRADERS_MEMORY").is_some() {
        memory();
    } else {
        let mut criterion = Criterion::default().configure_from_args();
        benchmark(&mut criterion);
        criterion.final_summary();
    }
}
