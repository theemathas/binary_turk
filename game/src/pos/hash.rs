use std::ops::BitXor;

use rand::{ChaChaRng, Rng, SeedableRng};

use piece::Piece;
use square::{Square, File};
use color::Color;
use castle::Side;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ZobristHash(pub u64);
impl BitXor for ZobristHash {
    type Output = Self;
    fn bitxor(self, other: Self) -> Self {
        ZobristHash(self.0 ^ other.0)
    }
}

lazy_static! {
    static ref RANDOM_VALUES: [ZobristHash; 781] = {
        let mut rng = ChaChaRng::from_seed(&[123, 456, 789]);
        let mut ans = [ZobristHash(0); 781];
        for x in ans.iter_mut() { *x = ZobristHash(rng.gen()); }
        ans
    };
    static ref PIECE: [[ZobristHash; 64]; 12] = {
        let mut ans = [[ZobristHash(0); 64]; 12];
        for i in 0..12 {
            for j in 0..64 {
                ans[i][j] = RANDOM_VALUES[i * 64 + j];
            }
        }
        ans
    };
    static ref BLACK_MOVE: ZobristHash = RANDOM_VALUES[768];
    static ref CASTLING: [ZobristHash; 4] = {
        let mut ans = [ZobristHash(0); 4];
        for i in 0..4 {
            ans[i] = RANDOM_VALUES[i + 769];
        }
        ans
    };
    static ref EN_PASSANT: [ZobristHash; 8] = {
        let mut ans = [ZobristHash(0); 8];
        for i in 0..8 {
            ans[i] = RANDOM_VALUES[i + 773];
        }
        ans
    };
}

pub fn piece_square(piece: Piece, square: Square) -> ZobristHash {
    PIECE[piece as usize][square.to_id() as usize]
}
pub fn side_to_move() -> ZobristHash { *BLACK_MOVE }
pub fn castling(side: Side, color: Color) -> ZobristHash {
    CASTLING[color as usize * 2 + side as usize]
}
pub fn en_passant(file: File) -> ZobristHash {
    EN_PASSANT[file.0 as usize]
}
