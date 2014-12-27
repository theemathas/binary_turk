//! The types for files, ranks, and squares

// File and Rank are 0-based.
#[deriving(PartialEq,Eq,Copy,Clone,Show)]
pub struct File(pub u8);

#[deriving(PartialEq,Eq,Copy,Clone,Show)]
pub struct Rank(pub u8);

#[deriving(PartialEq,Eq,Copy,Clone,Show)]
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
