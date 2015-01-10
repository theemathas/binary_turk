use game::{Position, Move};
use types::{NumNodes, NumMoves, NumPlies};

#[derive(Clone, Show)]
pub struct State {
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

#[derive(PartialEq, Eq, Copy, Clone, Show)]
pub enum Cmd {
    SetDebug(bool),
    PonderHit,
    Stop,
}
