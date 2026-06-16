use std::fmt::Display;

use crate::structural_eq::StructuralEq;

#[derive(Debug, Clone, PartialEq)]
pub enum CSTConstant {
    /// Unit
    Unit,
    /// Integers
    Int(u16),
    /// Float
    Float(f32),
    /// Strings
    String(String),
}

impl Default for CSTConstant {
    fn default() -> Self {
        Self::Unit
    }
}

impl Display for CSTConstant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CSTConstant::Int(i) => write!(f, "{i}"),
            CSTConstant::Float(fl) => write!(f, "{fl}"),
            CSTConstant::String(s) => write!(f, "'{s}'"),
            CSTConstant::Unit => write!(f, "()"),
        }
    }
}

impl StructuralEq for CSTConstant {
    fn structural_eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (CSTConstant::Int(i0), CSTConstant::Int(i1)) => i0 == i1,
            (CSTConstant::Float(f0), CSTConstant::Float(f1)) => f0 == f1,
            (CSTConstant::String(s0), CSTConstant::String(s1)) => s0 == s1,
            (CSTConstant::Unit, CSTConstant::Unit) => true,
            (_, _) => false,
        }
    }
}
