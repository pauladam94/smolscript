use crate::utils::span::Span;
use alloc::string::String;
use core::fmt::Display;

#[derive(Debug)]
pub struct CompilerError {
    pub message: String,
    pub span: Span,
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.message)
    }
}
