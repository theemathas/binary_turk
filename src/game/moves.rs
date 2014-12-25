//! The types for moves and plies.

use super::piece;
use super::square::Square;
use super::castle::Side;

pub struct Move {
    from: Square,
    to: Square,
    is_capture: bool,
    castle: Option<Side>,
    is_en_passant: bool,
    promote: Option<piece::Type>,
    is_pawn_double_move: bool,
}
impl Move {
    pub fn new(from: Square, to: Square) -> Move {
        Move {
            from: from,
            to: to,
            is_capture: false,
            castle: None,
            is_en_passant: false,
            promote: None,
            is_pawn_double_move: false,
        }
    }
    pub fn from(&self) -> Square {
        self.from
    }
    pub fn to(&self) -> Square {
        self.to
    }
    pub fn is_capture(&self) -> bool {
        self.is_capture
    }
    pub fn set_capture(&self, val: bool) -> Move {
        Move { is_capture: val, ..*self }
    }
    pub fn castle(&self) -> Option<Side> {
        self.castle
    }
    pub fn is_castle(&self) -> bool {
        self.castle.is_some()
    }
    pub fn set_castle(&self, val: Option<Side>) -> Move {
        Move { castle: val, ..*self }
    }
    pub fn is_en_passant(&self) -> bool {
        self.is_en_passant
    }
    pub fn set_en_passant(&self, val: bool) -> Move {
        Move { is_en_passant: val, ..*self }
    }
    pub fn promote(&self) -> Option<piece::Type> {
        self.promote
    }
    pub fn is_promote(&self) -> bool {
        self.promote.is_some()
    }
    pub fn set_promote(&self, val: Option<piece::Type>) -> Move {
        Move { promote: val, ..*self }
    }
    pub fn is_pawn_double_move(&self) -> bool {
        self.is_pawn_double_move
    }
    pub fn set_pawn_double_move(&self, val: bool) -> Move {
        Move { is_pawn_double_move: val, ..*self }
    }
}

#[deriving(PartialEq,Eq,Copy,Clone)]
pub struct Plies(pub u8);
