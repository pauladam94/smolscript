use crate::{
    binop::CSTBinary,
    expr::CSTExpr,
    parser::{
        combinator::alt::alt,
        error::ParseError,
        lexer::{Parser, Token},
    },
};
use logos::Lexer;

pub fn op_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<CSTExpr<'src>, ParseError> {
    use Token::Identifier;
    alt((
        (Identifier, "+", Identifier).map(|(lhs, _, rhs)| CSTExpr::BinaryOp {
            op: CSTBinary::Add,
            lhs,
            rhs,
        }),
        (Identifier, "-", Identifier).map(|(lhs, _, rhs)| CSTExpr::BinaryOp {
            op: CSTBinary::Sub,
            lhs,
            rhs,
        }),
    ))
    .parse(input)
    // todo!()
}
