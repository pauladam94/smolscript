use crate::parser::lexer::{Parser, Token};
use logos::Lexer;

pub fn opt<'src, O>(mut parser: impl Parser<'src, O>) -> impl Parser<'src, Option<O>> {
    move |input: &mut Lexer<'src, Token>| {
        let past_input = input.clone();
        match parser.parse(input) {
            Ok(o) => Ok(Some(o)),
            Err(_) => {
                *input = past_input;
                Ok(None)
            }
        }
    }
}
