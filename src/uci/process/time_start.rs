use std::sync::mpsc::{sync_channel, SyncSender};

use super::super::state::State;
use super::super::types::Cmd;

pub fn time_start(state: &mut State, cmd_tx: SyncSender<Cmd>) {
    let (timer_kill_tx, timer_kill_rx) = sync_channel::<()>(0);
    let c = state.search_state.as_ref()
                 .expect("invalid search_state")
                 .pos.side_to_move();

    state.timer.clone().start(c, cmd_tx, timer_kill_rx);

    state.timer_kill_tx = Some(timer_kill_tx);
}
