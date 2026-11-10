use crate::ast::{Constant, ExprId};
use crate::ast_interpreter::value::Value;
use alloc::collections::VecDeque;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct Memory<'src> {
    pub stack: Vec<(&'src str, Value)>,
}

impl<'src> Memory<'src> {
    pub const EMPTY: Self = Memory { stack: Vec::new() };

    pub fn bind_var(&mut self, var: &'src str, value: Value) {
        self.stack.push((var, value));
    }
    pub fn pop_var(&mut self) {
        self.stack.pop();
    }
    pub fn get_value(&self, var: &'src str) -> Value {
        self.stack
            .iter()
            .rev()
            .find(|(x, _)| *x == var)
            .map_or_else(
                || unreachable!("Variable '{var}' should be in the stack but is not find."),
                |(_, v)| v.clone(),
            )
    }
    pub fn get_ref_value(&self, var: &'src str) -> &Value {
        self.stack
            .iter()
            .rev()
            .find(|(x, _)| *x == var)
            .map_or_else(
                || unreachable!("Variable '{var}' should be in the stack but is not find."),
                |(_, v)| v,
            )
    }
    pub fn get_ref_mut_value(&mut self, var: &'src str) -> &mut Value {
        self.stack
            .iter_mut()
            .rev()
            .find(|(x, _)| *x == var)
            .map_or_else(
                || unreachable!("Variable '{var}' should be in the stack but is not find."),
                |(_, v)| v,
            )
    }
    pub fn drop(&mut self, var: &'src str) {
        let index_var = self.stack.iter().position(|(x, _)| x == &var).unwrap();
        self.stack.remove(index_var);
    }
    pub fn get_index_value(&self, index: ExprId) -> Value {
        todo!()
        // let Index { var, index } = index;
        // let mut value = self.get_ref_value(var);
        // for y in &index {
        //     match value {
        //         Value::Array(array) => {
        //             if let Value::Constant(Constant::Int(i)) = self.get_value(*y) {
        //                 value = &array[i as usize];
        //             } else {
        //                 unreachable!(
        //                     "The index value is not an integer. The value is this one : '{}'. From this variable '{y}'. This should be avoided by type checking.",
        //                     self.get_value(*y)
        //                 )
        //             }
        //         }
        //         Value::Constant(_) => {
        //             unreachable!(
        //                 "The value indexed by in not an array (address) but a constant. This should be avoided by type checking."
        //             )
        //         }
        //     }
        // }
        // value.clone()
    }

    // Push operation of this form : [index].push([arg])
    pub fn push_method(&mut self, index: ExprId, arg: ExprId) {
        // let Index { var, index } = index;
        // let arg_value = self.get_value(arg);
        // let index_values: Vec<Value> = index.iter().map(|y| self.get_value(*y)).collect();
        // let mut value = self.get_ref_mut_value(var);
        // for v in index_values {
        //     if let Value::Array(array) = value
        //         && let Value::Constant(Constant::Int(i)) = v
        //     {
        //         value = &mut array[i as usize];
        //     } else {
        //         unreachable!("Indexing inside a non array OR with not an integer.")
        //     }
        // }
        // if let Value::Array(array) = value {
        //     array.push(arg_value);
        // }
    }

    // Replace operation of this form : var[index].replace(arg)
    pub fn replace_method(&mut self, var: &'src str, index: VecDeque<Value>, arg: Value) -> Value {
        let mut value = self.get_ref_mut_value(var);
        for v in index {
            if let Value::Array(array) = value
                && let Value::Constant(Constant::Int(i)) = v
            {
                value = &mut array[i as usize];
            } else {
                unreachable!("Indexing inside a non array OR with not an integer.")
            }
        }
        core::mem::replace(value, arg)
    }

    // Pop operation of this form : [index].pop()
    pub fn pop_method(&mut self, index: ExprId) -> Value {
        todo!()
        // let Index { var, index } = index;
        // let index_var = self.stack.iter().position(|(x, _)| x == &var).unwrap();
        // let (x, mut var_value) = core::mem::take(&mut self.stack[index_var]);
        // let index_vec: VecDeque<usize> = index
        //     .into_iter()
        //     .map(|y| {
        //         if let Value::Constant(Constant::Int(i)) = self.get_value(y) {
        //             i as usize
        //         } else {
        //             unreachable!("")
        //         }
        //     })
        //     .collect();
        // let last_value = var_value.pop(index_vec);
        // self.stack[index_var] = (x, var_value);
        // last_value
    }

    pub(crate) fn len_method(&self, index: ExprId) -> usize {
        todo!()
        // let Index { var, index } = index;
        // let index_values: Vec<Value> = index.iter().map(|y| self.get_value(*y)).collect();
        // let mut value = self.get_ref_value(var);
        // for v in index_values {
        //     if let Value::Array(array) = value
        //         && let Value::Constant(Constant::Int(i)) = v
        //     {
        //         value = &array[i as usize];
        //     } else {
        //         unreachable!("Indexing inside a non array OR with not an integer.")
        //     }
        // }
        // if let Value::Array(array) = value {
        //     array.len()
        // } else {
        //     unreachable!("Calling 'len' method not on an array. Typechecking should avoid this.")
        // }
    }
}
