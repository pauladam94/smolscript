use crate::{
    ast::literal::Constant,
    parser::{
        combinator::alt::alt,
        lexer::{Parser, Token},
    },
};
use logos::Lexer;

pub fn literal_parser<'src>() -> impl Parser<Lexer<'src, Token>, Output = Constant> {
    alt((
        ("(", ")").map(|_| Constant::Unit),
        Token::Number.map(|span| Constant::Float(span.str.parse::<f64>().unwrap())),
    ))
}
