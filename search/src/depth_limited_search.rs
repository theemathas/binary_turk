use std::sync::mpsc::Sender;
use std::sync::atomic::AtomicBool;

use game::{Move, Position, Score, ScoreUnit, NumPlies};
use types::Data;
use negamax::{self, negamax};
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

    let next_depth = NumPlies(depth.0 - 1);
    let next_draw_val = -draw_val;

    let param = negamax::Param {
        draw_val: next_draw_val,
        depth: Some(next_depth),
    };

    let mut prev_ans_opt: Option<(Score, Move)> = None;
    let mut prev_data = Data::one_node();

    for curr_move_ref in search_moves.iter() {
        let curr_move = curr_move_ref.clone();

        let prev_best_score_opt = prev_ans_opt.as_ref().map(|x| x.0);
        let (temp_bound, _, temp_data) =
            pos.with_move(&curr_move, |new_pos|
                negamax(&mut new_pos.clone(),
                        prev_best_score_opt,
                        None,
                        param.clone(),
                        table,
                        is_killed));
        let curr_score = temp_bound.as_score().increment();
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
