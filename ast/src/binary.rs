use cst::binop::CSTBinary;

use crate::Binary;
use std::fmt::Display;

impl Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Binary::Add => write!(f, "+"),
            Binary::Sub => write!(f, "-"),
            Binary::Div => write!(f, "/"),
            Binary::Mult => write!(f, "*"),
            Binary::Equal => write!(f, "=="),
            Binary::LessThan => write!(f, "<"),
        }
    }
}

impl From<CSTBinary> for Binary {
    fn from(cst_binary: CSTBinary) -> Self {
        match cst_binary {
            CSTBinary::Add => Binary::Add,
            CSTBinary::Sub => Binary::Sub,
            CSTBinary::Div => Binary::Div,
        }
    }
}
