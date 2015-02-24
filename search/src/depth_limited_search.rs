use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use game::{Move, Position, Score, ScoreUnit, NumPlies};
use types::Data;
use negamax::{self, negamax};

pub fn depth_limited_search(search_move_pos_arc: Arc<Vec<(Move, Position)>>,
                            depth: NumPlies,
                            tx: Sender<(Score, Move, Data)>,
                            is_killed: Arc<AtomicBool>) {
    debug_assert!(!search_move_pos_arc.is_empty());
    debug_assert!(depth.0 >= 1);

    // TODO This clone() shouldn't be needed
    let mut search_move_pos = (*search_move_pos_arc).clone();

    // TODO Take this draw_val value from somewhere else
    let draw_val = ScoreUnit(0);

    let next_depth = NumPlies(depth.0 - 1);
    let next_draw_val = -draw_val;

    let param = negamax::Param {
        draw_val: next_draw_val,
        depth: next_depth,
    };

    let mut prev_ans_opt: Option<(Score, Move)> = None;
    let mut prev_data = Data::one_node();

    for &mut (ref curr_move_ref, ref mut curr_pos) in search_move_pos.iter_mut() {
        let curr_move = curr_move_ref.clone();

        let prev_best_score_opt = prev_ans_opt.as_ref().map(|x| x.0);
        let (temp_score, temp_data) =
            negamax(curr_pos,
                    prev_best_score_opt,
                    None,
                    param.clone(),
                    &*is_killed);
        let curr_score = temp_score.increment();
        let curr_data = temp_data.increment();

        let new_ans = match prev_ans_opt {
            None => (curr_score, curr_move),
            Some(prev_ans) => {
                let (prev_score, prev_move) = prev_ans;

                if curr_score > prev_score {
                    (curr_score, curr_move)
                } else {
                    (prev_score, prev_move)
                }
            },
        };
        let new_data = prev_data.combine(curr_data);

        prev_ans_opt = Some(new_ans);
        prev_data = new_data;
    }

    let (best_score, best_move) = prev_ans_opt.unwrap();

    let _ = tx.send((best_score, best_move, prev_data));
}
