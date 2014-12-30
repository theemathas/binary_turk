use std::thread::Thread;

use super::square::Square;
use super::castle;
use super::moves::Move;
use super::pos::Position;
use super::gen;
use super::make_move::make_move;

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
    let c = p.side_to_move();
    make_move(&mut p, curr_move);
    match curr_move.castle() {
        None => {
            !can_take_king(p)
        },
        Some(side) => {
            // Check for castling out of check, through check, and into check.
            let check_squares: Vec<Square> = castle::require_no_attack(side, c);
            check_squares.iter().all( |val| !gen::can_move_to(p.clone(), *val) )
        }
    }
}

pub fn can_take_king(p: Position) -> bool {
    let king_square = p.king_square(p.side_to_move().invert());
    gen::can_move_to(p, king_square)
}
