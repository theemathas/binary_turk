#![feature(core, io, collections, std_misc, unicode)]

extern crate time;
#[macro_use]
extern crate log;

use std::old_io as io;

mod types;
mod game;
mod eval;
mod search;
mod timer;
mod uci;

fn main() {
    uci::start(io::stdin(), io::stdout());
}
