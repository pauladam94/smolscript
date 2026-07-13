use crate::{
    CSTIndex,
    lexer::Token,
    parser::{Parser, error::ParseError}, parser_combinator::repeat::repeat,
};
use logos::Lexer;

pub fn index_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<CSTIndex<'src>, ParseError> {
    (
        Token::Identifier,
        repeat(0.., ("[", Token::Identifier, "]").map(|(_, v, _)| v)),
    )
        .map(|(var, index)| CSTIndex { var, index })
        .parse(input)
}
