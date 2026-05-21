use crate::{
    ast::expr::Expr,
    parser::{
        block::block_parser,
        combinator::{alt::alt, cut::cut, opt::opt, repeat::repeat, terminated::terminated},
        error::ParseError,
        index::index_parser,
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
        ("[", cut("]")).map(|_| Expr::EmptyVec),
        literal_parser().map(Expr::Constant),
        ("let", cut((Identifier, "=", block_parser)))
            .map(|(_, (var, _, block))| Expr::LetBindings { var, block }),
        ("while", cut(("(", Identifier, ")", "{", block_parser, "}")))
            .map(|(_, (_, cond, _, _, block, _))| Expr::While { cond, block }),
        (
            "if",
            cut((
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
            )),
        )
            .map(|(_, (_, cond, _, _, yes, _, _, _, no, _))| Expr::If { cond, yes, no }),
        op_parser,
        (
            Identifier,
            "(",
            cut((repeat(0.., terminated(Identifier, opt(","))), ")")),
        )
            .map(|(name, _, (args, _))| Expr::FunCall { name, args }),
        (
            index_parser,
            ".",
            cut((
                Identifier,
                "(",
                repeat(0.., terminated(Identifier, opt(","))),
                ")",
            )),
        )
            .map(|(index, _, (name, _, args, _))| Expr::MethodCall { index, name, args }),
        index_parser.map(Expr::Index),
    ))
    .parse(input)
}
