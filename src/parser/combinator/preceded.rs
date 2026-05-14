use crate::parser::lexer::Parser;

pub fn preceded<I, O1, O2>(begin: impl Parser<I, O1>, end: impl Parser<I, O2>) -> impl Parser<I, O2>
where
    O1: std::fmt::Debug,
    O2: std::fmt::Debug,
{
    (begin, end).apply(|(_, o)| o)
}
