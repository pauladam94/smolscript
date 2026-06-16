use logos::Lexer;

use crate::{
    parser::{
        combinator::alt::alt,
        error::ParseError,
        lexer::{Parser, Token},
    },
    types::CSTTypes,
};

pub fn types_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<CSTTypes, ParseError> {
    alt((
        "u16".map(|_| CSTTypes::U16),
        "u8".map(|_| CSTTypes::U8),
        ("[", types_parser, "]").map(|(_, mut t, _)| {
            t.depth += 1;
            t
        }),
    ))
    .parse(input)
}
