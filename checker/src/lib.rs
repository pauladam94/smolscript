use cst::{CSTTypes, parser::span::Span};
use std::collections::HashMap;

pub struct TypeEnv<'src> {
    mapping: HashMap<Span<'src>, CSTTypes>,
}
