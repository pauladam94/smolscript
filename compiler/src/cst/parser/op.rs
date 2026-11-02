use crate::cst::{
    CSTBinary, CSTUnary,
    lexer::Token,
    parser::{Parser, error::ParseError},
    parser_combinators::alt::alt,
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
            CSTBinary::Add => 3,
            CSTBinary::Sub => 3,
            CSTBinary::Div => 4,
            CSTBinary::Mult => 4,
            CSTBinary::EqEq => 1,
            CSTBinary::Neq => 1,
            CSTBinary::LessThan => 2,
            CSTBinary::LessThanEqual => 2,
            CSTBinary::MoreThan => 2,
            CSTBinary::MoreThanEqual => 2,
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
            | CSTBinary::Neq
            | CSTBinary::LessThan
            | CSTBinary::LessThanEqual
            | CSTBinary::MoreThan
            | CSTBinary::MoreThanEqual => Associativity::LeftToRight,
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
        "<".map(|_| CSTBinary::LessThan),
        "<=".map(|_| CSTBinary::LessThanEqual),
        ">".map(|_| CSTBinary::MoreThan),
        ">=".map(|_| CSTBinary::MoreThanEqual),
    ))
    .context("While parsing a binary operator")
    .parse(input)
}

pub fn unary_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<CSTUnary, ParseError> {
    alt(("not".map(|_| CSTUnary::Not), "!".map(|_| CSTUnary::Not)))
        .context("While parsing a unary operator")
        .parse(input)
}
