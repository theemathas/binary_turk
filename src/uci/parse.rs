use super::types::CmdVal;

pub fn parse(s: String) -> CmdVal {
    fn split_fn(ch: char) -> bool { ch.is_whitespace() };
    let words = s.split( split_fn );
    // TODO implement command parsing
    unimplemented!()
}
