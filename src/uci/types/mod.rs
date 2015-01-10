use game::{Position, Move, FromTo};

pub use self::param::{RegisterParam, GoParam, IdParam, InfoParam, ScoreType};

pub mod options;
mod param;

#[derive(PartialEq, Eq, Clone, Show)]
pub enum Cmd {
    Uci,
    Debug(bool),
    IsReady,
    SetOption(options::Name, Option<options::Val>),
    Register(Vec<RegisterParam>),
    UciNewGame,
    SetupPosition(Position, Vec<FromTo>),
    Go(Vec<GoParam>),
    Stop,
    PonderHit,
    Quit,
}

#[derive(PartialEq, Eq, Clone, Show)]
pub enum Response {
    Id(IdParam),
    UciOk,
    ReadyOk,
    BestMove(Move,Option<Move>),
    CopyProtection(VertifyingState),
    Registration(VertifyingState),
    Info(Vec<InfoParam>),
    ShowOption(options::Name, options::Val, options::Info),
}

#[derive(PartialEq, Eq, Copy, Clone, Show)]
pub enum VertifyingState {
    Checking,
    Ok,
    Error,
}

