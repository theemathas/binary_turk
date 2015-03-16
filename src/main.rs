extern crate env_logger;

extern crate uci;

use std::io::{stdin, stdout};

fn main() {
    env_logger::init().unwrap();
    //uci::start(stdin().lock(), stdout().lock());
    uci::start(stdin(), stdout());
}
