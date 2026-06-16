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
    pub block: Block,
}
pub mod ast;

pub struct Decl {
    name: String,
    args: VecDeque<(String, Types)>,
    output_type: Types,
    block: Block,
}
pub mod decl;

#[derive(Default)]
pub struct Block {
    pub statements: VecDeque<Expr>,
}
pub mod block;

pub struct Types {
    depth: u8,
    basic: BasicTypes,
}
pub mod types;

pub enum BasicTypes {
    U16,
    U8,
    Bool,
    Unit,
}
pub mod basic_types;

/// Owned Expression of the AST.
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
        block: Block,
    },
    While {
        cond: String,
        block: Block,
    },
    If {
        cond: String,
        yes: Block,
        no: Block,
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
    Block(Block),
    Constant(Constant),
    EmptyVec,
}
pub mod expr;

#[derive(Default)]
pub struct Index {
    pub var: String,
    pub index: VecDeque<String>,
}
pub mod index;

pub enum Method {
    Push,
    Pop,
    Replace,
}
pub mod method;

pub enum Binary {
    Add,
    Sub,
    Div,
    Mult,
    Equal,
    LessThan,
}
pub mod binary;

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
