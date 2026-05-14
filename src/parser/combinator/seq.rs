use crate::parser::{
    error::ParseError,
    lexer::{Parser, Token},
};
use logos::Lexer;

macro_rules! impl_seq_parser {
    ($([$o:ident, $t:ident]),+) => {
        impl<'src, $($o),+, $($t),+> Parser<'src, ($($o),+)> for ($($t),+)
        where
            $($t: Parser<'src, $o>),+ ,
        {
            #[allow(non_snake_case)]
            fn parse(&mut self, input: &mut Lexer<'src, Token>) -> Result<($($o),+), ParseError> {
                let ($($t),+) = self;
                // let result : [MaybeUninit<>; $n] = [MaybeUninit::uninit(); $n];

                let mut cpt = 0;

                $(
                    cpt += 1;
                    let $t = match $t.parse(input) {
                        Ok(o) => o,
                        Err(mut e) => {
                            e.errors.push(format!("Sequence combinator failed at his {} parser.", cpt));
                            return Err(e)
                        },
                    };
                )+
                Ok(($($t),+))
            }
        }
    };
    () => {};
}

macro_rules! inverse_tuples {
    ([$o:ident, $t:ident],) => {
        [$o, $t],
    };
    ([$head_o:ident, $head_t:ident], $([$tail_o:ident, $tail_t:ident],)*) => {
        inverse_tuples!($([$tail_o, $tail_t],)*) [$head_o, $head_t],
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
