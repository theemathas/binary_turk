use std::sync::mpsc::{sync_channel, SyncSender};
use std::thread;

use timer::TimeOut;
use state::State;
use types::Cmd;

pub fn time_start(state: &mut State, cmd_tx: SyncSender<Cmd>) {
    let c = state.search_state.as_ref()
                 .expect("invalid search_state")
                 .pos.side_to_move();
    let (time_out_tx, time_out_rx) = sync_channel::<TimeOut>(0);

    state.timer.clone().start(c, time_out_tx);
    thread::spawn(move || {
        let recv_res = time_out_rx.recv();
        if recv_res.is_ok() {
            cmd_tx.send(Cmd::Stop).unwrap();
        }
    });
}
