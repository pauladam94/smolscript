use crate::{CSTBasicTypes, CSTTypes};

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


