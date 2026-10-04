use crate::{ast::Expr, ast_interpreter::value::Value};
use core::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum State {
    Expr(Expr),
    Value(Value),
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        match self {
            State::Expr(expr) => {
                write!(f, "[Expr]\n{expr}")
            }
            State::Value(value) => write!(f, "[Value]\n{value}"),
        }
    }
}
