use crate::ast_interpreter::continuation::Continuation;
use alloc::vec::Vec;
use core::fmt::Display;

#[derive(Debug, Clone)]
pub struct ContinuationStack {
    pub stack: Vec<Continuation>,
}

impl ContinuationStack {
    pub const EMPTY: Self = Self { stack: Vec::new() };

    pub fn len(&self) -> usize {
        self.stack.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn pop(&mut self) -> Option<Continuation> {
        self.stack.pop()
    }
}

impl Display for ContinuationStack {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut iter = self.stack.iter().rev();
        if let Some(first) = iter.next() {
            write!(f, "{first}")?;
            for cont in iter {
                write!(f, " :: {cont}")?;
            }
        }
        Ok(())
    }
}
