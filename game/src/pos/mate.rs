use super::Position;

pub fn is_checkmated(p: &mut Position) -> bool {
    if p.has_legal_moves() {
        false
    } else {
        p.swap_side_to_move();
        let ans = p.can_take_king();
        p.swap_side_to_move();
        ans
    }
}

pub fn is_stalemated(p: &mut Position) -> bool {
    if p.has_legal_moves() {
        false
    } else {
        p.swap_side_to_move();
        let ans = !p.can_take_king();
        p.swap_side_to_move();
        ans
    }
}

pub fn has_legal_moves(p: &Position) -> bool {
    p.legal_iter().next().is_some()
}
