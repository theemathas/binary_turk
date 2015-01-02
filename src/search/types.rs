use uci::types::options;

#[deriving(PartialEq, Eq, Clone)]
pub enum SearchCmd {
    SetDebug(bool),
    SetOption(options::Name, options::Val),
    PonderHit,
    Stop,
}
