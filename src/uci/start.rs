use std::io::stdio::{StdinReader, StdWriter};
use std::io::LineBufferedWriter;

use super::command::{mod, command};

pub fn start(input: &mut StdinReader, output: &mut LineBufferedWriter<StdWriter>) {
    let mut inbuf = input.lock();
    for x in inbuf.lines() {
        let s = x.unwrap();
        let res = command(s.as_slice(), output);
        if res == command::Result::Quit {
            return;
        }
    }
}
