//! Implements a bitboard for a single piece.

use std::collections::bitv_set::{self, BitvSet};
use std::iter::Map;

use super::super::square::Square;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct BitBoard(BitvSet);
impl BitBoard {
    pub fn new() -> BitBoard { BitBoard(BitvSet::with_capacity(64)) }
    pub fn new_full() -> BitBoard {
        let mut ans = BitvSet::with_capacity(64);
        for i in 0..64_us {
            ans.insert(i);
        }
        BitBoard(ans)
    }

    pub fn at(&self, s: Square) -> bool { self.0.contains(&(s.to_id() as usize)) }
    pub fn set_at(&mut self, s: Square) { self.0.insert(s.to_id() as usize); }
    pub fn remove_at(&mut self, s: Square) { self.0.remove(&(s.to_id() as usize)); }

    pub fn iter(&self) -> Map< usize, Square, bitv_set::Iter, fn(usize)->Square > {
        fn map_fn(x: usize) -> Square { Square::from_id(x as i32) };
        self.0.iter().map( map_fn as fn(usize)->Square )
    }
}
