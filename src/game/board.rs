//! implements the board representation

use super::color::Color;
use super::piece::Piece;
use super::square::{File, Rank, Square};

#[deriving(Clone)]
pub struct Board {
    data: [[Option<Piece>, ..8], ..8],
}
impl Board {
    pub fn new() -> Board {
        Board {
            data: [[None, ..8], ..8],
        }
    }
    pub fn at(&self, s: Square) -> Option<Piece> {
        let (File(f), Rank(r)) = s.to_tuple();
        self.data[f as uint][r as uint]
    }
    pub fn is_empty_at(&self, s: Square) -> bool {
        self.at(s).is_none()
    }
    pub fn color_at(&self, s: Square) -> Option<Color> {
        self.at(s).map( |x| x.color() )
    }
    pub fn set_at_mut(&mut self, s: Square, val: Piece) {
        debug_assert!(self.at(s).is_none(), "set_at_mut(), s = {}", s);
        let (File(f), Rank(r)) = s.to_tuple();
        self.data[f as uint][r as uint] = Some(val);
    }
    pub fn remove_at_mut(&mut self, s: Square) {
        debug_assert!(self.at(s).is_some(), "remove_at_mut(), s = {}", s);
        let (File(f), Rank(r)) = s.to_tuple();
        self.data[f as uint][r as uint] = None;
    }
}
