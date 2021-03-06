pub use self::Color::{White, Black};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn invert(self) -> Color {
        match self {
            White => Black,
            Black => White,
        }
    }
}
