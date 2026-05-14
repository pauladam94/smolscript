use crate::{
    ast::ast::Ast,
    parser::{
        annotate::annotate,
        block::block_parser,
        decl::decl_parser,
        error::ParseError,
        lexer::{Parser, Token},
        span::Span,
    },
};
use logos::{Lexer, Logos};

/// Complete parser of the AST
fn ast_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<Ast<Span<&'src str>>, ParseError> {
    (decl_parser, block_parser)
        .apply(|(top_level, block)| Ast { top_level, block })
        .parse(input)
}

pub fn smolscript_parser<'src>(input_str: &'src str) -> Result<Ast<Span<&'src str>>, ParseError> {
    let mut input = Token::lexer(input_str);
    let ast = ast_parser.parse(&mut input);
    match ast {
        Ok(o) => match input.next() {
            Some(Ok(_)) => Err(ParseError {
                errors: vec![
                    format!("Result of parsing is :\n{}\n", o),
                    annotate(input_str, input, "There is some token left."),
                ],
            }),
            Some(Err(())) => Err(ParseError {
                errors: vec![annotate(
                    input_str,
                    input,
                    "Error in the lexer. More text but no more token.",
                )],
            }),
            None => Ok(o),
        },
        Err(mut e) => {
            e.errors.push(annotate(input_str, input, "Parsing Error."));
            Err(e)
        }
    }
}
