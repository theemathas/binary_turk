use std::sync::mpsc::{sync_channel, SyncSender};
use std::thread::Thread;

use timer;

use super::super::state::State;
use super::super::types::Cmd;

pub fn time_start(state: &mut State, cmd_tx: SyncSender<Cmd>) {
    let (time_kill_tx, time_kill_rx) = sync_channel::<()>(0);
    let time_data = state.time_data.clone().unwrap();
    let c = state.search_state.as_ref().unwrap().pos.side_to_move();
    Thread::spawn(move || timer::start(time_data, c, cmd_tx, time_kill_rx));

    state.time_kill_tx = Some(time_kill_tx);
}
