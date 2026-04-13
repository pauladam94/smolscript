use crate::ast::{block::Block, decl::Decl};

pub struct Ast<Ident> {
    pub top_level: Vec<Decl<Ident>>,
    pub block: Block<Ident>,
}
