//! Types that do not fit in any module.

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct NumNodes(pub u64);

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct NumVariations(pub u32);

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct PerMill(pub u32);

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct NumCpu(pub u32);

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct NumPlies(pub u32);

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct NumMoves(pub u32);
