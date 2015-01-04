//! Types that do not fit in any module.

#[derive(PartialEq,Eq,Copy,Clone)]
pub struct NumNodes(pub u64);

#[derive(PartialEq,Eq,Copy,Clone)]
pub struct NumVariations(u16);

#[derive(PartialEq,Eq,Copy,Clone)]
pub struct PerMill(u16);

#[derive(PartialEq,Eq,Copy,Clone)]
pub struct NumCpu(u16);

#[derive(PartialEq,Eq,Copy,Clone)]
pub struct NumPlies(pub u16);

#[derive(PartialEq,Eq,Copy,Clone)]
pub struct NumMoves(pub u16);
