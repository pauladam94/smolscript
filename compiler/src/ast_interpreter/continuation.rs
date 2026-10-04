use crate::ast::{Expr, Identifier};
use alloc::{boxed::Box, collections::vec_deque::VecDeque};
use core::fmt::{Display, Formatter, Result};
use itertools::Itertools;

#[derive(Debug, Clone)]
pub enum Continuation {
    LetCont { var: Identifier, expr: Box<Expr> },
    SeqCont(VecDeque<Expr>),
    IfCont { yes: Box<Expr>, no: Box<Expr> },
    DropCont(Identifier),
}

impl Display for Continuation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Continuation::LetCont { var, expr } => {
                write!(f, "let {var} = □;\n{}", expr)
            }
            Continuation::SeqCont(block) => write!(f, "{{ □; {}}}", block.iter().format(";\n")),
            Continuation::DropCont(var) => write!(f, "drop({var})"),
            Continuation::IfCont { yes, no } => {
                write!(f, "if (□) {{\n{}\n}} else {{\n{}\n}}", yes, no)
            }
        }
    }
}
