use std::sync::mpsc::Sender;
use std::sync::atomic::{AtomicBool, Ordering};

use game::{Move, Position, Score, ScoreUnit, NumPlies};
use types::Data;
use negamax::{negamax_root, Bound};
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

    let guess_score = table.get(pos).map_or(ScoreUnit(0), |x| {
        if let Bound::Exact(Score::Value(val)) = x.bound {
            val
        } else {
            ScoreUnit(0)
        }
    });

    let aspiration_width = [ScoreUnit(25), ScoreUnit(100), ScoreUnit(500)];

    let mut alpha_window = 0;
    let mut beta_window = 0;
    let mut best_score_move_opt = None;
    let mut data = Data::one_node();

    while best_score_move_opt.is_none() {
        let curr_alpha = aspiration_width.get(alpha_window)
                                         .map(|&diff| Score::Value(guess_score - diff));
        let curr_beta  = aspiration_width.get(beta_window)
                                         .map(|&diff| Score::Value(guess_score + diff));
        let curr_ans = negamax_root(pos, curr_alpha, curr_beta, draw_val,
                                    depth, table, is_killed, search_moves);
        if is_killed.load(Ordering::Relaxed) {
            // Thread killed. Bail out
            return;
        }
        let (curr_bound, curr_best_move_opt, curr_data) = curr_ans;
        data = data.combine(curr_data);
        match curr_bound {
            Bound::Lower(_) => beta_window += 1,
            Bound::Upper(_) => alpha_window += 1,
            Bound::Exact(x) => best_score_move_opt = Some((x, curr_best_move_opt.unwrap())),
        }
    }
    
    let (best_score, best_move) = best_score_move_opt.unwrap();

    let _ = tx.send((best_score, best_move, data));
}
