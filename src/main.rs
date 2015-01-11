#![allow(unstable)]

extern crate time;
#[macro_use]
extern crate log;

use std::io;

mod types;
mod game;
mod eval;
mod search;
mod timer;
mod uci;

fn main() {
    uci::start(io::stdin(), io::stdout());
}
