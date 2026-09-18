use crate::{
    CSTBinary, CSTUnary,
    lexer::Token,
    parser::{Parser, error::ParseError},
    parser_combinator::alt::alt,
};
use logos::Lexer;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Associativity {
    RightToLeft,
    LeftToRight,
}
impl CSTBinary {
    /// Precedence of binary operators following the C standard describe
    /// [here](https://fr.cppreference.com/c/language/operator_precedence)
    pub const fn precedence(&self) -> usize {
        match self {
            CSTBinary::Add => 2,
            CSTBinary::Sub => 2,
            CSTBinary::Div => 3,
            CSTBinary::Mult => 3,
            CSTBinary::EqEq => 1,
            CSTBinary::Neq => 1,
        }
    }
    /// Associativity of binary operators following the C standard describe
    /// [here](https://fr.cppreference.com/c/language/operator_precedence)
    pub const fn associativity(&self) -> Associativity {
        match self {
            CSTBinary::Add
            | CSTBinary::Sub
            | CSTBinary::Div
            | CSTBinary::Mult
            | CSTBinary::EqEq
            | CSTBinary::Neq => Associativity::LeftToRight,
        }
    }
}

pub fn binary_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<CSTBinary, ParseError> {
    alt((
        "+".map(|_| CSTBinary::Add),
        "-".map(|_| CSTBinary::Sub),
        "/".map(|_| CSTBinary::Div),
        "*".map(|_| CSTBinary::Mult),
        "==".map(|_| CSTBinary::EqEq),
        "!=".map(|_| CSTBinary::Neq),
    ))
    .context("While parsing a binary operator".to_string())
    .parse(input)
}

pub fn unary_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<CSTUnary, ParseError> {
    alt(("not".map(|_| CSTUnary::Not), "!".map(|_| CSTUnary::Not)))
        .context("While parsing a unary operator".to_string())
        .parse(input)
}
