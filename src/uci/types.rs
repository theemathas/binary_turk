use game::{Position, Move, Color, Plies, MilliSec};

pub enum CmdVal {
    Uci,
    Debug(bool),
    IsReady,
    SetOption(OptionName),
    Register(Vec<RegisterParam>),
    UciNewGame,
    SetupPosition(Position, Vec<Move>),
    Go(Vec<GoParam>),
    Stop,
    PonderHit,
    Quit,
}

pub enum OptionName {
    // TODO enumerate options.
    Dummy,
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
    MovesToGo(Plies),
    Depth(Plies),
    Nodes(Plies),
    Mate(Plies),
    MoveTime(MilliSec),
    Infinite,
}
