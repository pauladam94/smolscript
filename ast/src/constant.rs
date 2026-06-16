use crate::Constant;
use cst::literal::CSTConstant;
use std::fmt::{Display, Formatter};

impl Default for Constant {
    fn default() -> Self {
        Self::Unit
    }
}

impl Display for Constant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Constant::Int(i) => write!(f, "{i}"),
            Constant::Bool(b) => write!(f, "{b}"),
            Constant::Unit => write!(f, "()"),
        }
    }
}

impl From<CSTConstant> for Constant {
    fn from(cst_constant: CSTConstant) -> Self {
        match cst_constant {
            CSTConstant::Unit => Constant::Unit,
            CSTConstant::Int(i) => Constant::Int(i),
            CSTConstant::String(s) => todo!(),
            CSTConstant::Float(_) => todo!(),
        }
    }
}
