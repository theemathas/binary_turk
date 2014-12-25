//! Implements the game representation

use super::piece::Piece;
use super::color::{Color};
use super::square::{File,Square};
use super::moves::Plies;
use super::board::Board;
use super::castle::{mod,CastlingData,Side};

/// A complete representation of a chess position.
#[deriving(Clone)]
pub struct Position {
    data: Board,
    side_to_move: Color,
    castling: CastlingData,
    en_passant: Option<File>,
    ply_count: Plies,
}
impl Position {
    pub fn new() -> Position {
        Position {
            data: Board::new(),
            side_to_move: Color::White,
            castling: CastlingData::new(),
            en_passant: None,
            ply_count: Plies(0),
        }
    }
    pub fn at(&self, s: Square) -> Option<Piece> {
        self.data.at(s)
    }
    pub fn set_at_mut(&mut self, s: Square, val: Piece) {
        self.data.set_at_mut(s, val);
    }
    pub fn remove_at_mut(&mut self, s: Square) {
        self.data.remove_at_mut(s);
    }
    pub fn side_to_move(&self) -> Color {
        self.side_to_move
    }
    pub fn set_side_to_move_mut(&mut self, c: Color) {
        self.side_to_move = c;
    }
    pub fn can_castle(&self, side: Side, c: Color) -> bool {
        self.castling.get(side, c)
    }
    // Does not check for castling out of check, through check, or into check.
    pub fn can_castle_now(&self, side: Side, c: Color) -> bool {
        self.can_castle(side, c) &&
            castle::require_empty_squares(side, c).iter().all( |x| self.at(*x) == None )
    }
    pub fn set_castle_mut(&mut self, side:Side, c:Color, val: bool) {
        self.castling.set_mut(side, c, val);
    }
    pub fn en_passant(&self) -> Option<File> {
        self.en_passant
    }
    pub fn has_en_passant(&self) -> bool {
        self.en_passant.is_some()
    }
    pub fn set_en_passant_mut(&mut self, val: Option<File>) {
        self.en_passant = val;
    }
    pub fn ply_count(&self) -> Plies {
        self.ply_count
    }
    pub fn set_ply_count_mut(&mut self, val: Plies) {
        self.ply_count = val;
    }
}
