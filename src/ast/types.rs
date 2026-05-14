use std::fmt::Display;

use crate::structural_eq::StructuralEq;

#[derive(Debug, PartialEq, Eq)]
pub enum Types {
    U64,
    Unit,
    Array(Box<Types>),
}

impl Display for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Types::U64 => write!(f, "u64"),
            Types::Unit => write!(f, "()"),
            Types::Array(t) => write!(f, "[{}]", t),
        }
    }
}

impl StructuralEq for Types {
    fn structural_eq(&self, rhs: &Self) -> bool {
        self == rhs
    }
}
