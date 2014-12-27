//! implements the board representation

use std::vec;

use super::color::Color;
use super::piece::{Piece,King};
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
    pub fn is_piece_at(&self, p: Piece, s: Square) -> bool {
        self.at(s) == Some(p)
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
    pub fn king_square(&self, c: Color) -> Square {
        let curr_king = Piece::new(c, King);
        let temp = self.iter().find( |x| x.0 == curr_king ).unwrap();
        temp.1
    }
    fn piece_vec(&self) -> Vec<(Piece,Square)> {
        let mut ans: Vec<(Piece, Square)> = Vec::new();
        for f in range::<uint>(0,8) {
            for r in range::<uint>(0,8) {
                if self.data[f][r].is_some() {
                    ans.push((self.data[f][r].unwrap(), Square::new(File(f as u8), Rank(r as u8))));
                }
            }
        }
        ans
    }
    pub fn iter(&self) -> Iter { Iter(self.piece_vec().into_iter()) }
}

pub struct Iter(vec::IntoIter<(Piece,Square)>);
impl Iterator<(Piece,Square)> for Iter {
    fn next(&mut self) -> Option<(Piece,Square)> { self.0.next() }
    fn size_hint(&self) -> (uint, Option<uint>) { self.0.size_hint() }
}
impl DoubleEndedIterator<(Piece,Square)> for Iter {
    fn next_back(&mut self) -> Option<(Piece,Square)> { self.0.next_back() }
}
impl ExactSizeIterator<(Piece,Square)> for Iter {}
