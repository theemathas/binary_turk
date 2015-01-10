use time::precise_time_ns;

use std::sync::mpsc::{channel, Sender};
use std::thread::Thread;

use search;

use super::types::{Cmd, Response};
use super::state::{State, Mode};
use self::time_start::time_start;

mod go_param;
mod time_start;
mod pos;

pub fn process(state: &mut State, cmd: Cmd, output: &Sender<Response>) {
    match cmd {
        Cmd::Debug(val) => {
            state.is_debug = val;
            state.search_tx.as_ref().map(|tx| { let _ = tx.send(search::Cmd::Stop); } );
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
                        Cmd::SetupPosition(pos, from_to_vec) => {
                            if state.ucinewgame_support {
                                pos::setup_same(&mut state.search_state, pos, from_to_vec);
                                state.mode = Mode::Ready;
                            } else {
                                process(state, Cmd::UciNewGame, output);
                                state.ucinewgame_support = false;
                                process(state, Cmd::SetupPosition(pos, from_to_vec), output);
                            }
                        },
                        _ => {},
                    }
                },
                Mode::NewGame => {
                    if let Cmd::SetupPosition(pos, from_to_vec) = cmd {
                        pos::setup_new(&mut state.search_state, pos, from_to_vec);
                        state.mode = Mode::Ready;
                    }
                },
                Mode::Ready => {
                    assert!(state.search_state.is_some());
                    if let Cmd::Go(param) = cmd {
                        go_param::setup(state, param);
                        let (search_tx, search_rx) = channel::<search::Cmd>();
                        let search_state = state.search_state.as_ref().unwrap().clone();
                        let output = output.clone();
                        let temp = Thread::scoped(move ||
                                                  search::start(search_state, search_rx, output));

                        state.search_tx = Some(search_tx);
                        state.search_guard = Some(temp);

                        if !state.search_state.as_ref().unwrap().param.ponder {
                            time_start(state);
                        }

                        state.mode = Mode::Search;
                    }
                },
                Mode::Search => {
                    assert!(state.search_state.is_some());
                    assert!(state.search_guard.is_some());
                    assert!(state.search_tx.is_some());
                    match cmd {
                        Cmd::PonderHit => {
                            if !state.search_state.as_ref().unwrap().param.ponder {
                                return;
                            }
                            let _ = state.search_tx.as_ref().unwrap().send(search::Cmd::PonderHit);
                            state.search_state.as_mut().unwrap().param.ponder = false;
                            state.start_move_time = Some(precise_time_ns());
                            time_start(state);
                        },
                        Cmd::Stop => {
                            let _ = state.search_tx.as_ref().unwrap().send(search::Cmd::Stop);
                            let _ = state.search_guard.take().unwrap().join();
                            state.search_tx = None;
                            state.start_search_time = None;
                            state.start_move_time = None;
                            state.time_data = None;
                            if let Some(time_kill_tx) = state.time_kill_tx.take() {
                                let _ = time_kill_tx.send(());
                            }
                            state.time_rx = None;
                            state.mode = Mode::Wait;
                            // TODO what to do about search_state
                            unimplemented!();
                        },
                        _ => {},
                    }
                },
            }
        },
    }
}
