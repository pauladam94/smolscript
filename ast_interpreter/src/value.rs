use crate::{Address, heap::Heap};
use ast::Constant;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Address(Address),
    Constant(Constant),
}

impl Value {
    pub fn eq(&self, rhs: &Self, heap_lhs: &Heap, heap_rhs: &Heap) -> bool {
        match (self, rhs) {
            (Value::Address(addr_lhs), Value::Address(addr_rhs)) => {
                let region_lhs = &heap_lhs[addr_lhs];
                let region_rhs = &heap_rhs[addr_rhs];

                if region_lhs.len() != region_rhs.len() {
                    return false;
                }

                for (val_lhs, val_rhs) in region_lhs.iter().zip(region_rhs.iter()) {
                    if !val_lhs.eq(val_rhs, heap_lhs, heap_rhs) {
                        return false;
                    }
                }

                true
            }
            (Value::Constant(cons_lhs), Value::Constant(const_rhs)) => cons_lhs == const_rhs,
            (Value::Address(_), Value::Constant(_)) => false,
            (Value::Constant(_), Value::Address(_)) => false,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Address(address) => write!(f, "addr({address})")?,
            Value::Constant(constant) => write!(f, "{constant}")?,
        }
        Ok(())
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::Constant(Constant::default())
    }
}
