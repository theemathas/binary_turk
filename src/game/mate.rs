#![allow(dead_code)]

use super::moves::Move;
use super::pos::Position;
use super::legal;
use super::make_move::make_move;

pub fn is_checkmated(mut p: Position) -> bool {
    if has_legal_moves(p.clone()) {
        false
    } else {
        p.swap_side_to_move();
        legal::can_take_king(p)
    }
}

pub fn is_stalemated(mut p: Position) -> bool {
    if has_legal_moves(p.clone()) {
        false
    } else {
        p.swap_side_to_move();
        !legal::can_take_king(p)
    }
}

pub fn has_legal_moves(p: Position) -> bool {
    legal::receive_legal(p).iter().next().is_some()
}

pub fn is_stalemate(mut p: Position, m: &Move) -> bool {
    make_move(&mut p, m);
    is_stalemated(p)
}

pub fn is_checkmate(mut p: Position, m: &Move) -> bool {
    make_move(&mut p, m);
    is_checkmated(p)
}
