use std::time::Duration;

use game::{Position, Move, FromTo, Color, NumPlies, NumMoves};
use eval;
use types::{NumNodes, NumVariations, PerMill, NumCpu};

pub mod options;

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
pub enum RegisterParam {
    Later,
    Name(String),
    Code(String),
}

#[deriving(PartialEq, Eq, Clone)]
pub enum GoParam {
    SearchMoves(Vec<FromTo>),
    Ponder,
    Time(Color, Duration),
    IncTime(Color, Duration),
    MovesToGo(NumMoves),
    Depth(NumPlies),
    Nodes(NumNodes),
    Mate(NumMoves),
    MoveTime(Duration),
    Infinite,
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

#[deriving(PartialEq, Eq, Clone)]
pub enum IdParam {
    Name(String),
    Author(String),
}

#[deriving(PartialEq, Eq, Clone)]
pub enum InfoParam {
    Depth(NumPlies),
    SelDepth(NumPlies),
    TimeSearched(Duration),
    NodesSearched(NumNodes),
    PrincipalVariation(Vec<Move>),
    MultiPv(NumVariations),
    Score(Option<ScoreType>, eval::Result),
    CurrMoveNumber(NumMoves),
    HashFull(PerMill),
    ShowString(String),
    Refutation(Vec<Move>),
    CurrLine(Option<NumCpu>, Vec<Move>),
    ShowOption(options::Name, options::Info),
}

#[deriving(PartialEq,Eq,Copy,Clone)]
pub enum ScoreType {
    LowerBound,
    UpperBound,
}
