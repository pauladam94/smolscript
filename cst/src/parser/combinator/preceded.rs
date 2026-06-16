use crate::parser::{error::ParseError, lexer::Parser};

pub fn preceded<B, E>(begin: B, end: E) -> Preceded<B, E> {
    Preceded { begin, end }
}

pub struct Preceded<B, E> {
    begin: B,
    end: E,
}

impl<I, B, E, O1, O2> Parser<I> for Preceded<B, E>
where
    O1: std::fmt::Debug,
    O2: std::fmt::Debug,
    B: Parser<I, Output = O1>,
    E: Parser<I, Output = O2>,
{
    type Output = O2;

    fn parse(&mut self, input: &mut I) -> Result<Self::Output, ParseError> {
        self.begin.parse(input)?;
        self.end.parse(input)
        // (self.begin, self.end).apply(|(_, o)| o).parse(input)
    }
}
