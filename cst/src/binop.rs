use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum CSTBinary {
    Add,
    Sub,
    Div,
    Equal,
}

impl Display for CSTBinary {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            CSTBinary::Add => write!(f, "+"),
            CSTBinary::Sub => write!(f, "-"),
            CSTBinary::Div => write!(f, "/"),
            CSTBinary::Equal => write!(f, "=="),
        }
    }
}
