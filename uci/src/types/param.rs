use std::time::Duration;
use std::fmt;

use game::{Move, FromTo, Color};
use types::{NumNodes, NumVariations, PerMill, NumCpu, NumPlies, NumMoves, Score};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum RegisterParam {
    Later,
    Name(String),
    Code(String),
}

#[derive(PartialEq, Eq, Clone, Debug)]
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

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum IdParam {
    Name(&'static str),
    Author(&'static str),
}
impl fmt::Display for IdParam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IdParam::Name(ref val)   => write!(f, "name {}", val),
            IdParam::Author(ref val) => write!(f, "author {}", val),
        }
    }
}

pub const ID_DATA: [IdParam; 2] = [IdParam::Name("chess_project"),
                                   IdParam::Author("Theemathas Chirananthavat")];

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum InfoParam {
    Depth(NumPlies),
    SelDepth(NumPlies),
    TimeSearched(Duration),
    NodesSearched(NumNodes),
    PrincipalVariation(Vec<Move>),
    MultiPv(NumVariations),
    Score(Option<ScoreType>, Score),
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
impl fmt::Display for InfoParam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InfoParam::Depth(val)          => write!(f, "depth {}", val.0),
            InfoParam::SelDepth(val)       => write!(f, "seldepth {}", val.0),
            InfoParam::TimeSearched(val)   => write!(f, "time {}", val.num_milliseconds()),
            InfoParam::NodesSearched(val)  => write!(f, "nodes {}", val.0),
            InfoParam::PrincipalVariation(ref moves) => {
                try!(write!(f, "pv"));
                for x in moves.iter() { try!(write!(f, " {}", x)) }
                Ok(())
            },
            InfoParam::MultiPv(val)        => write!(f, "multipv {}", val.0),
            InfoParam::Score(score_type, val) => {
                try!(write!(f, "score {}", val));
                if let Some(x) = score_type { try!(write!(f, " {}", x)) }
                Ok(())
            },
            InfoParam::CurrMove(ref val)   => write!(f, "currmove {}", val),
            InfoParam::CurrMoveNumber(val) => write!(f, "currmovenumber {}", val.0),
            InfoParam::HashFull(val)       => write!(f, "hashfull {}", val.0),
            InfoParam::NodesPerSec(val)    => write!(f, "nps {}", val),
            InfoParam::TablebaseHits(val)  => write!(f, "tbhits {}", val),
            InfoParam::ShredderTablebaseHits(val) => write!(f, "sbhits {}", val),
            InfoParam::CpuLoad(val)        => write!(f, "cpuload {}", val.0),
            InfoParam::ShowString(ref val) => write!(f, "string {}", val),
            InfoParam::Refutation(ref moves) => {
                try!(write!(f, "refutation"));
                for x in moves.iter() { try!(write!(f, " {}", x)) }
                Ok(())
            },
            InfoParam::CurrLine(num_cpu, ref moves) => {
                try!(write!(f, "currline"));
                if let Some(x) = num_cpu { try!(write!(f, " {}", x.0)) }
                for x in moves.iter() { try!(write!(f, " {}", x)) }
                Ok(())
            },
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum ScoreType {
    LowerBound,
    UpperBound,
}
impl fmt::Display for ScoreType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            ScoreType::LowerBound => "lowerbound",
            ScoreType::UpperBound => "upperbound",
        })
    }
}
