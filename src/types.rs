//! Types that do not fit in any module.

#[deriving(PartialEq,Eq,Copy,Clone)]
pub struct NumNodes(pub u64);

#[deriving(PartialEq,Eq,Copy,Clone)]
pub struct NumVariations(u16);

#[deriving(PartialEq,Eq,Copy,Clone)]
pub struct PerMill(u16);

#[deriving(PartialEq,Eq,Copy,Clone)]
pub struct NumCpu(u16);

#[deriving(PartialEq,Eq,Copy,Clone)]
pub struct NumPlies(pub u16);

#[deriving(PartialEq,Eq,Copy,Clone)]
pub struct NumMoves(pub u16);
