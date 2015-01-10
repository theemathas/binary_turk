//! Types that do not fit in any module.

#[derive(PartialEq, Eq, Copy, Clone, Show)]
pub struct NumNodes(pub u64);

#[derive(PartialEq, Eq, Copy, Clone, Show)]
pub struct NumVariations(u16);

#[derive(PartialEq, Eq, Copy, Clone, Show)]
pub struct PerMill(u16);

#[derive(PartialEq, Eq, Copy, Clone, Show)]
pub struct NumCpu(u16);

#[derive(PartialEq, Eq, Copy, Clone, Show)]
pub struct NumPlies(pub u16);

#[derive(PartialEq, Eq, Copy, Clone, Show)]
pub struct NumMoves(pub u16);
