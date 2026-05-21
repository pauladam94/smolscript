use crate::{
    ast::ast::Ast,
    interpreter::{continuation_stack::ContinuationStack, memory::Memory, state::State},
};

pub struct Interpreter<Ident> {
    current: State<Ident>,
    continutation: ContinuationStack,
    memory: Memory<Ident>,
}

impl<Ident> Interpreter<Ident> {
    pub const fn new(ast: Ast<Ident>) -> Self {
        let state = todo!();
        Self {
            current: state,
            continutation: ContinuationStack::EMPTY,
            memory: Memory::EMPTY,
        }
    }
}

impl<Ident> Iterator for Interpreter<Ident> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
