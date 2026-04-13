use std::mem::MaybeUninit;

use crate::{
    ast::{ast::Ast, block::Block},
    parser::{pos::Pos, span::Span},
};
use logos::{Lexer, Logos, Skip};

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(extras = Pos)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
pub enum Token {
    #[token("(*", skip_comments)]
    Comments,
    // KEYWORDS
    #[token("fn")]
    Fn,
    #[token("return")]
    Return,
    #[token("let")]
    Let,
    // TYPE
    #[token("int")]
    Int,
    #[token("float")]
    Float,
    #[token("char")]
    Char,
    #[token("bool")]
    Bool,

    // SYMBOLS
    #[token(":")]
    Colon,
    #[token(";")]
    SemiColon,
    #[token(",")]
    Comma,
    #[token("(")]
    ParenOpen,
    #[token(")")]
    ParenClose,
    #[token("[")]
    BracketOpen,
    #[token("]")]
    BracketClose,
    #[token("{")]
    BraceOpen,
    #[token("}")]
    BraceClose,
    #[token(".")]
    Period,
    #[token("^")]
    Carrot,

    // Operator
    #[token("+")]
    Add,
    #[token("-")]
    Sub,
    #[token("*")]
    Mult,
    #[token("/")]
    Div,

    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    Number(f64),
    #[regex("(_|[a-zA-Z])(_|[0-9]|[a-zA-Z])*", |lex| lex.slice().to_owned())]
    Identifier(String),
}

fn skip_comments(lexer: &mut Lexer<Token>) -> Skip {
    // lexer.fold(init, f)
    Skip {}
}

// #[derive(Clone, Debug)]
// pub struct CompleteToken {
//     token: Token,
//     fragment: String,
//     pos: FilePosition,
// }
#[derive(Debug)]
pub enum ParseError {
    Incomplete(String),
    Complete,
    Error,
    Unknown,
}

/// A given [Token] implements the Parser trait because of
impl Parser<Pos> for Token {
    fn parse(self, input: &mut Lexer<Token>) -> Result<Pos, ParseError> {
        let past_input = input.clone();
        match input.next() {
            Some(Ok(t)) => {
                if t == self {
                    let _ = input.next();
                    Ok(input.extras.clone())
                } else {
                    Err(ParseError::Error)
                }
            }
            Some(Err(_)) | None => {
                *input = past_input;
                Err(ParseError::Error)
            }
        }
    }
}

/// A Given [&str] implements the [Parser] trait
/// by looking at the next token to see if it's string
/// representation is the same as the string
impl Parser<Pos> for &str {
    fn parse(self, input: &mut Lexer<Token>) -> Result<Pos, ParseError> {
        let past_input = input.clone();
        match input.next() {
            Some(Ok(o)) => {
                if input.slice() == self {
                    Ok(input.extras.clone())
                } else {
                    Err(ParseError::Unknown)
                }
            }
            Some(Err(_)) | None => {
                *input = past_input;
                Err(ParseError::Unknown)
            }
        }
    }
}

/// `[p1, ..., pn]` implements the [Parser] trait
/// it is a sequence that tries in order the parsers
/// - returns an array of output if all are sucessfull
/// - retuns an error if any of them fails
impl<O, T, const N: usize> Parser<[O; N]> for [T; N]
where
    T: Parser<O>,
{
    fn parse(self, input: &mut Lexer<Token>) -> Result<[O; N], ParseError> {
        let mut res: [MaybeUninit<O>; N] = [const { MaybeUninit::uninit() }; N];
        for (i, p) in self.into_iter().enumerate() {
            res[i].write(p.parse(input)?);
        }
        Ok(res.map(|o| unsafe { o.assume_init() }))
    }
}

/// `alt([p1, ..., pn])` test in order the parser
/// - returns the first sucessfull one
/// - returns an error if all are unsucessfull
fn alt<O, const N: usize>(parsers: [impl Parser<O>; N]) -> impl Parser<O> {
    move |input: &mut Lexer<Token>| {
        for parser in parsers {
            match parser.parse(input) {
                Ok(o) => return Ok(o),
                Err(_) => {}
            }
        }
        Err(ParseError::Error)
    }
}

/// Many Parser
///
/// repeats a given and returns a vector of the input
fn many<O>(parser: impl Parser<O>) -> impl Parser<Vec<O>> {
    move |input: &mut Lexer<Token>| {
        let mut res = vec![];
        loop {}
        Ok(res)
    }
}

/// Any function of type (&mut Tokens) -> Result<O, ParseError>
/// implements the [Parser] trait.
impl<T, O> Parser<O> for T
where
    T: FnOnce(&mut Lexer<Token>) -> Result<O, ParseError>,
{
    fn parse(self, input: &mut Lexer<Token>) -> Result<O, ParseError> {
        self(input)
    }
}

/// [Parser] trait is a trait to write
/// parser combinators
pub trait Parser<O> {
    fn parse(self, input: &mut Lexer<Token>) -> Result<O, ParseError>;
    fn apply<S, F>(self, f: F) -> impl Parser<S>
    where
        F: Fn(O) -> S,
        Self: Sized,
    {
        move |input: &mut Lexer<Token>| self.parse(input).map(|o| f(o))
    }
}

fn ast_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<Ast<Span<&'src str>>, ParseError> {
    ["fn", "{", "}"]
        .apply(|[_, _, _]| Ast {
            block: Block { statements: vec![] },
            top_level: vec![],
        })
        .parse(input)
}
pub fn smolscript_parser<'src>(input: &'src str) -> Result<Ast<Span<&'src str>>, ParseError> {
    let mut lexer = Token::lexer(input);
    let ast = ast_parser(&mut lexer);
    if lexer.next().is_none() {
        // Todo better error
        Err(ParseError::Error)
    } else {
        ast
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        let input = "fn f() { let a = 4; } ";
    }
}
