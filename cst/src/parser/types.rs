use crate::{
    lexer::Token,
    parser::{
        Parser,
        combinator::{alt::alt, cut::cut},
        error::ParseError,
    },
    types::CSTTypes,
};
use logos::Lexer;

pub fn types_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<CSTTypes, ParseError> {
    alt((
        "u16".map(|_| CSTTypes::U16),
        "u8".map(|_| CSTTypes::U8),
        ("[", cut((types_parser, "]"))).map(|(_, (mut t, _))| {
            t.depth += 1;
            t
        }),
    ))
    .parse(input)
}
