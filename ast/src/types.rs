use crate::{BasicTypes, Types};

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
