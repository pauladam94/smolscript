use std::fmt::Display;

#[derive(Debug)]
pub struct ParseError {
    pub errors: Vec<String>,
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
    pub const EMPTY: Self = ParseError { errors: vec![] };
}

pub enum ParseErrorKind {
    Incomplete(String),
    Complete,
    Error,
    Unknown,
}

impl Display for ParseErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErrorKind::Incomplete(s) => write!(f, "incomplete : {s}"),
            ParseErrorKind::Complete => write!(f, "complete"),
            ParseErrorKind::Error => write!(f, "error"),
            ParseErrorKind::Unknown => write!(f, "unkown"),
        }
    }
}
