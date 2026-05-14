use crate::{ast::expr::Expr, structural_eq::StructuralEq};
use std::fmt::{Display, Formatter, Result};

pub struct Block<Ident> {
    pub statements: Vec<Expr<Ident>>,
}

impl<Ident: Display> Display for Block<Ident> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for (i, statement) in self.statements.iter().enumerate() {
            write!(f, "{}", statement)?;
            if i != self.statements.len() - 1 {
                write!(f, ";\n")?;
            }
        }
        Ok(())
    }
}

impl<Ident: StructuralEq> StructuralEq for Block<Ident> {
    fn structural_eq(&self, rhs: &Self) -> bool {
        if self.statements.len() != rhs.statements.len() {
            return false;
        }

        for (statement, rhs) in self.statements.iter().zip(rhs.statements.iter()) {
            if !statement.structural_eq(rhs) {
                return false;
            }
        }

        true
    }
}
