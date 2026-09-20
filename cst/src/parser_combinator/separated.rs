use crate::{
    parser::Parser,
    parser_combinator::{opt::opt, repeat::repeat},
};
use alloc::{collections::vec_deque::VecDeque, fmt::Debug};
use core::ops::RangeBounds;

pub fn separated<I, O1, O2, R, P, S>(
    range: R,
    parser: P,
    sep: S,
) -> impl Parser<I, Output = VecDeque<O1>>
where
    P: Parser<I, Output = O1> + Clone,
    S: Parser<I, Output = O2>,
    O1: Debug,
    O2: Debug,
    I: Clone,
    R: RangeBounds<u16> + Debug,
{
    (
        repeat(range, (parser.clone(), sep).map(|(s, _)| s)),
        opt(parser),
    )
        .map(|(mut v, x)| {
            if let Some(x) = x {
                v.push_back(x);
            }
            v
        })
}
