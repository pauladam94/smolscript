use crate::{BasicTypes, Types};
use cst::types::{CSTBasicTypes, CSTTypes};
use std::fmt::{Display, Formatter};

impl Types {
    pub const UNIT: Self = Self {
        depth: 0,
        basic: BasicTypes::Unit,
    };
    pub const U16: Self = Self {
        depth: 0,
        basic: BasicTypes::U16,
    };
    pub const BOOL: Self = Self {
        depth: 0,
        basic: BasicTypes::Bool,
    };
}

impl Display for Types {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

impl From<CSTTypes> for Types {
    fn from(cst_types: CSTTypes) -> Self {
        let CSTTypes { depth, basic } = cst_types;

        Types {
            depth,
            basic: BasicTypes::from(basic),
        }
    }
}

impl Display for BasicTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BasicTypes::U16 => write!(f, "u16"),
            BasicTypes::U8 => write!(f, "u8"),
            BasicTypes::Bool => write!(f, "bool"),
            BasicTypes::Unit => write!(f, "unit"),
        }
    }
}

impl From<CSTBasicTypes> for BasicTypes {
    fn from(cst_basic_types: CSTBasicTypes) -> Self {
        match cst_basic_types {
            CSTBasicTypes::U16 => BasicTypes::U16,
            CSTBasicTypes::U8 => BasicTypes::U8,
            CSTBasicTypes::Unit => BasicTypes::Unit,
            CSTBasicTypes::Bool => BasicTypes::Bool,
        }
    }
}
