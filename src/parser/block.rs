use crate::{
    ast::{block::Block, expr::Expr, literal::Literal},
    parser::{
        combinator::{opt::opt, repeat::repeat, terminated::terminated},
        error::ParseError,
        expr::expr_parser,
        lexer::{Parser, Token},
        span::Span,
    },
};
use logos::Lexer;

pub fn block_parser<'src>(
    input: &mut Lexer<'src, Token>,
) -> Result<Block<Span<&'src str>>, ParseError> {
    (repeat(0.., terminated(expr_parser, ";")), opt(expr_parser))
        .apply(|(mut statements, last)| {
            match last {
                Some(last) => {
                    statements.push(last);
                }
                None => statements.push(Expr::Literal(Literal::Unit)),
            }
            Block { statements }
        })
        .parse(input)
}
