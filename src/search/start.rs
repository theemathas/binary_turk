use std::sync::mpsc::{SyncSender, Receiver, Sender, channel};
use std::sync::Arc;
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

    // This is a hack required because Send currently requires 'static
    // TODO remove the Arc when Send does not require 'static
    let search_pos_arc: Arc<Vec<Position>> = Arc::new({
        search_moves.iter().map(|x: &Move| {
            let mut new_pos = state.pos.clone();
            make_move(&mut new_pos, x);
            new_pos
        }).collect()
    });

    let mut best_score = None::<Score>;
    let mut best_move = search_moves[0].clone();
    let mut total_search_data = Data;

    let mut curr_plies = NumPlies(1);

    let (search_tx, mut search_rx) = channel::<(Score, Move, Data)>();
    let (mut kill_tx, kill_rx) = channel::<()>();

    let temp_search_pos_arc = search_pos_arc.clone();
    let mut search_guard = Thread::scoped(move ||
        depth_limited_search(temp_search_pos_arc, curr_plies, search_tx, kill_rx));

    loop {
        // This is a hack to get around a problem in select! {}
        // TODO remove this hack after the problem is solved
        let mut search_rx_opt: Option<Receiver<(Score, Move, Data)>> = None;
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
                        search_guard.join().ok().expect("depth_limited_search panicked");
                        if is_debug { debug!("stop search") }
                        // TODO send info again
                        tx.send(Response::BestMove(best_move, None))
                          .ok().expect("output channel closed");
                        return;
                    }
                }
            },
            search_res = search_rx.recv() => {
                search_guard.join().ok().expect("depth_limited_search panicked");

                let (temp_best_score, temp_best_move, curr_search_data) = search_res.ok()
                    .expect("depth_limited_search unexpectedly dropped Sender");

                best_score = Some(temp_best_score);
                best_move = temp_best_move;

                total_search_data = total_search_data.combine(curr_search_data);

                // TODO use search data
                // TODO send info

                curr_plies.0 += 1;

                let (new_search_tx, new_search_rx) = channel::<(Score, Move, Data)>();
                search_rx_opt = Some(new_search_rx);
                let (new_kill_tx, new_kill_rx) = channel::<()>();
                kill_tx = new_kill_tx;

                let temp_search_pos_arc = search_pos_arc.clone();
                search_guard = Thread::scoped(move ||
                    depth_limited_search(temp_search_pos_arc, curr_plies,
                                         new_search_tx, new_kill_rx));
            }
        }
        if let Some(val) = search_rx_opt { search_rx = val; }
    }
}

fn depth_limited_search(search_pos_arc: Arc<Vec<Position>>,
                        depth: NumPlies,
                        tx: Sender<(Score, Move, Data)>,
                        kill_rx: Receiver<()>) {
    debug_assert!(!search_pos_arc.is_empty());

    // TODO call negamax() for each Position
    unimplemented!()
}
