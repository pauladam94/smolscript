use ast::{Constant, VarIndex};
use itertools::Itertools;
use std::{collections::VecDeque, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Constant(Constant),
    Array(Vec<Value>),
}

impl Value {
    pub fn pop(&mut self, mut index: VecDeque<usize>) -> Value {
        if index.len() == 0 {
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
