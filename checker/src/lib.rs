use std::collections::HashMap;
use cst::{parser::span::Span, types::CSTTypes};

pub struct TypeEnv<'src> {
    mapping: HashMap<Span<'src>, CSTTypes>
}
