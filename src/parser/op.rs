use crate::{
    ast::{
        binop::BinOpKind::{Add, Sub},
        expr::Expr,
    },
    parser::{
        combinator::alt::alt,
        error::ParseError,
        lexer::{Parser, Token},
        span::Span,
    },
};
use logos::Lexer;

pub fn op_parser<'src>(
    input: &mut Lexer<'src, Token>,
) -> Result<Expr<Span<&'src str>>, ParseError> {
    use Token::Identifier;
    alt((
        (Identifier, "+", Identifier).map(|(lhs, _, rhs)| Expr::BinOp { op: Add, lhs, rhs }),
        (Identifier, "-", Identifier).map(|(lhs, _, rhs)| Expr::BinOp { op: Sub, lhs, rhs }),
    ))
    .parse(input)
}
