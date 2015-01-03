use std::time::Duration;

use game::{Move, FromTo, Color};
use eval;
use types::{NumNodes, NumVariations, PerMill, NumCpu, NumPlies, NumMoves};

use super::options;

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
    CurrMove(Move),
    CurrMoveNumber(NumMoves),
    HashFull(PerMill),
    NodesPerSec(u64),
    TablebaseHits(u64),
    ShredderTablebaseHits(u64),
    CpuLoad(PerMill),
    ShowString(String),
    Refutation(Vec<Move>),
    CurrLine(Option<NumCpu>, Vec<Move>),
}

#[deriving(PartialEq,Eq,Copy,Clone)]
pub enum ScoreType {
    LowerBound,
    UpperBound,
}
