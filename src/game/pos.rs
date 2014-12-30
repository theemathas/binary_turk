//! Implements the game representation

use super::piece::Piece;
use super::color::{Color};
use super::square::{File,Square};
use super::moves::Plies;
use super::board::{mod,Board};
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
    pub fn is_piece_at(&self, p: Piece, s: Square) -> bool {
        self.data.is_piece_at(p,s)
    }
    pub fn is_empty_at(&self, s: Square) -> bool {
        self.data.is_empty_at(s)
    }
    pub fn is_color_at(&self, s: Square, c: Color) -> bool {
        self.data.is_color_at(s,c)
    }

    pub fn set_at(&mut self, s: Square, p: Piece) {
        self.data.set_at(s, p);
    }
    pub fn remove_at(&mut self, s: Square, p: Piece) {
        self.data.remove_at(s, p);
    }

    pub fn king_square(&self, c: Color) -> Square {
        self.data.king_square(c)
    }
    pub fn piece_iter(&self) -> board::Iter {
        self.data.iter()
    }

    pub fn side_to_move(&self) -> Color {
        self.side_to_move
    }
    pub fn set_side_to_move(&mut self, c: Color) {
        self.side_to_move = c;
    }
    pub fn swap_side_to_move(&mut self) {
        let c = self.side_to_move.invert();
        self.set_side_to_move(c);
    }

    pub fn can_castle(&self, side: Side, c: Color) -> bool {
        self.castling.get(side, c)
    }
    // Does not check for castling out of check, through check, or into check.
    pub fn can_castle_now(&self, side: Side, c: Color) -> bool {
        self.can_castle(side, c) &&
            castle::require_empty_squares(side, c).iter().all( |x| self.is_empty_at(*x) )
    }
    pub fn set_castle(&mut self, side:Side, c:Color, val: bool) {
        self.castling.set(side, c, val);
    }

    pub fn en_passant(&self) -> Option<File> {
        self.en_passant
    }
    pub fn set_en_passant(&mut self, val: Option<File>) {
        self.en_passant = val;
    }

    pub fn ply_count(&self) -> Plies {
        self.ply_count
    }
    pub fn set_ply_count(&mut self, val: Plies) {
        self.ply_count = val;
    }
}
