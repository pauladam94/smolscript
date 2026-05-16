use crate::parser::lexer::Parser;
use std::ops::{Bound, RangeBounds};

/// Many Parser
///
/// repeats a given and returns a vector of the input
pub fn repeat<I, R, P, O>(range: R, parser: P) -> impl Parser<I, Output = Vec<O>>
where
    I: Clone,
    R: RangeBounds<u16> + std::fmt::Debug,
    P: Parser<I, Output = O>,
{
    Repeat { range, parser }
}

struct Repeat<R, P> {
    range: R,
    parser: P,
}

impl<I, R, P, O> Parser<I> for Repeat<R, P>
where
    I: Clone,
    R: RangeBounds<u16> + std::fmt::Debug,
    P: Parser<I, Output = O>,
{
    type Output = Vec<O>;

    fn parse(&mut self, input: &mut I) -> Result<Self::Output, crate::parser::error::ParseError> {
        let mut res: Vec<O> = vec![];
        let mut i: u16 = 0;
        loop {
            // Above end bound
            match self.range.end_bound() {
                Bound::Included(k) => {
                    if &i > k {
                        return Ok(res);
                    }
                }
                Bound::Excluded(k) => {
                    if &i >= k {
                        return Ok(res);
                    }
                }
                Bound::Unbounded => (),
            }
            let past_input = input.clone();
            match self.parser.parse(input) {
                Ok(o) => res.push(o),
                Err(e) => {
                    if e.is_cut() {
                        return Err(e);
                    }
                    if self.range.contains(&i) {
                        *input = past_input;
                        return Ok(res);
                    }
                    return Err(e.with_error(format!(
                        "Error Inside Repeat because {} is not inside {:?}",
                        i, self.range
                    )));
                }
            }

            match i.checked_add(1) {
                Some(n) => i = n,
                None => break,
            }
        }
        Ok(res)
    }
}
