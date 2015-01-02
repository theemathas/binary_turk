//! Represents the state of the engine.

#![allow(dead_code)]

use std::time::Duration;

use game::{Position, Color, Move, NumMoves, NumPlies};
use types::NumNodes;

pub use self::TimeData::{TimeLeft, ExactTime};

#[deriving(Clone)]
pub struct State {
    pub is_debug: bool,
    pub pos: Position,
    pub start_search: u64,
    pub search_param: SearchParam,
    pub time_data: TimeData,
}

#[deriving(Clone)]
pub struct SearchParam {
    pub ponder: Option<(Position, Move)>,
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
impl TimeLeftData {
    pub fn time(&self, c: Color) -> Duration {
        match c {
            Color::White => self.w_time,
            Color::Black => self.b_time,
        }
    }
    pub fn set_time(&mut self, c: Color, val: Duration) {
        match c {
            Color::White => self.w_time = val,
            Color::Black => self.b_time = val,
        }
    }
    pub fn inc(&self, c: Color) -> Duration {
        match c {
            Color::White => self.w_inc,
            Color::Black => self.b_inc,
        }
    }
    pub fn set_inc(&mut self, c: Color, val: Duration) {
        match c {
            Color::White => self.w_inc = val,
            Color::Black => self.b_inc = val,
        }
    }
}
