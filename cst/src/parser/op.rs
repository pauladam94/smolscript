use crate::{
    binop::CSTBinary,
    expr::CSTExpr,
    lexer::Token,
    parser::{Parser, combinator::alt::alt, error::ParseError},
};
use logos::Lexer;

pub fn op_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<CSTExpr<'src>, ParseError> {
    (
        Token::Identifier,
        alt((
            "==".map(|_| CSTBinary::Equal),
            "+".map(|_| CSTBinary::Add),
            "-".map(|_| CSTBinary::Sub),
        )),
        Token::Identifier,
    )
        .map(|(lhs, op, rhs)| CSTExpr::BinaryOp { lhs, op, rhs })
        .parse(input)
}
