use std::collections::HashMap;

use types::NumPlies;

use super::super::square::{File, Rank, Square};
use super::super::castle::{Kingside, Queenside};
use super::super::piece::Piece::*;
use super::super::color::Color::{White, Black};

use super::Position;

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
                let val = {
                    match ch.to_digit(10) {
                        Some(val) => val,
                        None => return Err("invalid character in fen"),
                    }
                };
                for _ in (0..val) {
                    pos_str.push('1')
                }
            }
            _ => pos_str.push(ch)
        }
    }
    let mut pos = Position::new();
    let (mut rank, mut file) = (7, 0);
    let mut decode = HashMap::new();
    decode.insert('P', WP);
    decode.insert('K', WK);
    decode.insert('Q', WQ);
    decode.insert('B', WB);
    decode.insert('N', WN);
    decode.insert('R', WR);
    decode.insert('p', BP);
    decode.insert('k', BK);
    decode.insert('q', BQ);
    decode.insert('b', BB);
    decode.insert('n', BN);
    decode.insert('r', BR);
    for ch in pos_str.chars() {
        if ch == '/' {
            rank = rank - 1;
            file = 0;
        } else if ch == '1' {
            file = file + 1;
        } else {
            match decode.get(&ch) {
                None => return Err("Unexpected charactor found."),
                Some(val) => {
                    pos.set_at(Square::new(File(file), Rank(rank)), *val);
                    file = file + 1;
                }
            }
        }
    }
    let side_to_move = fields[1].char_at(0);
    match side_to_move {
        'w' => pos.set_side_to_move(White),
        'b' => pos.set_side_to_move(Black),
        _ => return Err("Invalid side to move."),
    };
    let castle = fields[2];
    pos.set_castle(Kingside,  White, castle.char_at(0) == 'K');
    pos.set_castle(Queenside, White, castle.char_at(1) == 'Q');
    pos.set_castle(Kingside,  Black, castle.char_at(2) == 'k');
    pos.set_castle(Queenside, Black, castle.char_at(3) == 'q');
    let en_passant_char = fields[3].char_at(0);
    if en_passant_char != '-' {
        pos.set_en_passant(Some(File((en_passant_char as u8 - b'a') as i32)));
    }
    match fields[4].parse::<u32>() {
        None => return Err("Invalid number of plies."),
        Some(val) => pos.set_ply_count(NumPlies(val)),
    }
    Ok(pos)
}
