use crate::ast_interpreter::value::Value;
use alloc::vec::Vec;

pub mod continuation;
pub mod continuation_stack;
pub mod interpreter;
pub mod memory;
pub mod state;
pub mod value;

pub type Region = Vec<Value>;
