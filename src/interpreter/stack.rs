use crate::interpreter::value::Value;

pub struct Stack<Ident> {
    stack: Vec<(Ident, Value)>,
}

impl<Ident> Stack<Ident> {
    pub const EMPTY: Self = Stack { stack: vec![] };
}
