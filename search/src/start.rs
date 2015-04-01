use std::sync::mpsc::{Sender, Receiver, channel};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::mem::size_of;

use game::{Move, Score, ScoreUnit, NumPlies};

use types::{State, Cmd, Data, Report, BestMove, NumNodes};
use iterated_deepening::iterated_deepening;
use transposition_table::{self, TranspositionTable};

pub fn start(mut state: State, rx: Receiver<Cmd>,
             tx: Sender<Report>) -> BestMove {
    if state.param.ponder {
        debug!("pondering, waiting for next command");
        // Actually should ponder, but now just waits for our move.
        for cmd in rx.iter() {
            match cmd {
                Cmd::SetDebug(val) => {
                    // TODO set debug
                    debug!("debug is now {:?}", val);
                },
                Cmd::PonderHit => {
                    debug!("ponder hit when pondering");
                    state.param.ponder = false;
                    break;
                },
                Cmd::Stop => {
                    debug!("stop from pondering");
                    // TODO report stuff.about pondering and terminate.
                    unimplemented!();
                },
            }
        }
        if state.param.ponder {
            panic!("Sender hung up while pondering");
        }
        debug!("pondering finished");
    }

    let search_moves: Vec<(Move)> = {
        //let legal_moves_chan = receive_legal(state.pos.clone());
        let legal_moves = state.pos.legal_iter();
        match state.param.search_moves {
            None => legal_moves.collect(),
            Some(ref val) => legal_moves.filter(|x| val.contains(x)).collect(),
        }
    };
    if search_moves.is_empty() {
        panic!("No legal moves searched in searched position");
    }

    // this is just a placeholder report
    let mut last_report = Report { data: Data { nodes: NumNodes(0), depth: NumPlies(0) },
                                   score: Score::Value(ScoreUnit(0)),
                                   pv: vec![search_moves[0].clone()] };
    let table_capacity = state.param.hash_size /
                         size_of::<Option<transposition_table::Data>>();
    let table = TranspositionTable::with_capacity(table_capacity);

    let (search_tx, search_rx) = channel::<Report>();
    let is_killed = Arc::new(AtomicBool::new(false));

    let temp_is_killed = is_killed.clone();

    debug!("Starting iterated deepening search");
    thread::spawn(move ||
        iterated_deepening(state.pos, &search_moves, table, search_tx, temp_is_killed));

    loop {
        select! {
            val = rx.recv() => {
                let cmd = val.ok().expect("Sender hung up while calculating");
                debug!("received command {:?}", cmd);
                match cmd {
                    Cmd::SetDebug(val) => {
                        // TODO set debug
                        debug!("debug is now {:?}", val);
                    },
                    Cmd::PonderHit => {
                        debug!("ponder hit when not pondering (ignored)");
                        // Ignore this cmd
                    },
                    Cmd::Stop => {

                        debug!("killing search");
                        is_killed.store(true, Ordering::SeqCst);

                        debug!("reporting result");
                        tx.send(last_report.clone()).unwrap();

                        debug!("search stopping");
                        return BestMove(last_report.pv[0].clone(), None);
                    }
                }
            },
            search_res = search_rx.recv() => {
                debug!("receiving result from iterated_deepening");

                last_report = search_res.ok()
                                        .expect("iterated_deepening unexpectedly dropped Sender");

                tx.send(last_report.clone()).unwrap();
            }
        }
    }
}

