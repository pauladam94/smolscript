use crate::{
    CST, CSTExpr, lexer::Token, parser::{Parser, block::block_parser, decl::decl_parser, error::ParseError},
};
use logos::Lexer;

/// Complete parser of the AST
pub fn complete_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<CST<'src>, ParseError> {
    (decl_parser, block_parser)
        .map(|(top_level, block)| CST {
            top_level,
            expr: CSTExpr::Block(block),
        })
        .parse(input)
}
