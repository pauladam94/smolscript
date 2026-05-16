use crate::{
    ast::block::Block,
    parser::{
        combinator::separated::separated,
        error::ParseError,
        expr::expr_parser,
        lexer::{Parser, Token},
        span::Span,
    },
};
use logos::Lexer;

pub fn block_parser<'src>(
    input: &mut Lexer<'src, Token>,
) -> Result<Block<Span<&'src str>>, ParseError> {
    separated(0.., expr_parser, ";")
        .map(|statements| Block { statements })
        .parse(input)
}
