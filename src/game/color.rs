pub use self::Color::{White,Black};

#[deriving(PartialEq,Eq,Copy,Clone)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn invert(&self) -> Color {
        match *self {
            White => Black,
            Black => White,
        }
    }
}
