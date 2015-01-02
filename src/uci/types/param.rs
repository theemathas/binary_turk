use std::time::Duration;

use game::{Move, FromTo, Color, NumPlies, NumMoves};
use eval;
use types::{NumNodes, NumVariations, PerMill, NumCpu};

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
