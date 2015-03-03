use game::{Position, ZobristHash, NumPlies};

use negamax::Bound;

#[derive(Clone, Debug)]
pub struct Data {
    pub hash: ZobristHash,
    pub depth: NumPlies,
    pub bound: Bound,
}

pub struct TranspositionTable(Vec<Option<Data>>);
impl TranspositionTable {
    pub fn with_capacity(capacity: usize) -> Self {
        TranspositionTable(vec![None; capacity])
    }
    pub fn get<'a>(&'a self, pos: &Position) -> Option<&'a Data> {
        let hash = pos.hash();
        let idx = (hash.0 % (self.0.len() as u64)) as usize;
        self.0[idx].as_ref().and_then(|x| if x.hash == hash { Some(x) } else { None })
    }
    // TODO implement a better replacement scheme
    pub fn set(&mut self, pos: &Position, depth: NumPlies, bound: Bound) {
        let hash = pos.hash();
        let idx = (hash.0 % (self.0.len() as u64)) as usize;
        self.0[idx] = Some(Data { hash: hash, depth: depth, bound: bound });
    }
}
