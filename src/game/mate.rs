use super::moves::Move;
use super::pos::Position;
use super::legal;
use super::make_move::make_move_mut;

pub fn is_checkmated(mut p: Position) -> bool {
    if has_legal_moves(p.clone()) {
        return false;
    }
    let curr_side = p.side_to_move();
    p.set_side_to_move_mut(curr_side.invert());
    legal::can_take_king(&p)
}

pub fn is_stalemated(mut p: Position) -> bool {
    if has_legal_moves(p.clone()) {
        return false;
    }
    let curr_side = p.side_to_move();
    p.set_side_to_move_mut(curr_side.invert());
    !legal::can_take_king(&p)
}

pub fn has_legal_moves(p: Position) -> bool {
    for _ in legal::receive_legal(p).iter() {
        return true;
    }
    false
}

pub fn is_stalemate(mut p: Position, m: &Move) -> bool {
    make_move_mut(&mut p, m);
    is_stalemated(p)
}

pub fn is_checkmate(mut p: Position, m: &Move) -> bool {
    make_move_mut(&mut p, m);
    is_checkmated(p)
}