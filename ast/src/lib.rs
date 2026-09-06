use cst::CST;
use std::collections::VecDeque;

/// This is the identifier type. This is how we are going to represent variable.
/// We now forget any information about the original parsed file. We are in an
/// abstract form. Even if identifier are integers they are not to be mistaken with
/// Debruijn indexes, we are not using Debruijn indexes.
///
/// The main constrains we have is, if two [Span] are equal (not in position but
/// their underlying string) they their identifier is the same.
pub type Identifier = usize;

/// Main function of this crate.
///
/// This function converts the CST (concrete syntax tree) to the AST (abstract
/// syntax tree). It also takes into parameter the type information given by
/// the typechecking phase on the AST.
///
/// Multiple things are done during this translation :
/// - sugar syntax of surface language is removed
/// - type information is builtin the AST
/// - informations from original file (variable names, position, ..) are removed
pub fn cst_to_ast(cst: &CST) -> AST {
    let CST {
        top_level,
        expr: block,
    } = cst;
    AST {
        top_level: top_level.into_iter().map(|decl| Decl::from(decl)).collect(),
        expr: Expr::from(block),
    }
}

/// This is the module that contains all of the convertion code for the CST
pub mod from_cst;

/// Module containing all [Display] implementation for the AST
pub mod formatting;

/// An [Ast] is an abstract syntax tree.
/// It represents the language while abtracting the precise syntaxic element
/// of the language.
///
/// This is constructred after type checking and have types
/// information inside it that have all been resolved either by :
/// - input from the user
/// - or inferred by the type checker
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AST {
    pub top_level: VecDeque<Decl>,
    pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decl {
    pub name: String,
    pub args: VecDeque<(String, Types)>,
    pub output_type: Types,
    pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Types {
    depth: u8,
    basic: BasicTypes,
}
pub mod types;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BasicTypes {
    U64,
    U32,
    U16,
    U8,
    Bool,
    Unit,
}

/// Owned Expression of the AST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Index(VarIndex),
    MethodCall {
        index: VarIndex,
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
        expr: Box<Expr>,
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

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct VarIndex {
    pub var: String,
    pub index: VecDeque<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Method {
    Push,
    Pop,
    Replace,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Binary {
    Add,
    Sub,
    Div,
    Mult,
    Equal,
    NotEqual,
    LessThan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unary {
    Not,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Constant {
    Int(u16),
    Bool(bool),
    #[default]
    Unit,
}
