//! Implements the game representation

use std::str::FromStr;

pub use self::eval::{Score, ScoreUnit};

use super::piece::Piece;
use super::color::Color;
use super::square::{File, Square};
use super::castle::{CastlingData, Side};
use super::moves::{Move, NumPlies};

use self::board::Board;
use self::fen::{fen_to_position, ParsePosError};

mod board;
mod bitboard;
mod legal;
mod psudo_legal;
mod make_move;
mod mate;
mod fen;
mod eval;

/// Data required to unmake moves
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ExtraData {
    castling: CastlingData,
    en_passant: Option<File>,
    ply_count: NumPlies,
}

/// A complete representation of a chess position.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Position {
    data: Board,
    side_to_move: Color,
    extra_data: ExtraData,
}
impl Position {
    fn new() -> Position {
        Position {
            data: Board::new(),
            side_to_move: Color::White,
            extra_data: ExtraData {
                castling: CastlingData::new(),
                en_passant: None,
                ply_count: NumPlies(0),
            },
        }
    }
    pub fn start() -> Self {
        fen::start_pos()
    }

    fn at(&self, s: Square) -> Option<Piece> {
        self.data.at(s)
    }
    fn is_piece_at(&self, p: Piece, s: Square) -> bool {
        self.data.is_piece_at(p, s)
    }
    fn is_empty_at(&self, s: Square) -> bool {
        self.data.is_empty_at(s)
    }
    fn is_color_at(&self, s: Square, c: Color) -> bool {
        self.data.is_color_at(s, c)
    }

    fn set_at(&mut self, s: Square, p: Piece) {
        self.data.set_at(s, p);
    }
    fn remove_at(&mut self, s: Square, p: Piece) {
        self.data.remove_at(s, p);
    }

    fn king_square(&self, c: Color) -> Square {
        self.data.king_square(c)
    }
    fn piece_iter(&self) -> board::Iter {
        self.data.iter()
    }

    pub fn side_to_move(&self) -> Color {
        self.side_to_move
    }
    fn set_side_to_move(&mut self, c: Color) {
        self.side_to_move = c;
    }
    fn swap_side_to_move(&mut self) {
        let c = self.side_to_move.invert();
        self.set_side_to_move(c);
    }

    fn can_castle(&self, side: Side, c: Color) -> bool {
        self.extra_data.castling.get(side, c)
    }
    // Does not check for castling out of check, through check, or into check.
    fn can_castle_now(&self, side: Side, c: Color) -> bool {
        self.can_castle(side, c) &&
            side.require_empty_squares(c).iter().all( |x| self.is_empty_at(*x) )
    }
    fn set_castle(&mut self, side:Side, c:Color, val: bool) {
        self.extra_data.castling.set(side, c, val);
    }

    fn en_passant(&self) -> Option<File> {
        self.extra_data.en_passant
    }
    fn set_en_passant(&mut self, val: Option<File>) {
        self.extra_data.en_passant = val;
    }

    fn ply_count(&self) -> NumPlies {
        self.extra_data.ply_count
    }
    fn set_ply_count(&mut self, val: NumPlies) {
        self.extra_data.ply_count = val;
    }

    fn extra_data(&self) -> &ExtraData {
        &self.extra_data
    }
    fn set_extra_data(&mut self, val: ExtraData) {
        self.extra_data = val;
    }

    fn psudo_legal_iter<'a>(&'a self) -> psudo_legal::Iter<'a> {
        psudo_legal::iter(self)
    }
    fn psudo_legal_noisy_iter<'a>(&'a self) -> psudo_legal::NoisyIter {
        psudo_legal::noisy_iter(self)
    }
    fn psudo_legal_quiet_iter<'a>(&'a self) -> psudo_legal::QuietIter {
        psudo_legal::quiet_iter(self)
    }

    pub fn legal_iter<'a>(&'a self) -> legal::Iter<'a> {
        legal::iter(self)
    }
    pub fn legal_noisy_iter<'a>(&'a self) -> legal::NoisyIter<'a> {
        legal::noisy_iter(self)
    }
    pub fn legal_quiet_iter<'a>(&'a self) -> legal::QuietIter<'a> {
        legal::quiet_iter(self)
    }

    fn can_move_to(&self, to: Square) -> bool {
        self.psudo_legal_iter().any( |m| m.to() == to )
    }
    fn can_take_king(&self) -> bool {
        let king_square = self.king_square(self.side_to_move().invert());
        self.can_move_to(king_square)
    }

    pub fn make_move(&mut self, m: &Move) {
        make_move::make_move(self, m);
    }
    pub fn unmake_move(&mut self, m: &Move, extra_data: ExtraData) {
        make_move::unmake_move(self, m, extra_data);
    }

    pub fn is_checkmated(&mut self) -> bool {
        mate::is_checkmated(self)
    }
    pub fn is_stalemated(&mut self) -> bool {
        mate::is_stalemated(self)
    }
    fn has_legal_moves(&mut self) -> bool {
        mate::has_legal_moves(self)
    }

    pub fn eval(&mut self, draw_val: ScoreUnit) -> Score {
        eval::eval(self, draw_val)
    }

    pub fn with_move<T, F: FnOnce(&mut Position) -> T>(&mut self, curr_move: &Move,f: F) -> T {
        let extra_data = self.extra_data().clone();
        self.make_move(curr_move);
        let ans = f(self);
        self.unmake_move(curr_move, extra_data);
        ans
    }
}

impl FromStr for Position {
    type Err = ParsePosError;
    fn from_str(s: &str) -> Result<Self, ParsePosError> {
        fen_to_position(s)
    }
}

pub fn at_in_pos(pos: &Position, s: Square) -> Option<Piece> {
    pos.at(s)
}

pub fn is_empty_at_in_pos(pos: &Position, s: Square) -> bool {
    pos.is_empty_at(s)
}
