use crate::{
    CSTBinary, CSTExpr,
    lexer::Token,
    parser::{Parser, error::ParseError},
    parser_combinator::alt::alt,
};
use logos::Lexer;

pub fn op_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<CSTExpr<'src>, ParseError> {
    (
        Token::Identifier,
        alt((
            "!=".map(|_| CSTBinary::NotEqual),
            "==".map(|_| CSTBinary::Equal),
            "+".map(|_| CSTBinary::Add),
            "-".map(|_| CSTBinary::Sub),
        )),
        Token::Identifier,
    )
        .map(|(lhs, op, rhs)| CSTExpr::BinaryOp { lhs, op, rhs })
        .parse(input)
}
