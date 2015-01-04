use std::time::Duration;

use game::Color;
use types::NumMoves;

#[derive(Clone)]
pub enum TimeData {
    TimeLeft(TimeLeftData),
    ExactTime(Duration),
    Infinite,
}
impl TimeData {
    pub fn set_time(&mut self, c: Color, val: Option<Duration>) {
        self.force_time_left();
        match *self {
            TimeData::TimeLeft(ref mut x) => x.set_time(c, val),
            _ => unreachable!(),
        }
    }
    pub fn set_inc(&mut self, c: Color, val: Option<Duration>) {
        self.force_time_left();
        match *self {
            TimeData::TimeLeft(ref mut x) => x.set_inc(c, val),
            _ => unreachable!(),
        }
    }
    pub fn set_moves_to_go(&mut self, val: Option<NumMoves>) {
        self.force_time_left();
        match *self {
            TimeData::TimeLeft(ref mut x) => x.moves_to_go = val,
            _ => unreachable!(),
        }
    }
    fn force_time_left(&mut self) {
        match *self {
            TimeData::TimeLeft(_) => {},
            ref mut x => *x = TimeData::TimeLeft(TimeLeftData::new()),
        }
    }
}

#[derive(Clone)]
pub struct TimeLeftData {
    pub w_time: Option<Duration>,
    pub b_time: Option<Duration>,
    pub w_inc: Option<Duration>,
    pub b_inc: Option<Duration>,
    pub moves_to_go: Option<NumMoves>,
}
impl TimeLeftData {
    pub fn new() -> TimeLeftData {
        TimeLeftData {
            w_time: None,
            b_time: None,
            w_inc: None,
            b_inc: None,
            moves_to_go: None,
        }
    }
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
