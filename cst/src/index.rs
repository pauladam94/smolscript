use crate::{parser::span::Span, structural_eq::StructuralEq};
use std::{collections::VecDeque, fmt::Display};

#[derive(Debug)]
pub struct CSTIndex<'src> {
    pub var: Span<'src>,
    pub index: VecDeque<Span<'src>>,
}

impl<'src> Display for CSTIndex<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.var)?;
        for ident in self.index.iter() {
            write!(f, "[{ident}]")?;
        }
        Ok(())
    }
}

impl<'src> StructuralEq for CSTIndex<'src> {
    fn structural_eq(&self, rhs: &Self) -> bool {
        if !self.var.structural_eq(&rhs.var) {
            return false;
        }
        if self.index.len() != rhs.index.len() {
            return false;
        }
        for (ident, ident2) in self.index.iter().zip(rhs.index.iter()) {
            if !ident.structural_eq(ident2) {
                return false;
            }
        }
        true
    }
}

impl<'src> Default for CSTIndex<'src> {
    fn default() -> Self {
        Self {
            var: Default::default(),
            index: Default::default(),
        }
    }
}
