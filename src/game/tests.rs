use super::moves::Plies;
use super::pos::Position;
use super::fen;
use super::legal;
use super::make_move::make_move;
use super::mate;

#[test]
fn perf_init_pos_3_plies() {
    //perf_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    //         &[1, 20, 400, 8902, 197281, 4865609]);
    perf_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
             &[1, 20, 400]);
}

fn perf_fen(fen_str: &str, res: &[u64]) {
    let p = fen::fen_to_position(fen_str).unwrap();
    perf_pos(&p, res);
}

fn perf_pos(p: &Position, res: &[u64]) {
    for (num_plies, expect_num_from_pos) in res.iter().enumerate() {
        assert_eq!(*expect_num_from_pos, num_from_pos(p, Plies(num_plies as u8)));
    }
}

fn num_from_pos(p: &Position, plies: Plies) -> u64 {
    if plies == Plies(0) {
        return 1;
    }
    if !mate::has_legal_moves(p.clone()) {
        return 0;
    }
    let next_plies = Plies(plies.0 - 1);
    let mut ans = 0;
    for m in legal::receive_legal(p.clone()).iter() {
        let new_pos = make_move(p.clone(), &m);
        ans += num_from_pos(&new_pos, next_plies);
    }
    ans
}
