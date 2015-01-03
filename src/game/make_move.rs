use types::NumPlies;

use super::color::{White, Black};
use super::piece::{Piece, WK, WR, BK, BR, Pawn};
use super::square::{Square,File,Rank};
use super::moves::Move;
use super::pos::Position;
use super::castle::{Kingside,Queenside};

pub fn make_move(p: &mut Position, m: &Move) {
    let from = m.from();
    let to = m.to();
    let curr_piece = p.at(from).unwrap();

    if m.is_capture() || m.is_en_passant() || curr_piece.piece_type() == Pawn {
        p.set_ply_count(NumPlies(0));
    } else {
        let NumPlies(temp) = p.ply_count();
        p.set_ply_count(NumPlies(temp+1));
    }

    if m.castle().is_some() {

        //no en passant
        p.set_en_passant(None);

        let castle_color = match from.rank() {
            Rank(0) => White,
            Rank(7) => Black,
            _ => panic!(),
        };
        let castle_side = match to.file() {
            File(2) => Queenside,
            File(6) => Kingside,
            _ => panic!(),
        };

        let (rook_from, rook_to) = match (castle_color, castle_side) {
            (White, Kingside ) => (Square::new(File(7),Rank(0)), Square::new(File(5),Rank(0))),
            (White, Queenside) => (Square::new(File(0),Rank(0)), Square::new(File(3),Rank(0))),
            (Black, Kingside ) => (Square::new(File(7),Rank(7)), Square::new(File(5),Rank(7))),
            (Black, Queenside) => (Square::new(File(0),Rank(7)), Square::new(File(3),Rank(7))),
        };

        p.remove_at(from, curr_piece);
        p.set_at(to, curr_piece);
        let curr_rook = p.at(rook_from).unwrap();
        p.remove_at(rook_from, curr_rook);
        p.set_at(rook_to, curr_rook);

    } else if m.is_en_passant() {

        //no en passant after this
        p.set_en_passant(None);

        let captured = Square::new(to.file(), from.rank());
        let captured_piece = Piece::new(p.side_to_move().invert(), Pawn);

        p.remove_at(captured, captured_piece);
        p.remove_at(from, curr_piece);
        p.set_at(to, curr_piece);

    } else if m.promote().is_some() {
        //no en passant
        p.set_en_passant(None);

        //change the board and promote
        if m.is_capture() {
            let captured_piece = p.at(to).unwrap();
            p.remove_at(to, captured_piece);
        }
        p.remove_at(from, curr_piece);
        p.set_at(to, Piece::new(curr_piece.color(), m.promote().unwrap()));
    } else {

        if m.is_pawn_double_move() {
            p.set_en_passant(Some(to.file()));
        } else {
            p.set_en_passant(None);
        }
        
        //change the board
        if m.is_capture() {
            let captured_piece = p.at(to).unwrap();
            p.remove_at(to, captured_piece);
        }
        p.remove_at(from, curr_piece);
        p.set_at(to, curr_piece);
    }

    //castling
    if p.at(Square::new(File(4),Rank(0))) != Some(WK) {
        p.set_castle(Queenside, White, false);
        p.set_castle(Kingside , White, false);
    }
    if p.at(Square::new(File(0),Rank(0))) != Some(WR) {
        p.set_castle(Queenside, White, false);
    }
    if p.at(Square::new(File(7),Rank(0))) != Some(WR) {
        p.set_castle(Kingside , White, false);
    }
    if p.at(Square::new(File(4),Rank(7))) != Some(BK) {
        p.set_castle(Queenside, Black, false);
        p.set_castle(Kingside , Black, false);
    }
    if p.at(Square::new(File(0),Rank(7))) != Some(BR) {
        p.set_castle(Queenside, Black, false);
    }
    if p.at(Square::new(File(7),Rank(7))) != Some(BR) {
        p.set_castle(Kingside , Black, false);
    }

    p.swap_side_to_move();
}
