use std::io::stdio::{StdinReader, StdWriter};
use std::io::LineBufferedWriter;

use super::types::CmdVal;
use super::parse::parse;
use super::process::process;
use super::state::State;

pub fn start(input: &mut StdinReader, output: &mut LineBufferedWriter<StdWriter>) {
    let mut state = State::new();
    let mut inbuf = input.lock();
    for x in inbuf.lines() {
        let s = x.unwrap();
        if let Some(curr_cmd) =  parse(&*s) {
            if curr_cmd == CmdVal::Quit {
                return;
            } else {
                process(&mut state, &curr_cmd, output);
            }
        }
    }
}
