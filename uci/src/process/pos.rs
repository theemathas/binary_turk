use game::{Position, FromTo};
use search;
use state::State;

pub fn setup_new(state: &mut State,
                 mut pos: Position,
                 mut from_to_vec: Vec<FromTo>) {
    let mut prev_pos = None;
    let mut prev_move = None;
    for x in from_to_vec.drain() {
        let temp_move = x.to_move_with_pos(&pos);
        prev_pos = Some(pos.clone());
        pos.make_move(&temp_move);
        prev_move = Some(temp_move);
    }
    state.search_state = Some(search::State {
        pos: pos,
        prev_pos: prev_pos,
        prev_move: prev_move,
        param: search::Param::new((state.options.hash_size*(1<<20)) as usize),
    });
}

pub fn setup_same(state: &mut State,
                  pos: Position,
                  from_to_vec: Vec<FromTo>) {
    // TODO setup same game (currently just does new game)
    setup_new(state, pos, from_to_vec);
}
