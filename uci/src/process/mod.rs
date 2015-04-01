use time::precise_time_ns;

use std::sync::mpsc::{sync_channel, channel, SyncSender};
use std::thread;

use search;
use timer::Timer;
use types::{Cmd, Response, ID_DATA};
use state::{State, Mode};
use output::engine_response_output;
use types::options;

use self::time_start::time_start;

mod go_param;
mod time_start;
mod pos;

pub fn process(state: &mut State,
               cmd: Cmd,
               output: &SyncSender<Response>,
               cmd_tx: &SyncSender<Cmd>) {
    match cmd {
        Cmd::Debug(val) => {
            // TODO set debug
            debug!("debug is now {:?}", val);
            state.search_tx.as_ref().map(|tx| {
                tx.send(search::Cmd::SetDebug(val))
                  .ok().expect("state.search_tx closed") } );
        },
        Cmd::IsReady => {
            output.send(Response::ReadyOk)
                  .ok().expect("output channel closed");
        },
        Cmd::Register(..) => {
            // TODO register
            unimplemented!();
        },
        cmd => {
            match state.mode {
                Mode::Init => {
                    if cmd == Cmd::Uci {
                        for x in ID_DATA.iter() {
                            output.send(Response::Id(x.clone()))
                                  .ok().expect("output channel closed");
                        }
                        for x in &options::INFO {
                            output.send(Response::ShowOption(x.clone()))
                                  .ok().expect("output channel closed");
                        }
                        // TODO print option list
                        output.send(Response::UciOk)
                              .ok().expect("output channel closed");
                        state.mode = Mode::Wait;
                    }
                },
                Mode::Wait => {
                    match cmd {
                        Cmd::SetOption(val) => {
                            // TODO maybe initialize
                            state.options.set_value(val);
                        },
                        Cmd::UciNewGame => {
                            state.reset_new_game()
                        },
                        Cmd::SetupPosition(pos, from_to_vec) => {
                            if state.ucinewgame_support {
                                pos::setup_same(state, pos, from_to_vec);
                                state.mode = Mode::Ready;
                            } else {
                                process(state, Cmd::UciNewGame, output, cmd_tx);
                                state.ucinewgame_support = false;
                                process(state, Cmd::SetupPosition(pos, from_to_vec),
                                        output, cmd_tx);
                            }
                        },
                        _ => {},
                    }
                },
                Mode::NewGame => {
                    if let Cmd::SetupPosition(pos, from_to_vec) = cmd {
                        pos::setup_new(state, pos, from_to_vec);
                        state.mode = Mode::Ready;
                    }
                },
                Mode::Ready => {
                    assert!(state.search_state.is_some());
                    if let Cmd::Go(param) = cmd {
                        go_param::setup(state, param);
                        let (search_tx, search_rx) = sync_channel::<search::Cmd>(0);
                        let (response_tx, response_rx) = channel::<search::Response>();
                        let search_state = state.search_state.as_ref().unwrap().clone();
                        let output = output.clone();
                        let temp = thread::scoped(move ||
                            search::start(search_state, search_rx, response_tx));
                        thread::spawn(move || engine_response_output(response_rx, output));

                        state.search_tx = Some(search_tx);
                        state.search_guard = Some(temp);

                        if !state.search_state.as_ref().unwrap().param.ponder {
                            time_start(state, cmd_tx.clone());
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
                            state.search_tx.as_ref().unwrap().send(search::Cmd::PonderHit)
                                 .ok().expect("state.search_tx was closed");
                            state.search_state.as_mut().unwrap().param.ponder = false;
                            state.start_move_time = Some(precise_time_ns());
                            time_start(state, cmd_tx.clone());
                        },
                        Cmd::Stop => {
                            state.search_tx.as_ref().unwrap().send(search::Cmd::Stop)
                                 .ok().expect("state.search_tx was closed");
                            state.search_guard.take().unwrap().join();
                            state.search_tx = None;
                            state.start_search_time = None;
                            state.start_move_time = None;
                            state.timer = Timer::new();
                            state.mode = Mode::Wait;
                        },
                        _ => {},
                    }
                },
            }
        },
    }
}
