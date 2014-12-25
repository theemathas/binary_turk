use super::moves::Plies;
use super::square::{File, Rank, Square};
use super::pos::{Position, Kingside, Queenside};
use super::piece;
use super::color::Color;

use std::collections::HashMap;

pub fn start_pos() -> Position {
    fen_to_position("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
}

pub fn fen_to_position(fen: &str) -> Result<Position, &str> {
    let fields: Vec<&str> = fen.split(' ').collect();
    if fields.len() < 6 {
        return Err("Not enough fields in FEN input.");
    };
    let mut pos_str = "".to_string();
    for ch in fields[0].chars() {
        match ch {
            '0' ... '9' => {
                for _ in range(0, ch as uint - '0' as uint) {
                    pos_str.push('1')
                }
            }
            _ => pos_str.push(ch)
        }
    }
    let mut pos = Position::new();
    let (mut rank, mut file) = (7, 0);
    let mut decode = HashMap::new();
    decode.insert('P', piece::WP);
    decode.insert('K', piece::WK);
    decode.insert('Q', piece::WQ);
    decode.insert('B', piece::WB);
    decode.insert('N', piece::WN);
    decode.insert('R', piece::WR);
    decode.insert('p', piece::BP);
    decode.insert('k', piece::BK);
    decode.insert('q', piece::BQ);
    decode.insert('b', piece::BB);
    decode.insert('n', piece::BN);
    decode.insert('r', piece::BR);
    for ch in fields[0].chars() {
        if ch == '/' {
            rank = rank - 1;
            file = 0;
        } else {
            match decode.get(&ch) {
                None => return Err("Unexpected charactor found."),
                Some(val) => {
                    pos.set_at_mut(Square::new(File(file),Rank(rank)), *val);
                    file = file + 1;
                }
            }
        }
    }
    let side_to_move = fields[1].char_at(0);
    match side_to_move {
        'w' => pos.set_side_to_move_mut(Color::White),
        'b' => pos.set_side_to_move_mut(Color::Black),
        _ => return Err("Invalid side to move."),
    };
    let castle = fields[2];
    pos.set_castle_mut(Kingside,  Color::White, castle.char_at(0) == 'K');
    pos.set_castle_mut(Queenside, Color::White, castle.char_at(1) == 'Q');
    pos.set_castle_mut(Kingside,  Color::Black, castle.char_at(2) == 'k');
    pos.set_castle_mut(Queenside, Color::Black, castle.char_at(3) == 'q');
    let en_passant_char = fields[3].char_at(0);
    if en_passant_char != '-' {
        pos.set_en_passant_mut(Some(File(en_passant_char as u8 - 'a' as u8)));
    }
    match fields[4].parse::<u8>() {
        None => return Err("Invalid number of plies."),
        Some(val) => pos.set_ply_count_mut(Plies(val)),
    }
    Ok(pos)
}
