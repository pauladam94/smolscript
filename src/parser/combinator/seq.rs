use crate::parser::{error::ParseError, lexer::Parser};

macro_rules! impl_seq_parser {
    ($([$o:ident, $t:ident]),+) => {
        impl<I, $($o),+, $($t),+> Parser<I> for ($($t),+)
        where
            $($o : std::fmt::Debug),+ ,
            $($t: Parser<I, Output=$o>),+ ,
        {
            type Output =  ($($o),+);

            #[allow(non_snake_case)]
            fn parse(&mut self, input: &mut I) -> Result<($($o),+), ParseError> {
                let ($($t),+) = self;
                let mut errors : Vec<String> = vec![];

                let mut cpt : i32 = 0;

                $(
                    cpt += 1;
                    let $t = match $t.parse(input) {
                        Ok(o) => {
                            errors.push(format!("{:?}", o));
                             o
                         },
                        Err(mut e) => {
                            e = e
                            .with_error("\n".to_string())
                            .with_error(format!("Sequence combinator failed at his {} parser.", cpt))
                            .with_error(format!("But it has worked for the {} parsers before", cpt - 1));
                            for s in errors {
                                e = e.with_error(format!("\t{s}"));
                            }
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

// macro_rules! inverse_tuples {
//     ([$o:ident, $t:ident],) => {
//         [$o, $t],
//     };
//     ([$head_o:ident, $head_t:ident], $([$tail_o:ident, $tail_t:ident],)*) => {
//         inverse_tuples!($([$tail_o, $tail_t],)*) [$head_o, $head_t],
//     };
//     () => {};
// }

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
