//! This module is for everything about the rules of chess.

#![feature(core, collections, unicode)]

#[macro_use]
extern crate log;
extern crate types;

pub use self::color::{Color, White, Black};
pub use self::piece::Piece;
pub use self::piece::Type as PieceType;
pub use self::piece::Type::*;
pub use self::moves::{Move, FromTo};

pub use self::pos::Position;
pub use self::pos::ExtraData as PosExtraData;

mod color;
mod piece;
mod square;
mod moves;
mod castle;
mod pos;
