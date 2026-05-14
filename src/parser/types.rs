use crate::{
    ast::types::Types,
    parser::{
        combinator::alt::alt,
        error::ParseError,
        lexer::{Parser, Token},
    },
};
use logos::Lexer;

pub fn types_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<Types, ParseError> {
    alt((
        "u64".apply(|_| Types::U64),
        ("[", types_parser, "]").apply(|(_, t, _)| Types::Array(Box::new(t))),
    ))
    .parse(input)
}
