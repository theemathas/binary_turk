use std::sync::mpsc::SyncSender;

use uci::Response::{self, Info};
use uci::InfoParam::{self, Depth, NodesSearched, PrincipalVariation};
use game::Move;
use types::{Score, NumPlies};

use super::types::Data;

pub fn send_info(tx: &SyncSender<Response>,
             best_move: Move,
             score: Score,
             depth: NumPlies,
             data: &Data) {
    tx.send(Info(vec![Depth(depth),
                      NodesSearched(data.nodes)]))
      .unwrap();
    tx.send(Info(vec![InfoParam::Score(None, score),
                      PrincipalVariation(vec![best_move])]))
      .unwrap();
}
