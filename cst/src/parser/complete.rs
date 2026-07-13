use crate::{
    CST,
    lexer::Token,
    parser::{
        Parser, annotate::annotate, block::block_parser, decl::decl_parser, error::ParseError,
    },
};
use logos::{Lexer, Logos};

/// Complete parser of the AST
pub fn complete_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<CST<'src>, ParseError> {
    (decl_parser, block_parser)
        .map(|(top_level, block)| CST { top_level, block })
        .parse(input)
}


