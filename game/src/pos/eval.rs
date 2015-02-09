//! This module is for statically evaluating a position.

use types::{ScoreUnit, Score, NumMoves};

use super::super::{Color, Piece, PieceType};
use super::super::{Pawn, King, Queen, Bishop, Knight, Rook};
use super::Position;

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
