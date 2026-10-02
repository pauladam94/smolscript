use alloc::{
    fmt::{Display, Formatter, Result}, string::String, vec::Vec,
};

#[derive(Debug, Clone, Default)]
pub struct ParseError {
    errors: Vec<String>,
    kind: ParseErrorKind,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for err in &self.errors {
            writeln!(f, "> {err}")?;
        }
        Ok(())
    }
}

impl ParseError {
    pub const EMPTY: Self = ParseError {
        errors: Vec::new(),
        kind: ParseErrorKind::Error,
    };
    pub fn with_error(mut self, s: String) -> Self {
        self.errors.push(s);
        self
    }
    pub fn unrecovarable(mut self) -> Self {
        self.kind = ParseErrorKind::Unrecoverable;
        self
    }
    pub fn is_cut(&self) -> bool {
        self.kind == ParseErrorKind::Unrecoverable
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ParseErrorKind {
    #[default]
    Error,
    Incomplete,
    Complete,
    Unknown,
    Unrecoverable,
}

impl Display for ParseErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ParseErrorKind::Incomplete => write!(f, "incomplete"),
            ParseErrorKind::Complete => write!(f, "complete"),
            ParseErrorKind::Error => write!(f, "error"),
            ParseErrorKind::Unknown => write!(f, "unkown"),
            ParseErrorKind::Unrecoverable => write!(f, "unrecovarable"),
        }
    }
}
