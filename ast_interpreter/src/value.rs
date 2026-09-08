use ast::{Binary, Constant, Unary};
use itertools::Itertools;
use std::{collections::VecDeque, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Constant(Constant),
    Array(Vec<Value>),
}

impl Value {
    pub fn pop(&mut self, mut index: VecDeque<usize>) -> Value {
        if index.is_empty() {
            match self {
                Value::Constant(_) => {
                    unreachable!("Pop done on a constant. This should be avoided at type checking.")
                }
                Value::Array(values) => values.pop().unwrap(),
            }
        } else {
            let i = index.pop_front().unwrap();
            match self {
                Value::Constant(_) => {
                    unreachable!("Pop done on a constant. This should be avoided at type checking.")
                }
                Value::Array(values) => values[i].pop(index),
            }
        }
    }

    pub(crate) fn compute_binary(self, op: Binary, rhs: Value) -> Value {
        use Value::*;
        match (op, self, rhs) {
            (_, Constant(_), Array(_)) | (_, Array(_), Constant(_)) => {
                unreachable!("Type checking should prevent this operation.")
            }
            (op, Constant(lhs), Constant(rhs)) => Value::Constant(lhs.compute_binary(op, rhs)),
            (op, Array(lhs), Array(rhs)) => Value::Array(
                lhs.into_iter()
                    .zip(rhs)
                    .map(|(l, r)| l.compute_binary(op, r))
                    .collect(),
            ),
        }
    }
    pub fn compute_unary(self, op: Unary) -> Value {
        match (op, self) {
            (Unary::Not, Value::Constant(Constant::Bool(b))) => Value::Constant(Constant::Bool(!b)),
            _ => todo!(),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Constant(constant) => write!(f, "{constant}"),
            Value::Array(array) => write!(f, "[{}]", array.iter().format(",")),
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::Constant(Constant::default())
    }
}
