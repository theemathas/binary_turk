use moves::Move;

use super::Position;

pub struct Iter<'a>(Box<Iterator<Item = Move> + 'a>);
impl<'a> Iterator for Iter<'a> {
    type Item = Move;
    fn next(&mut self) -> Option<Move> { self.0.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.0.size_hint() }
}

pub fn iter<'a>(p: &'a Position) -> Iter<'a> {
    Iter(Box::new(p.legal_iter().filter(|x| x.is_noisy())))
}
