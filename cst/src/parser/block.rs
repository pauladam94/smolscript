use crate::{
    block::CSTBlock,
    lexer::Token,
    parser::{Parser, combinator::separated::separated, error::ParseError, expr::expr_parser},
};
use logos::Lexer;

pub fn block_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<CSTBlock<'src>, ParseError> {
    separated(0.., expr_parser, ";")
        .map(|statements| CSTBlock { statements })
        .context("While parsing a block".to_string())
        .parse(input)
}
