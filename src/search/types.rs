#[deriving(PartialEq, Eq, Copy, Clone)]
pub enum SearchCmd {
    SetDebug(bool),
    PonderHit,
    Stop,
}
