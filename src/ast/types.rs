
pub enum Types {
    U64,
    Array(Box<Types>),
}
