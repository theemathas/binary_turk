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
        let Square(ans, _) = self;
        ans
    }
    pub fn rank(self) -> Rank {
        let Square(_, ans) = self;
        ans
    }
    pub fn to_tuple(self) -> (File, Rank) {
        let Square(f, r) = self;
        (f, r)
    }
    /*
    pub fn is_valid(&self) -> bool {
        let Square(File(f), Rank(r)) = *self;
        f < 8 && r < 8
    }
    */
    pub fn from_int(file: int, rank: int) -> Option<Square> {
        if file>=0 && file<8 && rank>=0 && rank<8 {
            Some(Square(File(file as u8), Rank(rank as u8)))
        } else {
            None
        }
    }
}
