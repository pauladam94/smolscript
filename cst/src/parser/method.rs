use crate::{
    method::CSTMethod, parser::{
        combinator::alt::alt,
        error::ParseError,
        lexer::{Parser, Token},
    }
};
use logos::Lexer;

pub fn method_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<CSTMethod, ParseError> {
    alt((
        "replace".map(|_| CSTMethod::Replace),
        "push".map(|_| CSTMethod::Push),
        "pop".map(|_| CSTMethod::Pop),
    ))
    .parse(input)
}
