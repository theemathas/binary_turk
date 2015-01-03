#[deriving(PartialEq, Eq, Copy, Clone)]
pub enum UciState {
    // * `uci`
    // send all "id" and "option"  messages
    // send one "uciok" message
    // go to state "Wait"
    Init,
    // * `setoption`
    // maybe initialize stuff
    // set the option
    // stay in state Wait
    // * `ucinewgame`
    // reset status
    // take note that GUI supports the ucinewgame command
    // go to NewGame state
    // * `position`
    // if GUI does not support ucinewgame, then:
    //     simulate `ucinewgame` command
    //     reprocess `position` command
    // set up position for same game
    // go to Ready state
    Wait,
    // * `position`
    // set up position for new game
    // go to Ready state
    NewGame,
    // * `go`
    // process the arguments
    // start searching
    // go to state Search
    Ready,
    // * `ponderhit`
    // Execute the ponder move
    // stay in state Search
    // * `stop`
    // * OR
    // * engine decides on best move
    // stop searching
    // send all "info" data/messages
    // send one "bestmove" message
    // stop search
    // go to state Wait
    // * engine runs for a while
    // send all "info" data/messages
    Search,
    // For any state:
    //
    // * `debug`
    // set debug as on or off
    // * `isready`
    // if state is not Search, then: wait for everything to finish
    // send one "readyok" message
}
