use crate::{
    ast::expr::Expr,
    parser::{
        block::block_parser,
        combinator::alt::alt,
        error::ParseError,
        lexer::{Parser, Token},
        op::op_parser,
        span::Span,
    },
};
use logos::Lexer;
pub fn expr_parser<'src>(
    input: &mut Lexer<'src, Token>,
) -> Result<Expr<Span<&'src str>>, ParseError> {
    alt((
        ("let", Token::Identifier, "=", block_parser)
            .apply(|(_, var, _, block)| Expr::LetBindings { var, block }),
        ("while", "(", Token::Identifier, ")", "{", block_parser, "}")
            .apply(|(_, _, cond, _, _, block, _)| Expr::While { cond, block }),
        (
            "if",
            "(",
            Token::Identifier,
            ")",
            "{",
            block_parser,
            "}",
            "else",
            "{",
            block_parser,
            "}",
        )
            .apply(|(_, _, cond, _, _, yes, _, _, _, no, _)| Expr::If { cond, yes, no }),
        op_parser,
        (Token::Identifier, "(", ")").apply(|(fun, _, _)| Expr::FunCall {
            name: fun,
            args: vec![],
        }),
        Token::Identifier.apply(|var| Expr::Variable(var)),
    ))
    .parse(input)
}
