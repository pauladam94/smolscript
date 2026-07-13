use crate::{Address, heap::Heap, stack::Stack, value::Value};
use ast::{Constant, Index};
use std::fmt::{Display, Formatter, Result};

pub struct Memory {
    pub stack: Stack,
    pub heap: Heap,
}

impl Memory {
    pub const EMPTY: Self = Memory {
        stack: Stack::EMPTY,
        heap: Heap::EMPTY,
    };

    pub fn bind_var(&mut self, var: String, value: Value) {
        self.stack.bind_var(var, value)
    }

    pub fn alloc(&mut self) -> Address {
        self.heap.alloc()
    }

    pub fn drop(&mut self, var: String) {
        let value = self.stack.remove(var);
        self.heap.drop(value);
    }
    pub fn index_value(&self, index: Index) -> Value {
        let Index { var, index } = index;
        let mut value = &self.stack[&var];
        for y in index.iter() {
            match value {
                Value::Address(address) => {
                    let region = &self.heap[address];
                    if let Value::Constant(Constant::Int(i)) = &self.stack[y] {
                        value = region.get(*i as usize).expect(&format!(
                            "Indexing in {:?} at index '{}' but region is of size {}.",
                            region,
                            i,
                            region.len()
                        ));
                    } else {
                        unreachable!(
                            "The index value is not an integer. The value is this one : '{}'. From this variable '{y}'. This should be avoided by type checking.",
                            self.stack[y]
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
    pub fn push_method(&mut self, index: Index, arg: String) {
        match self.index_value(index) {
            Value::Address(address) => {
                let value = self.copy_var(&arg);
                self.heap.push_method(address, value);
            }
            Value::Constant(_) => {
                unreachable!(
                    "Push on a contant. This is an error. This should be avoided by type checking."
                )
            }
        }
    }

    // Replace operation of this form : [index].replace([arg])
    pub fn replace_method(&mut self, index: Index, arg: String) {
        todo!()
    }

    // Pop operation of this form : [index].pop()
    pub fn pop_method(&mut self, index: Index) -> Value {
        match self.index_value(index) {
            Value::Address(address) => self.heap.pop_method(address),
            Value::Constant(_) => unreachable!(
                "Pop on a constant. This is an error. This should be avoided by type checking"
            ),
        }
    }

    pub fn copy_var(&mut self, var: &String) -> Value {
        self.heap.copy_value(&self.stack[var])
    }
}

impl Display for Memory {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "\t> Stack :\n{}", self.stack)?;
        writeln!(f, "\t> Heap :\n{}", self.heap)
    }
}
