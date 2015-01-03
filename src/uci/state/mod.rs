use std::time::Duration;

use game::Color;
use types::NumMoves;
use search;

use self::mode::Mode;

pub mod mode;

#[deriving(Clone)]
pub struct State {
    search_state: Option<search::State>,
    mode: Mode,
    start_search_time: Option<u64>,
    start_move_time: Option<u64>,
    time_data: Option<TimeData>,
}
impl State {
    pub fn new() -> State {
        State {
            search_state: None,
            mode: Mode::new(),
            start_search_time: None,
            start_move_time: None,
            time_data: None,
        }
    }
}

#[deriving(Copy, Clone)]
pub enum TimeData {
    TimeLeft(TimeLeftData),
    ExactTime(Duration),
}

#[deriving(Copy, Clone)]
pub struct TimeLeftData {
    pub w_time: Option<Duration>,
    pub b_time: Option<Duration>,
    pub w_inc: Option<Duration>,
    pub b_inc: Option<Duration>,
    pub moves_to_go: Option<NumMoves>,
}
impl TimeLeftData {
    pub fn time(&self, c: Color) -> Option<Duration> {
        match c {
            Color::White => self.w_time,
            Color::Black => self.b_time,
        }
    }
    pub fn set_time(&mut self, c: Color, val: Option<Duration>) {
        match c {
            Color::White => self.w_time = val,
            Color::Black => self.b_time = val,
        }
    }
    pub fn inc(&self, c: Color) -> Option<Duration> {
        match c {
            Color::White => self.w_inc,
            Color::Black => self.b_inc,
        }
    }
    pub fn set_inc(&mut self, c: Color, val: Option<Duration>) {
        match c {
            Color::White => self.w_inc = val,
            Color::Black => self.b_inc = val,
        }
    }
}
