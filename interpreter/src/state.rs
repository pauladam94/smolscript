use crate::value::Value;
use ast::{Block, Expr};
use std::fmt::Display;

pub enum State {
    Block(Block),
    Expr(Expr),
    Value(Value),
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Block(block) => write!(f, "Block : {block}"),
            State::Expr(expr) => write!(f, "Expr : {expr}"),
            State::Value(value) => write!(f, "Val : {value}"),
        }
    }
}
