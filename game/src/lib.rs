//! This module is for everything about the rules of chess.

#![feature(core, collections, unicode)]

#[macro_use]
extern crate log;
extern crate types;

pub use color::{Color, White, Black};
pub use piece::Piece;
pub use piece::Type as PieceType;
pub use piece::Type::*;
pub use moves::{Move, FromTo};

pub use pos::Position;
pub use pos::ExtraData as PosExtraData;

mod color;
mod piece;
mod square;
mod moves;
mod castle;
mod pos;
