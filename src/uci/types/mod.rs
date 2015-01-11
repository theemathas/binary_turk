use std::fmt;

use game::{Position, Move, FromTo};

pub use self::param::{RegisterParam, GoParam, IdParam, InfoParam, ScoreType};
pub use self::param::ID_DATA;

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
impl fmt::String for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Response::Id(ref val) => write!(f, "id {}", val),
            Response::UciOk => write!(f, "uciok"),
            Response::ReadyOk => write!(f, "readyok"),
            Response::BestMove(ref best, ref ponder) => {
                try!(write!(f, "bestmove {}", best));
                if let Some(ref val) = ponder.as_ref() { try!(write!(f, " ponder {}", val)) }
                Ok(())
            },
            Response::CopyProtection(val) => write!(f, "copyprotection {}", val),
            Response::Registration(val) => write!(f, "registration {}", val),
            Response::Info(ref params) => {
                try!(write!(f, "info"));
                for x in params.iter() { try!(write!(f, " {}", x)) }
                Ok(())
            },
            Response::ShowOption(name, ref default, ref info) => {
                // TODO implement fmt::String for Response::ShowOption
                unimplemented!()
            },
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Show)]
pub enum VertifyingState {
    Checking,
    Ok,
    Error,
}
impl fmt::String for VertifyingState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            VertifyingState::Checking => "checking",
            VertifyingState::Ok       => "ok",
            VertifyingState::Error    => "error",
        })
    }
}
