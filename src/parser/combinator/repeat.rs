use crate::parser::lexer::Parser;
use std::ops::{Bound, RangeBounds};

/// Many Parser
///
/// repeats a given and returns a vector of the input
pub fn repeat<I, O, R>(
    range: R,
    mut parser: impl Parser<I, O>,
    // init: I,
    // fold: impl FnMut(I, O) -> I,
) -> impl Parser<I, Vec<O>>
where
    I: Clone,
    R: RangeBounds<u16> + std::fmt::Debug,
{
    move |input: &mut I| {
        let mut res: Vec<O> = vec![];
        let mut i: u16 = 0;
        loop {
            // Above end bound
            match range.end_bound() {
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
            match parser.parse(input) {
                Ok(o) => res.push(o),
                Err(mut e) => {
                    if range.contains(&i) {
                        *input = past_input;
                        return Ok(res);
                    }
                    e.errors.push(format!(
                        "Error Inside Repeat because {} is not inside {:?}",
                        i, range
                    ));
                    return Err(e);
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
