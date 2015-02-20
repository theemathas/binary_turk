#![feature(old_io)]

extern crate env_logger;

extern crate uci;

use std::old_io as io;

fn main() {
    env_logger::init().unwrap();
    uci::start(io::stdin(), io::stdout());
}
