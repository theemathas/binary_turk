use super::types::CmdVal;

pub fn parse(s: &str) -> Option<CmdVal> {
    fn split_fn(ch: char) -> bool { ch.is_whitespace() };
    let words = s.split( split_fn );

    // TODO implement command parsing
    unimplemented!()
}
