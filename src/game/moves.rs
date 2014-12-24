//! The types for moves and plies.

use super::piece;
use super::square::Square;

pub struct Move {
    from: Square,
    to: Square,
    is_capture: bool,
    is_castle: bool,
    is_en_passant: bool,
    promote: Option<piece::Type>,
}
impl Move {
    pub fn new(from: Square, to: Square) -> Move {
        Move {
            from: from,
            to: to,
            is_capture: false,
            is_castle: false,
            is_en_passant: false,
            promote: None,
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
    pub fn is_castle(&self) -> bool {
        self.is_castle
    }
    pub fn set_castle(&self, val: bool) -> Move {
        Move { is_castle: val, ..*self }
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
}

#[deriving(PartialEq,Eq,Copy,Clone)]
pub struct Plies(pub u8);
