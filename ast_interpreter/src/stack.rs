use crate::value::Value;
use std::{fmt::Display, ops::Index};

pub struct Stack {
    stack: Vec<(String, Value)>,
}

impl Stack {
    pub const EMPTY: Self = Stack { stack: vec![] };

    pub fn bind_var(&mut self, var: String, value: Value) {
        self.stack.push((var, value))
    }
    pub fn remove(&mut self, var: String) -> Value {
        let index = self
            .stack
            .iter()
            .rev()
            .position(|(v, value)| v == &var)
            .unwrap_or_else(|| unreachable!("Variable not found in stack. This error should be unreachable by type checking."));
        let (_, value) = self.stack.remove(self.stack.len() - index - 1);
        value
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.stack.iter();
        if let Some((var, value)) = iter.next() {
            write!(f, "{var} : {value}")?;
            for (var, value) in iter {
                write!(f, ", {var} : {value}")?;
            }
        }
        Ok(())
    }
}

impl Index<&String> for Stack {
    type Output = Value;

    fn index(&self, index: &String) -> &Self::Output {
        for (var, val) in self.stack.iter().rev() {
            if var == index {
                return val;
            }
        }
        unreachable!("Variable not found in stack. Type Checking should check this error.")
    }
}
