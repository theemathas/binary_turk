//! This module is for statically evaluating a position.

use std::fmt;
use std::ops::{Add, Sub, Neg};
use std::cmp::Ordering;

use moves::NumMoves;
use {Color, Piece, PieceType};
use {Pawn, King, Queen, Bishop, Knight, Rook};
use super::Position;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct ScoreUnit(pub i32);
impl Add for ScoreUnit {
    type Output = Self;
    fn add(self, rhs: Self) -> Self { ScoreUnit(self.0 + rhs.0) }
}
impl Sub for ScoreUnit {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self { ScoreUnit(self.0 - rhs.0) }
}
impl Neg for ScoreUnit {
    type Output = Self;
    fn neg(self) -> Self { ScoreUnit(-self.0) }
}

/// An assessment of the position.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Score {
    // Positive: advantage for side to move.
    // Negative: disadvantage for side to move.
    Value(ScoreUnit),
    // Side to move can checkmate in x moves.
    // WinIn(NumMoves(1)): can checkmate now.
    // WinIn(NumMoves(2)): can checkmate next move.
    WinIn(NumMoves),
    // Side to move will be checkmated in x moves.
    // WinIn(NumMoves(0)): already checkmated.
    // WinIn(NumMoves(1)): Will be immediately checkmated after any move.
    LoseIn(NumMoves),
}
impl Score {
    pub fn increment(self) -> Score {
        match self {
            Score::Value(val) => Score::Value(-val),
            Score::WinIn(val) => Score::LoseIn(val),
            Score::LoseIn(val) => Score::WinIn(NumMoves(val.0+1)),
        }
    }
    pub fn decrement(self) -> Score {
        match self {
            Score::Value(val) => Score::Value(-val),
            Score::WinIn(val) => Score::LoseIn(NumMoves(
                if val.0 == 0 { 0 } else { val.0 - 1 } )),
            Score::LoseIn(val) => Score::WinIn(val),
        }
    }
}
impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Score::Value(val)   => write!(f, "cp {}", val.0),
            Score::WinIn(val)   => write!(f, "mate {}", val.0 as i32),
            Score::LoseIn(val)  => write!(f, "mate {}", (val.0 as i32) * -1),
        }
    }
}
impl Ord for Score {
    fn cmp(&self, other: &Score) -> Ordering {
        match *self {
            Score::WinIn(val1) => match *other {
                Score::WinIn(val2) => val2.cmp(&val1),
                _ => Ordering::Greater,
            },
            Score::LoseIn(val1) => match *other {
                Score::LoseIn(val2) => val1.cmp(&val2),
                _ => Ordering::Less,
            },
            Score::Value(val1) => match *other {
                Score::WinIn(_) => Ordering::Less,
                Score::LoseIn(_) => Ordering::Greater,
                Score::Value(val2) => val1.cmp(&val2),
            },
        }
    }
}
impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Evaluates the position without searching.
pub fn eval(p: &mut Position, draw_val: ScoreUnit) -> Score {
    if p.is_checkmated() {
        Score::LoseIn(NumMoves(0))
    } else if p.is_stalemated() {
        Score::Value(draw_val)
    } else {
        let c = p.side_to_move();
        // TODO change fold() to sum() when possible
        Score::Value(p.piece_iter().map( |(piece, _pos)| val_for_color(piece, c) ).fold(ScoreUnit(0), |x, y| x+y))
    }
}

fn val_for_color(piece: Piece, c: Color) -> ScoreUnit {
    let val = val_of_piece_type(piece.piece_type());
    if piece.color() == c {
        val
    } else {
        -val
    }
}

fn val_of_piece_type(piece_type: PieceType) -> ScoreUnit {
    ScoreUnit(match piece_type {
        King   => 100000,
        Pawn   =>    100,
        Queen  =>    900,
        Bishop =>    300,
        Knight =>    300,
        Rook   =>    500,
    })
}
