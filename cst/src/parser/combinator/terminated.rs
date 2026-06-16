use crate::parser::{error::ParseError, lexer::Parser};

pub fn terminated<B, E, O1, O2, I>(begin: B, end: E) -> Terminated<B, E>
where
    O1: std::fmt::Debug,
    O2: std::fmt::Debug,
    B: Parser<I, Output = O1>,
    E: Parser<I, Output = O2>,
{
    Terminated { begin, end }
}

pub struct Terminated<B, E> {
    begin: B,
    end: E,
}

impl<I, B, E, O1, O2> Parser<I> for Terminated<B, E>
where
    O1: std::fmt::Debug,
    O2: std::fmt::Debug,
    B: Parser<I, Output = O1>,
    E: Parser<I, Output = O2>,
{
    type Output = O1;

    fn parse(&mut self, input: &mut I) -> Result<Self::Output, ParseError> {
        let o = self.begin.parse(input)?;
        self.end.parse(input)?;
        Ok(o)
    }
}
