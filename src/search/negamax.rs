use std::sync::atomic::{AtomicBool, Ordering};

use game::{Position, Move};
use types::{NumPlies, Score, ScoreUnit};

// TODO put actual data here
pub struct Data;
impl Data {
    pub fn combine(self, other: Data) -> Data {
        Data
    }
}

// TODO put more parameters here
#[derive(Clone)]
pub struct Param {
    pub draw_val: ScoreUnit,
}

pub fn negamax(pos: &mut Position, depth: NumPlies, param: Param,
               is_killed: &AtomicBool) -> (Score, Option<Move>, Data) {
    if is_killed.load(Ordering::Relaxed) {
        return (Score::Value(ScoreUnit(0)), None, Data);
    }
    if depth == NumPlies(0) {
        return (pos.eval(param.draw_val), None, Data);
    }

    let ans_opt: Option<(Score, Move, Data)> = {
        let temp = pos.clone();
        let move_iter = temp.legal_iter();
        let mut ans_iter = move_iter.map( |curr_move| {
            let new_param = Param { draw_val: -param.draw_val };
            let (score, _next_best_move, data) = with_move(&curr_move, pos, move |new_pos| {
                negamax(new_pos, NumPlies(depth.0 - 1), new_param, is_killed)
            });
            (score.increment(), curr_move, data)
        });

        ans_iter.next().map(move |first_ans|
            ans_iter.fold(first_ans, |prev_ans, curr_ans| {

                let (prev_score, prev_move, prev_data) = prev_ans;
                let (curr_score, curr_move, curr_data) = curr_ans;

                let combined_data = prev_data.combine(curr_data);

                if curr_score > prev_score {
                    (curr_score, curr_move, combined_data)
                } else {
                    (prev_score, prev_move, combined_data)
                }
            }
        ))
    };

    match ans_opt {
        Some((ans_score, ans_move, ans_data)) => (ans_score, Some(ans_move), ans_data),
        None => return (pos.eval(param.draw_val), None, Data),
    }
}

fn with_move<T, F: FnOnce(&mut Position) -> T>(curr_move: &Move, pos: &mut Position, f: F) -> T {
    let extra_data = pos.extra_data().clone();
    pos.make_move(curr_move);
    let ans = f(pos);
    pos.unmake_move(curr_move, extra_data);
    ans
}
