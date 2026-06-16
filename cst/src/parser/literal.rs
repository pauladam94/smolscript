use crate::{
    literal::CSTConstant,
    parser::{
        combinator::alt::alt,
        lexer::{Parser, Token},
    },
};
use logos::Lexer;

pub fn literal_parser<'src>() -> impl Parser<Lexer<'src, Token>, Output = CSTConstant> {
    alt((
        ("(", ")").map(|_| CSTConstant::Unit),
        Token::Number.map(|span| match span.ident.parse::<u16>() {
            Ok(i) => CSTConstant::Int(i),
            Err(_) => CSTConstant::Float(span.ident.parse::<f32>().unwrap()),
        }),
    ))
}
