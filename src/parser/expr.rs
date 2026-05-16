use crate::{
    ast::expr::Expr,
    parser::{
        block::block_parser,
        combinator::{alt::alt, opt::opt, repeat::repeat, terminated::terminated},
        error::ParseError,
        lexer::{Parser, Token},
        literal::literal_parser,
        op::op_parser,
        span::Span,
    },
};
use Token::Identifier;
use logos::Lexer;
pub fn expr_parser<'src>(
    input: &mut Lexer<'src, Token>,
) -> Result<Expr<Span<&'src str>>, ParseError> {
    alt((
        ("[", "]").map(|_| Expr::EmptyVec),
        literal_parser().map(|l| Expr::Constant(l)),
        ("let", Identifier, "=", block_parser)
            .map(|(_, var, _, block)| Expr::LetBindings { var, block }),
        ("while", "(", Identifier, ")", "{", block_parser, "}")
            .map(|(_, _, cond, _, _, block, _)| Expr::While { cond, block }),
        (
            "if",
            "(",
            Identifier,
            ")",
            "{",
            block_parser,
            "}",
            "else",
            "{",
            block_parser,
            "}",
        )
            .map(|(_, _, cond, _, _, yes, _, _, _, no, _)| Expr::If { cond, yes, no }),
        op_parser,
        (
            Identifier,
            "(",
            repeat(0.., terminated(Identifier, opt(","))),
            ")",
        )
            .map(|(name, _, args, _)| Expr::FunCall { name, args }),
        (
            Identifier,
            ".",
            Identifier,
            "(",
            repeat(0.., terminated(Identifier, opt(","))),
            ")",
        )
            .map(|(var, _, name, _, mut args, _)| {
                args.push(var);
                Expr::FunCall { name, args }
            }),
        Identifier.map(|var| Expr::Variable(var)),
    ))
    .parse(input)
}
