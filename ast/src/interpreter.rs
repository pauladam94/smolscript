use crate::interpreter::value::Value;

pub mod continuation;
pub mod continuation_stack;
pub mod interpreter;
pub mod memory;
pub mod state;
pub mod value;

pub type Region = Vec<Value>;