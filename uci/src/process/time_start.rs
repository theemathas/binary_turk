use std::sync::mpsc::{sync_channel, SyncSender};
use std::thread::Thread;

use timer::TimeOut;
use state::State;
use types::Cmd;

pub fn time_start(state: &mut State, cmd_tx: SyncSender<Cmd>) {
    let c = state.search_state.as_ref()
                 .expect("invalid search_state")
                 .pos.side_to_move();
    let (timer_kill_tx, timer_kill_rx) = sync_channel::<()>(0);
    let (time_out_tx, time_out_rx) = sync_channel::<TimeOut>(0);

    state.timer.clone().start(c, time_out_tx, timer_kill_rx);
    //state.timer.clone().start(c, cmd_tx, timer_kill_rx);
    Thread::spawn(move || {
        let _ = time_out_rx.recv().unwrap();
        cmd_tx.send(Cmd::Stop).unwrap();
    });

    state.timer_kill_tx = Some(timer_kill_tx);
}
