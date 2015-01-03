use game::{Position, Move};
use types::{NumNodes, NumMoves, NumPlies};

#[deriving(Clone)]
pub struct State {
    pub is_debug: bool,
    pub pos: Position,
    pub param: Param,
}

#[deriving(Clone)]
pub struct Param {
    pub ponder: Option<(Position, Move)>,
    pub search_moves: Option<Vec<Move>>,
    pub depth: Option<NumPlies>,
    pub nodes: Option<NumNodes>,
    pub mate: Option<NumMoves>,
}

#[deriving(PartialEq, Eq, Copy, Clone)]
pub enum Cmd {
    SetDebug(bool),
    PonderHit,
    Stop,
}
