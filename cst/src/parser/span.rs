use crate::lexer::pos::Pos;
use alloc::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Default)]
pub struct Span<'src> {
    pub pos: Pos,
    pub ident: &'src str,
}

impl<'src> Display for Span<'src> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.ident)
    }
}

impl<'src> PartialEq for Span<'src> {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident
    }
}
