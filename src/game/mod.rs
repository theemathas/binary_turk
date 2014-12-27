//! This module is for everything about the rules of chess.

#![allow(dead_code)]

mod color;
mod piece;
mod square;
mod moves;

mod bitboard;
mod board;
mod castle;
mod pos;

mod fen;
mod gen;

mod legal;
mod make_move;

mod mate;

#[cfg(test)]
mod tests;
