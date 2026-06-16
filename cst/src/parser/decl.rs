use crate::{
    decl::CSTDecl,
    parser::{
        args::args_parser,
        block::block_parser,
        combinator::{cut::cut, opt::opt, preceded::preceded, repeat::repeat},
        error::ParseError,
        lexer::{Parser, Token},
        types::types_parser,
    },
    types::CSTTypes,
};
use logos::Lexer;
use std::collections::VecDeque;

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
                    block,
                    output_type,
                },
            ),
    )
    .parse(input)
}
