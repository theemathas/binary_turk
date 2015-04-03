//! The types for moves and plies.

use std::str::FromStr;
use std::fmt;

use piece::{self, Piece, Queen, Bishop, Knight, Rook, King, Pawn};
use square::{Square, File, ParseSquareError};
use castle::{Side, Kingside, Queenside};
use pos::{Position, at_in_pos, is_empty_at_in_pos};

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct NumPlies(pub u32);

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct NumMoves(pub u32);

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Move {
    from: Square,
    to: Square,
	capture_normal: Option<Piece>,
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
            capture_normal: None,
            castle: None,
            is_en_passant: false,
            promote: None,
            is_pawn_double_move: false,
        }
    }

    pub fn from(&self) -> Square { self.from }
    pub fn to(&self) -> Square { self.to }

    pub fn capture_normal(&self) -> Option<Piece> { self.capture_normal }
    pub fn set_capture_normal(&mut self, val: Option<Piece>) {
        self.capture_normal = val;
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

    pub fn is_noisy(&self) -> bool {
        self.capture_normal().is_some() || self.is_en_passant() || self.promote().is_some()
    }
    pub fn is_quiet(&self) -> bool {
        !self.is_noisy()
    }
}
impl fmt::Display for Move {
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

pub struct ParseFromToError(());
impl From<ParseSquareError> for ParseFromToError {
    fn from(_: ParseSquareError) -> Self { ParseFromToError(()) }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
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
        if !is_empty_at_in_pos(pos, self.to) {
            ans.set_capture_normal(at_in_pos(pos, self.to));
        }
        match at_in_pos(pos, self.from).map(|x| x.piece_type()) {
            Some(King) => {
                match (self.from.file(), self.to.file()) {
                    (File(4), File(6)) => ans.set_castle(Some(Kingside)),
                    (File(4), File(2)) => ans.set_castle(Some(Queenside)),
                    _ => {},
                }
            },
            Some(Pawn) => {
                if self.from.file() != self.to.file() && ans.capture_normal().is_none() {
                    ans.set_en_passant(true);
                } else if ((self.from.rank().0) - (self.to.rank().0)).abs() != 1 {
                    ans.set_pawn_double_move(true);
                }
            },
            _ => {},
        }
        debug!("Converted {:?} into {:?}", *self, ans);
        ans
    }
}
impl FromStr for FromTo {
    type Err = ParseFromToError;
    fn from_str(s: &str) -> Result<FromTo, ParseFromToError> {
        if s.len() != 4 && s.len() != 5 { return Err(ParseFromToError(())); }
        let from: Square = try!(FromStr::from_str(&s[0..2]));
        let to  : Square = try!(FromStr::from_str(&s[2..4]));
        let mut ans = FromTo::new(from, to);
        if s.len() == 5 {
            ans.promote = Some( match s.as_bytes()[4] {
                b'q' => Queen,
                b'b' => Bishop,
                b'n' => Knight,
                b'r' => Rook,
                _ => return Err(ParseFromToError(())),
            });
        }
        Ok(ans)
    }
}
