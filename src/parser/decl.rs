use crate::{
    ast::{decl::Decl, types::Types},
    parser::{
        args::args_parser,
        block::block_parser,
        combinator::{opt::opt, preceded::preceded, repeat::repeat},
        error::ParseError,
        lexer::{Parser, Token},
        span::Span,
        types::types_parser,
    },
};
use logos::Lexer;

pub fn decl_parser<'src>(
    input: &mut Lexer<'src, Token>,
) -> Result<Vec<Decl<Span<&'src str>>>, ParseError> {
    repeat(
        0..,
        (
            "fn",
            Token::Identifier,
            "(",
            args_parser,
            ")",
            opt(preceded("->", types_parser)).apply(|t| t.unwrap_or(Types::Unit)),
            "{",
            block_parser,
            "}",
        )
            .apply(|(_, name, _, args, _, output_type, _, block, _)| Decl {
                name,
                args,
                block,
                output_type,
            }),
    )
    .parse(input)
}
