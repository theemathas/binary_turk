//! The types for files, ranks, and squares

use std::str::FromStr;
use std::fmt;

// File and Rank are 0-based.
#[derive(PartialEq,Eq,Copy,Clone)]
pub struct File(pub u8);
impl FromStr for File {
    fn from_str(s: &str) -> Option<File> {
        if s.len() != 1 { return None; }
        match s.as_bytes()[0] {
            ch @ b'a' ... b'h' => Some(File(ch - b'a')),
            _ => None,
        }
    }
}
impl fmt::Show for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",(b'a' + self.0) as char)
    }
}

#[derive(PartialEq,Eq,Copy,Clone)]
pub struct Rank(pub u8);
impl FromStr for Rank {
    fn from_str(s: &str) -> Option<Rank> {
        if s.len() != 1 { return None; }
        match s.as_bytes()[0] {
            ch @ b'1' ... b'8' => Some(Rank(ch - b'1')),
            _ => None,
        }
    }
}
impl fmt::Show for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",(b'1' + self.0) as char)
    }
}

#[derive(PartialEq,Eq,Copy,Clone)]
pub struct Square(File, Rank);
impl Square {
    pub fn new(f: File, r: Rank) -> Square {
        Square(f, r)
    }

    pub fn file(self) -> File {
        self.0
    }
    pub fn rank(self) -> Rank {
        self.1
    }

    pub fn to_tuple(self) -> (File, Rank) {
        let Square(f, r) = self;
        (f, r)
    }
    pub fn from_int(file: int, rank: int) -> Option<Square> {
        if file>=0 && file<8 && rank>=0 && rank<8 {
            Some(Square(File(file as u8), Rank(rank as u8)))
        } else {
            None
        }
    }

    pub fn to_id(&self) -> u8 {
        let (File(f), Rank(r)) = self.to_tuple();
        f*8 + r
    }
    pub fn from_id(val: u8) -> Square {
        let (f, r) = (val/8, val%8);
        Square::new(File(f),Rank(r))
    }
}
impl FromStr for Square {
    fn from_str(s: &str) -> Option<Square> {
        if s.len() != 2 { return None; }
        let f: File = match FromStr::from_str(&*s.slice(0,1)) {
            Some(val) => val, None => return None };
        let r: Rank = match FromStr::from_str(&*s.slice(1,2)) {
            Some(val) => val, None => return None };
        Some(Square::new(f, r))
    }
}
impl fmt::Show for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
    }
}
