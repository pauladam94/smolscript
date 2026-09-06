use ast::Expr;
use itertools::Itertools;
use std::{collections::VecDeque, fmt::Display};

pub enum Continuation {
    LetCont { var: String, block: VecDeque<Expr> },
    BlockCont(VecDeque<Expr>),
    DropCont(String),
}

impl Display for Continuation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Continuation::LetCont { var, block } => {
                write!(f, "let {var} = _ in {}", block.iter().format(";\n"))
            }
            Continuation::BlockCont(block) => write!(f, "{{ _; {}}}", block.iter().format(";\n")),
            Continuation::DropCont(var) => write!(f, "drop({var})"),
        }
    }
}
