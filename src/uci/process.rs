use std::io::LineBufferedWriter;
use std::io::stdio::StdWriter;

use super::types::CmdVal;
use super::state::State;

pub fn process(state: &mut State, curr_cmd: &CmdVal, output: &mut LineBufferedWriter<StdWriter>) {
    // TODO implement uci command processing
    unimplemented!()
}
