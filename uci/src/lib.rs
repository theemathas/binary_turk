#![feature(std_misc, old_io, collections, str_words)]

extern crate time;
#[macro_use]
extern crate log;

extern crate game;
extern crate search;
extern crate timer;

pub use self::start::start;
pub use self::types::{Cmd, Response, InfoParam};

mod types;
mod state;

mod start;
mod input;
mod output;
mod parse;
mod process;
