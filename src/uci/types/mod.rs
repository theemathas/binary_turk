pub use self::param::{RegisterParam, GoParam, IdParam, InfoParam, ScoreType};

use game::{Position, Move, FromTo};

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

