use crate::{block::CSTBlock, parser::span::Span, structural_eq::StructuralEq, types::CSTTypes};
use std::{collections::VecDeque, fmt::Display};

/// Declaration of a function that is displayed this way :
/// - `fn [name] ([args]) [block]`
#[derive(Debug)]
pub struct CSTDecl<'src> {
    pub name: Span<'src>,
    pub args: VecDeque<(Span<'src>, CSTTypes)>,
    pub output_type: CSTTypes,
    pub block: CSTBlock<'src>,
}

impl<'src> Display for CSTDecl<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fn {} (", self.name)?;
        for (i, (s, t)) in self.args.iter().enumerate() {
            write!(f, "{s} : {t}")?;
            if i != self.args.iter().len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, ") {{")?;
        write!(f, "{}", self.block)?;
        write!(f, "}}")
    }
}

impl<'src> StructuralEq for CSTDecl<'src> {
    fn structural_eq(&self, rhs: &Self) -> bool {
        if !self.block.structural_eq(&rhs.block) {
            return false;
        }
        if self.args.len() != rhs.args.len() {
            return false;
        }
        for ((arg, t), (arg2, t2)) in self.args.iter().zip(rhs.args.iter()) {
            if !arg.structural_eq(arg2) {
                return false;
            }
            if !t.structural_eq(t2) {
                return false;
            }
        }

        true
    }
}
