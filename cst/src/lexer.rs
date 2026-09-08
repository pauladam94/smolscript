pub mod pos;

use logos::{Lexer, Logos, Skip};
use std::fmt::Display;

use crate::lexer::pos::Pos;

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
            Token::NewLine => writeln!(f),
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
    loop {
        let past_input = input.clone();
        match input.next() {
            Some(Ok(_)) => {
                if input.extras.seen_new_line {
                    *input = past_input;
                    return Skip;
                }
            }
            Some(Err(())) | None => return Skip,
        }
    }
}
