use game::{Position, FromTo, make_move};
use search;

pub fn setup_new(state: &mut Option<search::State>,
                 mut pos: Position,
                 mut from_to_vec: Vec<FromTo>) {
    let mut prev_pos = None;
    let mut prev_move = None;
    for x in from_to_vec.drain() {
        let temp_move = x.to_move_with_pos(&pos);
        prev_pos = Some(pos.clone());
        make_move(&mut pos, &temp_move);
        prev_move = Some(temp_move);
    }
    *state = Some(search::State {
        pos: pos,
        prev_pos: prev_pos,
        prev_move: prev_move,
        param: search::Param::new(),
    });
}

pub fn setup_same(state: &mut Option<search::State>,
                  pos: Position,
                  from_to_vec: Vec<FromTo>) {
    // TODO setup same game (currently just does new game)
    setup_new(state, pos, from_to_vec);
}
