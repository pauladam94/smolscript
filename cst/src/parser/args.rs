use crate::{
    parser::{
        combinator::{cut::cut, separated::separated},
        error::ParseError,
        lexer::{Parser, Token},
        span::Span,
        types::types_parser,
    },
    types::CSTTypes,
};
use logos::Lexer;
use std::collections::VecDeque;

pub fn args_parser<'src>(
    input: &mut Lexer<'src, Token>,
) -> Result<VecDeque<(Span<'src>, CSTTypes)>, ParseError> {
    separated(
        0..,
        (Token::Identifier, cut((":", types_parser))).map(|(var, (_, t))| (var, t)),
        ",",
    )
    .context("While parsing arguments".to_string())
    .parse(input)
}
