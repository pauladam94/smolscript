use alloc::vec::Vec;
use core::ops::{Index, IndexMut};
pub type ArenaId = u16;

pub struct Arena<T> {
    pub items: Vec<T>,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
        }
    }
    pub fn alloc(&mut self, node: T) -> ArenaId {
        self.items.push(node);
        return (self.items.len() - 1) as u16;
    }
    pub fn iter(&self) -> () {
        todo!()
    }
}

impl<T> Index<ArenaId> for Arena<T> {
    type Output = T;

    fn index(&self, index: ArenaId) -> &Self::Output {
        &self.items[index as usize]
    }
}

impl<T> IndexMut<ArenaId> for Arena<T> {
    fn index_mut(&mut self, index: ArenaId) -> &mut Self::Output {
        &mut self.items[index as usize]
    }
}
