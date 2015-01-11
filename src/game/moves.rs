//! The types for moves and plies.

use std::str::FromStr;
use std::fmt;
use std::num::SignedInt;

use super::piece::{self, Queen, Bishop, Knight, Rook, King, Pawn};
use super::square::{Square, File};
use super::castle::{Side, Kingside, Queenside};
use super::pos::Position;

#[derive(PartialEq, Eq, Clone, Show)]
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
impl fmt::String for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}{}", self.from, self.to));
        if let Some(val) = self.promote() {
            try!(write!(f, "{}", match val {
                Queen  => 'q',
                Bishop => 'b',
                Knight => 'n',
                Rook   => 'r',
                _ => return Err(fmt::Error),
            }))
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Show)]
pub struct FromTo {
    from: Square,
    to: Square,
    promote: Option<piece::Type>,
}
impl FromTo {
    pub fn new(from: Square, to: Square) -> FromTo {
        FromTo { from: from, to: to, promote: None }
    }
    pub fn to_move_with_pos(&self, pos: &Position) -> Move {
        let mut ans = Move::new(self.from, self.to);
        ans.set_promote(self.promote);
        if !pos.is_empty_at(self.to) {
            ans.set_capture(true);
        }
        match pos.at(self.to).map(|x| x.piece_type()) {
            Some(King) => {
                match (self.from.file(), self.to.file()) {
                    (File(4), File(6)) => ans.set_castle(Some(Kingside)),
                    (File(4), File(2)) => ans.set_castle(Some(Queenside)),
                    _ => {},
                }
            },
            Some(Pawn) => {
                if !ans.is_capture() && self.from.file() != self.to.file() {
                    ans.set_en_passant(true);
                } else if ((self.from.rank().0 as i8) - (self.to.rank().0 as i8)).abs() != 1 {
                    ans.set_pawn_double_move(true);
                }
            },
            _ => {},
        }
        ans
    }
}
impl FromStr for FromTo {
    fn from_str(s: &str) -> Option<FromTo> {
        if s.len() != 4 && s.len() != 5 { return None; }
        let from: Square = match FromStr::from_str(&s[0..2]) {
            Some(val) => val, None => return None };
        let to:   Square = match FromStr::from_str(&s[2..4]) {
            Some(val) => val, None => return None };
        let mut ans = FromTo::new(from, to);
        if s.len() == 5 {
            ans.promote = Some( match s.as_bytes()[4] {
                b'q' => Queen,
                b'b' => Bishop,
                b'n' => Knight,
                b'r' => Rook,
                _ => return None,
            });
        }
        Some(ans)
    }
}
