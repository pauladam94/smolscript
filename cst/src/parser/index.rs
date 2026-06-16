use crate::{
    index::CSTIndex,
    parser::{
        combinator::repeat::repeat,
        error::ParseError,
        lexer::{Parser, Token},
    },
};
use logos::Lexer;

pub fn index_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<CSTIndex<'src>, ParseError> {
    (
        Token::Identifier,
        repeat(0.., ("[", Token::Identifier, "]").map(|(_, v, _)| v)),
    )
        .map(|(var, index)| CSTIndex { var, index })
        .parse(input)
}
