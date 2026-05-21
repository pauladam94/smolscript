use crate::structural_eq::StructuralEq;
use std::fmt::Display;

#[derive(Debug)]
pub struct Index<Ident> {
    pub var: Ident,
    pub index: Vec<Ident>,
}

impl<Ident: Display> Display for Index<Ident> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.var)?;
        for ident in self.index.iter() {
            write!(f, "[{ident}]")?;
        }
        Ok(())
    }
}

impl<Ident: StructuralEq> StructuralEq for Index<Ident> {
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
