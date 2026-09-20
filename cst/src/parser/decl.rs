use crate::{
    CSTDecl, CSTExpr, CSTTypes,
    lexer::Token,
    parser::{
        Parser, args::args_parser, block::block_parser, error::ParseError, types::types_parser,
    },
    parser_combinator::{cut::cut, opt::opt, repeat::repeat},
};
use alloc::collections::vec_deque::VecDeque;
use logos::Lexer;

pub fn decl_parser<'src>(
    input: &mut Lexer<'src, Token>,
) -> Result<VecDeque<CSTDecl<'src>>, ParseError> {
    repeat(
        0..,
        (
            "fn",
            cut((
                Token::Identifier,
                "(",
                args_parser,
                ")",
                opt(("->", types_parser).map(|(_, t)| t)).map(|t| t.unwrap_or(CSTTypes::UNIT)),
                "{",
                block_parser,
                "}",
            )),
        )
            .map(
                |(_, (name, _, args, _, output_type, _, block, _))| CSTDecl {
                    name,
                    args,
                    expr: CSTExpr::Block(block),
                    output_type,
                },
            ),
    )
    .parse(input)
}
