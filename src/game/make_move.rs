#![allow(unused_variables)]

use super::color::{White, Black};
use super::piece::{mod, Piece, WK, WR, BK, BR};
use super::square::{Square,File,Rank};
use super::moves::{Move,Plies};
use super::pos::{Position,Kingside,Queenside};

pub fn make_move(mut p: Position, m: &Move) -> Position {
    make_move_mut(&mut p, m);
    p
}

pub fn make_move_mut(p: &mut Position, m: &Move) {
    let from = m.from();
    let to = m.to();
    let curr_piece = p.at(from).unwrap();

    if m.is_capture() || m.is_en_passant() || curr_piece.piece_type() == piece::Type::Pawn {
        p.set_ply_count_mut(Plies(0));
    } else {
        let Plies(temp) = p.ply_count();
        p.set_ply_count_mut(Plies(temp+1));
    }

    if m.is_castle() {
        //TODO implement castling
        unimplemented!();
    } else if m.is_en_passant() {
        //TODO implement en passant
        unimplemented!();
    } else if m.is_promote() {
        //no en passant
        p.set_en_passant_mut(None);

        //change the board and promote
        if m.is_capture() {
            p.remove_at_mut(to);
        }
        p.remove_at_mut(from);
        p.set_at_mut(to, Piece::new(curr_piece.color(), m.promote().unwrap()));
    } else {
        //TODO set enpassant
        
        //change the board
        if m.is_capture() {
            p.remove_at_mut(to);
        }
        p.remove_at_mut(from);
        p.set_at_mut(to, curr_piece);
    }

    //castling
    if p.at(Square::new(File(4),Rank(0))) != Some(WK) {
        p.set_castle_mut(Queenside, White, false);
        p.set_castle_mut(Kingside , White, false);
    }
    if p.at(Square::new(File(0),Rank(0))) != Some(WR) {
        p.set_castle_mut(Queenside, White, false);
    }
    if p.at(Square::new(File(7),Rank(0))) != Some(WR) {
        p.set_castle_mut(Kingside , White, false);
    }
    if p.at(Square::new(File(4),Rank(7))) != Some(BK) {
        p.set_castle_mut(Queenside, Black, false);
        p.set_castle_mut(Kingside , Black, false);
    }
    if p.at(Square::new(File(0),Rank(7))) != Some(BR) {
        p.set_castle_mut(Queenside, Black, false);
    }
    if p.at(Square::new(File(7),Rank(7))) != Some(BR) {
        p.set_castle_mut(Kingside , Black, false);
    }

    let next_color = p.side_to_move().invert();
    p.set_side_to_move_mut(next_color);
}
