use game::{Position, Move, Color, NumPlies, NumMoves, MilliSec};
use eval;

pub mod options;

#[deriving(PartialEq, Eq, Clone)]
pub enum CmdVal {
    Uci,
    Debug(bool),
    IsReady,
    SetOption(options::Name, options::Val),
    Register(Vec<RegisterParam>),
    UciNewGame,
    SetupPosition(Option<Position>, Vec<Move>),
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
    SearchMoves(Vec<Move>),
    Ponder,
    Time(Color, MilliSec),
    IncTime(Color, MilliSec),
    MovesToGo(NumMoves),
    Depth(NumPlies),
    Nodes(NumNodes),
    Mate(NumMoves),
    MoveTime(MilliSec),
    Infinite,
}

#[deriving(PartialEq,Eq,Copy,Clone)]
pub struct NumNodes(pub u64);

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
    TimeSearched(MilliSec),
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
pub struct NumVariations(u16);

#[deriving(PartialEq,Eq,Copy,Clone)]
pub enum ScoreType {
    LowerBound,
    UpperBound,
}

#[deriving(PartialEq,Eq,Copy,Clone)]
pub struct PerMill(u16);

#[deriving(PartialEq,Eq,Copy,Clone)]
pub struct NumCpu(u16);
