use crate::{
    ast::ast::Ast,
    parser::{
        block::block_parser,
        decl::decl_parser,
        error::ParseError,
        lexer::{CurrentLexerPos, Parser, Token},
        span::Span,
    },
};
use logos::{Lexer, Logos};

/// Complete parser of the AST
fn ast_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<Ast<Span<&'src str>>, ParseError> {
    (decl_parser, block_parser)
        .apply(|(decl, block)| Ast {
            top_level: vec![decl],
            block,
        })
        .parse(input)
}

pub fn smolscript_parser<'src>(input_str: &'src str) -> Result<Ast<Span<&'src str>>, ParseError> {
    let mut input = Token::lexer(input_str);
    let ast = ast_parser.parse(&mut input);
    match ast {
        Ok(o) => match input.next() {
            Some(Ok(token)) => Err(ParseError {
                errors: vec![
                    format!("Result of parsing is :\n{}\n", o),
                    format!(
                        "There is some token left. The next token is : '{}' at position {}",
                        token,
                        input.pos()
                    ),
                ],
            }),
            Some(Err(_)) => Err(ParseError {
                errors: vec![format!("Error in the lexer. More text but no more token.")],
            }),
            None => Ok(o),
        },
        Err(e) => Err(e),
    }
}
