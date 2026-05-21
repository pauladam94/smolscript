use crate::interpreter::{heap::Heap, stack::Stack};

pub struct Memory<Ident> {
    stack: Stack<Ident>,
    heap: Heap,
}

impl<Ident> Memory<Ident> {
    pub const EMPTY: Self = Memory {
        stack: Stack::EMPTY,
        heap: Heap::EMPTY,
    };
}
