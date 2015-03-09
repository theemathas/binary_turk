use game::{Position, Move, ZobristHash, NumPlies};

use negamax::Bound;

#[derive(Clone, Debug)]
pub struct Data {
    pub hash: ZobristHash,
    pub depth: NumPlies,
    pub bound: Bound,
    pub best_move_opt: Option<Move>,
}

pub struct TranspositionTable(Vec<Option<Data>>);
impl TranspositionTable {
    pub fn with_capacity(capacity: usize) -> Self {
        TranspositionTable(vec![None; capacity])
    }
    pub fn get<'a>(&'a self, pos: &Position) -> Option<&'a Data> {
        let hash = pos.hash();
        let idx = (hash.0 % (self.0.len() as u64)) as usize;
        self.0[idx].as_ref().and_then(|x| {
            let is_correct_pos = x.hash == hash &&
                                 x.best_move_opt.as_ref()
                                  .map_or(true, |y| pos.legal_iter().any(|z| *y == z));
            if is_correct_pos { Some(x) } else { None }
        })
    }
    // TODO implement a better replacement scheme
    pub fn set(&mut self,
               pos: &Position,
               depth: NumPlies,
               best_move_opt: Option<Move>,
               bound: Bound) {
        let hash = pos.hash();
        let idx = (hash.0 % (self.0.len() as u64)) as usize;
        self.0[idx] = Some(Data {
            hash: hash,
            depth: depth,
            best_move_opt: best_move_opt,
            bound: bound
        });
    }
}
