use crate::structural_eq::StructuralEq;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CSTTypes {
    pub depth: u8,
    pub basic: CSTBasicTypes,
}

impl CSTTypes {
    pub const UNIT: Self = Self {
        depth: 0,
        basic: CSTBasicTypes::Unit,
    };
    pub const BOOL: Self = Self {
        depth: 0,
        basic: CSTBasicTypes::Bool,
    };
    pub const U16: Self = Self {
        depth: 0,
        basic: CSTBasicTypes::U16,
    };
    pub const U8: Self = Self {
        depth: 0,
        basic: CSTBasicTypes::U8,
    };
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CSTBasicTypes {
    U16,
    U8,
    Unit,
    Bool,
}

impl Display for CSTBasicTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CSTBasicTypes::U16 => write!(f, "u16"),
            CSTBasicTypes::U8 => write!(f, "u8"),
            CSTBasicTypes::Unit => write!(f, "()"),
            CSTBasicTypes::Bool => write!(f, "bool"),
        }
    }
}

impl Display for CSTTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.depth {
            write!(f, "[")?;
        }
        write!(f, "{}", self.basic)?;
        for _ in 0..self.depth {
            write!(f, "]")?;
        }
        Ok(())
    }
}

impl StructuralEq for CSTTypes {
    fn structural_eq(&self, rhs: &Self) -> bool {
        self == rhs
    }
}
