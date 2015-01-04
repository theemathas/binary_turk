use std::io::LineBufferedWriter;
use std::io::stdio::StdWriter;

use search::Cmd;

use super::types::CmdVal;
use super::state::{State, Mode};

pub fn process(state: &mut State, cmd: &CmdVal, output: &mut LineBufferedWriter<StdWriter>) {
    match *cmd {
        CmdVal::Debug(val) => {
            state.search_state.as_mut().map(|x| { x.is_debug = val; } );
            state.search_chan.as_mut().map(|tx| { let _ = tx.send(Cmd::Stop); } );
        },
        CmdVal::IsReady => {
            // TODO implement IsReady
            unimplemented!();
        },
        CmdVal::Register(ref param) => {
            // TODO register
            unimplemented!();
        },
        ref cmd => {
            match state.mode {
                Mode::Init => {
                    if *cmd == CmdVal::Uci {
                        // TODO print id/option/uciok
                        state.mode = Mode::Wait;
                        unimplemented!();
                    }
                },
                Mode::Wait => {
                    match *cmd {
                        CmdVal::SetOption(ref name, ref val_opt) => {
                            // TODO maybe initialize
                            // TODO set options
                            unimplemented!();
                        },
                        CmdVal::UciNewGame => {
                            // TODO note that ucinewgame supported
                            // TODO reset game status
                            state.mode = Mode::NewGame;
                            unimplemented!();
                        },
                        ref cmd @CmdVal::SetupPosition(..) => {
                            if state.ucinewgame_support {
                                // TODO set up position for same game
                                state.mode = Mode::Ready;
                                unimplemented!();
                            } else {
                                process(state, &CmdVal::UciNewGame, output);
                                state.ucinewgame_support = false;
                                process(state, cmd, output);
                            }
                        },
                        _ => {},
                    }
                },
                Mode::NewGame => {
                    if let CmdVal::SetupPosition(ref pos, ref from_to_vec) = *cmd {
                        // TODO setup position for new game
                        state.mode = Mode::Ready;
                        unimplemented!();
                    }
                },
                Mode::Ready => {
                    if let CmdVal::Go(ref param) = *cmd {
                        // TODO start searching
                        state.mode = Mode::Search;
                        unimplemented!();
                    }
                },
                Mode::Search => {
                    match *cmd {
                        CmdVal::PonderHit => {
                            // TODO ponder hit
                            unimplemented!();
                        },
                        CmdVal::Stop => {
                            // TODO stop search
                            // Also if time is out
                            unimplemented!();
                        },
                        _ => {},
                    }
                },
            }
        },
    }
}
