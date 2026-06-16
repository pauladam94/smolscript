use crate::parser::{error::ParseError, lexer::Parser};

pub fn opt<P, I, O>(parser: P) -> impl Parser<I, Output = Option<O>>
where
    P: Parser<I, Output = O>,
    I: Clone,
{
    Opt { parser }
}

struct Opt<P> {
    parser: P,
}

impl<P, I, O> Parser<I> for Opt<P>
where
    P: Parser<I, Output = O>,
    I: Clone,
{
    type Output = Option<O>;

    fn parse(&mut self, input: &mut I) -> Result<Self::Output, ParseError> {
        let past_input = input.clone();
        match self.parser.parse(input) {
            Ok(o) => Ok(Some(o)),
            Err(_) => {
                *input = past_input;
                Ok(None)
            }
        }
    }
}
