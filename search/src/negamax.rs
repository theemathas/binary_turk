use std::sync::atomic::{AtomicBool, Ordering};
use std::cmp::max;

use game::Position;
use types::{NumPlies, Score, ScoreUnit, Data};

// TODO put more parameters here
#[derive(Clone)]
pub struct Param {
    pub draw_val: ScoreUnit,
}

pub fn negamax(pos: &mut Position,
               alpha: Option<Score>,
               beta: Option<Score>,
               depth: NumPlies,
               param: Param,
               is_killed: &AtomicBool) -> (Score, Data) {
    if is_killed.load(Ordering::Relaxed) {
        return (Score::Value(ScoreUnit(0)), Data::one_node());
    }
    if depth == NumPlies(0) {
        return (pos.eval(param.draw_val), Data::one_node());
    }

    let (has_legal, score_opt, data): (bool, Option<Score>, Data) = {
        let temp = pos.clone();
        let move_iter = temp.legal_iter();

        let mut has_legal = false;
        let mut prev_score_opt: Option<Score> = alpha;
        let mut prev_data = Data::one_node();

        for curr_move in move_iter {

            let new_param = Param { draw_val: -param.draw_val };
            let new_alpha = beta.map(|x| x.decrement());
            let new_beta = prev_score_opt.map(|x| x.decrement());
            let (temp_score, temp_data) = pos.with_move(&curr_move, move |new_pos| {
                negamax(new_pos,
                        new_alpha,
                        new_beta,
                        NumPlies(depth.0 - 1),
                        new_param,
                        is_killed)
            });
            let curr_score = temp_score.increment();
            let curr_data = temp_data.increment();

            let new_score = match prev_score_opt {
                None => curr_score,
                Some(prev_score) => max(prev_score, curr_score),
            };
            let new_data = prev_data.combine(curr_data);

            has_legal = true;
            prev_score_opt = Some(new_score);
            prev_data = new_data;

            if let Some(beta_val) = beta {
                if new_score >= beta_val {
                    prev_score_opt = beta;
                    break;
                }
            }
        }

        (has_legal, prev_score_opt, prev_data)
    };

    let score = if has_legal {
        score_opt.unwrap()
    } else {
        pos.eval(param.draw_val)
    };

    (score, data)
}
