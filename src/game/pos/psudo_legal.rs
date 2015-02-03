use std::vec;

use super::super::square::{Square, Rank, File};
use super::super::moves::Move;
use super::super::color::{White, Black};
use super::super::piece::Piece;
use super::super::piece::Type::{Pawn, King, Queen, Bishop, Knight, Rook};
use super::super::castle::{Kingside, Queenside};

use super::Position;

pub struct Iter<'a>(Box<Iterator<Item=Move>+'a>);
impl<'a> Iterator for Iter<'a> {
    type Item = Move;
    fn next(&mut self) -> Option<Move> { self.0.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.0.size_hint() }
}

pub fn iter<'a>(p: &'a Position) -> Iter<'a> {
    Iter(Box::new(en_passant_iter(p).chain(castle_iter(p)).chain(
        p.piece_iter().flat_map(move |(piece_id, from)| -> vec::IntoIter<Move> {
            move_from_iter(p, piece_id, from)
        })
    )))
}

fn move_from_iter(p: &Position, piece_id: Piece, from: Square) -> vec::IntoIter<Move> {
    if piece_id.color() != p.side_to_move() {
        Vec::new().into_iter()
    } else {
        match piece_id.piece_type() {
            Pawn => pawn_from_iter(p, piece_id, from),
            Queen|Bishop|Rook => slider_from_iter(p, piece_id, from),
            King|Knight => fixed_from_iter(p, piece_id, from),
        }
    }
}

// (file, rank)
type Diff = (i32, i32);

fn shift(s: Square, dir: Diff) -> Option<Square> {
    let (dx, dy) = dir;
    let (File(file), Rank(rank)) = s.to_tuple();
    Square::from_i32(file + dx, rank + dy)
}

fn slider_from_iter(p: &Position, piece_id: Piece, from: Square) -> vec::IntoIter<Move> {
    let rook_slide:   &[Diff] = &[(1, 0), (0, 1), (-1, 0), (0, -1)];
    let bishop_slide: &[Diff] = &[(1, 1), (1, -1), (-1, -1), (-1, 1)];
    let queen_slide:  &[Diff] = &[(1, 0), (0, 1), (-1, 0), (0, -1),
                                  (1, 1), (1, -1), (-1, -1), (-1, 1)];

    let mut ans = Vec::new();

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
            ans.push(curr_move);
            curr_pos = shift(curr_pos.unwrap(), *dir);
        }
        //if curr_pos is valid
        if curr_pos.is_some() {
            let to = curr_pos.unwrap();
            //if there is an enemy at the destination
            if p.is_color_at(to, piece_color.invert()) {
                let mut curr_move = Move::new(from, to);
                curr_move.set_capture_normal(p.at(to));
                ans.push(curr_move);
            }
        }
    }

    ans.into_iter()
}

fn fixed_from_iter(p: &Position, piece_id: Piece, from: Square) -> vec::IntoIter<Move> {
    let king_fixed:   &[Diff] = &[(1, 0), (0, 1), (-1, 0), (0, -1),
                                  (1, 1), (1, -1), (-1, -1), (-1, 1)];
    let knight_fixed: &[Diff] = &[(2, 1), (2, -1), (-2, -1), (-2, 1),
                                  (1, 2), (1, -2), (-1, -2), (-1, 2)];

    let mut ans = Vec::new();

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
				if is_capture { curr_move.set_capture_normal(p.at(to)); }
                ans.push(curr_move);
            }
        }
    }

    ans.into_iter()
}

fn pawn_from_iter(p: &Position, piece_id: Piece, from: Square) -> vec::IntoIter<Move> {
    let mut ans = Vec::new();

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
                    ans.push(curr_move);
                }
            },
            2 => {
                let curr_move = Move::new(from, to);
                ans.push(curr_move);
                let to2: Square = shift(to, move_dir).unwrap();
                if p.is_empty_at(to2) {
                    let mut curr_move2 = Move::new(from, to2);
                    curr_move2.set_pawn_double_move(true);
                    ans.push(curr_move2);
                }
            },
            _ => {
                let curr_move = Move::new(from, to);
                ans.push(curr_move);
            },
        }
    }

    for dx in [1, -1].iter() {
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
                    curr_move.set_capture_normal(p.at(capture_to));
                    curr_move.set_promote(Some(*new_piece));
                    ans.push(curr_move);
                }
            } else {
                let mut curr_move = Move::new(from, capture_to);
                curr_move.set_capture_normal(p.at(capture_to));
                ans.push(curr_move);
            }
        }
    }

    ans.into_iter()
}

fn en_passant_iter(p: &Position) -> vec::IntoIter<Move> {
    let mut ans = Vec::new();

    let to_file = match p.en_passant() {
        Some(f) => f,
        None => return ans.into_iter(),
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
            ans.push(curr_move);
        }
    }

    ans.into_iter()
}

fn castle_iter(p: &Position) -> vec::IntoIter<Move> {
    let mut ans = Vec::new();
    match p.side_to_move() {
        White => {
            if p.can_castle_now(Kingside, White) {
                let from = Square::new(File(4), Rank(0));
                let to   = Square::new(File(6), Rank(0));
                let mut curr_move = Move::new(from, to);
                curr_move.set_castle(Some(Kingside));
                ans.push(curr_move);
            }
            if p.can_castle_now(Queenside, White) {
                let from = Square::new(File(4), Rank(0));
                let to   = Square::new(File(2), Rank(0));
                let mut curr_move = Move::new(from, to);
                curr_move.set_castle(Some(Queenside));
                ans.push(curr_move);
            }
        }
        Black => {
            if p.can_castle_now(Kingside, Black) {
                let from = Square::new(File(4), Rank(7));
                let to   = Square::new(File(6), Rank(7));
                let mut curr_move = Move::new(from, to);
                curr_move.set_castle(Some(Kingside));
                ans.push(curr_move);
            }
            if p.can_castle_now(Queenside, Black) {
                let from = Square::new(File(4), Rank(7));
                let to   = Square::new(File(2), Rank(7));
                let mut curr_move = Move::new(from, to);
                curr_move.set_castle(Some(Queenside));
                ans.push(curr_move);
            }
        }
    }

    ans.into_iter()
}
