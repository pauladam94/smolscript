use crate::{
    ast::literal::Literal,
    parser::{
        combinator::alt::alt,
        error::ParseError,
        lexer::{Parser, Token},
    },
};
use logos::Lexer;

pub fn literal_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<Literal, ParseError> {
    alt((
        ("(", ")").apply(|_| Literal::Unit),
        Token::Number.apply(|span| Literal::Float(span.str.parse::<f64>().unwrap())),
    ))
    .parse(input)
}
