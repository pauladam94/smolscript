use crate::{CSTConstant, lexer::Token, parser::Parser, parser_combinator::alt::alt};
use logos::Lexer;

pub fn literal_parser<'src>() -> impl Parser<Lexer<'src, Token>, Output = CSTConstant> {
    alt((
        ("(", ")").map(|_| CSTConstant::Unit),
        ("false").map(|_| CSTConstant::Bool(false)),
        ("true").map(|_| CSTConstant::Bool(true)),
        Token::Number.map(|span| match span.ident.parse::<u64>() {
            Ok(i) => CSTConstant::Int(i),
            Err(_) => CSTConstant::Float(span.ident.parse::<f32>().unwrap()),
        }),
    ))
}
