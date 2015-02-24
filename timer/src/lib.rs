#![feature(std_misc, old_io)]

extern crate game;

use std::time::Duration;
use std::sync::mpsc::{SyncSender, Receiver};
use std::thread;

use game::{Color, NumMoves};

mod control;

#[derive(Copy, Clone, Debug)]
pub struct TimeOut(());

#[derive(Clone, Debug)]
pub enum Timer {
    Remain(RemainData),
    Exact(Duration),
    Infinite,
}
impl Timer {
    pub fn new() -> Timer {
        // TODO what is the right default value for Timer?
        Timer::Infinite
    }
    pub fn start(self, c: Color, tx: SyncSender<TimeOut>, rx_kill: Receiver<()>) {
        thread::spawn(move || control::start(self, c, tx, rx_kill));
    }

    pub fn time(&mut self, c: Color, val: Duration) -> &mut Timer {
        self.force_time_left();
        match *self {
            Timer::Remain(ref mut x) => x.set_time(c, val),
            _ => unreachable!(),
        }
        self
    }
    pub fn inc(&mut self, c: Color, val: Duration) -> &mut Timer {
        self.force_time_left();
        match *self {
            Timer::Remain(ref mut x) => x.set_inc(c, val),
            _ => unreachable!(),
        }
        self
    }
    pub fn moves_to_go(&mut self, val: NumMoves) -> &mut Timer {
        self.force_time_left();
        match *self {
            Timer::Remain(ref mut x) => x.moves_to_go = Some(val),
            _ => unreachable!(),
        }
        self
    }
    pub fn exact(&mut self, val: Duration) -> &mut Timer {
        *self = Timer::Exact(val);
        self
    }
    pub fn infinite(&mut self) -> &mut Timer {
        *self = Timer::Infinite;
        self
    }

    fn force_time_left(&mut self) {
        match *self {
            Timer::Remain(_) => {},
            ref mut x => *x = Timer::Remain(RemainData::new()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RemainData {
    w_time: Option<Duration>,
    b_time: Option<Duration>,
    w_inc: Duration,
    b_inc: Duration,
    moves_to_go: Option<NumMoves>,
}
impl RemainData {
    fn new() -> RemainData {
        RemainData {
            w_time: None,
            b_time: None,
            w_inc: Duration::zero(),
            b_inc: Duration::zero(),
            moves_to_go: None,
        }
    }
    fn time(&self, c: Color) -> Option<Duration> {
        match c {
            Color::White => self.w_time,
            Color::Black => self.b_time,
        }
    }
    fn set_time(&mut self, c: Color, val: Duration) {
        match c {
            Color::White => self.w_time = Some(val),
            Color::Black => self.b_time = Some(val),
        }
    }
    fn inc(&self, c: Color) -> Duration {
        match c {
            Color::White => self.w_inc,
            Color::Black => self.b_inc,
        }
    }
    fn set_inc(&mut self, c: Color, val: Duration) {
        match c {
            Color::White => self.w_inc = val,
            Color::Black => self.b_inc = val,
        }
    }
}
