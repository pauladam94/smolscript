use crate::ast::{block::Block, expr::Expr};

pub enum State<Ident> {
    Block(Block<Ident>),
    Expr(Expr<Ident>),
}
