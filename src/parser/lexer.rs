use crate::parser::{error::ParseError, pos::Pos, span::Span};
use logos::{Lexer, Logos, Skip};
use std::fmt::Display;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(extras = Pos)]
#[logos(skip(r"\n", newline_callback))]
#[logos(skip(r"[ \t\f]+"))] // Ignore this regex pattern between tokens
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

    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?")]
    Number,
    // #[regex("[a-zA-Z]+")]
    // Identifier,
    #[regex("(_|[a-zA-Z])(_|[0-9]|[a-zA-Z])*")]
    Identifier,
}

/// Update the line count and the char index.
fn newline_callback(lex: &mut Lexer<Token>) -> Skip {
    lex.extras.line += 1;
    lex.extras.column = lex.span().end as u32;
    Skip
}

pub trait CurrentLexerPos {
    fn pos(&self) -> Pos;
}

impl<'src> CurrentLexerPos for Lexer<'src, Token> {
    fn pos(&self) -> Pos {
        Pos {
            line: self.extras.line,
            column: self.span().end as u32 - self.extras.column,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Comments => write!(f, "comments"),
            Token::Fn => write!(f, "fn"),
            Token::Return => write!(f, "return"),
            Token::Let => write!(f, "let"),
            Token::Int => write!(f, "int"),
            Token::Float => write!(f, "float"),
            Token::Char => write!(f, "char"),
            Token::Bool => write!(f, "bool"),
            Token::Colon => write!(f, ":"),
            Token::SemiColon => write!(f, ";"),
            Token::Comma => write!(f, ","),
            Token::ParenOpen => write!(f, "("),
            Token::ParenClose => write!(f, ")"),
            Token::BracketOpen => write!(f, "["),
            Token::BracketClose => write!(f, "]"),
            Token::BraceOpen => write!(f, "{{"),
            Token::BraceClose => write!(f, "}}"),
            Token::Period => write!(f, "."),
            Token::Carrot => write!(f, "^"),
            Token::Add => write!(f, "+"),
            Token::Sub => write!(f, "-"),
            Token::Mult => write!(f, "*"),
            Token::Div => write!(f, "/"),
            Token::Number => write!(f, "number"),
            Token::Identifier => write!(f, "identifier"),
        }
    }
}

fn skip_comments(lexer: &mut Lexer<Token>) -> Skip {
    // lexer.fold(init, f)
    Skip {}
}

/// A given [Token] implements the Parser trait because of
impl<'src> Parser<Lexer<'src, Token>, Span<&'src str>> for Token {
    fn parse(&mut self, input: &mut Lexer<'src, Token>) -> Result<Span<&'src str>, ParseError> {
        match input.next() {
            Some(Ok(t)) => {
                if &t == self {
                    Ok(Span {
                        pos: input.pos(),
                        str: input.slice(),
                    })
                } else {
                    Err(ParseError {
                        errors: vec![format!(
                            "Got the token '{t}' when waiting for the token '{self}'"
                        )],
                    })
                }
            }
            Some(Err(_)) | None => Err(ParseError {
                errors: vec![format!(
                    "No token anymore when waiting for '{self}' at position : {}",
                    input.pos()
                )],
            }),
        }
    }
}

/// A Given [&str] implements the [Parser] trait
/// by looking at the next token to see if it's string
/// representation is the same as the string
impl<'src> Parser<Lexer<'src, Token>, Pos> for &str {
    fn parse(&mut self, input: &mut Lexer<Token>) -> Result<Pos, ParseError> {
        let past_input = input.clone();
        match input.next() {
            Some(Ok(_)) => {
                if &input.slice() == self {
                    Ok(input.pos())
                } else {
                    Err(ParseError {
                        errors: vec![format!(
                            "Expected the &str '{}' but got '{}'",
                            self,
                            input.slice()
                        )],
                    })
                }
            }
            Some(Err(())) | None => {
                *input = past_input;
                Err(ParseError {
                    errors: vec![format!(
                        "Expected the &str '{}' but got no more tokens in stream",
                        self
                    )],
                })
            }
        }
    }
}

/// Any function of type (&mut Tokens) -> Result<O, ParseError>
/// implements the [Parser] trait.
impl<I, T, O> Parser<I, O> for T
where
    T: FnMut(&mut I) -> Result<O, ParseError>,
{
    fn parse(&mut self, input: &mut I) -> Result<O, ParseError> {
        self(input)
    }
}
  
/// Any function of type (&mut Tokens) -> Result<O, ParseError>
/// implements the [Parser] trait.
// impl<I, T, O> Parser<I, O> for T
// where
//     T: Fn(&mut I) -> Result<O, ParseError>,
// {
//     fn parse(&mut self, input: &mut I) -> Result<O, ParseError> {
//         self(input)
//     }
// }
/// [Parser] trait is a trait to write
/// parser combinators
pub trait Parser<I, O> {
    fn parse(&mut self, input: &mut I) -> Result<O, ParseError>;

    fn apply<S, F>(mut self, mut f: F) -> impl Parser<I, S>
    where
        Self: Sized,
        F: FnMut(O) -> S,
    {
        move |input: &mut I| self.parse(input).map(|o| f(o))
    }
}
