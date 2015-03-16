use std::sync::mpsc::{SyncSender, Receiver};
use std::time::Duration;
use std::old_io::Timer as StdTimer;
use std::cmp;

use game::Color;

use {Timer, TimeOut};

pub fn start(data: Timer, c: Color, tx: SyncSender<TimeOut>, rx_kill: Receiver<()>) {
    match data {
        Timer::Infinite => return,
        Timer::Exact(val) => {
            send_after(val, tx, rx_kill);
        },
        Timer::Remain(val) => {
            // TODO what is the right default value for base time?
            let base = val.time(c).unwrap_or(Duration::zero());
            let inc = val.inc(c);
            let val = calc_time(base, inc);
            send_after(val, tx, rx_kill);
        },
    }
}

fn send_after(delay: Duration, tx: SyncSender<TimeOut>, rx_kill: Receiver<()>) {
    let mut timer = StdTimer::new().unwrap();
    let rx_timer = timer.oneshot(delay);
    select!(
        _ = rx_timer.recv() => { let _ = tx.send(TimeOut(())); },
        _ = rx_kill.recv() => {}
    )
}

fn calc_time(base: Duration, inc: Duration) -> Duration {
    cmp::min( base / 40 + inc, base )
}
