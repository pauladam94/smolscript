use crate::parser::{error::ParseError, lexer::Parser};

pub fn cut<P>(parser: P) -> Cut<P> {
    Cut { parser }
}

#[derive(Clone)]
pub struct Cut<P> {
    parser: P,
}

impl<I, P, O> Parser<I> for Cut<P>
where
    P: Parser<I, Output = O>,
{
    type Output = O;

    fn parse(&mut self, input: &mut I) -> Result<Self::Output, ParseError> {
        match self.parser.parse(input) {
            Ok(o) => Ok(o),
            Err(e) => Err(e.unrecovarable()),
        }
    }
}
