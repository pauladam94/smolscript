use crate::{lexer::pos::Pos, structural_eq::StructuralEq};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Default)]
pub struct Span<'src> {
    pub pos: Pos,
    pub ident: &'src str,
}

impl<'src> Span<'src> {
    pub fn to_string(self) -> String {
        self.ident.to_string()
    }
}

impl<'src> Display for Span<'src> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ident)
    }
}

impl<'src> StructuralEq for Span<'src> {
    fn structural_eq(&self, rhs: &Self) -> bool {
        self.ident == rhs.ident
    }
}

impl<'src> PartialEq for Span<'src> {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident
    }
}
