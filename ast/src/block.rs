use crate::{Block, Expr};
use cst::block::CSTBlock;
use std::fmt::Display;

impl<'src> From<CSTBlock<'src>> for Block {
    fn from(block: CSTBlock<'src>) -> Self {
        let CSTBlock { statements } = block;
        Block {
            statements: statements
                .into_iter()
                .map(|expr| Expr::from(expr))
                .collect(),
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.statements.iter();
        if let Some(expr) = iter.next() {
            write!(f, "{expr}")?;

            for expr in iter {
                write!(f, " {expr}")?;
            }
        }
        Ok(())
    }
}
