//! This module is for everything about the rules of chess.

#![allow(dead_code)]

pub use self::color::{Color, White, Black};
pub use self::piece::Piece;
pub use self::piece::Type as PieceType;
pub use self::piece::{WP,WK,WQ,WB,WN,WR};
pub use self::piece::{BP,BK,BQ,BB,BN,BR};
pub use self::piece::Type::{Pawn,King,Queen,Bishop,Knight,Rook};
pub use self::moves::{Move, FromTo};
pub use self::pos::Position;
pub use self::fen::{fen_to_position, start_pos};
pub use self::legal::receive_legal;
pub use self::make_move::{make_move};
pub use self::mate::{is_checkmated, is_stalemated};
pub use self::mate::{is_checkmate, is_stalemate};

mod color;
mod piece;
mod square;
mod moves;

mod bitboard;
mod board;
mod castle;
mod pos;

mod fen;
mod gen;

mod legal;
mod make_move;

mod mate;

#[cfg(test)]
mod tests;
