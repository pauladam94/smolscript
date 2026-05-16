use crate::parser::lexer::CurrentLexerPos;
use crate::parser::pos::Pos;
use crate::parser::{error::ParseError, lexer::Parser};

#[derive(Clone)]
pub struct Alt<T> {
    parsers: T,
}

pub fn alt<T>(parsers: T) -> Alt<T> {
    Alt { parsers }
}

macro_rules! impl_alt_parser {
    // Entry point
    ($( $name:ident ),+) => {
        impl<I, O, $( $name ),+> Parser<I> for Alt<($( $name, )+)>
        where I : Clone + CurrentLexerPos, $( $name: Parser<I, Output = O>, )+
        {
            type Output = O;
            #[allow(non_snake_case)]
            fn parse(&mut self, input: &mut I) -> Result<Self::Output, ParseError> {
                let Alt {
                    parsers: ($( $name, )+),
                } = self;
                let mut furthest_error = ParseError::EMPTY;
                let mut furthest_pos = Pos { line: 0, column: 0 };
                // todo this clone can be avoided
                let mut furthest_input : I = input.clone();
                $(
                    {
                        let past_input = input.clone();

                        match $name.parse(input) {
                            Ok(o) => return Ok(o),
                            Err(e) => {
                                if input.pos() > furthest_pos {
                                    furthest_pos = input.pos();
                                    furthest_error = e;
                                    furthest_input = past_input.clone();
                                }
                                *input = past_input;
                            }
                        }
                    }
                )+

                *input = furthest_input;
                Err(furthest_error.with_error(format!("All possibilities in an 'alt' combinator failed.
                        The parser that did go the furthest to {}.", furthest_pos)))
            }
        }
    };
}

macro_rules! impl_alt_parser_tuples {
    ($head:ident $(, $tail:ident)*) => {
        impl_alt_parser!($head $(, $tail)*);
        impl_alt_parser_tuples!($( $tail ),*);
    };
    () => {};
}

impl_alt_parser_tuples!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16
);
