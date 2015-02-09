use super::super::square::Square;
use super::super::moves::Move;

use super::Position;

pub struct Iter<'a>(Box<Iterator<Item=Move>+'a>);
impl<'a> Iterator for Iter<'a> {
    type Item = Move;
    fn next(&mut self) -> Option<Move> { self.0.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.0.size_hint() }
}

pub fn iter<'a>(p: &'a Position) -> Iter<'a> {
    let temp = p.clone();
    Iter(Box::new(p.psudo_legal_iter().filter(move |x| is_legal(temp.clone(), x))))
}

fn is_legal(mut p: Position, curr_move: &Move) -> bool {
    let c = p.side_to_move();
    p.make_move(curr_move);
    match curr_move.castle() {
        None => {
            !p.can_take_king()
        },
        Some(side) => {
            // Check for castling out of check, through check, and into check.
            let check_squares: Vec<Square> = side.require_no_attack(c);
            check_squares.iter().all( |val| !p.can_move_to(*val) )
        }
    }
}
