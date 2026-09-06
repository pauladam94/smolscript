use crate::{CSTBasicTypes, CSTTypes};

impl CSTTypes {
    pub const UNIT: Self = Self {
        array_depth: 0,
        basic: CSTBasicTypes::Unit,
    };
    pub const BOOL: Self = Self {
        array_depth: 0,
        basic: CSTBasicTypes::Bool,
    };
    pub const U64: Self = Self {
        array_depth: 0,
        basic: CSTBasicTypes::U64,
    };
    pub const U32: Self = Self {
        array_depth: 0,
        basic: CSTBasicTypes::U32,
    };
    pub const U16: Self = Self {
        array_depth: 0,
        basic: CSTBasicTypes::U16,
    };
    pub const U8: Self = Self {
        array_depth: 0,
        basic: CSTBasicTypes::U8,
    };
}
