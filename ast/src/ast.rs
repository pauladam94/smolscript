use crate::{AST, Block, Decl};
use cst::cst::CST;

impl<'src> From<CST<'src>> for AST {
    fn from(cst: CST) -> Self {
        let CST { top_level, block } = cst;
        AST {
            top_level: top_level.into_iter().map(|decl| Decl::from(decl)).collect(),
            block: Block::from(block),
        }
    }
}
