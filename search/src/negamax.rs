use std::sync::atomic::{AtomicBool, Ordering};
use std::cmp::max;

use game::{Position, Move};
use types::{NumPlies, Score, ScoreUnit, Data};

// TODO put more parameters here
#[derive(Clone)]
pub struct Param {
    pub draw_val: ScoreUnit,
    pub depth: NumPlies,
}

pub fn negamax(pos: &mut Position,
               alpha: Option<Score>,
               beta: Option<Score>,
               param: Param,
               is_killed: &AtomicBool) -> (Score, Data) {
    negamax_generic(pos, alpha, beta, param, is_killed,
                    &mut |x| Box::new(x.legal_iter()),
                    &mut |x, draw_val| (x.eval(draw_val), Data::one_node()))
}

// TODO somehow eliminate the Box
fn negamax_generic<F, G>(pos: &mut Position,
                         alpha: Option<Score>,
                         beta: Option<Score>,
                         param: Param,
                         is_killed: &AtomicBool,
                         move_gen_fn: &mut F,
                         eval_fn: &mut G) -> (Score, Data) where
for<'a> F: FnMut(&'a Position) -> Box<Iterator<Item = Move> + 'a>,
for<'b> G: FnMut(&'b mut Position, ScoreUnit) -> (Score, Data) {
    if is_killed.load(Ordering::Relaxed) {
        return (Score::Value(ScoreUnit(0)), Data::one_node());
    }
    if param.depth == NumPlies(0) {
        return eval_fn(pos, param.draw_val);
    }

    let (has_legal, score_opt, data): (bool, Option<Score>, Data) = {
        let temp = pos.clone();
        let move_iter = move_gen_fn(&temp);

        let mut has_legal = false;
        let mut prev_score_opt: Option<Score> = alpha;
        let mut prev_data = Data::one_node();

        for curr_move in move_iter {

            let new_alpha = beta.map(|x| x.decrement());
            let new_beta = prev_score_opt.map(|x| x.decrement());
            let new_param = Param {
                draw_val: -param.draw_val,
                depth: NumPlies(param.depth.0 - 1)
            };
            let (temp_score, temp_data) = pos.with_move(&curr_move, |new_pos|
                negamax_generic(new_pos,
                                new_alpha,
                                new_beta,
                                new_param,
                                is_killed,
                                move_gen_fn,
                                eval_fn));
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

    if has_legal {
        (score_opt.unwrap(), data)
    } else {
        eval_fn(pos, param.draw_val)
    }
}
