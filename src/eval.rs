//! This module is for statically evaluating a position.

#![allow(dead_code)]

use std::iter::AdditiveIterator;
use std::fmt;

use types::NumMoves;

use super::game::{Position, Color, Piece, PieceType};
use super::game::{Pawn, King, Queen, Bishop, Knight, Rook};
use super::game::{is_checkmated, is_stalemated};

pub use self::Result::{Score, WinIn, LoseIn, Draw};

type ScoreUnit = i32;

const CENTIPAWNS_PER_UNIT: i32 = 1;

/// An assessment of the position.
#[derive(PartialEq, Eq, Copy, Clone, Show)]
pub enum Result {
    // Positive: advantage for side to move.
    // Negative: disadvantage for side to move.
    Score(ScoreUnit),
    // Side to move can checkmate in x moves.
    // WinIn(NumMoves(1)): can checkmate now.
    // WinIn(NumMoves(2)): can checkmate next move.
    WinIn(NumMoves),
    // Side to move will be checkmated in x moves.
    // WinIn(NumMoves(0)): already checkmated.
    // WinIn(NumMoves(1)): Will be immediately checkmated after any move.
    LoseIn(NumMoves),
    Draw,
}
impl fmt::String for Result {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Result::Score(val)   => write!(f, "cp {}", val * CENTIPAWNS_PER_UNIT),
            Result::WinIn(val)   => write!(f, "mate {}", val.0 as i32),
            Result::LoseIn(val)  => write!(f, "mate {}", (val.0 as i32) * -1),
            Result::Draw         => write!(f, "cp {}", 0),
        }
    }
}

/// Evaluates the position without searching.
pub fn eval(p: &Position) -> Result {
    if is_checkmated(p.clone()) {
        LoseIn(NumMoves(0))
    } else if is_stalemated(p.clone()) {
        Draw
    } else {
        let c = p.side_to_move();
        Score(p.piece_iter().map( |(piece, _pos)| val_for_color(piece, c) ).sum())
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
    match piece_type {
        King   => 100000,
        Pawn   =>    100,
        Queen  =>    900,
        Bishop =>    300,
        Knight =>    300,
        Rook   =>    500,
    }
}
