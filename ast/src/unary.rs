use crate::Unary;
use cst::CSTUnary;
use std::fmt::Display;

impl Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unary::Not => write!(f, "not"),
        }
    }
}

impl From<CSTUnary> for Unary {
    fn from(cst_unary: CSTUnary) -> Self {
        match cst_unary {
            CSTUnary::Not => Unary::Not,
        }
    }
}
