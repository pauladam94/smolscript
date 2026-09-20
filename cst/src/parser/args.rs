use crate::{
    CSTTypes,
    lexer::Token,
    parser::{Parser, error::ParseError, span::Span, types::types_parser},
    parser_combinator::{cut::cut, separated::separated},
};
use alloc::{collections::vec_deque::VecDeque, string::ToString};
use logos::Lexer;

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
