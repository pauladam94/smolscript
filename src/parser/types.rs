use logos::Lexer;

use crate::{
    ast::types::Types,
    parser::{
        combinator::alt::alt, error::ParseError, lexer::{Parser, Token}
    },
};

pub fn types_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<Types, ParseError> {
    alt((
        "u64".map(|_| Types::U64),
        ("[", types_parser, "]").map(|(_, t, _)| Types::Array(Box::new(t))),
    )).parse(input)
}
