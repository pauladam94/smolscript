use crate::{Block, Decl, Types};
use cst::decl::CSTDecl;

impl<'src> From<CSTDecl<'src>> for Decl {
    fn from(cst_decl: CSTDecl<'src>) -> Self {
        let CSTDecl {
            name,
            args,
            output_type,
            block,
        } = cst_decl;

        Decl {
            name: name.to_string(),
            args: args
                .into_iter()
                .map(|(var, t)| (var.to_string(), Types::from(t)))
                .collect(),
            output_type: Types::from(output_type),
            block: Block::from(block),
        }
    }
}
