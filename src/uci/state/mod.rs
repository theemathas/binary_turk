use std::thread::JoinGuard;
use std::sync::mpsc::SyncSender;

use search;
use timer;

pub use self::mode::Mode;

mod mode;

pub struct State<'a> {
    pub is_debug: bool,
    pub search_state: Option<search::State>,
    pub search_guard: Option<JoinGuard<'a, ()>>,
    pub search_tx: Option<SyncSender<search::Cmd>>,
    pub mode: Mode,
    pub start_search_time: Option<u64>,
    pub start_move_time: Option<u64>,
    pub time_data: Option<timer::Data>,
    pub time_kill_tx: Option<SyncSender<()>>,
    pub ucinewgame_support: bool,
}
impl<'a> State<'a> {
    pub fn new() -> State<'a> {
        State {
            is_debug: false,
            search_state: None,
            search_guard: None,
            search_tx: None,
            mode: Mode::new(),
            start_search_time: None,
            start_move_time: None,
            time_data: None,
            time_kill_tx: None,
            ucinewgame_support: false,
        }
    }
}
