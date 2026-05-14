use crate::{
    ast::decl::Decl,
    parser::{
        block::block_parser,
        error::ParseError,
        lexer::{Parser, Token},
        span::Span,
    },
};
use logos::Lexer;

pub fn decl_parser<'src>(
    input: &mut Lexer<'src, Token>,
) -> Result<Decl<Span<&'src str>>, ParseError> {
    ("fn", Token::Identifier, "(", ")", "{", block_parser, "}")
        .apply(|(_, var, _, _, _, block, _)| Decl {
            name: var,
            args: vec![],
            block,
        })
        .parse(input)
}
