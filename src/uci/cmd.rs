use std::io::LineBufferedWriter;
use std::io::stdio::StdWriter;

#[deriving(PartialEq, Eq)]
pub enum Result {
    Continue,
    Quit,
}

pub fn cmd(s: String, output: &mut LineBufferedWriter<StdWriter>) -> Result {
    // TODO implement uci command processing
    unimplemented!()
}
