use std::fmt::Display;

use crate::{
    ast::{block::Block, types::Types},
    structural_eq::StructuralEq,
};

/// Declaration of a function that is displayed this way :
/// - `fn [name] ([args]) [block]`
#[derive(Debug)]
pub struct Decl<Ident> {
    pub name: Ident,
    pub args: Vec<(Ident, Types)>,
    pub output_type: Types,
    pub block: Block<Ident>,
}

impl<S: Display> Display for Decl<S> {
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

impl<Ident: StructuralEq> StructuralEq for Decl<Ident> {
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
            if !t.structural_eq(t2) { return false }
        }

        true
    }
}
