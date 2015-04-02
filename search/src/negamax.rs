use std::sync::atomic::{AtomicBool, Ordering};
use std::cmp::max;

use game::{Position, Move, Score, ScoreUnit, NumPlies};
use types::InnerData;

use transposition_table::TranspositionTable;

#[derive(Copy, Clone, Debug)]
pub enum Bound {
    Exact(Score),
    Lower(Score),
    Upper(Score),
}
impl Bound {
    pub fn as_score(self) -> Score {
        match self {
            Bound::Exact(x) => x,
            Bound::Lower(x) => x,
            Bound::Upper(x) => x,
        }
    }
    pub fn is_exact(self) -> bool {
        if let Bound::Exact(_) = self { true } else { false }
    }
    pub fn is_lower(self) -> bool {
        if let Bound::Lower(_) = self { true } else { false }
    }
    pub fn is_upper(self) -> bool {
        if let Bound::Upper(_) = self { true } else { false }
    }
}

// TODO put more parameters here
#[derive(Clone)]
pub struct Param {
    pub eval_depth: Option<NumPlies>,
    pub table_depth: NumPlies,
}

pub fn negamax_root(pos: &mut Position,
                    alpha: Option<Score>,
                    beta: Option<Score>,
                    depth: NumPlies,
                    table: &mut TranspositionTable,
                    is_killed: &AtomicBool,
                    search_moves: &[Move]) -> (Bound, Option<Move>, InnerData) {
    let next_depth = NumPlies(depth.0 - 1);
    let param = Param { eval_depth: Some(NumPlies(1)),
                        table_depth: depth };
    negamax_generic(pos, alpha, beta, param, table, is_killed,
                    &mut |_| Box::new(search_moves.to_vec().into_iter()),
                    &mut |inner_pos, inner_alpha, inner_beta, inner_table| {
                        let inner_param = Param {
                            eval_depth: Some(next_depth),
                            table_depth: next_depth,
                        };
                        let (bound, _, data) =
                            negamax_inner(inner_pos, inner_alpha, inner_beta,
                                          inner_param, inner_table, is_killed);
                        (bound, data)
                    },
                    &mut |_| None)
}

fn negamax_inner(pos: &mut Position,
                 alpha: Option<Score>,
                 beta: Option<Score>,
                 param: Param,
                 table: &mut TranspositionTable,
                 is_killed: &AtomicBool) -> (Bound, Option<Move>, InnerData) {
    negamax_generic(pos, alpha, beta, param, table, is_killed,
                    &mut |x| Box::new(x.legal_iter()),
                    &mut |x, inner_alpha, inner_beta, table| {
                        let quiescence_param = Param {
                            eval_depth: None,
                            table_depth: NumPlies(0),
                        };
                        let (bound, _, data) = quiescence(x, inner_alpha, inner_beta,
                                                          quiescence_param, table, is_killed);
                        (bound, data)
                    },
                    &mut |_| None)
}

fn quiescence(pos: &mut Position,
              alpha: Option<Score>,
              beta: Option<Score>,
              param: Param,
              table: &mut TranspositionTable,
              is_killed: &AtomicBool) -> (Bound, Option<Move>, InnerData) {
    negamax_generic(pos, alpha, beta, param, table, is_killed,
                    &mut |x| Box::new(x.legal_noisy_iter()),
                    &mut |x, _, _, _|
                        (Bound::Exact(x.eval()), InnerData::one_node()),
                    &mut |x| Some(x.eval()))
}

// TODO somehow eliminate the Box
fn negamax_generic<F, G, H>(pos: &mut Position,
                            alpha: Option<Score>,
                            beta: Option<Score>,
                            param: Param,
                            table: &mut TranspositionTable,
                            is_killed: &AtomicBool,
                            move_gen_fn: &mut F,
                            eval_fn: &mut G,
                            stand_pat_fn: &mut H) -> (Bound, Option<Move>, InnerData) where
for<'a> F: FnMut(&'a Position) -> Box<Iterator<Item = Move> + 'a>,
for<'b> G: FnMut(&'b mut Position, Option<Score>, Option<Score>,
                 &mut TranspositionTable) -> (Bound, InnerData),
for<'c> H: FnMut(&'c mut Position) -> Option<Score> {
    if is_killed.load(Ordering::Relaxed) {
        return (Bound::Exact(Score::Value(ScoreUnit(0))), None, InnerData::one_node());
    }

    let mut table_best_move_opt = None;
    if let Some(data_ref) = table.get(pos) {
        table_best_move_opt = data_ref.best_move_opt.clone();
        if data_ref.depth >= param.table_depth {
            let table_bound = data_ref.bound;
            let lower_than_alpha = alpha.is_some() &&
                                   !table_bound.is_lower() &&
                                   table_bound.as_score() <= alpha.unwrap();
            if lower_than_alpha {
                return (Bound::Upper(alpha.unwrap()), table_best_move_opt, InnerData::one_node());
            }
            let higher_than_beta = beta.is_some() &&
                                   !table_bound.is_upper() &&
                                   table_bound.as_score() >= beta.unwrap();
            if higher_than_beta {
                return (Bound::Lower(beta.unwrap()), table_best_move_opt, InnerData::one_node());
            }
            if table_bound.is_exact() {
                return (table_bound, table_best_move_opt, InnerData::one_node());
            }
        }
    }
    let table_best_move_opt = table_best_move_opt;

    if param.eval_depth == Some(NumPlies(0)) {
        let (bound, data) = eval_fn(pos, alpha, beta, table);
        table.set(pos, NumPlies(0), None, bound);
        return (bound, None, data);
    }

    let (has_legal, score_opt, best_move_opt, data):
        (bool, Option<Score>, Option<Move>, InnerData) = (|| {
        let temp = pos.clone();
        let move_iter: Box<Iterator<Item = Move>> = {
            let normal_iter = move_gen_fn(&temp);
            if let Some(ref table_move) = table_best_move_opt {
                let is_table_move_valid = move_gen_fn(&temp).any(|y| y == *table_move);
                if is_table_move_valid {
                    Box::new(Some(
                        table_move.clone()).into_iter()
                                  .chain(normal_iter.filter(move |x| *x != *table_move)))
                } else {
                    normal_iter
                }
            } else {
                normal_iter
            }
        };

        let mut has_legal = false;
        let mut prev_score_opt: Option<Score> = alpha;
        let mut prev_best_move_opt: Option<Move> = None;

        if let Some(stand_pat_score) = stand_pat_fn(pos) {
            let new_score = match prev_score_opt {
                None => stand_pat_score,
                Some(prev_score) => max(prev_score, stand_pat_score),
            };

            has_legal = true;
            prev_score_opt = Some(new_score);

            if let Some(beta_val) = beta {
                if new_score >= beta_val {
                    return (true, beta, None, InnerData::one_node());
                }
            }
        }

        let mut prev_data = InnerData::one_node();

        for curr_move in move_iter {

            let new_alpha = beta.map(|x| x.decrement());
            let new_beta = prev_score_opt.map(|x| x.decrement());
            let new_param = Param {
                eval_depth: param.eval_depth.map(|x| NumPlies(x.0 - 1)),
                table_depth: NumPlies(param.table_depth.0.saturating_sub(1)),
            };
            let (temp_bound, _, temp_data) = pos.with_move(&curr_move, |new_pos|
                negamax_generic(new_pos,
                                new_alpha,
                                new_beta,
                                new_param,
                                table,
                                is_killed,
                                move_gen_fn,
                                eval_fn,
                                stand_pat_fn));
            let curr_score = temp_bound.as_score().increment();
            let curr_data = temp_data.increment();

            let (new_score, new_best_move_opt) = match prev_score_opt {
                None => (curr_score, Some(curr_move)),
                Some(prev_score) => {
                    if curr_score <= prev_score {
                        (prev_score, prev_best_move_opt)
                    } else {
                        (curr_score, Some(curr_move))
                    }
                }
            };
            let new_data = prev_data.combine(curr_data);

            has_legal = true;
            prev_score_opt = Some(new_score);
            prev_best_move_opt = new_best_move_opt;
            prev_data = new_data;

            if let Some(beta_val) = beta {
                if new_score >= beta_val {
                    prev_score_opt = beta;
                    break;
                }
            }
        }

        (has_legal, prev_score_opt, prev_best_move_opt, prev_data)
    })();

    if has_legal {
        let score = score_opt.unwrap();
        let bound = {
            if alpha.is_some() && score <= alpha.unwrap() {
                Bound::Upper(alpha.unwrap())
            } else if beta.is_some() && score >= beta.unwrap() {
                Bound::Lower(beta.unwrap())
            } else {
                Bound::Exact(score)
            }
        };
        table.set(pos, param.table_depth, best_move_opt.clone(), bound);
        (bound, best_move_opt, data)
    } else {
        let (bound, data) = eval_fn(pos, alpha, beta, table);
        table.set(pos, param.table_depth, None, bound);
        (bound, None, data)
    }
}
