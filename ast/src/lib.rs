use cst::parser::{complete::cst_parser, error::ParseError};
use std::collections::VecDeque;

/// An [Ast] is an abstract syntax tree.
/// It represents the language while abtracting the precise syntaxic element
/// of the language.
///
/// This is constructred after type checking and have types
/// information inside it that have all been resolved either by :
/// - input from the user
/// - or inferred by the type checker
pub struct AST {
    pub top_level: VecDeque<Decl>,
    pub block: Expr,
}
pub mod ast;

pub struct Decl {
    pub name: String,
    pub args: VecDeque<(String, Types)>,
    pub output_type: Types,
    pub block: Expr,
}
pub mod decl;

// #[derive(Default)]
// pub struct Block {
//     pub statements: VecDeque<Expr>,
// }
// pub mod block;

#[derive(Clone)]
pub struct Types {
    depth: u8,
    basic: BasicTypes,
}
pub mod types;

#[derive(Clone)]
pub enum BasicTypes {
    U16,
    U8,
    Bool,
    Unit,
}
pub mod basic_types;

/// Owned Expression of the AST.
#[derive(Clone)]
pub enum Expr {
    Index(Index),
    MethodCall {
        index: Index,
        method: Method,
        args: VecDeque<String>,
    },
    FunCall {
        name: String,
        args: VecDeque<String>,
    },
    LetBindings {
        var: String,
        t: Types,
        block: VecDeque<Expr>,
    },
    While {
        cond: String,
        block: VecDeque<Expr>,
    },
    If {
        cond: String,
        yes: VecDeque<Expr>,
        no: VecDeque<Expr>,
    },
    BinaryOp {
        lhs: String,
        op: Binary,
        rhs: String,
    },
    UnaryOp {
        lhs: String,
        op: Unary,
    },
    Block(VecDeque<Expr>),
    Constant(Constant),
    EmptyVec,
}
pub mod expr;

#[derive(Default, Clone)]
pub struct Index {
    pub var: String,
    pub index: VecDeque<String>,
}
pub mod index;

#[derive(Clone)]
pub enum Method {
    Push,
    Pop,
    Replace,
}
pub mod method;

#[derive(Clone)]
pub enum Binary {
    Add,
    Sub,
    Div,
    Mult,
    Equal,
    LessThan,
}
pub mod binary;

#[derive(Clone)]
pub enum Unary {
    Not,
}
pub mod unary;

#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    Int(u16),
    Bool(bool),
    Unit,
}
pub mod constant;

pub fn ast_parser(input: &str) -> Result<AST, ParseError> {
    let cst = cst_parser(input)?;
    Ok(AST::from(cst))
}
