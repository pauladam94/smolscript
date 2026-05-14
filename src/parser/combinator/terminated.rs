use logos::Lexer;

use crate::parser::lexer::{Parser, Token};

pub fn terminated<'src, O1, O2>(
    mut begin: impl Parser<'src, O1>,
    mut end: impl Parser<'src, O2>,
) -> impl Parser<'src, O1> {
    // (begin, end).apply(|(o, _)| o)

    move |input: &mut Lexer<'src, Token>| {
        let o1 = match begin.parse(input) {
            Ok(o) => o,
            Err(e) => return Err(e),
        };
        end.parse(input)?;
        Ok(o1)
    }
}



