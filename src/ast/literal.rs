use std::fmt::Display;

use crate::structural_eq::StructuralEq;

#[derive(Debug)]
pub enum Constant {
    /// Integers
    Int(u64),
    /// Float
    Float(f64),
    /// Strings
    String(String),
    /// Unit
    Unit,
}

impl Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Constant::Int(i) => write!(f, "{i}"),
            Constant::Float(fl) => write!(f, "{fl}"),
            Constant::String(s) => write!(f, "{s}"),
            Constant::Unit => write!(f, "()"),
        }
    }
}

impl StructuralEq for Constant {
    fn structural_eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Constant::Int(i0), Constant::Int(i1)) => i0 == i1,
            (Constant::Float(f0), Constant::Float(f1)) => f0 == f1,
            (Constant::String(s0), Constant::String(s1)) => s0 == s1,
            (Constant::Unit, Constant::Unit) => true,
            (_, _) => false,
        }
    }
}
