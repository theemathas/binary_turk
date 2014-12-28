//! This module is for statically evaluating a position.

#![allow(dead_code)]

use std::iter::AdditiveIterator;

use super::game::{Position, Color, Piece, PieceType, Plies};
use super::game::{Pawn, King, Queen, Bishop, Knight, Rook};
use super::game::{is_checkmated, is_stalemated};

pub use self::Result::{Score, WinIn, LoseIn, Draw};

type ScoreUnit = int;

/// An assessment of the position.
pub enum Result {
    // Positive: advantage for side to move.
    // Negative: disadvantage for side to move.
    Score(ScoreUnit),
    // Side to move can checkmate in x moves.
    // WinIn(Plies(1)): can checkmate now.
    // WinIn(Plies(2)): can checkmate next move.
    WinIn(Plies),
    // Side to move will be checkmated in x moves.
    // WinIn(Plies(0)): already checkmated.
    // WinIn(Plies(1)): Will be immediately checkmated after any move.
    LoseIn(Plies),
    Draw,
}

/// Evaluates the position without searching.
pub fn eval(p: &Position) -> Result {
    if is_checkmated(p.clone()) {
        LoseIn(Plies(0))
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
