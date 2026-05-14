use std::ops::RangeBounds;

use crate::parser::{
    combinator::{opt::opt, repeat::repeat, terminated::terminated},
    lexer::Parser,
};

pub fn seperated<I, O1, O2, R, P>(
    range: R,
    parser: P,
    sep: impl Parser<I, O2>,
) -> impl Parser<I, Vec<O1>>
where
    P: Parser<I, O1>,
    O1: std::fmt::Debug,
    O2: std::fmt::Debug,
    I: Clone,
    R: RangeBounds<u16> + std::fmt::Debug,
{
    // Separated {}
    (repeat(range, terminated(parser, sep)), opt(parser)).apply(|(mut v, x)| {
        match x {
            Some(x) => v.push(x),
            None => (),
        }
        v
    })
}

// struct Separated {};

// impl Parser<I, O> for Separated {
//     fn parse(&mut self, input: &mut I) -> Result<O, crate::parser::error::ParseError> {
//         todo!()
//     }
// }
