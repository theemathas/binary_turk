#![feature(macro_rules)]

extern crate time;

use std::io;

mod types;
mod game;
mod eval;
mod search;
mod uci;

fn main() {
    uci::start(io::stdin(), io::stdout());
}
