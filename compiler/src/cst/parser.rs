use crate::{
    cst::{
        CST, CSTConstant, CSTDecl, CSTExpr, CSTIndex, CSTMethod,
        lexer::{CurrentLexerPos, Token, pos::Pos},
        parser::{error::ParseError, expr::expr_parser, span::Span},
        parser_combinators::{alt::alt, cut::cut, many::many, opt::opt, separated::separated},
    },
    types::Type,
};
use alloc::{boxed::Box, collections::vec_deque::VecDeque, format, string::ToString};
use logos::Lexer;

pub mod error;
pub mod span;

pub mod expr;
pub mod op;

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
        Map { parser: self, f }
    }

    fn context(self, info: &'static str) -> Context<Self>
    where
        Self: Sized,
    {
        Context { parser: self, info }
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
    info: &'static str,
}

impl<I, O, P> Parser<I> for Context<P>
where
    P: Parser<I, Output = O>,
{
    type Output = O;

    fn parse(&mut self, input: &mut I) -> Result<Self::Output, ParseError> {
        self.parser
            .parse(input)
            .map_err(|e| e.with_error(self.info.to_string()))
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

/// A Given [&str] implements the [Parser] trait
/// by looking at the next token to see if it's string
/// representation is the same as the string
impl Parser<Lexer<'_, Token>> for &str {
    type Output = Pos;
    fn parse(&mut self, input: &mut Lexer<Token>) -> Result<Pos, ParseError> {
        let past_input = input.clone();
        match input.next() {
            Some(Ok(_)) => {
                if &input.slice() == self {
                    Ok(input.pos())
                } else {
                    Err(ParseError::default().with_error(format!(
                        "Expected the &str '{}' but got '{}'",
                        self,
                        input.slice()
                    )))
                }
            }
            Some(Err(())) | None => {
                *input = past_input;
                Err(ParseError::default().with_error(format!(
                    "Expected the &str '{self}' but got no more tokens in stream",
                )))
            }
        }
    }
}

/// Complete parser of the AST
pub fn complete_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<CST<'src>, ParseError> {
    (decl_parser, block_parser)
        .map(|(top_level, block)| CST {
            top_level,
            expr: block,
        })
        .parse(input)
}

/// Parsing of a block in the AST
pub fn block_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<CSTExpr<'src>, ParseError> {
    separated(expr_parser, ";")
        .map(|block| {
            if block.len() == 1 {
                block.into_iter().next().unwrap()
            } else {
                CSTExpr::Block(block)
            }
        })
        .context("While parsing a block")
        .parse(input)
}

///
pub fn decl_parser<'src>(
    input: &mut Lexer<'src, Token>,
) -> Result<VecDeque<CSTDecl<'src>>, ParseError> {
    many(
        (
            "fn",
            cut((
                Token::Identifier,
                "(",
                args_parser,
                ")",
                opt(("->", types_parser).map(|(_, t)| t)).map(|t| t.unwrap_or(Type::unit())),
                "{",
                block_parser,
                "}",
            )),
        )
            .map(|(_, (name, _, args, _, output_type, _, expr, _))| CSTDecl {
                name,
                args,
                expr,
                return_type: output_type,
            }),
    )
    .parse(input)
}

pub fn args_parser<'src>(
    input: &mut Lexer<'src, Token>,
) -> Result<VecDeque<(Span<'src>, Type)>, ParseError> {
    separated(
        (Token::Identifier, cut((":", types_parser))).map(|(var, (_, t))| (var, t)),
        ",",
    )
    .context("While parsing arguments")
    .parse(input)
}

pub fn if_else_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<CSTExpr<'src>, ParseError> {
    (
        "if",
        cut((
            "(",
            expr_parser,
            ")",
            "{",
            block_parser,
            "}",
            opt((
                "else",
                cut(alt((
                    ("{", block_parser, "}").map(|(_, block, _)| block),
                    expr_parser,
                ))),
            )
                .map(|(_, no)| no)),
        )),
    )
        .map(|(if_pos, (_, cond, _, _, yes, _, no))| CSTExpr::If {
            cond: Box::new(cond),
            yes: Box::new(yes),
            no: Box::new(no.unwrap_or(CSTExpr::Constant(CSTConstant::Unit))),
            pos: if_pos,
        })
        .parse(input)
}

pub fn index_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<CSTIndex<'src>, ParseError> {
    (
        Token::Identifier,
        many(("[", Token::Identifier, "]").map(|(_, v, _)| v)),
    )
        .map(|(var, index)| CSTIndex { var, index })
        .parse(input)
}

pub fn literal_parser<'src>() -> impl Parser<Lexer<'src, Token>, Output = CSTConstant> {
    alt((
        ("(", ")").map(|_| CSTConstant::Unit),
        ("false").map(|_| CSTConstant::Bool(false)),
        ("true").map(|_| CSTConstant::Bool(true)),
        Token::Number.map(|span| match span.ident.parse::<u64>() {
            Ok(i) => CSTConstant::Int(i),
            Err(_) => CSTConstant::Float(span.ident.parse::<f32>().unwrap()),
        }),
    ))
}

pub fn method_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<CSTMethod, ParseError> {
    alt((
        "replace".map(|_| CSTMethod::Replace),
        "push".map(|_| CSTMethod::Push),
        "pop".map(|_| CSTMethod::Pop),
        "len".map(|_| CSTMethod::Len),
    ))
    .parse(input)
}

/// A given [Token] implements the [Parser] trait.
/// The parser just check if the next token is the one given or returns an
/// error.
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
                    Err(ParseError::default().with_error(format!(
                        "Got the token '{t}' when waiting for the token '{self}'"
                    )))
                }
            }
            Some(Err(_)) | None => Err(ParseError::default().with_error(format!(
                "No token anymore when waiting for '{self}' at position : {}",
                input.pos()
            ))),
        }
    }
}

pub fn types_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<Type, ParseError> {
    alt((
        "u64".map(|_| Type::U64),
        "u32".map(|_| Type::U32),
        "u16".map(|_| Type::U16),
        "u8".map(|_| Type::U8),
        "bool".map(|_| Type::BOOL),
        ("(", ")").map(|_| Type::UNIT),
        ("[", cut((types_parser, "]"))).map(|(_, (mut t, _))| {
            t.array_depth += 1;
            t
        }),
    ))
    .parse(input)
}
