//! Types that do not fit in any module.

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Sub, Neg};

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct NumNodes(pub u64);

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct NumVariations(pub u32);

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct PerMill(pub u32);

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct NumCpu(pub u32);

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct NumPlies(pub u32);

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct NumMoves(pub u32);

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct ScoreUnit(pub i32);
impl Add for ScoreUnit {
    type Output = Self;
    fn add(self, rhs: Self) -> Self { ScoreUnit(self.0 + rhs.0) }
}
impl Sub for ScoreUnit {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self { ScoreUnit(self.0 - rhs.0) }
}
impl Neg for ScoreUnit {
    type Output = Self;
    fn neg(self) -> Self { ScoreUnit(-self.0) }
}

const CENTIPAWNS_PER_UNIT: i32 = 1;

/// An assessment of the position.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Score {
    // Positive: advantage for side to move.
    // Negative: disadvantage for side to move.
    Value(ScoreUnit),
    // Side to move can checkmate in x moves.
    // WinIn(NumMoves(1)): can checkmate now.
    // WinIn(NumMoves(2)): can checkmate next move.
    WinIn(NumMoves),
    // Side to move will be checkmated in x moves.
    // WinIn(NumMoves(0)): already checkmated.
    // WinIn(NumMoves(1)): Will be immediately checkmated after any move.
    LoseIn(NumMoves),
}
impl Score {
    pub fn increment(self) -> Score {
        match self {
            Score::Value(val) => Score::Value(-val),
            Score::WinIn(val) => Score::LoseIn(val),
            Score::LoseIn(val) => Score::WinIn(NumMoves(val.0+1)),
        }
    }
}
impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Score::Value(val)   => write!(f, "cp {}", val.0 * CENTIPAWNS_PER_UNIT),
            Score::WinIn(val)   => write!(f, "mate {}", val.0 as i32),
            Score::LoseIn(val)  => write!(f, "mate {}", (val.0 as i32) * -1),
        }
    }
}
impl Ord for Score {
    fn cmp(&self, other: &Score) -> Ordering {
        match *self {
            Score::WinIn(val1) => match *other {
                Score::WinIn(val2) => val2.cmp(&val1),
                _ => Ordering::Greater,
            },
            Score::LoseIn(val1) => match *other {
                Score::LoseIn(val2) => val1.cmp(&val2),
                _ => Ordering::Less,
            },
            Score::Value(val1) => match *other {
                Score::WinIn(_) => Ordering::Less,
                Score::LoseIn(_) => Ordering::Greater,
                Score::Value(val2) => val1.cmp(&val2),
            },
        }
    }
}
impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
