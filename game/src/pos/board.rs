//! implements the board representation

use std::vec;
use std::collections::VecMap;

use color::{Color, White, Black};
use piece::{self, Piece, King};
use square::Square;

use super::bitboard::BitBoard;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Board {
    data: VecMap<BitBoard>,
    white_data: BitBoard,
    black_data: BitBoard,
    empty_data: BitBoard,
}

impl Board {
    pub fn new() -> Board {
        let mut ans = VecMap::new();
        for x in piece::ALL.iter() {
            ans.insert(*x as usize, BitBoard::new());
        }
        Board {
            data: ans,
            white_data: BitBoard::new(),
            black_data: BitBoard::new(),
            empty_data: BitBoard::new_full(),
        }
    }

    fn piece_data(&self, p: Piece) -> &BitBoard {
        self.data.get(&(p as usize)).unwrap()
    }
    fn piece_data_mut(&mut self, p: Piece) -> &mut BitBoard {
        self.data.get_mut(&(p as usize)).unwrap()
    }

    fn color_data(&self, c: Color) -> &BitBoard {
        match c {
            White => &self.white_data,
            Black => &self.black_data,
        }
    }
    fn color_data_mut(&mut self, c: Color) -> &mut BitBoard {
        match c {
            White => &mut self.white_data,
            Black => &mut self.black_data,
        }
    }

    pub fn at(&self, s: Square) -> Option<Piece> {
        for x in piece::ALL.iter() {
            if self.is_piece_at(*x, s) {
                return Some(*x);
            }
        }
        None
    }
    pub fn is_piece_at(&self, p: Piece, s: Square) -> bool { self.piece_data(p).at(s) }
    pub fn is_empty_at(&self, s: Square) -> bool { self.empty_data.at(s) }
    pub fn is_color_at(&self, s: Square, c: Color) -> bool { self.color_data(c).at(s) }

    pub fn set_at(&mut self, s: Square, p: Piece) {
        debug_assert!(self.is_empty_at(s), "set_at(), s = {:?}, p = {:?}", s, p);
        self.piece_data_mut(p).set_at(s);
        self.color_data_mut(p.color()).set_at(s);
        self.empty_data.remove_at(s);
    }
    pub fn remove_at(&mut self, s: Square, p: Piece) {
        debug_assert!(self.is_piece_at(p, s), "remove_at(), s = {:?}, p = {:?}", s, p);
        self.piece_data_mut(p).remove_at(s);
        self.color_data_mut(p.color()).remove_at(s);
        self.empty_data.set_at(s);
    }

    pub fn king_square(&self, c: Color) -> Square {
        let curr_king = Piece::new(c, King);
        self.piece_data(curr_king).iter().next().unwrap()
    }

    fn piece_vec(&self) -> Vec<(Piece, Square)> {
        let mut ans: Vec<(Piece, Square)> = Vec::new();
        for p in piece::ALL.iter() {
            ans.extend(self.piece_data(*p).iter().map( |s: Square| (*p, s)  ) );
        }
        ans
    }
    pub fn iter(&self) -> Iter { Iter(self.piece_vec().into_iter()) }
}

pub struct Iter(vec::IntoIter<(Piece, Square)>);
impl Iterator for Iter {
    type Item = (Piece, Square);
    fn next(&mut self) -> Option<(Piece, Square)> { self.0.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.0.size_hint() }
}
impl DoubleEndedIterator for Iter {
    fn next_back(&mut self) -> Option<(Piece, Square)> { self.0.next_back() }
}
impl ExactSizeIterator for Iter {}