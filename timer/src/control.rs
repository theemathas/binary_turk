use std::sync::mpsc::SyncSender;
use std::time::Duration;
use std::cmp;
use std::thread;

use game::Color;

use {Timer, TimeOut};

pub fn start(data: Timer, c: Color, tx: SyncSender<TimeOut>) {
    match data {
        Timer::Infinite => return,
        Timer::Exact(val) => {
            send_after(val, tx);
        },
        Timer::Remain(val) => {
            // TODO what is the right default value for base time?
            let base = val.time(c).unwrap_or(Duration::new(0, 0));
            let inc = val.inc(c);
            let val = calc_time(base, inc);
            send_after(val, tx);
        },
    }
}

fn send_after(delay: Duration, tx: SyncSender<TimeOut>) {
    thread::sleep(delay);
    let _ = tx.send(TimeOut(()));
}

fn calc_time(base: Duration, inc: Duration) -> Duration {
    cmp::min( base / 40 + inc, base )
}
