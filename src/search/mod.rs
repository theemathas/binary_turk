use state::State;
use uci::types::Response;
use game::{Move,receive_legal};

pub use self::types::SearchCmd;

mod types;

pub fn start(mut state: State, rx: Receiver<SearchCmd>, tx:Sender<Response>) {
    if state.search_param.ponder.is_some() {
        // Actually should ponder, but now just waits for our move.
        for cmd in rx.iter() {
            match cmd {
                SearchCmd::SetDebug(val) => state.is_debug = val,
                SearchCmd::SetOption(_name, _val) => {
                    // TODO set options in start_search
                    unimplemented!();
                },
                SearchCmd::PonderHit => {
                    state.search_param.ponder = None;
                    break;
                },
                SearchCmd::Stop => {
                    // TODO report stuff.about pondering and terminate.
                    unimplemented!();
                },
            }
        }
        if state.search_param.ponder.is_some() {
            panic!("Sender hung up while pondering");
        }
    }
    let legal_moves_chan = receive_legal(state.pos.clone());
    let legal_moves = legal_moves_chan.iter();
    let search_moves: Vec<Move> = match state.search_param.search_moves {
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
            SearchCmd::SetDebug(val) => state.is_debug = val,
            SearchCmd::SetOption(_name, _val) => {
                // TODO Set options in start_search
                unimplemented!();
            },
            SearchCmd::PonderHit => {
                // TODO Report unexpected message.
                unimplemented!();
            },
            SearchCmd::Stop => {
                tx.send(Response::BestMove(best_move, None));
                return;
            }
        }
    }
    panic!("Sender hung up while calculating");
}
