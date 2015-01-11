//! Types that do not fit in any module.

#[derive(PartialEq, Eq, Copy, Clone, Show)]
pub struct NumNodes(pub u64);

#[derive(PartialEq, Eq, Copy, Clone, Show)]
pub struct NumVariations(pub u16);

#[derive(PartialEq, Eq, Copy, Clone, Show)]
pub struct PerMill(pub u16);

#[derive(PartialEq, Eq, Copy, Clone, Show)]
pub struct NumCpu(pub u16);

#[derive(PartialEq, Eq, Copy, Clone, Show)]
pub struct NumPlies(pub u16);

#[derive(PartialEq, Eq, Copy, Clone, Show)]
pub struct NumMoves(pub u16);
