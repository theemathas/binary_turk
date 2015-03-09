use std::vec;
use std::iter;

use square::{Square, Rank, File};
use moves::Move;
use color::{White, Black};
use piece::Piece;
use piece::Type::{Pawn, King, Queen, Bishop, Knight, Rook};
use castle::{Kingside, Queenside};

use super::Position;
use super::bitboard::BitBoard;

pub struct Iter<'a>(iter::Chain<NoisyIter<'a>, QuietIter<'a>>);
impl<'a> Iterator for Iter<'a> {
    type Item = Move;
    fn next(&mut self) -> Option<Move> { self.0.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.0.size_hint() }
}

pub struct NoisyIter<'a>(Box<Iterator<Item = Move> + 'a>);
impl<'a> Iterator for NoisyIter<'a> {
    type Item = Move;
    fn next(&mut self) -> Option<Move> { self.0.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.0.size_hint() }
}

pub struct QuietIter<'a>(Box<Iterator<Item = Move> + 'a>);
impl<'a> Iterator for QuietIter<'a> {
    type Item = Move;
    fn next(&mut self) -> Option<Move> { self.0.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.0.size_hint() }
}

pub fn iter<'a>(p: &'a Position) -> Iter<'a> {
    Iter(noisy_iter(p).chain(quiet_iter(p)))
}

pub fn noisy_iter<'a>(p: &'a Position) -> NoisyIter<'a> {
    NoisyIter(Box::new(en_passant_iter(p).chain(
        p.piece_iter().flat_map(move |(piece_id, from)| {
            noisy_move_from_iter(p, piece_id, from)
        })
    )))
}

pub fn quiet_iter<'a>(p: &'a Position) -> QuietIter<'a> {
    QuietIter(Box::new(castle_iter(p).chain(
        p.piece_iter().flat_map(move |(piece_id, from)| {
            quiet_move_from_iter(p, piece_id, from)
        })
    )))
}

fn quiet_move_from_iter(p: &Position,
                        piece_id: Piece,
                        from: Square) -> Box<Iterator<Item = Move>> {
    if piece_id.color() != p.side_to_move() {
        Box::new(None.into_iter())
    } else {
        match piece_id.piece_type() {
            Pawn => quiet_pawn_from_iter(p, piece_id, from),
            Queen|Bishop|Rook => quiet_slider_from_iter(p, piece_id, from),
            King|Knight => quiet_fixed_from_iter(p, piece_id, from),
        }
    }
}

fn noisy_move_from_iter<'a>(p: &'a Position,
                            piece_id: Piece,
                            from: Square) -> Box<Iterator<Item = Move> + 'a> {
    if piece_id.color() != p.side_to_move() {
        Box::new(None.into_iter())
    } else {
        match piece_id.piece_type() {
            Pawn => noisy_pawn_from_iter(p, piece_id, from),
            Queen|Bishop|Rook => noisy_slider_from_iter(p, piece_id, from),
            King|Knight => noisy_fixed_from_iter(p, piece_id, from),
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

static ROOK_SLIDE:   [Diff; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
static BISHOP_SLIDE: [Diff; 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];
static QUEEN_SLIDE:  [Diff; 8] = [(1, 0), (0, 1), (-1, 0), (0, -1),
                                  (1, 1), (1, -1), (-1, -1), (-1, 1)];

lazy_static! {
    static ref ROOK_SLIDE_TABLE: [BitBoard; 64] = slider_table_gen(&ROOK_SLIDE);
    static ref BISHOP_SLIDE_TABLE: [BitBoard; 64] = slider_table_gen(&BISHOP_SLIDE);
    static ref QUEEN_SLIDE_TABLE: [BitBoard; 64] = slider_table_gen(&QUEEN_SLIDE);
    static ref BEHIND_TABLE: [[BitBoard; 64]; 64] = behind_table_gen();
}

fn behind_table_gen() -> [[BitBoard; 64]; 64] {
    let mut ans = [[BitBoard::new(); 64]; 64];
    for i in 0..64 {
        ans[i as usize] = behind_from_square_gen(Square::from_id(i));
    }
    ans
}

fn behind_from_square_gen(from: Square) -> [BitBoard; 64] {
    let mut ans = [BitBoard::new(); 64];

    for &dir in &QUEEN_SLIDE {

        let mut to = from;
        while let Some(temp_to) = shift(to, dir) {
            to = temp_to;

            let curr_ans: &mut BitBoard = &mut ans[to.to_id() as usize];

            let mut blocked = to;
            while let Some(temp_blocked) = shift(blocked, dir) {
                blocked = temp_blocked;

                curr_ans.set_at(blocked);
            }
        }
    }
    ans
}

fn slider_table_gen(diffs: &[Diff]) -> [BitBoard; 64] {
    let mut ans = [BitBoard::new(); 64];
    for i in 0..64 {
        ans[i as usize] = slider_from_square_gen(Square::from_id(i), diffs);
    }
    ans
}

fn slider_from_square_gen(from: Square, diffs: &[Diff]) -> BitBoard {
    let mut ans = BitBoard::new();
    for &dir in diffs {
        let mut to = from;
        while let Some(temp) = shift(to, dir) {
            to = temp;
            ans.set_at(to);
        }
    }
    ans
}

fn reachable_from_bitboard(p: &Position, piece_id: Piece, from: Square) -> BitBoard {
    let piece_type = piece_id.piece_type();
    let table: &[BitBoard; 64] = &match piece_type {
        Rook => *ROOK_SLIDE_TABLE,
        Bishop => *BISHOP_SLIDE_TABLE,
        Queen => *QUEEN_SLIDE_TABLE,
        _ => panic!(),
    };
    let mut ans: BitBoard = table[from.to_id() as usize];
    let potential_blocker: BitBoard = ans.intersect(!p.data.empty_data());

    for blocker_square in potential_blocker.iter() {
        ans = ans.intersect(
            !BEHIND_TABLE[from.to_id() as usize][blocker_square.to_id() as usize]);
    }

    ans
}

fn quiet_slider_from_iter(p: &Position,
                          piece_id: Piece,
                          from: Square) -> Box<Iterator<Item = Move>> {
    let to_bitboard = reachable_from_bitboard(p, piece_id, from);

    let ans = to_bitboard.intersect(p.data.empty_data());

    Box::new(ans.iter().map(move |to: Square| Move::new(from, to)))
}

fn noisy_slider_from_iter<'a>(p: &'a Position,
                              piece_id: Piece,
                              from: Square) -> Box<Iterator<Item = Move> + 'a> {
    let to_bitboard = reachable_from_bitboard(p, piece_id, from);

    let ans = to_bitboard.intersect(p.data.color_data(p.side_to_move().invert()));

    Box::new(ans.iter().map(move |to: Square| {
        let mut ans = Move::new(from, to);
        ans.set_capture_normal(p.at(to));
        ans
    }))
}

static KING_FIXED:   [Diff; 8] = [(1, 0), (0, 1), (-1, 0), (0, -1),
                                  (1, 1), (1, -1), (-1, -1), (-1, 1)];
static KNIGHT_FIXED: [Diff; 8] = [(2, 1), (2, -1), (-2, -1), (-2, 1),
                                  (1, 2), (1, -2), (-1, -2), (-1, 2)];

lazy_static! {
    static ref KING_FIXED_TABLE: [BitBoard; 64] = fixed_table_gen(&KING_FIXED);
    static ref KNIGHT_FIXED_TABLE: [BitBoard; 64] = fixed_table_gen(&KNIGHT_FIXED);
}

fn fixed_table_gen(diffs: &[Diff]) -> [BitBoard; 64] {
    let mut ans = [BitBoard::new(); 64];
    for i in 0..64 {
        ans[i as usize] = fixed_from_square_gen(Square::from_id(i), diffs);
    }
    ans
}

fn fixed_from_square_gen(from: Square, diffs: &[Diff]) -> BitBoard {
    let mut ans = BitBoard::new();
    for dir in diffs {
        if let Some(to) = shift(from, *dir) {
            ans.set_at(to);
        }
    }
    ans
}

fn quiet_fixed_from_iter(p: &Position,
                         piece_id: Piece,
                         from: Square) -> Box<Iterator<Item = Move>> {

    let table: &[BitBoard; 64] = &match piece_id.piece_type() {
        King => *KING_FIXED_TABLE,
        Knight => *KNIGHT_FIXED_TABLE,
        _ => panic!(),
    };
    let to_bits = table[from.to_id() as usize].intersect(p.data.empty_data());

    Box::new(to_bits.iter().map(move |to: Square| Move::new(from, to)))
}

fn noisy_fixed_from_iter<'a>(p: &'a Position,
                             piece_id: Piece,
                             from: Square) -> Box<Iterator<Item = Move> + 'a> {
    let table: &[BitBoard; 64] = &match piece_id.piece_type() {
        King => *KING_FIXED_TABLE,
        Knight => *KNIGHT_FIXED_TABLE,
        _ => panic!(),
    };
    let other_color = piece_id.color().invert();
    let to_bits = table[from.to_id() as usize].intersect(p.data.color_data(other_color));

    Box::new(to_bits.iter().map(move |to: Square| {
        let mut curr_move = Move::new(from, to);
        curr_move.set_capture_normal(p.at(to));
        curr_move
    }))
}

fn quiet_pawn_from_iter(p: &Position,
                        piece_id: Piece,
                        from: Square) -> Box<Iterator<Item = Move>> {
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
            7 => {},
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

    Box::new(ans.into_iter())
}

fn noisy_pawn_from_iter(p: &Position,
                        piece_id: Piece,
                        from: Square) -> Box<Iterator<Item = Move>> {
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
            _ => {},
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

    Box::new(ans.into_iter())
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
