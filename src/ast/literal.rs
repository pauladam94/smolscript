pub enum Literal {
    /// Empty vector written : `[]`
    EmptyVec,
    /// Integers
    Int(u64),
    /// Float
    Float(f64),
    /// Strings
    String(String),
}
