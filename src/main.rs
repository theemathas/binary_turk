#![feature(macro_rules)]

use std::io;

mod game;
mod eval;
mod uci;

fn main() {
    uci::start(&mut io::stdin(), &mut io::stdout());
}
