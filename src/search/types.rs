use game::{Position, Move};
use types::{NumNodes, NumMoves, NumPlies};

#[derive(Clone, Show)]
pub struct State {
    pub is_debug: bool,
    pub pos: Position,
    pub prev_pos: Option<Position>,
    pub prev_move: Option<Move>,
    pub param: Param,
}

#[derive(Clone, Show)]
pub struct Param {
    pub ponder: bool,
    pub search_moves: Option<Vec<Move>>,
    pub depth: Option<NumPlies>,
    pub nodes: Option<NumNodes>,
    pub mate: Option<NumMoves>,
}

#[derive(PartialEq, Eq, Copy, Clone, Show)]
pub enum Cmd {
    SetDebug(bool),
    PonderHit,
    Stop,
}
