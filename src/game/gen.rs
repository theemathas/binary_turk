use std::thread::Thread;
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};

use super::square::{Square,Rank,File};
use super::moves::Move;
use super::color::{White,Black};
use super::piece::Piece;
use super::piece::Type::{Pawn,King,Queen,Bishop,Knight,Rook};
use super::pos::Position;
use super::castle::{Kingside,Queenside};

use self::Action::{Continue,Stop};

macro_rules! send {
    ($tx: expr, $x: expr) => ({
        match $tx.send($x) {
            Ok(()) => (),
            Err(_) => return Stop,
        }
    })
}

enum Action {
    Continue,
    Stop,
}
impl Action {
    fn is_stop(&self) -> bool {
        match *self {
            Continue => false,
            Stop => true,
        }
    }
}

pub fn receive_psudo_legal(p: Position) -> Receiver<Move> {
    let (tx,rx) = sync_channel(0);
    Thread::spawn(move || gen_psudo_legal(&p, tx));
    rx
}

pub fn gen_psudo_legal(p: &Position, tx: SyncSender<Move>) {
    if gen_en_passant(p, &tx).is_stop() {
        return;
    }
    if gen_castle(p, &tx).is_stop() {
        return;
    }
    for (piece_id, from) in p.piece_iter() {
        if gen_move_from(p, piece_id, from, &tx).is_stop() {
            return;
        }
    }
}

pub fn can_move_to(p: Position, to: Square) -> bool {
    receive_psudo_legal(p).iter().any( |m| m.to() == to )
}

//Functions below return Stop if the receiver hung up.

fn gen_move_from(p: &Position, piece_id: Piece, from: Square, tx: &SyncSender<Move>) -> Action {
    if piece_id.color() != p.side_to_move() {
        return Continue;
    }
    match piece_id.piece_type() {
        Pawn => gen_pawn_from(p, piece_id, from, tx),
        Queen|Bishop|Rook => gen_slider_from(p, piece_id, from, tx),
        King|Knight => gen_fixed_from(p, piece_id, from, tx),
    }
}

// (file, rank)
type Diff = (i32, i32);

fn shift(s: Square, dir: Diff) -> Option<Square> {
    let (dx, dy) = dir;
    let (File(file), Rank(rank)) = s.to_tuple();
    Square::from_i32(file + dx, rank + dy)
}

fn gen_slider_from(p: &Position, piece_id: Piece, from: Square, tx: &SyncSender<Move>) -> Action {
    let rook_slide:   &[Diff] = &[(1,0), (0,1), (-1,0), (0,-1)];
    let bishop_slide: &[Diff] = &[(1,1), (1,-1), (-1,-1), (-1,1)];
    let queen_slide:  &[Diff] = &[(1,0), (0,1), (-1,0), (0,-1), (1,1), (1,-1), (-1,-1), (-1,1)];

    let piece_type = piece_id.piece_type();
    let piece_color = piece_id.color();

    let slide = match piece_type {
        Rook => rook_slide,
        Bishop => bishop_slide,
        Queen => queen_slide,
        _ => panic!(),
    };

    for dir in slide.iter() {
        let mut curr_pos = shift(from, *dir);
        //while curr_pos is valid and there is no piece there.
        while curr_pos.is_some() && p.is_empty_at(curr_pos.unwrap()) {
            let curr_move = Move::new(from, curr_pos.unwrap());
            send!(tx, curr_move);
            curr_pos = shift(curr_pos.unwrap(), *dir);
        }
        //if curr_pos is valid
        if curr_pos.is_some() {
            let to = curr_pos.unwrap();
            //if there is an enemy at the destination
            if p.is_color_at(to, piece_color.invert()) {
                let mut curr_move = Move::new(from, to);
                curr_move.set_capture(true);
                send!(tx, curr_move);
            }
        }
    }

    Continue
}

fn gen_fixed_from(p: &Position, piece_id: Piece, from: Square, tx: &SyncSender<Move>) -> Action {
    let king_fixed:   &[Diff] = &[(1,0), (0,1), (-1,0), (0,-1), (1,1), (1,-1), (-1,-1), (-1,1)];
    let knight_fixed: &[Diff] = &[(2,1), (2,-1), (-2,-1), (-2,1), (1,2), (1,-2), (-1,-2), (-1,2)];

    let piece_type = piece_id.piece_type();
    let piece_color = piece_id.color();

    let fixed = match piece_type {
        King => king_fixed,
        Knight => knight_fixed,
        _ => panic!(),
    };

    for dir in fixed.iter() {
        let new_pos = shift(from, *dir);
        if new_pos.is_some() {
            let to = new_pos.unwrap();
            let (is_valid, is_capture) = {
                if p.is_empty_at(to) {
                    (true, false)
                } else if p.is_color_at(to, piece_color) {
                    (false, false)
                } else {
                    (true, true)
                }
            };
            if is_valid {
                let mut curr_move = Move::new(from, to);
                curr_move.set_capture(is_capture);
                send!(tx, curr_move);
            }
        }
    }

    Continue
}

fn gen_pawn_from(p: &Position, piece_id: Piece, from: Square, tx: &SyncSender<Move>) -> Action {
    let piece_color = piece_id.color();
    let from_rank = from.rank().0;
    //rank_up is the 1-based rank from the piece-owner's side.
    let (dy, rank_up): (i32, i32) = match piece_color {
        White => ( 1, 1 + from_rank ),
        Black => (-1, 8 - from_rank ),
    };
    let move_dir: Diff = (0, dy);
    let to: Square = shift(from, move_dir).unwrap();
    // if destination is empty
    if p.is_empty_at(to) {
        match rank_up {
            7 => {
                for new_piece in [Queen, Knight, Rook, Bishop].iter() {
                    let mut curr_move = Move::new(from, to);
                    curr_move.set_promote(Some(*new_piece));
                    send!(tx, curr_move);
                }
            },
            2 => {
                let curr_move = Move::new(from, to);
                send!(tx, curr_move);
                let to2: Square = shift(to, move_dir).unwrap();
                if p.is_empty_at(to2) {
                    let mut curr_move2 = Move::new(from, to2);
                    curr_move2.set_pawn_double_move(true);
                    send!(tx, curr_move2);
                }
            },
            _ => {
                let curr_move = Move::new(from, to);
                send!(tx, curr_move);
            },
        }
    }

    for dx in [1,-1].iter() {
        let capture_dir: Diff = (*dx, dy);
        let capture_new_pos: Option<Square> = shift(from, capture_dir);
        let capture_to: Square = match capture_new_pos {
            Some(val) => val,
            None => continue,
        };
        if p.is_color_at(capture_to, piece_color.invert()) {
            if rank_up == 7 {
                for new_piece in [Queen, Knight, Rook, Bishop].iter() {
                    let mut curr_move = Move::new(from, capture_to);
                    curr_move.set_capture(true);
                    curr_move.set_promote(Some(*new_piece));
                    send!(tx, curr_move);
                }
            } else {
                let mut curr_move = Move::new(from, capture_to);
                curr_move.set_capture(true);
                send!(tx, curr_move);
            }
        }
    }

    Continue
}

fn gen_en_passant(p: &Position, tx: &SyncSender<Move>) -> Action {
    let to_file = match p.en_passant() {
        Some(f) => f,
        None => return Continue,
    };
    let (from_rank, to_rank) = match p.side_to_move() {
        White => (Rank(4), Rank(5)),
        Black => (Rank(3), Rank(2)),
    };
    let (x, y, z);
    let from_file_all: &[File] = match to_file {
        File(0) => { x = [File(1)]; &x },
        File(7) => { y = [File(6)]; &y },
        File(f) => { z = [File(f-1), File(f+1)]; &z },
    };

    let expect_piece = Piece::new(p.side_to_move(), Pawn);
    let to = Square::new(to_file, to_rank);

    for &from_file in from_file_all.iter() {
        let from = Square::new(from_file, from_rank);
        if p.is_piece_at(expect_piece, from) {
            let mut curr_move = Move::new(from, to);
            curr_move.set_en_passant(true);
            send!(tx, curr_move);
        }
    }

    Continue
}

fn gen_castle(p: &Position, tx: &SyncSender<Move>) -> Action {
    match p.side_to_move() {
        White => {
            if p.can_castle_now(Kingside, White) {
                let from = Square::new(File(4),Rank(0));
                let to   = Square::new(File(6),Rank(0));
                let mut curr_move = Move::new(from, to);
                curr_move.set_castle(Some(Kingside));
                send!(tx, curr_move);
            }
            if p.can_castle_now(Queenside, White) {
                let from = Square::new(File(4),Rank(0));
                let to   = Square::new(File(2),Rank(0));
                let mut curr_move = Move::new(from, to);
                curr_move.set_castle(Some(Queenside));
                send!(tx, curr_move);
            }
        }
        Black => {
            if p.can_castle_now(Kingside, Black) {
                let from = Square::new(File(4),Rank(7));
                let to   = Square::new(File(6),Rank(7));
                let mut curr_move = Move::new(from, to);
                curr_move.set_castle(Some(Kingside));
                send!(tx, curr_move);
            }
            if p.can_castle_now(Queenside, Black) {
                let from = Square::new(File(4),Rank(7));
                let to   = Square::new(File(2),Rank(7));
                let mut curr_move = Move::new(from, to);
                curr_move.set_castle(Some(Queenside));
                send!(tx, curr_move);
            }
        }
    }

    Continue
}
