use std::sync::mpsc::{Sender, Receiver};

use uci::types::Response;
use game::{Move,receive_legal};

pub use self::types::{State, Param, Cmd};

mod types;

pub fn start(mut state: State, rx: Receiver<Cmd>, tx:Sender<Response>) {
    if state.param.ponder.is_some() {
        // Actually should ponder, but now just waits for our move.
        for cmd in rx.iter() {
            match cmd {
                Cmd::SetDebug(val) => state.is_debug = val,
                Cmd::PonderHit => {
                    state.param.ponder = None;
                    break;
                },
                Cmd::Stop => {
                    // TODO report stuff.about pondering and terminate.
                    unimplemented!();
                },
            }
        }
        if state.param.ponder.is_some() {
            panic!("Sender hung up while pondering");
        }
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
    // TODO send info
    for cmd in rx.iter() {
        match cmd {
            Cmd::SetDebug(val) => state.is_debug = val,
            Cmd::PonderHit => {
                // TODO Report unexpected message.
                unimplemented!();
            },
            Cmd::Stop => {
                let _ = tx.send(Response::BestMove(best_move, None));
                return;
            }
        }
    }
    panic!("Sender hung up while calculating");
}
