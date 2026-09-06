use crate::{
    CSTTypes,
    lexer::Token,
    parser::{Parser, error::ParseError},
    parser_combinator::{alt::alt, cut::cut},
};
use logos::Lexer;

pub fn types_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<CSTTypes, ParseError> {
    alt((
        "u64".map(|_| CSTTypes::U64),
        "u32".map(|_| CSTTypes::U32),
        "u16".map(|_| CSTTypes::U16),
        "u8".map(|_| CSTTypes::U8),
        "bool".map(|_| CSTTypes::BOOL),
        "()".map(|_| CSTTypes::UNIT),
        ("[", cut((types_parser, "]"))).map(|(_, (mut t, _))| {
            t.array_depth += 1;
            t
        }),
    ))
    .parse(input)
}
