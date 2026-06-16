use crate::{block::CSTBlock, decl::CSTDecl, structural_eq::StructuralEq};
use std::{collections::VecDeque, fmt::Display};

/// Complete AST of a program
pub struct CST<'src> {
    pub top_level: VecDeque<CSTDecl<'src>>,
    pub block: CSTBlock<'src>,
}

impl<'src> Display for CST<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for decl in &self.top_level {
            write!(f, "{}", decl)?;
        }
        write!(f, "{}", self.block)
    }
}

impl<'src> StructuralEq for CST<'src> {
    fn structural_eq(&self, rhs: &Self) -> bool {
        for (decl, rhs) in self.top_level.iter().zip(rhs.top_level.iter()) {
            if !decl.structural_eq(rhs) {
                return false;
            }
        }
        true
    }
}
