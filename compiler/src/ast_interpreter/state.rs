use crate::{ast::Expr, ast_interpreter::value::Value};
use alloc::collections::vec_deque::VecDeque;
use core::fmt::{Display, Formatter};
use itertools::Itertools;

#[derive(Debug, Clone)]
pub enum State {
    Block(VecDeque<Expr>),
    Value(Value),
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        match self {
            State::Block(block) => {
                write!(f, "[Block]\n{}", block.iter().format(";\n"))
            }
            State::Value(value) => write!(f, "[Value] {value}"),
        }
    }
}
