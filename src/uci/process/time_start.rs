use std::sync::mpsc::channel;
use std::thread::Thread;

use timer;

use super::super::state::State;

pub fn time_start(state: &mut State) {
    let (time_tx, time_rx) = channel::<()>();
    let (time_kill_tx, time_kill_rx) = channel::<()>();
    let time_data = state.time_data.clone().unwrap();
    let c = state.search_state.as_ref().unwrap().pos.side_to_move();
    Thread::spawn(move || timer::start(time_data, c, time_tx, time_kill_rx));

    state.time_rx = Some(time_rx);
    state.time_kill_tx = Some(time_kill_tx);
}
