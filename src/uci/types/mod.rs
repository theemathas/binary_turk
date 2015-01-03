pub use self::param::{RegisterParam, GoParam, IdParam, InfoParam, ScoreType};

use std::time::Duration;

use types::NumMoves;
use game::{Position, Color, Move, FromTo};
use state::State;

use super::state::UciState;

pub mod options;
mod param;

#[deriving(PartialEq, Eq, Clone)]
pub enum CmdVal {
    Uci,
    Debug(bool),
    IsReady,
    SetOption(options::Name, options::Val),
    Register(Vec<RegisterParam>),
    UciNewGame,
    SetupPosition(Option<Position>, Vec<FromTo>),
    Go(Vec<GoParam>),
    Stop,
    PonderHit,
    Quit,
}

#[deriving(PartialEq, Eq, Clone)]
pub enum Response {
    Id(IdParam),
    UciOk,
    ReadyOk,
    BestMove(Move,Option<Move>),
    CopyProtectionIsOk(bool),
    RegistrationIsOk(bool),
    Info(Vec<InfoParam>),
}

// When starting program, start at state "Init"
// Always allow commands "debug" and "isready"
#[deriving(Clone)]
pub struct UciData {
    search_state: Option<State>,
    uci_state: UciState,
    start_search: Option<u64>,
    start_move: Option<u64>,
    time_data: TimeData,
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
