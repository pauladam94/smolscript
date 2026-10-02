use crate::ast::{Expr, Identifier};
use alloc::collections::vec_deque::VecDeque;
use core::fmt::{Display, Formatter, Result};
use itertools::Itertools;

#[derive(Debug, Clone)]
pub enum Continuation {
    LetCont {
        var: Identifier,
        block: VecDeque<Expr>,
    },
    BlockCont(VecDeque<Expr>),
    DropCont(Identifier),
    WhileCont {
        cond: Expr,
        body: VecDeque<Expr>,
    },
}

impl Display for Continuation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Continuation::LetCont { var, block } => {
                write!(f, "let {var} = □;\n{}", block.iter().format(";"))
            }
            Continuation::BlockCont(block) => write!(f, "{{ □; {}}}", block.iter().format(";\n")),
            Continuation::DropCont(var) => write!(f, "drop({var})"),
            Continuation::WhileCont { cond, body } => {
                write!(f, "while(□ | {cond}) {{\n\t")?;
                write!(f, "{}", body.iter().format(";\n\t"))?;
                write!(f, "\n}}")
            }
        }
    }
}
