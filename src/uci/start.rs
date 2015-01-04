use std::io::stdio::{StdinReader, StdWriter};
use std::io::LineBufferedWriter;
use std::thread::Thread;
use std::sync::mpsc::sync_channel;

use super::types::{Cmd, Response};
use super::process::process;
use super::state::State;
use super::input::parse_input;
use super::output::format_output;

pub fn start(input: StdinReader, output: LineBufferedWriter<StdWriter>) {
    let mut state = State::new();
    let (cmd_tx, cmd_rx) = sync_channel::<Cmd>(0);
    let _input_guard = Thread::spawn(move || parse_input(input, cmd_tx));
    let (resp_tx, resp_rx) = sync_channel::<Response>(0);
    let _output_guard = Thread::spawn(move || format_output(output, resp_rx));
    for cmd in cmd_rx.iter() {
        if cmd == Cmd::Quit {
            return;
        } else {
            process(&mut state, cmd, &resp_tx);
        }
    }
}
