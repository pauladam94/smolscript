use logos::Lexer;

use crate::{
    ast::types::Types,
    parser::{
        combinator::separated::separated,
        error::ParseError,
        lexer::{Parser, Token},
        span::Span,
        types::types_parser,
    },
};

pub fn args_parser<'src>(
    input: &mut Lexer<'src, Token>,
) -> Result<Vec<(Span<&'src str>, Types)>, ParseError> {
    separated(
        0..,
        (Token::Identifier, ":", types_parser).map(|(var, _, t)| (var, t)),
        ",",
    )
    .parse(input)
}
