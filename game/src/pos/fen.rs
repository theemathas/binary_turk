use std::collections::HashMap;

use types::NumPlies;

use square::{File, Rank, Square};
use castle::{Kingside, Queenside};
use piece::Piece::*;
use color::Color::{White, Black};

use super::Position;

#[derive(Debug)]
pub struct ParsePosError(&'static str);

pub fn start_pos() -> Position {
    fen_to_position("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
}

pub fn fen_to_position(fen: &str) -> Result<Position, ParsePosError> {
    let fields: Vec<&str> = fen.split(' ').collect();
    if fields.len() < 6 {
        return Err(ParsePosError("Not enough fields in FEN input."));
    };
    let mut pos_str = "".to_string();
    for ch in fields[0].chars() {
        match ch {
            '0' ... '9' => {
                let val = {
                    match ch.to_digit(10) {
                        Some(val) => val,
                        None => return Err(ParsePosError("invalid character in fen")),
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
                None => return Err(ParsePosError("Unexpected charactor found.")),
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
        _ => return Err(ParsePosError("Invalid side to move.")),
    };
    let castle = fields[2];
    for ch in castle.chars() {
        match ch {
            'K' => pos.set_castle(Kingside , White, true),
            'Q' => pos.set_castle(Queenside, White, true),
            'k' => pos.set_castle(Kingside , Black, true),
            'q' => pos.set_castle(Queenside, Black, true),
            _ => {}
        }
    }
    let en_passant_char = fields[3].char_at(0);
    if en_passant_char != '-' {
        pos.set_en_passant(Some(File((en_passant_char as u8 - b'a') as i32)));
    }
    match fields[4].parse::<u32>() {
        Ok(val) => pos.set_ply_count(NumPlies(val)),
        Err(_) => return Err(ParsePosError("Invalid number of plies.")),
    }
    Ok(pos)
}
