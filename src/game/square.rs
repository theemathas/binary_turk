//! The types for files, ranks, and squares

use std::str::FromStr;
use std::fmt;
use std::error::FromError;

pub struct ParseFileError(());
pub struct ParseRankError(());
pub struct ParseSquareError(());
impl FromError<ParseFileError> for ParseSquareError {
    fn from_error(_: ParseFileError) -> Self { ParseSquareError(()) }
}
impl FromError<ParseRankError> for ParseSquareError {
    fn from_error(_: ParseRankError) -> Self { ParseSquareError(()) }
}

// File and Rank are 0-based.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct File(pub i32);
impl FromStr for File {
    type Err = ParseFileError;
    fn from_str(s: &str) -> Result<File, ParseFileError> {
        if s.len() != 1 { return Err(ParseFileError(())); }
        match s.as_bytes()[0] {
            ch @ b'a' ... b'h' => Ok(File((ch - b'a') as i32)),
            _ => Err(ParseFileError(())),
        }
    }
}
impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		debug_assert!(self.0 >= 0 && self.0 < 8);
        write!(f, "{}", (self.0 as u8 + b'a') as char)
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Rank(pub i32);
impl FromStr for Rank {
    type Err = ParseRankError;
    fn from_str(s: &str) -> Result<Rank, ParseRankError> {
        if s.len() != 1 { return Err(ParseRankError(())); }
        match s.as_bytes()[0] {
            ch @ b'1' ... b'8' => Ok(Rank((ch - b'1') as i32)),
            _ => Err(ParseRankError(())),
        }
    }
}
impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		debug_assert!(self.0 >= 0 && self.0 < 8);
        write!(f, "{}", (self.0 as u8 + b'1') as char)
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
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
    pub fn from_i32(file: i32, rank: i32) -> Option<Square> {
        if file>=0 && file<8 && rank>=0 && rank<8 {
            Some(Square(File(file), Rank(rank)))
        } else {
            None
        }
    }

    pub fn to_id(&self) -> i32 {
        let (File(f), Rank(r)) = self.to_tuple();
        f*8 + r
    }
    pub fn from_id(val: i32) -> Square {
        let (f, r) = (val/8, val%8);
        Square::new(File(f), Rank(r))
    }
}
impl FromStr for Square {
    type Err = ParseSquareError;
    fn from_str(s: &str) -> Result<Square, ParseSquareError> {
        if s.len() != 2 { return Err(ParseSquareError(())); }
        let f: File = try!(FromStr::from_str(&s[0..1]));
        let r: Rank = try!(FromStr::from_str(&s[1..2]));
        Ok(Square::new(f, r))
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
    }
}
