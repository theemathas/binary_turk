//! Implements a bitboard for a single piece.

use std::num::Int;

use square::Square;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct BitBoard(u64);
impl BitBoard {
    pub fn new() -> BitBoard { BitBoard(0_u64) }
    pub fn new_full() -> BitBoard { BitBoard(!0_u64) }

    pub fn at(self, s: Square) -> bool {
        self.0 & (1_u64 << s.to_id()) != 0
    }
    pub fn set_at(&mut self, s: Square) {
        debug_assert!(!self.at(s));
        self.0 |= 1_u64 << s.to_id();
    }
    pub fn remove_at(&mut self, s: Square) {
        debug_assert!(self.at(s));
        self.0 ^= 1_u64 << s.to_id();
    }

    pub fn iter(self) -> Iter {
        Iter(self.0)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Iter(u64);
impl Iterator for Iter {
    type Item = Square;
    fn next(&mut self) -> Option<Square> {
        if self.0 == 0 {
            None
        } else {
            let res = self.0.trailing_zeros();
            self.0 &= !(1 << res);
            Some(Square::from_id(res as i32))
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let res = self.0.count_ones();
        (res, Some(res))
    }
}
