use game::{Position, Move, Color, NumPlies, NumMoves, MilliSec};

pub mod options;

pub enum CmdVal {
    Uci,
    Debug(bool),
    IsReady,
    SetOption(options::Val),
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
