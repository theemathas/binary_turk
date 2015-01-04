use std::thread::JoinGuard;
use std::sync::mpsc::SyncSender;

use search;
use timer::TimeData;

pub use self::mode::Mode;

mod mode;

pub struct State {
    pub search_state: Option<search::State>,
    pub search_guard: Option<JoinGuard<()>>,
    pub search_chan: Option<SyncSender<search::Cmd>>,
    pub mode: Mode,
    pub start_search_time: Option<u64>,
    pub start_move_time: Option<u64>,
    pub time_data: Option<TimeData>,
    pub ucinewgame_support: bool,
}
impl State {
    pub fn new() -> State {
        State {
            search_state: None,
            search_guard: None,
            search_chan: None,
            mode: Mode::new(),
            start_search_time: None,
            start_move_time: None,
            time_data: None,
            ucinewgame_support: false,
        }
    }
}
