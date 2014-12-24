#![allow(unused_variables)]

use std::thread::Thread;

use super::moves::Move;
use super::pos::Position;
use super::gen;

pub fn gen_legal(p: &Position, out: SyncSender<Move>) {
    let (tx, rx) = sync_channel::<Move>(1);
    let temp = (*p).clone();
    Thread::spawn(move || gen::gen_psudo_legal(temp, tx)).detach();
    filter_legal(p, out, rx);
}

fn filter_legal(p: &Position, out: SyncSender<Move>, rx: Receiver<Move>) {
    for curr_move in rx.iter() {
        if is_legal(p, &curr_move) {
            if out.send_opt(curr_move).is_err() {
                return;
            }
        }
    }
}

pub fn is_legal(p: &Position, curr_move: &Move) -> bool {
    //TODO implement legality check
    unimplemented!();
}
