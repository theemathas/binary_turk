use std::iter;

use square::Square;
use moves::Move;

use super::Position;

pub struct Iter<'a>(iter::Chain<NoisyIter<'a>, QuietIter<'a>>);
impl<'a> Iterator for Iter<'a> {
    type Item = Move;
    fn next(&mut self) -> Option<Move> { self.0.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.0.size_hint() }
}

pub struct NoisyIter<'a>(Box<Iterator<Item = Move> + 'a>);
impl<'a> Iterator for NoisyIter<'a> {
    type Item = Move;
    fn next(&mut self) -> Option<Move> { self.0.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.0.size_hint() }
}

pub struct QuietIter<'a>(Box<Iterator<Item = Move> + 'a>);
impl<'a> Iterator for QuietIter<'a> {
    type Item = Move;
    fn next(&mut self) -> Option<Move> { self.0.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.0.size_hint() }
}

pub fn iter<'a>(p: &'a Position) -> Iter<'a> {
    Iter(p.legal_noisy_iter().chain(p.legal_quiet_iter()))
}

pub fn noisy_iter<'a>(p: &'a Position) -> NoisyIter<'a> {
    let mut temp = p.clone();
    NoisyIter(Box::new(p.psudo_legal_noisy_iter().filter(move |x| is_legal(&mut temp, x))))
}

pub fn quiet_iter<'a>(p: &'a Position) -> QuietIter<'a> {
    let mut temp = p.clone();
    QuietIter(Box::new(p.psudo_legal_quiet_iter().filter(move |x| is_legal(&mut temp, x))))
}

fn is_legal(p: &mut Position, curr_move: &Move) -> bool {
    let c = p.side_to_move();
    p.with_move(curr_move, |new_pos| {
        match curr_move.castle() {
            None => {
                !new_pos.can_take_king()
            },
            Some(side) => {
                // Check for castling out of check, through check, and into check.
                let check_squares: Vec<Square> = side.require_no_attack(c);
                check_squares.iter().all( |val| !new_pos.can_move_to(*val) )
            }
        }
    })
}
