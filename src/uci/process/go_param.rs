use time::precise_time_ns;

use game::Move;

use super::super::types::GoParam;
use super::super::state::State;

pub fn setup(state: &mut State, mut data: Vec<GoParam>) {
    let ref mut search_state = state.search_state.as_mut()
                                    .expect("invalid search state");
    let ref mut timer = state.timer;
    let ref mut param = search_state.param;
    let ref pos = search_state.pos;
    for go_param in data.drain() {
        match go_param {
            GoParam::SearchMoves(mut from_to_vec) => {
                let move_vec: Vec<Move> = from_to_vec.drain().map(|x| x.to_move_with_pos(pos)).collect();
                param.search_moves = Some(move_vec);
            },
            GoParam::Ponder => param.ponder = true,
            GoParam::Time(c, val) => { let _ = timer.time(c, val); },
            GoParam::IncTime(c, val) => { let _ = timer.inc(c, val); },
            GoParam::MovesToGo(val) => { let _ = timer.moves_to_go(val); },
            GoParam::Depth(val) => param.depth = Some(val),
            GoParam::Nodes(val) => param.nodes = Some(val),
            GoParam::Mate(val) => param.mate = Some(val),
            GoParam::MoveTime(val) => { let _ = timer.exact(val); },
            GoParam::Infinite => { let _ = timer.infinite(); },
        }
    }
    state.start_search_time = Some(precise_time_ns());
    if !param.ponder {
        state.start_move_time = state.start_search_time;
    }
}
