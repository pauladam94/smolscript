use crate::cst::lexer::pos::Pos;
use alloc::string::String;

#[derive(Debug, Clone)]
pub struct TypeError {
    pub message: String,
    pub pos: Pos,
}
