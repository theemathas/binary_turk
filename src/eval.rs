//! This module is for statically evaluating a position.

use super::game::{Position, Color, Piece, PieceType};
use super::game::{Pawn, King, Queen, Bishop, Knight, Rook};
use super::game::{is_checkmated, is_stalemated};

pub use self::Result::{Score, WinIn, LoseIn, Draw};

/// An assessment of the position.
pub enum Result {
    // Positive: advantage for side to move.
    // Negative: disadvantage for side to move.
    Score(int),
    // Side to move can checkmate in x moves.
    // WinIn(1): can checkmate now.
    // WinIn(2): can checkmate next move.
    WinIn(uint),
    // Side to move will be checkmated in x moves.
    // WinIn(0): already checkmated.
    // WinIn(1): Will be immediately checkmated after any move.
    LoseIn(uint),
    Draw,
}

/// Evaluates the position without searching.
pub fn eval(p: &Position) -> Result {
    unimplemented!()
}
