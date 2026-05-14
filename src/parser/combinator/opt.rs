use crate::parser::lexer::Parser;

pub fn opt<I: Clone, O>(mut parser: impl Parser<I, O>) -> impl Parser<I, Option<O>> {
    move |input: &mut I| {
        let past_input = input.clone(); // todo remove ?
        match parser.parse(input) {
            Ok(o) => Ok(Some(o)),
            Err(_) => {
                *input = past_input;
                Ok(None)
            }
        }
    }
}
