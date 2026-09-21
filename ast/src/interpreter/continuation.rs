use crate::{Expr, Identifier};
use itertools::Itertools;
use std::{collections::VecDeque, fmt::Display};

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
