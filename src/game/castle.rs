use super::color::{Color,White,Black};
use super::square::{Square,File,Rank};

pub use self::Side::{Kingside,Queenside};

#[deriving(PartialEq,Eq,Copy,Clone)]
pub enum Side {
    Kingside,
    Queenside,
}

pub fn require_empty_squares(side: Side, c: Color) -> Vec<Square> {
    match (c, side) {
        (White, Kingside)  => vec![Square::new(File(5),Rank(0)),
                                   Square::new(File(6),Rank(0))],
        (White, Queenside) => vec![Square::new(File(3),Rank(0)),
                                   Square::new(File(2),Rank(0)),
                                   Square::new(File(1),Rank(0))],
        (Black, Kingside)  => vec![Square::new(File(5),Rank(7)),
                                   Square::new(File(6),Rank(7))],
        (Black, Queenside) => vec![Square::new(File(3),Rank(7)),
                                   Square::new(File(2),Rank(7)),
                                   Square::new(File(1),Rank(7))],
    }
}

pub fn require_no_attack(side: Side, c: Color) -> Vec<Square> {
    match (c, side) {
        (White, Kingside)  => vec![Square::new(File(4),Rank(0)),
                                   Square::new(File(5),Rank(0)),
                                   Square::new(File(6),Rank(0))],
        (White, Queenside) => vec![Square::new(File(4),Rank(0)),
                                   Square::new(File(3),Rank(0)),
                                   Square::new(File(2),Rank(0))],
        (Black, Kingside)  => vec![Square::new(File(4),Rank(7)),
                                   Square::new(File(5),Rank(7)),
                                   Square::new(File(6),Rank(7))],
        (Black, Queenside) => vec![Square::new(File(4),Rank(7)),
                                   Square::new(File(3),Rank(7)),
                                   Square::new(File(2),Rank(7))],
    }
}

#[deriving(PartialEq,Eq,Copy,Clone)]
pub struct CastlingData {
    w_kingside: bool,
    w_queenside: bool,
    b_kingside: bool,
    b_queenside: bool,
}
impl CastlingData {
    pub fn new() -> CastlingData {
        CastlingData {
            w_kingside: false,
            w_queenside: false,
            b_kingside: false,
            b_queenside: false,
        }
    }
    pub fn get(&self, side: Side, c: Color) -> bool {
        match (c, side) {
            (White, Kingside)  => self.w_kingside,
            (White, Queenside) => self.w_queenside,
            (Black, Kingside)  => self.b_kingside,
            (Black, Queenside) => self.b_queenside,
        }
    }
    pub fn set_mut(&mut self, side: Side, c: Color, val: bool) {
        match (c, side) {
            (White, Kingside)  => self.w_kingside  = val,
            (White, Queenside) => self.w_queenside = val,
            (Black, Kingside)  => self.b_kingside  = val,
            (Black, Queenside) => self.b_queenside = val,
        }
    }
}
