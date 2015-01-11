use std::sync::mpsc::{SyncSender, Receiver};

use uci::Response;
use game::{Move,receive_legal};

pub use self::types::{State, Param, Cmd};

mod types;

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
        Some(val) => legal_moves.filter(|x| val.contains(x)).collect(),
    };
    if search_moves.is_empty() {
        panic!("No legal moves searched in searched position");
    }
    // TODO Actually find the best move. (Currently any move.)
    let best_move = search_moves[0].clone();
    if is_debug {
        debug!("best move is {:?}", best_move);
        debug!("waiting for next command");
    }
    // TODO send info
    for cmd in rx.iter() {
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
                if is_debug { debug!("stop search") }
                // TODO send info again
                tx.send(Response::BestMove(best_move, None))
                  .ok().expect("output channel closed");
                return;
            }
        }
    }
    panic!("Sender hung up while calculating");
}
