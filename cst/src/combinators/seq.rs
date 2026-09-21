use crate::parser::{Parser, error::ParseError};

macro_rules! impl_seq_parser {
    ($([$o:ident, $t:ident]),+) => {
        impl<I, $($o),+, $($t),+> Parser<I> for ($($t),+)
        where
            $($t: Parser<I, Output=$o>),+ ,
        {
            type Output =  ($($o),+);

            #[allow(non_snake_case)]
            fn parse(&mut self, input: &mut I) -> Result<($($o),+), ParseError> {
                let ($($t),+) = self;

                $(
                    let $t = $t.parse(input)?;
                )+
                Ok(($($t),+))
            }
        }
    };
    () => {};
}

macro_rules! impl_seq_parser_tuples {
    ([$head_o:ident, $head_t:ident]) => {};
    ([$head_o:ident, $head_t:ident] $(, [$tail_o:ident, $tail_t:ident])*) => {
        impl_seq_parser!([$head_o, $head_t] $(, [$tail_o, $tail_t])*);
        impl_seq_parser_tuples!($([$tail_o, $tail_t]),*);
    };
    () => {};
}

impl_seq_parser_tuples!(
    [O1, T1],
    [O2, T2],
    [O3, T3],
    [O4, T4],
    [O5, T5],
    [O6, T6],
    [O7, T7],
    [O8, T8],
    [O9, T9],
    [O10, T10],
    [O11, T11],
    [O12, T12]
);
