use std::io::stdio::{StdinReader, StdWriter};
use std::io::LineBufferedWriter;

use super::cmd::{mod, cmd};

pub fn start(input: &mut StdinReader, output: &mut LineBufferedWriter<StdWriter>) {
    let mut inbuf = input.lock();
    for x in inbuf.lines() {
        let s = x.unwrap();
        let res = cmd(s, output);
        if res == cmd::Result::Quit {
            return;
        }
    }
}
