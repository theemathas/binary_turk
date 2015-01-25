//! Implements the game representation

use types::NumPlies;

use super::piece::Piece;
use super::color::{Color};
use super::square::{File,Square};
use super::board::{self,Board};
use super::castle::{self,CastlingData,Side};

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
    pub fn new() -> Position {
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
        self.extra_data.castling.get(side, c)
    }
    // Does not check for castling out of check, through check, or into check.
    pub fn can_castle_now(&self, side: Side, c: Color) -> bool {
        self.can_castle(side, c) &&
            castle::require_empty_squares(side, c).iter().all( |x| self.is_empty_at(*x) )
    }
    pub fn set_castle(&mut self, side:Side, c:Color, val: bool) {
        self.extra_data.castling.set(side, c, val);
    }

    pub fn en_passant(&self) -> Option<File> {
        self.extra_data.en_passant
    }
    pub fn set_en_passant(&mut self, val: Option<File>) {
        self.extra_data.en_passant = val;
    }

    pub fn ply_count(&self) -> NumPlies {
        self.extra_data.ply_count
    }
    pub fn set_ply_count(&mut self, val: NumPlies) {
        self.extra_data.ply_count = val;
    }

    pub fn extra_data(&self) -> &ExtraData {
        &self.extra_data
    }
    pub fn set_extra_data(&mut self, val: ExtraData) {
        self.extra_data = val;
    }
}

