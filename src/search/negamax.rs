use std::sync::atomic::{AtomicBool, Ordering};

use game::Position;
use types::{NumPlies, Score, ScoreUnit};

// TODO put actual data here
pub struct Data;
impl Data {
    pub fn combine(self, _: Data) -> Data {
        Data
    }
}

// TODO put more parameters here
#[derive(Clone)]
pub struct Param {
    pub draw_val: ScoreUnit,
}

pub fn negamax(pos: &mut Position, depth: NumPlies, param: Param,
               is_killed: &AtomicBool) -> (Score, Data) {
    if is_killed.load(Ordering::Relaxed) {
        return (Score::Value(ScoreUnit(0)), Data);
    }
    if depth == NumPlies(0) {
        return (pos.eval(param.draw_val), Data);
    }

    let ans_opt: Option<(Score, Data)> = {
        let temp = pos.clone();
        let move_iter = temp.legal_iter();

        let mut prev_ans_opt: Option<(Score, Data)> = None;

        for curr_move in move_iter {
            let new_param = Param { draw_val: -param.draw_val };
            let (temp_score, curr_data) = pos.with_move(&curr_move, move |new_pos| {
                negamax(new_pos, NumPlies(depth.0 - 1), new_param, is_killed)
            });
            let curr_score = temp_score.increment();

            let new_ans = match prev_ans_opt {
                None => (curr_score, curr_data),
                Some(prev_ans) => {

                    let (prev_score, prev_data) = prev_ans;

                    let combined_data = prev_data.combine(curr_data);

                    if curr_score > prev_score {
                        (curr_score, combined_data)
                    } else {
                        (prev_score, combined_data)
                    }
                },
            };
            prev_ans_opt = Some(new_ans);
        }

        prev_ans_opt
    };

    ans_opt.unwrap_or_else(||(pos.eval(param.draw_val), Data))
}
