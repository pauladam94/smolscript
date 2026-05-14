use crate::parser::lexer::CurrentLexerPos;
use crate::parser::pos::Pos;
use crate::parser::{
    error::ParseError,
    lexer::{Parser, Token},
};
use logos::Lexer;

pub struct Alt<T> {
    parsers: T,
}

pub fn alt<T>(parsers: T) -> Alt<T> {
    Alt { parsers }
}

macro_rules! impl_alt_parser {
    // Entry point
    ($( $name:ident ),+) => {
        impl<'src, O, $( $name ),+> Parser<'src, O> for Alt<($( $name, )+)>
        where $( $name: Parser<'src, O>, )+
        {
            #[allow(non_snake_case)]
            fn parse(&mut self, input: &mut Lexer<'src, Token>) -> Result<O, ParseError> {
                let Alt {
                    parsers: ($( $name, )+),
                } = self;
                let mut furthest_error = ParseError::EMPTY;
                let mut furthest_pos = Pos { line: 0, column: 0 };
                $(
                    {
                        let past_input = input.clone();

                        match $name.parse(input) {
                            Ok(o) => return Ok(o),
                            Err(e) => {
                                if input.pos() > furthest_pos {
                                    furthest_pos = input.pos();
                                    furthest_error = e;
                                }
                                *input = past_input;
                            }
                        }
                    }
                )+

                furthest_error.errors.push(
                    format!("All possibilities in an 'alt' combinator failed.
                        The parser that did go the furthest to {}.", furthest_pos)
                );
                Err(furthest_error)
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
