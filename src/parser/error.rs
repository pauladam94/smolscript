use std::fmt::Display;

#[derive(Debug)]
pub struct ParseError {
    errors: Vec<String>,
    kind: ParseErrorKind,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for err in &self.errors {
            writeln!(f, "> {err}")?;
        }
        Ok(())
    }
}

impl ParseError {
    pub const EMPTY: Self = ParseError {
        errors: vec![],
        kind: ParseErrorKind::Error,
    };
    pub const fn new() -> Self {
        Self::EMPTY
    }
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

#[derive(Debug, PartialEq, Eq)]
pub enum ParseErrorKind {
    Incomplete,
    Complete,
    Error,
    Unknown,
    Unrecoverable,
}

impl Display for ParseErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErrorKind::Incomplete => write!(f, "incomplete"),
            ParseErrorKind::Complete => write!(f, "complete"),
            ParseErrorKind::Error => write!(f, "error"),
            ParseErrorKind::Unknown => write!(f, "unkown"),
            ParseErrorKind::Unrecoverable => write!(f, "unrecovarable"),
        }
    }
}
