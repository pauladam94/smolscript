use std::collections::VecDeque;

use crate::{
    CSTExpr,
    lexer::Token,
    parser::{Parser, error::ParseError, expr::expr_parser},
    parser_combinator::separated::separated,
};
use logos::Lexer;

pub fn block_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<VecDeque<CSTExpr<'src>>, ParseError> {
    separated(0.., expr_parser, ";")
        .context("While parsing a block".to_string())
        .parse(input)
}
