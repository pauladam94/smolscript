use crate::{ast::AST, utils::arena::ArenaId};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct TypeId(pub u16);

impl ArenaId for TypeId {
    fn from_usize(index: usize) -> Self {
        Self(index as u16)
    }

    fn to_usize(self) -> usize {
        self.0 as usize
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum TypeNode {
    U64,
    U32,
    U16,
    U8,
    Bool,
    #[default]
    Unit,
    Array(TypeId),
    Tuple {
        head: TypeId,
        tail: TypeId,
    },
    Fun {
        args: TypeId,
        return_type: TypeId,
    },
    Unknown,
    String,
}

pub struct TypeView<'a, 'src> {
    pub id: TypeId,
    pub ast: &'a AST<'src>,
}
