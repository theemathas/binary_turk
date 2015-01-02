//! Represents the state of the engine.

#![allow(dead_code)]

use std::time::Duration;

use game::{Position, Color, Move, NumMoves, NumPlies};
use types::NumNodes;

pub use self::TimeData::{TimeLeft, ExactTime};

pub struct State {
    pub is_debug: bool,
    pub my_side: Color,
    pub pos: Position,
    pub start_search: u64,
    pub search_param: SearchParam,
    pub time_data: TimeData,
}
impl State {
    pub fn is_my_turn(&self) -> bool {
        self.my_side == self.pos.side_to_move()
    }
}

pub struct SearchParam {
    pub search_moves: Option<Vec<Move>>,
    pub depth: Option<NumPlies>,
    pub nodes: Option<NumNodes>,
    pub mate: Option<NumMoves>,
}

#[deriving(Copy, Clone)]
pub enum TimeData {
    TimeLeft(TimeLeftData),
    ExactTime(Duration),
}

#[deriving(Copy, Clone)]
pub struct TimeLeftData {
    pub w_time: Duration,
    pub b_time: Duration,
    pub w_inc: Duration,
    pub b_inc: Duration,
    pub moves_to_go: NumMoves,
}
