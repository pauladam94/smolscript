use crate::ast::block::Block;

pub struct Decl<S> {
    name: S,
    args: Vec<S>,
    block: Block<S>,
}
