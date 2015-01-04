use std::sync::mpsc::{channel, Sender};
use std::thread::Thread;

use search;

use super::types::{Cmd, Response};
use super::state::{State, Mode};

mod go_param;

pub fn process(state: &mut State, cmd: Cmd, output: &Sender<Response>) {
    match cmd {
        Cmd::Debug(val) => {
            state.search_state.as_mut().map(|x| { x.is_debug = val; } );
            state.search_tx.as_mut().map(|tx| { let _ = tx.send(search::Cmd::Stop); } );
        },
        Cmd::IsReady => {
            // TODO implement IsReady
            unimplemented!();
        },
        Cmd::Register(ref param) => {
            // TODO register
            unimplemented!();
        },
        cmd => {
            match state.mode {
                Mode::Init => {
                    if cmd == Cmd::Uci {
                        // TODO print id/option/uciok
                        state.mode = Mode::Wait;
                        unimplemented!();
                    }
                },
                Mode::Wait => {
                    match cmd {
                        Cmd::SetOption(ref name, ref val_opt) => {
                            // TODO maybe initialize
                            // TODO set options
                            unimplemented!();
                        },
                        Cmd::UciNewGame => {
                            // TODO note that ucinewgame supported
                            // TODO reset game status
                            state.mode = Mode::NewGame;
                            unimplemented!();
                        },
                        cmd @Cmd::SetupPosition(..) => {
                            if state.ucinewgame_support {
                                // TODO set up position for same game
                                state.mode = Mode::Ready;
                                unimplemented!();
                            } else {
                                process(state, Cmd::UciNewGame, output);
                                state.ucinewgame_support = false;
                                process(state, cmd, output);
                            }
                        },
                        _ => {},
                    }
                },
                Mode::NewGame => {
                    if let Cmd::SetupPosition(ref pos, ref from_to_vec) = cmd {
                        // TODO setup position for new game
                        state.mode = Mode::Ready;
                        unimplemented!();
                    }
                },
                Mode::Ready => {
                    if let Cmd::Go(param) = cmd {
                        go_param::setup(state, param);
                        let (search_tx, search_rx) = channel::<search::Cmd>();
                        let search_state = state.search_state.as_ref().unwrap().clone();
                        let output = output.clone();
                        let temp = Thread::spawn(move ||
                                                 search::start(search_state, search_rx, output));
                        state.search_tx = Some(search_tx);
                        state.search_guard = Some(temp);
                        // TODO start timer
                        state.mode = Mode::Search;
                        unimplemented!();
                    }
                },
                Mode::Search => {
                    match cmd {
                        Cmd::PonderHit => {
                            // TODO ponder hit
                            unimplemented!();
                        },
                        Cmd::Stop => {
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
