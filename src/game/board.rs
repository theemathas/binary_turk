//! implements the board representation

use std::vec;
use std::collections::VecMap;

use super::color::Color;
use super::piece::{Piece, King, WHITE_PIECES, BLACK_PIECES, ALL_PIECES};
use super::square::{File, Rank, Square};
use super::bitboard::BitBoard;

#[deriving(Clone)]
pub struct Board(VecMap<BitBoard>);

impl Board {
    pub fn new() -> Board {
        let mut ans = VecMap::new();
        for x in ALL_PIECES.iter() {
            ans.insert(*x as uint, BitBoard::new());
        }
        Board(ans)
    }
    pub fn at(&self, s: Square) -> Option<Piece> {
        for x in ALL_PIECES.iter() {
            if self.is_piece_at(*x, s) {
                return Some(*x);
            }
        }
        None
    }
    pub fn is_piece_at(&self, p: Piece, s: Square) -> bool {
        self.0.get(&(p as uint)).unwrap().at(s)
    }
    pub fn is_empty_at(&self, s: Square) -> bool {
        self.at(s).is_none()
    }
    pub fn color_at(&self, s: Square) -> Option<Color> {
        self.at(s).map( |x| x.color() )
    }
    pub fn set_at_mut(&mut self, s: Square, val: Piece) {
        debug_assert!(self.is_empty_at(s), "set_at_mut(), s = {}", s);
        self.0.get_mut(&(val as uint)).unwrap().set_at_mut(s);
    }
    pub fn remove_at_mut(&mut self, s: Square) {
        debug_assert!(!self.is_empty_at(s), "remove_at_mut(), s = {}", s);
        let val = self.at(s).unwrap();
        self.0.get_mut(&(val as uint)).unwrap().remove_at_mut(s);
    }
    pub fn king_square(&self, c: Color) -> Square {
        let curr_king = Piece::new(c, King);
        let temp = self.iter().find( |x| x.0 == curr_king ).unwrap();
        temp.1
    }
    fn piece_vec(&self) -> Vec<(Piece,Square)> {
        let mut ans: Vec<(Piece, Square)> = Vec::new();
        for f in range::<u8>(0,8) {
            for r in range::<u8>(0,8) {
                let s = Square::new(File(f),Rank(r));
                let x = self.at(s);
                if x.is_some() {
                    ans.push((x.unwrap(), s));
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
