use std::fmt::Display;

use crate::Method;
use cst::method::CSTMethod;

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::Push => write!(f, "push"),
            Method::Pop => write!(f, "pop"),
            Method::Replace => write!(f, "replace"),
        }
    }
}

impl From<CSTMethod> for Method {
    fn from(cst_method: CSTMethod) -> Self {
        match cst_method {
            CSTMethod::Replace => Self::Replace,
            CSTMethod::Push => Self::Push,
            CSTMethod::Pop => Self::Pop,
        }
    }
}
