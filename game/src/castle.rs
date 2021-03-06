use color::{Color, White, Black};
use square::{Square, File, Rank};

pub use self::Side::{Kingside, Queenside};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Side {
    Kingside,
    Queenside,
}
impl Side {
    pub fn require_empty_squares(self, c: Color) -> Vec<Square> {
        match (c, self) {
            (White, Kingside)  => vec![Square::new(File(5), Rank(0)),
                                       Square::new(File(6), Rank(0))],
            (White, Queenside) => vec![Square::new(File(3), Rank(0)),
                                       Square::new(File(2), Rank(0)),
                                       Square::new(File(1), Rank(0))],
            (Black, Kingside)  => vec![Square::new(File(5), Rank(7)),
                                       Square::new(File(6), Rank(7))],
            (Black, Queenside) => vec![Square::new(File(3), Rank(7)),
                                       Square::new(File(2), Rank(7)),
                                       Square::new(File(1), Rank(7))],
        }
    }

    pub fn require_no_attack(self, c: Color) -> Vec<Square> {
        match (c, self) {
            (White, Kingside)  => vec![Square::new(File(4), Rank(0)),
                                       Square::new(File(5), Rank(0)),
                                       Square::new(File(6), Rank(0))],
            (White, Queenside) => vec![Square::new(File(4), Rank(0)),
                                       Square::new(File(3), Rank(0)),
                                       Square::new(File(2), Rank(0))],
            (Black, Kingside)  => vec![Square::new(File(4), Rank(7)),
                                       Square::new(File(5), Rank(7)),
                                       Square::new(File(6), Rank(7))],
            (Black, Queenside) => vec![Square::new(File(4), Rank(7)),
                                       Square::new(File(3), Rank(7)),
                                       Square::new(File(2), Rank(7))],
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
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
    pub fn set(&mut self, side: Side, c: Color, val: bool) {
        match (c, side) {
            (White, Kingside)  => self.w_kingside  = val,
            (White, Queenside) => self.w_queenside = val,
            (Black, Kingside)  => self.b_kingside  = val,
            (Black, Queenside) => self.b_queenside = val,
        }
    }
}
