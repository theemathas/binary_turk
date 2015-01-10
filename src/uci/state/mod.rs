use std::thread::JoinGuard;
use std::sync::mpsc::Sender;

use search;
use timer;

pub use self::mode::Mode;

mod mode;

pub struct State<'a> {
    pub search_state: Option<search::State>,
    pub search_guard: Option<JoinGuard<'a, ()>>,
    pub search_tx: Option<Sender<search::Cmd>>,
    pub mode: Mode,
    pub start_search_time: Option<u64>,
    pub start_move_time: Option<u64>,
    pub time_data: Option<timer::Data>,
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
            time_data: None,
            ucinewgame_support: false,
        }
    }
}
