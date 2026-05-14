use logos::Lexer;

use crate::{
    ast::types::Types,
    parser::{
        combinator::{opt::opt, repeat::repeat, seperated::seperated, terminated::terminated},
        error::ParseError,
        lexer::{Parser, Token},
        span::Span,
        types::types_parser,
    },
};

// pub fn arg_parser<'src>(
//     input: &mut Lexer<'src, Token>,
// ) -> impl Parser<&mut Lexer<'src, Token>, (&'src str, Types)> {
//     (Token::Identifier, types_parser)
// }

pub fn args_parser<'src>(
    input: &mut Lexer<'src, Token>,
) -> Result<Vec<(Span<&'src str>, Types)>, ParseError> {
    // return Ok(vec![]);
    seperated(0.., (Token::Identifier, ":", types_parser).apply(|(var, _, t)| (var, t)), ",").parse(input)
}
