use crate::{
    CSTMethod,
    lexer::Token,
    parser::{Parser, error::ParseError},
    parser_combinator::alt::alt,
};
use logos::Lexer;

pub fn method_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<CSTMethod, ParseError> {
    alt((
        "replace".map(|_| CSTMethod::Replace),
        "push".map(|_| CSTMethod::Push),
        "pop".map(|_| CSTMethod::Pop),
        "len".map(|_| CSTMethod::Len),
    ))
    .parse(input)
}
