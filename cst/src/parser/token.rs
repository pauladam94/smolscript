use crate::{
    lexer::{CurrentLexerPos, Token},
    parser::{Parser, error::ParseError, span::Span},
};
use logos::Lexer;

/// A given [Token] implements the [Parser] trait.
/// The parser just check if the next token is the one given or returns an
/// error.
impl<'src> Parser<Lexer<'src, Token>> for Token {
    type Output = Span<'src>;
    fn parse(&mut self, input: &mut Lexer<'src, Token>) -> Result<Span<'src>, ParseError> {
        match input.next() {
            Some(Ok(t)) => {
                if &t == self {
                    Ok(Span {
                        pos: input.pos(),
                        ident: input.slice(),
                    })
                } else {
                    Err(ParseError::new().with_error(format!(
                        "Got the token '{t}' when waiting for the token '{self}'"
                    )))
                }
            }
            Some(Err(_)) | None => Err(ParseError::new().with_error(format!(
                "No token anymore when waiting for '{self}' at position : {}",
                input.pos()
            ))),
        }
    }
}
