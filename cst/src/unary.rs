use crate::CSTUnary;
use std::fmt::Display;

impl Display for CSTUnary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CSTUnary::Not => write!(f, "not"),
        }
    }
}
