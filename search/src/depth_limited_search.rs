use std::sync::mpsc::Sender;
use std::sync::atomic::AtomicBool;

use game::{Move, Position, Score, ScoreUnit, NumPlies};
use types::Data;
use negamax::negamax_root;
use transposition_table::TranspositionTable;

pub fn depth_limited_search(pos: &mut Position,
                            search_moves: &[Move],
                            depth: NumPlies,
                            table: &mut TranspositionTable,
                            tx: Sender<(Score, Move, Data)>,
                            is_killed: &AtomicBool) {
    assert!(!search_moves.is_empty());
    assert!(depth.0 >= 1);

    // TODO Take this draw_val value from somewhere else
    let draw_val = ScoreUnit(0);

    let (bound, best_move_opt, data) =
        negamax_root(pos, None, None, draw_val, depth, table, is_killed, search_moves);
    let best_score = bound.as_score();
    let best_move = best_move_opt.unwrap();

    let _ = tx.send((best_score, best_move, data));
}
