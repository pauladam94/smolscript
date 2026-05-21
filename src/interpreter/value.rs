use crate::{ast::literal::Constant, interpreter::address::Address};

pub enum Value {
    Address(Address),
    Constant(Constant),
}
