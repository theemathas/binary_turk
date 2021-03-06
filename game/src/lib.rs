//! This module is for everything about the rules of chess.

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

extern crate rand;

pub use color::{Color, White, Black};
pub use piece::Piece;
pub use piece::Type as PieceType;
pub use piece::Type::*;
pub use moves::{Move, FromTo};
pub use moves::{NumPlies, NumMoves};

pub use pos::Position;
pub use pos::ExtraData as PosExtraData;
pub use pos::{ScoreUnit, Score};
pub use pos::ZobristHash;

mod color;
mod piece;
mod square;
mod moves;
mod castle;
mod pos;
