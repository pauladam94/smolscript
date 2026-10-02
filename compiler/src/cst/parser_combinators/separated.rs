use crate::cst::{
    parser::Parser,
    parser_combinators::{many::many, opt::opt},
};
use alloc::{collections::vec_deque::VecDeque, fmt::Debug};

pub fn separated<I, O1, O2, P, S>(parser: P, sep: S) -> impl Parser<I, Output = VecDeque<O1>>
where
    P: Parser<I, Output = O1> + Clone,
    S: Parser<I, Output = O2>,
    O1: Debug,
    O2: Debug,
    I: Clone,
{
    (many((parser.clone(), sep).map(|(s, _)| s)), opt(parser)).map(|(mut v, x)| {
        if let Some(x) = x {
            v.push_back(x);
        }
        v
    })
}
