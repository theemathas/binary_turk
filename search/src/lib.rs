#![feature(std_misc)]

#[macro_use]
extern crate log;

extern crate game;

pub use types::{NumNodes, State, Param, Cmd, Report, BestMove};
pub use start::start;

mod types;

mod start;

mod iterated_deepening;
mod depth_limited_search;
mod negamax;

mod transposition_table;
