use crate::ast::{Constant, Identifier, Index};
use crate::ast_interpreter::value::Value;
use alloc::{collections::vec_deque::VecDeque, vec::Vec};
use core::fmt::{Display, Formatter, Result};
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Memory {
    pub stack: Vec<(Identifier, Value)>,
}

impl Memory {
    pub const EMPTY: Self = Memory { stack: Vec::new() };

    pub fn bind_var(&mut self, var: Identifier, value: Value) {
        self.stack.push((var, value));
    }
    pub fn get_value(&self, var: Identifier) -> Value {
        self.stack
            .iter()
            .rev()
            .find(|(x, _)| *x == var)
            .map_or_else(
                || unreachable!("Variable '{var}' should be in the stack but is not find."),
                |(_, v)| v.clone(),
            )
    }
    pub fn get_ref_value(&self, var: Identifier) -> &Value {
        self.stack
            .iter()
            .rev()
            .find(|(x, _)| *x == var)
            .map_or_else(
                || unreachable!("Variable '{var}' should be in the stack but is not find."),
                |(_, v)| v,
            )
    }
    pub fn get_ref_mut_value(&mut self, var: Identifier) -> &mut Value {
        self.stack
            .iter_mut()
            .rev()
            .find(|(x, _)| *x == var)
            .map_or_else(
                || unreachable!("Variable '{var}' should be in the stack but is not find."),
                |(_, v)| v,
            )
    }
    pub fn drop(&mut self, var: Identifier) {
        let index_var = self.stack.iter().position(|(x, _)| x == &var).unwrap();
        self.stack.remove(index_var);
    }
    pub fn get_index_value(&self, index: Index) -> Value {
        let Index { var, index } = index;
        let mut value = self.get_ref_value(var);
        for y in &index {
            match value {
                Value::Array(array) => {
                    if let Value::Constant(Constant::Int(i)) = self.get_value(*y) {
                        value = &array[i as usize];
                    } else {
                        unreachable!(
                            "The index value is not an integer. The value is this one : '{}'. From this variable '{y}'. This should be avoided by type checking.",
                            self.get_value(*y)
                        )
                    }
                }
                Value::Constant(_) => {
                    unreachable!(
                        "The value indexed by in not an array (address) but a constant. This should be avoided by type checking."
                    )
                }
            }
        }
        value.clone()
    }

    // Push operation of this form : [index].push([arg])
    pub fn push_method(&mut self, index: Index, arg: Identifier) {
        let Index { var, index } = index;
        let arg_value = self.get_value(arg);
        let index_values: Vec<Value> = index.iter().map(|y| self.get_value(*y)).collect();
        let mut value = self.get_ref_mut_value(var);
        for v in index_values {
            if let Value::Array(array) = value
                && let Value::Constant(Constant::Int(i)) = v
            {
                value = &mut array[i as usize];
            } else {
                unreachable!("Indexing inside a non array OR with not an integer.")
            }
        }
        if let Value::Array(array) = value {
            array.push(arg_value);
        }
    }

    // Replace operation of this form : [index].replace([arg])
    pub fn replace_method(&mut self, index: Index, arg: Identifier) -> Value {
        let Index { var, index } = index;
        let arg_value = self.get_value(arg);
        let index_values: Vec<Value> = index.iter().map(|y| self.get_value(*y)).collect();
        let mut value = self.get_ref_mut_value(var);
        for v in index_values {
            if let Value::Array(array) = value
                && let Value::Constant(Constant::Int(i)) = v
            {
                value = &mut array[i as usize];
            } else {
                unreachable!("Indexing inside a non array OR with not an integer.")
            }
        }
        core::mem::replace(value, arg_value)
    }

    // Pop operation of this form : [index].pop()
    pub fn pop_method(&mut self, index: Index) -> Value {
        let Index { var, index } = index;
        let index_var = self.stack.iter().position(|(x, _)| x == &var).unwrap();
        let (x, mut var_value) = core::mem::take(&mut self.stack[index_var]);
        let index_vec: VecDeque<usize> = index
            .into_iter()
            .map(|y| {
                if let Value::Constant(Constant::Int(i)) = self.get_value(y) {
                    i as usize
                } else {
                    unreachable!("")
                }
            })
            .collect();
        let last_value = var_value.pop(index_vec);
        self.stack[index_var] = (x, var_value);
        last_value
    }

    pub(crate) fn len_method(&self, index: Index) -> usize {
        let Index { var, index } = index;
        let index_values: Vec<Value> = index.iter().map(|y| self.get_value(*y)).collect();
        let mut value = self.get_ref_value(var);
        for v in index_values {
            if let Value::Array(array) = value
                && let Value::Constant(Constant::Int(i)) = v
            {
                value = &array[i as usize];
            } else {
                unreachable!("Indexing inside a non array OR with not an integer.")
            }
        }
        if let Value::Array(array) = value {
            array.len()
        } else {
            unreachable!("Calling 'len' method not on an array. Typechecking should avoid this.")
        }
    }
}

impl Display for Memory {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            self.stack
                .iter()
                .format_with("\n", |(x, v), f| f(&format_args!("{x} = {v}")))
        )
    }
}
