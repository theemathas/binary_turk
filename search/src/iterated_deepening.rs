use std::sync::mpsc::Sender;
use std::sync::atomic::{AtomicBool, Ordering};

use game::{Position, Move, Score, NumPlies};
use types::Data;
use transposition_table::TranspositionTable;
use depth_limited_search::depth_limited_search;

pub fn iterated_deepening(pos: Position,
                          search_moves: &[Move],
                          mut table: TranspositionTable,
                          tx: Sender<(Score, Move, NumPlies, Data)>,
                          is_killed: &AtomicBool) {
    let mut best_score;
    let mut best_move;
    let mut total_search_data = Data::one_node();
    let mut curr_depth = NumPlies(1);

    while !is_killed.load(Ordering::SeqCst) {
        debug!("Starting depth limited search with depth = {} plies", curr_depth.0);
        let mut temp_pos = pos.clone();
        let result_opt = depth_limited_search(&mut temp_pos, search_moves,
                                              curr_depth, &mut table, &is_killed);
        if let None = result_opt { break; }
        let result = result_opt.unwrap();

        let (temp_best_score, temp_best_move, curr_search_data) = result;
        best_score = temp_best_score;
        best_move = temp_best_move;
        total_search_data = total_search_data.combine(curr_search_data);

        let _ = tx.send((best_score, best_move, curr_depth, total_search_data.clone()));

        curr_depth.0 += 1;
    }
}
