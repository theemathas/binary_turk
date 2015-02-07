#![feature(core, io, collections, std_misc, unicode)]

extern crate time;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::old_io as io;

mod types;
mod game;
mod search;
mod timer;
mod uci;

fn main() {
    env_logger::init().unwrap();
    uci::start(io::stdin(), io::stdout());
}
