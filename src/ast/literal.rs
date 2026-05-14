use std::fmt::Display;

use crate::structural_eq::StructuralEq;

#[derive(Debug)]
pub enum Literal {
    /// Empty vector written : `[]`
    EmptyVec,
    /// Integers
    Int(u64),
    /// Float
    Float(f64),
    /// Strings
    String(String),
    /// Unit
    Unit,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::EmptyVec => write!(f, "[]"),
            Literal::Int(i) => write!(f, "{i}"),
            Literal::Float(fl) => write!(f, "{fl}"),
            Literal::String(s) => write!(f, "{s}"),
            Literal::Unit => write!(f, "()"),
        }
    }
}

impl StructuralEq for Literal {
    fn structural_eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Literal::EmptyVec, Literal::EmptyVec) => true,
            (Literal::Int(i0), Literal::Int(i1)) => i0 == i1,
            (Literal::Float(f0), Literal::Float(f1)) => f0 == f1,
            (Literal::String(s0), Literal::String(s1)) => s0 == s1,
            (Literal::Unit, Literal::Unit) => true,
            (_, _) => false,
        }
    }
}
