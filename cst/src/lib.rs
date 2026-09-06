use crate::{
    lexer::Token,
    parser::{
        Parser, annotate::annotate, complete::complete_parser, error::ParseError, span::Span,
    },
};
use logos::Logos;
use std::collections::VecDeque;

/// Here we implement the [Display] trait for the whole [CST].
///
/// In Rust any type that implements the [Display] trait can be used in any
/// of the standard librairy formatting function like [println!] or [write!]
///
/// This code is pretty commun in the tests :
/// ```rust
/// let cst : CST = ...;
/// println!("Here is the concrete syntax tree parsed :\n{}", cst);
/// ```
pub mod formatting;

pub mod lexer;
pub mod parser;
pub mod parser_combinator;

/// Main function of this crate.
///
/// This function parse from an `&str` the concrete syntax tree.
/// This operation can fail and returns us with as much error as possible with
/// the [ParseError] type that can be displayed.
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

#[derive(Debug, Clone)]
pub enum CSTUnary {
    Not,
}

#[derive(Debug)]
pub enum CSTBinary {
    Add,
    Sub,
    Div,
    Equal,
    NotEqual,
}

/// Complete AST of a program
pub struct CST<'src> {
    pub top_level: VecDeque<CSTDecl<'src>>,
    pub expr: CSTExpr<'src>,
}

/// Declaration of a function that is displayed this way :
/// - `fn [name] ([args]) [block]`
#[derive(Debug)]
pub struct CSTDecl<'src> {
    pub name: Span<'src>,
    pub args: VecDeque<(Span<'src>, CSTTypes)>,
    pub output_type: CSTTypes,
    pub expr: CSTExpr<'src>,
}

#[derive(Debug)]
pub enum CSTExpr<'src> {
    // A list of expression that can be viewed as statements
    Block(VecDeque<CSTExpr<'src>>),
    // x or x[y] or x[y][z] ...
    Index(CSTIndex<'src>),
    // Variable(Ident),
    MethodCall {
        index: CSTIndex<'src>,
        method: CSTMethod,
        args: VecDeque<Span<'src>>,
    },
    /// f(x, y, ...)
    /// The builtin ones are :
    /// - x.replace(b)
    /// - x.append(b)
    /// - x.pop()
    FunCall {
        name: Span<'src>,
        args: VecDeque<CSTExpr<'src>>,
    },
    /// let [var] = [block]
    LetBindings {
        var: Span<'src>,
        expr: Box<CSTExpr<'src>>,
    },
    /// while ([cond]) { [block] }
    While {
        cond: Span<'src>,
        block: VecDeque<CSTExpr<'src>>,
    },
    /// if ([cond]) { [yes] } else { [no] }
    If {
        cond: Box<CSTExpr<'src>>,
        yes: VecDeque<CSTExpr<'src>>,
        no: VecDeque<CSTExpr<'src>>,
    },
    /// Constants : integer, strings, booleans, unit
    Constant(CSTConstant),
    /// [lhs] [op] [rhs]
    BinaryOp {
        lhs: Span<'src>,
        op: CSTBinary,
        rhs: Span<'src>,
    },
    /// [lhs] [op] [rhs]
    UnaryOp {
        lhs: Span<'src>,
        op: CSTUnary,
    },
    /// Empty vector written : `[]`
    EmptyVec,
}

#[derive(Debug, Default)]
pub struct CSTIndex<'src> {
    pub var: Span<'src>,
    pub index: VecDeque<Span<'src>>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum CSTConstant {
    /// Unit
    #[default]
    Unit,
    /// Integers
    Int(u16),
    /// Float
    Float(f32),
    /// Strings
    String(String),
}

#[derive(Debug)]
pub enum CSTMethod {
    Replace,
    Push,
    Pop,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CSTTypes {
    pub array_depth: u8,
    pub basic: CSTBasicTypes,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CSTBasicTypes {
    U64,
    U32,
    U16,
    U8,
    Unit,
    Bool,
}
pub mod types;
