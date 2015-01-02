#![feature(macro_rules)]

use std::io;

mod types;
mod state;
mod game;
mod eval;
mod search;
mod uci;

fn main() {
    uci::start(&mut io::stdin(), &mut io::stdout());
}
