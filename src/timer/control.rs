use std::sync::mpsc::{SyncSender, Receiver};
use std::time::Duration;
use std::io::Timer;
use std::cmp;

use game::Color;
use uci;

use super::types::Data;

pub fn start(data: Data, c: Color, tx: SyncSender<uci::Cmd>, rx_kill: Receiver<()>) {
    match data {
        Data::Infinite => return,
        Data::Exact(val) => {
            let mut timer = Timer::new().unwrap();
            let rx_timer = timer.oneshot(val);
            select!(
                _ = rx_timer.recv() => { let _ = tx.send(uci::Cmd::Stop); },
                _ = rx_kill.recv() => {}
            )
        },
        Data::Remain(val) => {
            let base = val.time(c).unwrap_or(Duration::zero());
            let inc = val.inc(c).unwrap_or(Duration::zero());
            let val = calc_time(base, inc);
            let mut timer = Timer::new().unwrap();
            let rx_timer = timer.oneshot(val);
            select!(
                _ = rx_timer.recv() => { let _ = tx.send(uci::Cmd::Stop); },
                _ = rx_kill.recv() => {}
            )
        },
    }
}

fn calc_time(base: Duration, inc: Duration) -> Duration {
    cmp::min( base / 40 + inc, base )
}
