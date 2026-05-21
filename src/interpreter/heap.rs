use crate::interpreter::value::Value;

pub struct Heap {
    data: Vec<Region>,
}

pub struct Region {
    array: Vec<Value>,
}

impl Heap {
    pub const EMPTY: Self = Self { data : vec![] };
}
