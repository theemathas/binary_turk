use game::{Position, Move, Color, NumPlies, NumMoves, MilliSec};
use eval;

pub mod options;

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

pub enum RegisterParam {
    Later,
    Name(String),
    Code(String),
}

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

pub struct NumNodes(pub u64);

pub enum Response {
    Id(IdParam),
    UciOk,
    ReadyOk,
    BestMove(Move,Option<Move>),
    CopyProtectionIsOk(bool),
    RegistrationIsOk(bool),
    Info(Vec<InfoParam>),
}

pub enum IdParam {
    Name(String),
    Author(String),
}

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

pub struct NumVariations(u16);

pub enum ScoreType {
    LowerBound,
    UpperBound,
}

pub struct PerMill(u16);

pub struct NumCpu(u16);
