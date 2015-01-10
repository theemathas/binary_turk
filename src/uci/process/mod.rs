use time::precise_time_ns;

use std::sync::mpsc::{channel, Sender};
use std::thread::Thread;

use search;
use timer;

use super::types::{Cmd, Response};
use super::state::{State, Mode};

mod go_param;

pub fn process(state: &mut State, cmd: Cmd, output: &Sender<Response>) {
    match cmd {
        Cmd::Debug(val) => {
            state.is_debug = val;
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
                    assert!(state.search_state.is_some());
                    if let Cmd::Go(param) = cmd {
                        go_param::setup(state, param);
                        let (search_tx, search_rx) = channel::<search::Cmd>();
                        let search_state = state.search_state.as_ref().unwrap().clone();
                        let output = output.clone();
                        let temp = Thread::scoped(move ||
                                                  search::start(search_state, search_rx, output));

                        state.search_tx = Some(search_tx.clone());
                        state.search_guard = Some(temp);

                        let time_data = state.time_data.clone().unwrap();
                        let c = state.search_state.as_ref().unwrap().pos.side_to_move();

                        Thread::spawn(move || timer::start(time_data, c, search_tx));

                        state.mode = Mode::Search;
                    }
                },
                Mode::Search => {
                    assert!(state.search_state.is_some());
                    assert!(state.search_guard.is_some());
                    assert!(state.search_tx.is_some());
                    match cmd {
                        Cmd::PonderHit => {
                            state.search_tx.as_ref().map(|ref tx| tx.send(search::Cmd::PonderHit));
                            state.search_state.as_mut().map(|ref mut x| x.param.ponder = false);
                            state.start_move_time = Some(precise_time_ns());
                        },
                        Cmd::Stop => {
                            state.search_tx.as_mut().map(|ref tx| tx.send(search::Cmd::Stop));
                            state.search_guard.take().map(|x| x.join());
                            state.search_tx = None;
                            state.mode = Mode::Wait;
                            state.start_search_time = None;
                            state.start_move_time = None;
                            state.time_data = None;
                            // TODO what to do about search_state
                            // TODO what to do about the timer thread
                            // TODO what if time is out
                            unimplemented!();
                        },
                        _ => {},
                    }
                },
            }
        },
    }
}
