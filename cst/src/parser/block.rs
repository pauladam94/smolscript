use crate::{
    CSTBlock,
    lexer::Token,
    parser::{Parser, error::ParseError, expr::expr_parser}, parser_combinator::separated::separated,
};
use logos::Lexer;

pub fn block_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<CSTBlock<'src>, ParseError> {
    separated(0.., expr_parser, ";")
        .map(|statements| CSTBlock { statements })
        .context("While parsing a block".to_string())
        .parse(input)
}
