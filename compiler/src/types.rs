use crate::utils::arena::ArenaId;
use alloc::vec::Vec;

pub type TypeId = ArenaId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeNode {
    U64,
    U32,
    U16,
    U8,
    Bool,
    Unit,
    Array(TypeId),
    Pair(TypeId, TypeId),
    Unknown,
}

// TODO remove
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
    pub store: Vec<TypeNode>,
}

// TODO remove
impl Type {
    pub fn unknown() -> Self {
        Self {
            store: Vec::from([TypeNode::Unknown]),
            root: 0,
        }
    }
    pub fn u64() -> Self {
        Self {
            store: Vec::from([TypeNode::U64]),
            root: 0,
        }
    }
    pub fn u16() -> Self {
        Self {
            store: Vec::from([TypeNode::U16]),
            root: 0,
        }
    }
    pub fn unit() -> Self {
        Self {
            store: Vec::from([TypeNode::Unit]),
            root: 0,
        }
    }
    pub fn bool() -> Self {
        Self {
            store: Vec::from([TypeNode::Bool]),
            root: 0,
        }
    }
    pub fn array(mut self) -> Self {
        self.store.push(TypeNode::Array(self.store.len() - 1));
        self
    }
    pub fn pair(mut self, rhs: Self) -> Self {
        self.store.extend(rhs.store);
        self
    }
    pub fn is_unknown(&self) -> bool {
        todo!()
    }
}
