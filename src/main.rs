#![feature(macro_rules)]

use std::io;

mod types;
mod game;
mod eval;
mod search;
mod uci;

fn main() {
    uci::start(io::stdin(), io::stdout());
}
