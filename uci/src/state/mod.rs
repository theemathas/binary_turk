use std::thread::JoinGuard;
use std::sync::mpsc::SyncSender;

use search;
use timer::Timer;
use types::options;

pub use self::mode::Mode;

mod mode;

pub struct State {
    pub search_state: Option<search::State>,
    pub search_guard: Option<JoinGuard<'static, search::BestMove>>,
    pub search_tx: Option<SyncSender<search::Cmd>>,
    pub mode: Mode,
    pub start_search_time: Option<u64>,
    pub start_move_time: Option<u64>,
    pub timer: Timer,
    pub ucinewgame_support: bool,
    pub options: options::Data,
}
impl State {
    pub fn new() -> State {
        State {
            search_state: None,
            search_guard: None,
            search_tx: None,
            mode: Mode::new(),
            start_search_time: None,
            start_move_time: None,
            timer: Timer::new(),
            ucinewgame_support: false,
            options: options::Data::new(),
        }
    }
    pub fn reset_new_game(&mut self) {
        *self = State {
            mode: Mode::NewGame,
            ucinewgame_support: true,
            options: self.options.clone(),
            ..State::new()
        }
    }
}
