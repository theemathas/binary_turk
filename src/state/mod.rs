//! Represents the state of the engine.

#![allow(dead_code)]


use game::{Position, Move, NumMoves, NumPlies};
use types::NumNodes;

#[deriving(Clone)]
pub struct State {
    pub is_debug: bool,
    pub pos: Position,
    pub search_param: SearchParam,
}

#[deriving(Clone)]
pub struct SearchParam {
    pub ponder: Option<(Position, Move)>,
    pub search_moves: Option<Vec<Move>>,
    pub depth: Option<NumPlies>,
    pub nodes: Option<NumNodes>,
    pub mate: Option<NumMoves>,
}
