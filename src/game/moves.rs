//! The types for moves and plies.

use std::str::FromStr;
use std::fmt;

use super::piece::{mod, Queen, Bishop, Knight, Rook};
use super::square::Square;
use super::castle::Side;

#[deriving(PartialEq, Eq, Clone)]
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

    pub fn from(&self) -> Square { self.from }
    pub fn to(&self) -> Square { self.to }

    pub fn is_capture(&self) -> bool { self.is_capture }
    pub fn set_capture(&mut self, val: bool) {
        self.is_capture = val;
    }

    pub fn castle(&self) -> Option<Side> { self.castle }
    pub fn set_castle(&mut self, val: Option<Side>) {
        self.castle = val;
    }

    pub fn is_en_passant(&self) -> bool { self.is_en_passant }
    pub fn set_en_passant(&mut self, val: bool) {
        self.is_en_passant = val;
    }

    pub fn promote(&self) -> Option<piece::Type> { self.promote }
    pub fn set_promote(&mut self, val: Option<piece::Type>) {
        self.promote = val;
    }

    pub fn is_pawn_double_move(&self) -> bool { self.is_pawn_double_move }
    pub fn set_pawn_double_move(&mut self, val: bool) {
        self.is_pawn_double_move = val;
    }
}
impl FromStr for Move {
    fn from_str(s: &str) -> Option<Move> {
        if s.len() != 4 && s.len() != 5 { return None; }
        let from: Square = match FromStr::from_str(&*s.slice(0,2)) {
            Some(val) => val, None => return None };
        let to:   Square = match FromStr::from_str(&*s.slice(2,4)) {
            Some(val) => val, None => return None };
        let mut ans = Move::new(from, to);
        if s.len() == 5 {
            ans.set_promote(Some( match s.as_bytes()[4] {
                b'q' => Queen,
                b'b' => Bishop,
                b'n' => Knight,
                b'r' => Rook,
                _ => return None,
            }));
        }
        Some(ans)
    }
}
impl fmt::Show for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}{}", self.from, self.to));
        match self.promote() {
            Some(val) => {
                write!(f, "{}", match val {
                    Queen  => 'q',
                    Bishop => 'b',
                    Knight => 'n',
                    Rook   => 'r',
                    _ => return Err(fmt::Error),
                })
            }
            None => Ok(())
        }
    }
}

#[deriving(PartialEq,Eq,Copy,Clone)]
pub struct NumPlies(pub u16);

#[deriving(PartialEq,Eq,Copy,Clone)]
pub struct NumMoves(pub u16);
