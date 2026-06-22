use crate::{
    cst::CST,
    lexer::Token,
    parser::{
        Parser, annotate::annotate, block::block_parser, decl::decl_parser, error::ParseError,
    },
};
use logos::{Lexer, Logos};

/// Complete parser of the AST
fn complete_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<CST<'src>, ParseError> {
    (decl_parser, block_parser)
        .map(|(top_level, block)| CST { top_level, block })
        .parse(input)
}

pub fn cst_parser<'src>(input: &'src str) -> Result<CST<'src>, ParseError> {
    let mut lexer = Token::lexer(input);
    let ast = complete_parser.parse(&mut lexer);
    match ast {
        Ok(o) => match lexer.next() {
            Some(Ok(_)) => Err(ParseError::new()
                .with_error(format!("Result of parsing is :\n{}\n", o))
                .with_error(annotate(input, lexer, "There is some token left."))),
            Some(Err(())) => Err(ParseError::new().with_error(annotate(
                input,
                lexer,
                "Error in the lexer. More text but no more token.",
            ))),
            None => Ok(o),
        },
        Err(e) => Err(e.with_error(annotate(input, lexer, "Parsing Error."))),
    }
}
