use std::sync::mpsc::{SyncSender, Receiver, Sender, channel};
use std::thread::Thread;

use uci::Response;
use game::{Move, Position, receive_legal, make_move};
use types::NumPlies;
use eval::Score;

use super::types::{State, Cmd};
use super::negamax::{negamax, Data};

pub fn start(mut state: State, rx: Receiver<Cmd>, tx:SyncSender<Response>) {
    let mut is_debug = false;
    if state.param.ponder {
        if is_debug { debug!("pondering, waiting for next command") }
        // Actually should ponder, but now just waits for our move.
        for cmd in rx.iter() {
            match cmd {
                Cmd::SetDebug(val) => {
                    is_debug = val;
                    debug!("debug is now {:?}", val);
                },
                Cmd::PonderHit => {
                    if is_debug { debug!("ponder hit when pondering") }
                    state.param.ponder = false;
                    break;
                },
                Cmd::Stop => {
                    if is_debug { debug!("stop from pondering") }
                    // TODO report stuff.about pondering and terminate.
                    unimplemented!();
                },
            }
        }
        if state.param.ponder {
            panic!("Sender hung up while pondering");
        }
        if is_debug { debug!("pondering finished") }
    }

    let legal_moves_chan = receive_legal(state.pos.clone());
    let legal_moves = legal_moves_chan.iter();
    let search_moves: Vec<Move> = match state.param.search_moves {
        None => legal_moves.collect(),
        Some(ref val) => legal_moves.filter(|x| val.contains(x)).collect(),
    };
    if search_moves.is_empty() {
        panic!("No legal moves searched in searched position");
    }

    let search_pos: Vec<Position> = {
        search_moves.iter().map(|x: &Move| {
            let mut new_pos = state.pos.clone();
            make_move(&mut new_pos, x);
            new_pos
        }).collect()
    };

    let mut best_move = search_moves[0].clone();

    let (search_tx, mut search_rx) = channel::<(Score, Move, Data)>();
    let (mut kill_tx, kill_rx) = channel::<()>();
    Thread::spawn(move || depth_limited_search(&*search_pos, NumPlies(1), search_tx, kill_rx));

    loop {
        select! {
            val = rx.recv() => {
                let cmd = val.ok().expect("Sender hung up while calculating");
                match cmd {
                    Cmd::SetDebug(val) => {
                        is_debug = val;
                        debug!("debug is now {:?}", val);
                    },
                    Cmd::PonderHit => {
                        if is_debug { debug!("ponder hit when not pondering (ignored)") }
                        // Ignore this cmd
                    },
                    Cmd::Stop => {
                        let _ = kill_tx.send(());
                        if is_debug { debug!("stop search") }
                        // TODO send info again
                        tx.send(Response::BestMove(best_move, None))
                          .ok().expect("output channel closed");
                        return;
                    }
                }
            },
            search_res = search_rx.recv() => {
                // TODO use search_res
                // TODO send info
                // TODO do next iteration of depth limited search
                unimplemented!()
            }
        }
    }
}

fn depth_limited_search(search_pos: &[Position],
                        depth: NumPlies,
                        tx: Sender<(Score, Move, Data)>,
                        kill_rx: Receiver<()>) {
    // TODO call negamax() for each Position
    unimplemented!()
}
