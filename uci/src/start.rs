use std::io::{Read, BufReader, Write};
use std::thread;
use std::sync::mpsc::sync_channel;

use types::{Cmd, Response};
use process::process;
use state::State;
use input::parse_input;
use output::format_output;

pub fn start<R, W>(input: R, output: W) where
R: Read + Send + 'static, W: Write + Send + 'static {
    let mut state = State::new();
    let (cmd_tx, cmd_rx) = sync_channel::<Cmd>(0);
    let temp = cmd_tx.clone();
    let _input_guard = thread::spawn(move || parse_input(BufReader::new(input), temp));
    let (resp_tx, resp_rx) = sync_channel::<Response>(0);
    let _output_guard = thread::spawn(move || format_output(output, resp_rx));
    for cmd in cmd_rx.iter() {
        debug!("cmd = {:?}", cmd);
        if cmd == Cmd::Quit {
            return;
        } else {
            process(&mut state, cmd, &resp_tx, &cmd_tx);
        }
        debug!("state.mode = {:?}", state.mode);
    }
}
