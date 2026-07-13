use crate::value::Value;

pub mod continuation;
pub mod continuation_stack;
pub mod heap;
pub mod interpreter;
pub mod memory;
pub mod stack;
pub mod state;
pub mod value;

pub type Address = usize;
pub type Region = Vec<Value>;
