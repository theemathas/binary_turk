use super::color::{Color,White,Black};

pub use self::Side::{Kingside,Queenside};

#[deriving(PartialEq,Eq,Copy,Clone)]
pub enum Side {
    Kingside,
    Queenside,
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
    pub fn set(self, side: Side, c: Color, val: bool) -> CastlingData {
        match (c, side) {
            (White, Kingside)  => CastlingData { w_kingside:  val, ..self },
            (White, Queenside) => CastlingData { w_queenside: val, ..self },
            (Black, Kingside)  => CastlingData { b_kingside:  val, ..self },
            (Black, Queenside) => CastlingData { b_queenside: val, ..self },
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
