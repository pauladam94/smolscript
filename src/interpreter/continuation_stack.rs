use crate::interpreter::continuation::Continuation;

pub struct ContinuationStack {
    stack: Vec<Continuation>,
}

impl ContinuationStack {
    pub const EMPTY: Self = Self { stack: vec![] };
}
