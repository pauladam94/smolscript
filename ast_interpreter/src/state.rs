use crate::value::Value;
use ast::Expr;
use itertools::Itertools;
use std::{
    collections::VecDeque,
    fmt::{Display, Formatter},
};

pub enum State {
    Block(VecDeque<Expr>),
    Value(Value),
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            State::Block(block) => write!(
                f,
                "Block :\n{}",
                block.iter().map(|e| e.pretty_printer(0)).format(";\n")
            ),
            State::Value(value) => write!(f, "Val : {value}"),
        }
    }
}
