use game::{Position, Move, Score, NumPlies, NumMoves};

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct NumNodes(pub u64);


#[derive(Clone, Debug)]
pub struct State {
    pub pos: Position,
    pub prev_pos: Option<Position>,
    pub prev_move: Option<Move>,
    pub param: Param,
}

#[derive(Clone, Debug)]
pub struct Param {
    pub ponder: bool,
    pub search_moves: Option<Vec<Move>>,
    pub depth: Option<NumPlies>,
    pub nodes: Option<NumNodes>,
    pub mate: Option<NumMoves>,
    pub hash_size: usize,
}
impl Param {
    pub fn new(hash_size: usize) -> Self {
        Param {
            ponder: false,
            search_moves: None,
            depth: None,
            nodes: None,
            mate: None,
            hash_size: hash_size,
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Cmd {
    SetDebug(bool),
    PonderHit,
    Stop,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct BestMove(pub Move, pub Option<Move>);

#[derive(Clone, Debug)]
pub struct Report {
    pub data: Data,
    pub score: Score,
    pub pv: Vec<Move>,
}

#[derive(Clone, Debug)]
pub struct Data {
    pub nodes: NumNodes,
    pub depth: NumPlies,
}

// TODO put actual data here
#[derive(Clone, Debug)]
pub struct InnerData {
    pub nodes: NumNodes,
}
impl InnerData {
    pub fn one_node() -> InnerData { InnerData { nodes: NumNodes(1) } }
    pub fn combine(self, other: InnerData) -> InnerData {
        InnerData { nodes: NumNodes(self.nodes.0 + other.nodes.0) }
    }
    pub fn increment(self) -> InnerData {
        InnerData { nodes: NumNodes(self.nodes.0 + 1) }
    }
}
