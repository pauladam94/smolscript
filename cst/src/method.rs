use std::fmt::Display;

#[derive(Debug)]
pub enum CSTMethod {
    Replace,
    Push,
    Pop,
}

impl Display for CSTMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CSTMethod::Replace => write!(f, "replace"),
            CSTMethod::Push => write!(f, "push"),
            CSTMethod::Pop => write!(f, "pop"),
        }
    }
}
