use std::sync::mpsc::Sender;
use std::time::Duration;
use std::io::timer;
use std::cmp;

use game::Color;
use search;

use super::types::Data;

pub fn start(data: Data, c: Color, tx: Sender<search::Cmd>) {
    match data {
        Data::Infinite => return,
        Data::Exact(val) => {
            timer::sleep(val);
            let _ = tx.send(search::Cmd::Stop);
        },
        Data::Remain(val) => {
            let base = val.time(c).unwrap_or(Duration::zero());
            let inc = val.inc(c).unwrap_or(Duration::zero());
            timer::sleep(calc_time(base, inc));
            let _ = tx.send(search::Cmd::Stop);
        },
    }
}

fn calc_time(base: Duration, inc: Duration) -> Duration {
    cmp::min( base / 40 + inc, base )
}
