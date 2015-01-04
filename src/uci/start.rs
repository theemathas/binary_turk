use std::io::stdio::{StdinReader, StdWriter};
use std::io::LineBufferedWriter;
use std::thread::Thread;
use std::sync::mpsc::sync_channel;

use super::types::CmdVal;
use super::process::process;
use super::state::State;
use super::input::parse_input;

pub fn start(input: StdinReader, mut output: LineBufferedWriter<StdWriter>) {
    let mut state = State::new();
    let (cmd_tx, cmd_rx) = sync_channel::<CmdVal>(0);
    let _input_guard = Thread::spawn(move || parse_input(input, cmd_tx));
    for cmd in cmd_rx.iter() {
        if cmd == CmdVal::Quit {
            return;
        } else {
            process(&mut state, cmd, &mut output);
        }
    }
}
