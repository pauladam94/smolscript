use crate::{
    lexer::{CurrentLexerPos, Token, pos::Pos},
    parser::error::ParseError,
};
use logos::Lexer;

pub mod span;

// Complete AST
pub mod block;
pub mod complete;
pub mod decl;
pub mod expr;
pub mod method;
pub mod token;

pub mod args;
pub mod index;
pub mod literal;
pub mod op;
pub mod types;

// All the combinators useful for parsing
pub mod combinator;

pub mod error;

// Utilities
pub mod annotate;

/// [Parser] trait is a trait to write
/// parser combinators
pub trait Parser<I> {
    type Output;
    fn parse(&mut self, input: &mut I) -> Result<Self::Output, ParseError>;

    fn map<F, S>(self, f: F) -> Map<Self, F>
    // impl Parser<I, Output = S>
    where
        F: Fn(Self::Output) -> S,
        Self: Sized,
    {
        // move |input: &mut I| self.parse(input).map(|o| f(o))
        Map { parser: self, f }
    }

    fn context(self, string: String) -> Context<Self>
    where
        Self: Sized,
    {
        Context {
            parser: self,
            string,
        }
    }
}

#[derive(Clone)]
pub struct Map<P, F> {
    parser: P,
    f: F,
}

impl<I, F, S, O, P> Parser<I> for Map<P, F>
where
    P: Parser<I, Output = O>,
    F: Fn(O) -> S,
{
    type Output = S;

    fn parse(&mut self, input: &mut I) -> Result<Self::Output, ParseError> {
        self.parser.parse(input).map(|o| (self.f)(o))
    }
}

#[derive(Clone)]
pub struct Context<P> {
    parser: P,
    string: String,
}

impl<I, O, P> Parser<I> for Context<P>
where
    P: Parser<I, Output = O>,
{
    type Output = O;

    fn parse(&mut self, input: &mut I) -> Result<Self::Output, ParseError> {
        self.parser
            .parse(input)
            .map_err(|e| e.with_error(self.string.clone()))
    }
}

/// Any function of type (&mut Tokens) -> Result<O, ParseError>
/// implements the [Parser] trait.
impl<I, O, T> Parser<I> for T
where
    T: FnMut(&mut I) -> Result<O, ParseError>,
{
    type Output = O;
    fn parse(&mut self, input: &mut I) -> Result<O, ParseError> {
        self(input)
    }
}

/// A Given [&str] implements the [Parser] trait
/// by looking at the next token to see if it's string
/// representation is the same as the string
impl<'src> Parser<Lexer<'src, Token>> for &str {
    type Output = Pos;
    fn parse(&mut self, input: &mut Lexer<Token>) -> Result<Pos, ParseError> {
        let past_input = input.clone();
        match input.next() {
            Some(Ok(_)) => {
                if &input.slice() == self {
                    Ok(input.pos())
                } else {
                    Err(ParseError::new().with_error(format!(
                        "Expected the &str '{}' but got '{}'",
                        self,
                        input.slice()
                    )))
                }
            }
            Some(Err(())) | None => {
                *input = past_input;
                Err(ParseError::new().with_error(format!(
                    "Expected the &str '{}' but got no more tokens in stream",
                    self
                )))
            }
        }
    }
}
