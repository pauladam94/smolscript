use crate::{expr::CSTExpr, structural_eq::StructuralEq};
use std::{
    collections::VecDeque,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub struct CSTBlock<'src> {
    pub statements: VecDeque<CSTExpr<'src>>,
}

impl<'src> Default for CSTBlock<'src> {
    fn default() -> Self {
        Self {
            statements: Default::default(),
        }
    }
}

impl<'src> Display for CSTBlock<'src> {
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

impl<'src> StructuralEq for CSTBlock<'src> {
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
