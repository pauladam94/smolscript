use crate::{
    CSTConstant, CSTExpr,
    lexer::Token,
    parser::{Parser, block::block_parser, error::ParseError, expr::expr_parser},
    parser_combinator::{alt::alt, cut::cut, opt::opt},
};
use alloc::{boxed::Box, collections::vec_deque::VecDeque};
use logos::Lexer;

pub fn if_else_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<CSTExpr<'src>, ParseError> {
    (
        "if",
        cut((
            "(",
            expr_parser,
            ")",
            "{",
            block_parser,
            "}",
            opt((
                "else",
                alt((
                    ("{", block_parser, "}").map(|(_, block, _)| block),
                    expr_parser.map(|expr| VecDeque::from([expr])),
                )),
            )
                .map(|(_, no)| no)),
        )),
    )
        .map(|(_, (_, cond, _, _, yes, _, no))| CSTExpr::If {
            cond: Box::new(cond),
            yes,
            no: no.unwrap_or(VecDeque::from([CSTExpr::Constant(CSTConstant::Unit)])),
        })
        .parse(input)
}
