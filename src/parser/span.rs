use crate::parser::pos::Pos;

#[derive(Debug, Clone)]
pub struct Span<S> {
    pos: Pos,
    str: S,
}
