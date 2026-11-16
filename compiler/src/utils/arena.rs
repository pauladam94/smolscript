use alloc::vec::Vec;
use core::{
    fmt::{Debug, Formatter, Result},
    marker::PhantomData,
    ops::{Index, IndexMut},
};

/// Implemented by every id type usable with an Arena.
pub trait ArenaId: Copy {
    fn from_usize(index: usize) -> Self;
    fn to_usize(self) -> usize;
}

#[derive(Clone, Default, PartialEq)]
pub struct Arena<T, Id: ArenaId> {
    pub items: Vec<T>,
    // _marker: PhantomData<fn() -> Id>,
    _marker: PhantomData<Id>,
}

impl<T, Id: ArenaId> Arena<T, Id> {
    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl<T, Id: ArenaId> Arena<T, Id> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            _marker: PhantomData,
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
            _marker: PhantomData,
        }
    }
    pub fn alloc(&mut self, node: T) -> Id {
        self.items.push(node);
        return Id::from_usize(self.items.len() - 1);
    }
}

impl<T, Id: ArenaId> Index<Id> for Arena<T, Id> {
    type Output = T;

    fn index(&self, index: Id) -> &Self::Output {
        &self.items[Id::to_usize(index)]
    }
}

impl<T, Id: ArenaId> IndexMut<Id> for Arena<T, Id> {
    fn index_mut(&mut self, index: Id) -> &mut Self::Output {
        &mut self.items[Id::to_usize(index)]
    }
}

impl<T: Debug, Id: Debug + ArenaId> Debug for Arena<T, Id> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Arena {{\n")?;
        for (i, elem) in self.items.iter().enumerate() {
            writeln!(f, "{:<6}|  {:?}", i, elem)?;
        }
        write!(f, "\n}}")
    }
}
