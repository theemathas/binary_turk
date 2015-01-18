#![allow(dead_code)]

use std::cmp;

use game::{Position, Move, make_move, unmake_move, receive_legal};
use eval::{eval, Score, ScoreUnit};
use types::NumPlies;

pub type Result = (Score, Data);

// TODO put actual data here
pub struct Data;

// TODO put more parameters here
pub struct Param {
    pub draw_val: ScoreUnit,
}

pub fn negamax(pos: &mut Position, depth: NumPlies, param: Param) -> Result {
    if depth == NumPlies(0) {
        return (eval(pos, param.draw_val), Data);
    }

    let (ans_val_opt, ans_data): (Option<Score>, Data) = {

        let move_chan = receive_legal(pos.clone());
        let move_iter = move_chan.iter();
        let res_iter = move_iter.map( |curr_move| {
            let new_param = Param { draw_val: -param.draw_val };
            with_move(&curr_move, pos, move |new_pos| {
                negamax(new_pos, NumPlies(depth.0 - 1), new_param)
            })
        });

        res_iter.fold((None::<Score>, Data), |prev_res_tuple, curr_res| {
            let (curr_val, curr_data) = curr_res;
            let (prev_val_opt, prev_data) = prev_res_tuple;

            let new_val = match prev_val_opt {
                Some(prev_val) => cmp::max(curr_val, prev_val),
                None => curr_val,
            };
            let new_data = Data;

            (Some(new_val), new_data)
        })
    };

    let ans_val = ans_val_opt.unwrap_or_else(|| eval(pos, param.draw_val));
    (ans_val, ans_data)
}

fn with_move<T, F: FnOnce(&mut Position) -> T>(curr_move: &Move, pos: &mut Position, f: F) -> T {
    let extra_data = pos.extra_data().clone();
    make_move(pos, curr_move);
    let ans = f(pos);
    unmake_move(pos, curr_move, extra_data);
    ans
}
