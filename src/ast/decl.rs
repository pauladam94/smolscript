use std::fmt::Display;

use crate::{ast::block::Block, structural_eq::StructuralEq};

/// Declaration of a function that is displayed this way :
/// - `fn [name] ([args]) [block]`
pub struct Decl<Ident> {
    pub name: Ident,
    pub args: Vec<Ident>,
    pub block: Block<Ident>,
}

impl<S: Display> Display for Decl<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fn {} (", self.name)?;
        for arg in &self.args {
            write!(f, "{}", arg)?;
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
        for (arg, rhs) in self.args.iter().zip(rhs.args.iter()) {
            if !arg.structural_eq(rhs) {
                return false;
            }
        }

        true
    }
}
