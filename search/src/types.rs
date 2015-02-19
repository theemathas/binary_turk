extern crate types;

pub use self::types::*;

use game::{Position, Move};

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
}
impl Param {
    pub fn new() -> Self {
        Param {
            ponder: false,
            search_moves: None,
            depth: None,
            nodes: None,
            mate: None,
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
pub enum Response {
    BestMove(Move, Option<Move>),
    Report(NumPlies, NumNodes, Score, Vec<Move>),
}

// TODO put actual data here
#[derive(Debug)]
pub struct Data {
    pub nodes: NumNodes,
}
impl Data {
    pub fn one_node() -> Data { Data { nodes: NumNodes(1) } }
    pub fn combine(self, other: Data) -> Data {
        Data { nodes: NumNodes(self.nodes.0 + other.nodes.0) }
    }
    pub fn increment(self) -> Data {
        Data { nodes: NumNodes(self.nodes.0 + 1) }
    }
}
