use crate::{ast::expr::Expr, parser::span::Span};

pub struct Block<Ident> {
    pub statements: Vec<Expr<Ident>>,
}
