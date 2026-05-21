use crate::{
    ast::index::Index,
    parser::{
        combinator::repeat::repeat,
        error::ParseError,
        lexer::{Parser, Token},
        span::Span,
    },
};
use logos::Lexer;

pub fn index_parser<'src>(
    input: &mut Lexer<'src, Token>,
) -> Result<Index<Span<&'src str>>, ParseError> {
    (
        Token::Identifier,
        repeat(0.., ("[", Token::Identifier, "]").map(|(_, v, _)| v)),
    )
        .map(|(var, index)| Index { var, index })
        .parse(input)
}
