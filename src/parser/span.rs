use std::fmt::Display;

use crate::{parser::pos::Pos, structural_eq::StructuralEq};

#[derive(Debug, Clone)]
pub struct Span<S> {
    pub pos: Pos,
    pub str: S,
}

impl<S: Display> Display for Span<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str)
    }
}

impl<S: PartialEq> StructuralEq for Span<S> {
    fn structural_eq(&self, rhs: &Self) -> bool {
        self.str == rhs.str
    }
}
