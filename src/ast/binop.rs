use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum BinOpKind {
    Add,
    Sub,
}

impl Display for BinOpKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            BinOpKind::Add => write!(f, "+"),
            BinOpKind::Sub => write!(f, "-"),
        }
    }
}
