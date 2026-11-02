use crate::ast::{Binary, Constant, Unary};
use alloc::{collections::vec_deque::VecDeque, vec::Vec};
use core::fmt::{Display, Formatter, Result};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Constant(Constant),
    Array(Vec<Value>),
}

impl Constant {
    pub fn compute_binary(self, op: Binary, rhs: Constant) -> Constant {
        use Binary::*;
        use Constant::*;
        match (op, self, rhs) {
            (_, Unit, _)
            | (_, _, Unit)
            | (_, Int(_), Bool(_))
            | (_, Bool(_), Int(_))
            | (Add, Bool(_), _)
            | (Div, Bool(_), _)
            | (Mult, Bool(_), _)
            | (Sub, Bool(_), _)
            | (LessThan | LessThanEqual | MoreThan | MoreThanEqual, Bool(_), Bool(_)) => {
                // dbg!(op, self, rhs);
                unreachable!("Type checking should prevent this operation.")
            }
            (Equal, l, r) => Bool(l == r),
            (NotEqual, l, r) => Bool(l != r),
            (Add, Int(l), Int(r)) => Int(l + r),
            (Sub, Int(l), Int(r)) => Int(l - r),
            (Mult, Int(l), Int(r)) => Int(l * r),
            (Div, Int(l), Int(r)) => Int(l / r),
            (LessThan, Int(l), Int(r)) => Bool(l < r),
            (LessThanEqual, Int(l), Int(r)) => Bool(l <= r),
            (MoreThan, Int(l), Int(r)) => Bool(l > r),
            (MoreThanEqual, Int(l), Int(r)) => Bool(l >= r),
        }
    }
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
