//! The types and some utility functions related to pieces.

use super::color::{Color,White,Black};

pub use self::Piece::{WP,WK,WQ,WB,WN,WR};
pub use self::Piece::{BP,BK,BQ,BB,BN,BR};
pub use self::Type::{Pawn,King,Queen,Bishop,Knight,Rook};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Piece {
    WP,
    WK,
    WQ,
    WB,
    WN,
    WR,

    BP,
    BK,
    BQ,
    BB,
    BN,
    BR,
}

pub static ALL_PIECES: [Piece; 12] = [WP, WK, WQ, WB, WN, WR,
                                      BP, BK, BQ, BB, BN, BR];

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Type {
    Pawn,
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
}

impl Piece {
    pub fn piece_type(&self) -> Type {
        match *self {
            WP | BP => Pawn,
            WK | BK => King,
            WQ | BQ => Queen,
            WB | BB => Bishop,
            WN | BN => Knight,
            WR | BR => Rook,
        }
    }
    pub fn color(&self) -> Color {
        match *self {
            WP | WK | WQ | WB | WN | WR => White,
            BP | BK | BQ | BB | BN | BR => Black,
        }
    }
    pub fn new(c : Color, t : Type) -> Piece {
        match (c,t) {
            (White,Pawn) => WP,
            (White,King) => WK,
            (White,Queen) => WQ,
            (White,Bishop) => WB,
            (White,Knight) => WN,
            (White,Rook) => WR,
            (Black,Pawn) => BP,
            (Black,King) => BK,
            (Black,Queen) => BQ,
            (Black,Bishop) => BB,
            (Black,Knight) => BN,
            (Black,Rook) => BR,
        }
    }
}
