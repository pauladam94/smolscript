use crate::{parser::Parser, parser_combinator::{opt::opt, repeat::repeat}};
use std::{collections::VecDeque, ops::RangeBounds};

pub fn separated<I, O1, O2, R, P, S>(
    range: R,
    parser: P,
    sep: S,
) -> impl Parser<I, Output = VecDeque<O1>>
where
    P: Parser<I, Output = O1> + Clone,
    S: Parser<I, Output = O2>,
    O1: std::fmt::Debug,
    O2: std::fmt::Debug,
    I: Clone,
    R: RangeBounds<u16> + std::fmt::Debug,
{
    (
        repeat(range, ((parser.clone(), sep)).map(|(s, _)| s)),
        opt(parser),
    )
        .map(|(mut v, x)| {
            match x {
                Some(x) => v.push_back(x),
                None => (),
            }
            v
        })
}

