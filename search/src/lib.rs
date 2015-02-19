#![feature(std_misc)]

#[macro_use]
extern crate log;

extern crate game;

pub use types::{State, Param, Cmd, Response};
pub use start::start;

mod types;

mod start;

mod depth_limited_search;
mod negamax;
