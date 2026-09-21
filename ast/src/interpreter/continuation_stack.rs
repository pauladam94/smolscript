use std::fmt::Display;
use crate::interpreter::continuation::Continuation;

#[derive(Debug, Clone)]
pub struct ContinuationStack {
    pub stack: Vec<Continuation>,
}

impl ContinuationStack {
    pub const EMPTY: Self = Self { stack: vec![] };

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
