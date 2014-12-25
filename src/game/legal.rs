#![allow(unused_variables)]

use std::thread::Thread;

use super::piece::King;
use super::moves::Move;
use super::pos::Position;
use super::gen;
use super::make_move::make_move_mut;

pub fn receive_legal(p: Position) -> Receiver<Move> {
    let (tx,rx) = sync_channel(0);
    Thread::spawn(move || gen_legal(&p, tx)).detach();
    rx
}

pub fn gen_legal(p: &Position, out: SyncSender<Move>) {
    let (tx, rx) = sync_channel::<Move>(1);
    let temp = (*p).clone();
    Thread::spawn(move || gen::gen_psudo_legal(&temp, tx)).detach();
    filter_legal(p, out, rx);
}

fn filter_legal(p: &Position, out: SyncSender<Move>, rx: Receiver<Move>) {
    for curr_move in rx.iter() {
        if is_legal(p.clone(), &curr_move) {
            if out.send_opt(curr_move).is_err() {
                return;
            }
        }
    }
}

pub fn is_legal(mut p: Position, curr_move: &Move) -> bool {
    make_move_mut(&mut p, curr_move);
    !can_take_king(&p)
}

pub fn can_take_king(p: &Position) -> bool {
    for m in gen::receive_psudo_legal(p.clone()).iter() {
        let dest = p.at(m.to());
        if dest.is_some() && dest.unwrap().piece_type() == King {
            return true;
        }
    }
    false
}
