use crate::{
    block::CSTBlock,
    expr::CSTExpr,
    parser::{
        block::block_parser,
        combinator::{alt::alt, cut::cut, opt::opt, repeat::repeat, terminated::terminated},
        error::ParseError,
        index::index_parser,
        lexer::{Parser, Token},
        literal::literal_parser,
        method::method_parser,
        op::op_parser,
    },
};
use Token::Identifier;
use logos::Lexer;
use std::collections::VecDeque;

pub fn expr_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<CSTExpr<'src>, ParseError> {
    alt((
        ("[", cut("]")).map(|_| CSTExpr::EmptyVec),
        literal_parser().map(CSTExpr::Constant),
        (
            "let",
            cut((
                Identifier,
                "=",
                expr_parser.map(|e| CSTBlock {
                    statements: VecDeque::from([e]),
                }),
            )),
        )
            .map(|(_, (var, _, block))| CSTExpr::LetBindings { var, block }),
        ("while", cut(("(", Identifier, ")", "{", block_parser, "}")))
            .map(|(_, (_, cond, _, _, block, _))| CSTExpr::While { cond, block })
            .context("While parsing a 'while' expression".to_string()),
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
            .map(|(_, (_, cond, _, _, yes, _, _, _, no, _))| CSTExpr::If { cond, yes, no })
            .context("While parsing an 'if' expression".to_string()),
        op_parser,
        (
            Identifier,
            "(",
            cut((repeat(0.., terminated(block_parser, opt(","))), ")")),
        )
            .map(|(name, _, (args, _))| CSTExpr::FunCall { name, args })
            .context("While parsing a function call".to_string()),
        (
            index_parser,
            ".",
            cut((
                method_parser,
                "(",
                repeat(0.., terminated(Identifier, opt(","))),
                ")",
            )),
        )
            .map(|(index, _, (method, _, args, _))| CSTExpr::MethodCall {
                index,
                method,
                args,
            })
            .context("While parsing a method call".to_string()),
        index_parser.map(CSTExpr::Index),
    ))
    .context("While parsing an expression".to_string())
    .parse(input)
}
