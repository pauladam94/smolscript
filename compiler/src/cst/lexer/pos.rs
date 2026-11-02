use alloc::fmt::Display;
use core::cmp::Ordering;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Pos {
    pub line: u32,
    pub column: u32,
}

impl Display for Pos {
    fn fmt(&self, f: &mut alloc::fmt::Formatter<'_>) -> alloc::fmt::Result {
        write!(f, "pos : ({}, {})", self.line, self.column)
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.line.partial_cmp(&other.line) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.column.partial_cmp(&other.column)
    }
}
