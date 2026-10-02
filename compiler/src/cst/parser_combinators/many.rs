use crate::cst::parser::{Parser, error::ParseError};
use alloc::collections::vec_deque::VecDeque;

/// Many Parser :
///
/// repeats a given and returns a vector of the input
pub fn many<I, P, O>(parser: P) -> impl Parser<I, Output = VecDeque<O>>
where
    I: Clone,
    P: Parser<I, Output = O>,
{
    Many { parser }
}

struct Many<P> {
    parser: P,
}

impl<I, P, O> Parser<I> for Many<P>
where
    I: Clone,
    P: Parser<I, Output = O>,
{
    type Output = VecDeque<O>;

    fn parse(&mut self, input: &mut I) -> Result<Self::Output, ParseError> {
        let mut res: VecDeque<O> = VecDeque::new();
        loop {
            let past_input = input.clone();
            match self.parser.parse(input) {
                Ok(o) => res.push_back(o),
                Err(e) => {
                    if e.is_cut() {
                        return Err(e);
                    }
                    *input = past_input;
                    return Ok(res);
                }
            }
        }
    }
}
