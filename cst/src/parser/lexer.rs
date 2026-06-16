use crate::parser::{error::ParseError, pos::Pos, span::Span};
use logos::{Lexer, Logos, Skip};
use std::fmt::Display;

#[derive(Default, Clone)]
pub struct LexerState {
    pos: Pos,
    seen_new_line: bool,
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(extras = LexerState)]
#[logos(skip(r"[ \t\f]+"))] // Ignore this regex pattern between tokens
pub enum Token {
    #[token("\n", newline_callback)]
    NewLine,
    #[token("/*", skip_multiline_comments)]
    MultiLineComments,
    #[token("*/")]
    EndMultiLineComments,
    #[token("//", skip_line_comments)]
    SingleLineComments,
    // KEYWORDS
    #[token("fn")]
    Fn,
    #[token("return")]
    Return,
    #[token("let")]
    Let,
    // TYPE
    #[token("u64")]
    U64,
    #[token("u32")]
    U32,
    #[token("u16")]
    U16,
    #[token("u8")]
    U8,
    #[token("int")]
    Int,
    #[token("float")]
    Float,
    #[token("char")]
    Char,
    #[token("bool")]
    Bool,

    // SYMBOLS
    #[token("!=")]
    Neq,
    #[token("==")]
    EqEq,
    #[token("=")]
    Eq,
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
    #[token("->")]
    Arrow,

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
    lex.extras.pos.line += 1;
    lex.extras.pos.column = lex.span().end as u32;
    lex.extras.seen_new_line = true;
    Skip
}

pub trait CurrentLexerPos {
    fn pos(&self) -> Pos;
}

impl<'src> CurrentLexerPos for Lexer<'src, Token> {
    fn pos(&self) -> Pos {
        Pos {
            line: self.extras.pos.line,
            column: self.span().end as u32 - self.extras.pos.column,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::NewLine => write!(f, "\n"),
            Token::MultiLineComments => write!(f, "/*"),
            Token::EndMultiLineComments => write!(f, "*/"),
            Token::SingleLineComments => write!(f, "//"),
            Token::Fn => write!(f, "fn"),
            Token::Return => write!(f, "return"),
            Token::Let => write!(f, "let"),
            Token::U64 => write!(f, "u64"),
            Token::U32 => write!(f, "u32"),
            Token::U16 => write!(f, "u16"),
            Token::U8 => write!(f, "u8"),
            Token::Int => write!(f, "int"),
            Token::Float => write!(f, "float"),
            Token::Char => write!(f, "char"),
            Token::Bool => write!(f, "bool"),
            Token::Neq => write!(f, "!="),
            Token::EqEq => write!(f, "=="),
            Token::Eq => write!(f, "="),
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
            Token::Arrow => write!(f, "->"),
            Token::Add => write!(f, "+"),
            Token::Sub => write!(f, "-"),
            Token::Mult => write!(f, "*"),
            Token::Div => write!(f, "/"),
            Token::Number => write!(f, "number"),
            Token::Identifier => write!(f, "identifier"),
        }
    }
}

fn skip_multiline_comments(lexer: &mut Lexer<Token>) -> Skip {
    loop {
        match lexer.next() {
            Some(Ok(t)) => {
                if t == Token::EndMultiLineComments {
                    return Skip;
                }
            }
            Some(Err(())) | None => return Skip,
        }
    }
}
fn skip_line_comments(input: &mut Lexer<Token>) -> Skip {
    input.extras.seen_new_line = false;
    // println!("- Inside skip line comments");

    loop {
        let past_input = input.clone();
        match input.next() {
            Some(Ok(t)) => {
                // println!(
                //     "Seen this token '{t}' | new_line : {}",
                //     input.extras.seen_new_line
                // );
                if input.extras.seen_new_line {
                    *input = past_input;
                    return Skip;
                }
            }
            Some(Err(())) | None => return Skip,
        }
    }
}

/// A given [Token] implements the Parser trait because of
impl<'src> Parser<Lexer<'src, Token>> for Token {
    type Output = Span<'src>;
    fn parse(&mut self, input: &mut Lexer<'src, Token>) -> Result<Span<'src>, ParseError> {
        match input.next() {
            Some(Ok(t)) => {
                if &t == self {
                    Ok(Span {
                        pos: input.pos(),
                        ident: input.slice(),
                    })
                } else {
                    Err(ParseError::new().with_error(format!(
                        "Got the token '{t}' when waiting for the token '{self}'"
                    )))
                }
            }
            Some(Err(_)) | None => Err(ParseError::new().with_error(format!(
                "No token anymore when waiting for '{self}' at position : {}",
                input.pos()
            ))),
        }
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
