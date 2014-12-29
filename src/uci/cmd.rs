use std::io::LineBufferedWriter;
use std::io::stdio::StdWriter;

use super::parse::parse;

#[deriving(PartialEq, Eq)]
pub enum Result {
    Continue,
    Quit,
}

pub fn cmd(s: String, output: &mut LineBufferedWriter<StdWriter>) -> Result {
    let val = parse(s);
    // TODO implement uci command processing
    unimplemented!()
}
