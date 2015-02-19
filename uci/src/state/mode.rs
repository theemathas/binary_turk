// When starting program, start at state "Init"
// Always allow commands "debug" and "isready"
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Mode {
    // * `uci`
    // send all "id" and "option"  messages
    // send one "uciok" message
    // go to mode "Wait"
    Init,
    // * `setoption`
    // maybe initialize stuff
    // set the option
    // stay in mode Wait
    // * `ucinewgame`
    // reset status
    // take note that GUI supports the ucinewgame command
    // go to NewGame mode
    // * `position`
    // if GUI does not support ucinewgame, then:
    //     simulate `ucinewgame` command
    //     reprocess `position` command
    // set up position for same game
    // go to Ready mode
    Wait,
    // * `position`
    // set up position for new game
    // go to Ready mode
    NewGame,
    // * `go`
    // process the arguments
    // start searching
    // go to mode Search
    Ready,
    // * `ponderhit`
    // Execute the ponder move
    // stay in mode Search
    // * `stop`
    // * OR
    // * engine decides on best move
    // stop searching
    // send all "info" data/messages
    // send one "bestmove" message
    // stop search
    // go to mode Wait
    // * engine runs for a while
    // send all "info" data/messages
    Search,
    // For any mode:
    //
    // * `debug`
    // set debug as on or off
    // * `isready`
    // if mode is not Search, then: wait for everything to finish
    // send one "readyok" message
}
impl Mode {
    pub fn new() -> Mode {
        Mode::Init
    }
}
