#![feature(io, collections, std_misc)]

extern crate time;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate types;
extern crate game;
extern crate search;

use std::old_io as io;

mod timer;
mod uci;

fn main() {
    env_logger::init().unwrap();
    uci::start(io::stdin(), io::stdout());
}
