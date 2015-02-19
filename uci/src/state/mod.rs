use std::thread::JoinGuard;
use std::sync::mpsc::SyncSender;

use search;
use timer::Timer;

pub use self::mode::Mode;

mod mode;

pub struct State<'a> {
    pub search_state: Option<search::State>,
    pub search_guard: Option<JoinGuard<'a, ()>>,
    pub search_tx: Option<SyncSender<search::Cmd>>,
    pub mode: Mode,
    pub start_search_time: Option<u64>,
    pub start_move_time: Option<u64>,
    pub timer: Timer,
    pub timer_kill_tx: Option<SyncSender<()>>,
    pub ucinewgame_support: bool,
}
impl<'a> State<'a> {
    pub fn new() -> State<'a> {
        State {
            search_state: None,
            search_guard: None,
            search_tx: None,
            mode: Mode::new(),
            start_search_time: None,
            start_move_time: None,
            timer: Timer::new(),
            timer_kill_tx: None,
            ucinewgame_support: false,
        }
    }
}