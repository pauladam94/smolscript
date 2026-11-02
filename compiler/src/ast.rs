use crate::{
    checker::{Checker, type_error::TypeError},
    cst::CST,
    types::Type,
};
use alloc::{boxed::Box, collections::vec_deque::VecDeque, format, vec::Vec};

/// Module containing all [Display] implementation for the AST
pub mod formatting;

/// This function converts the CST (concrete syntax tree) to the AST (abstract
/// syntax tree). It also typechecks the CST that have partial type information
/// and procude the AST that has complete type information.
///
/// Multiple things are done during this translation :
/// - desugar the syntax of the surface language
/// - type information is builtin inside the AST
/// - typechecking is done on the CST which may fail
/// - informations from the source file (variables names, positions) are removed
pub fn type_check(cst: &CST) -> Result<(AST, Type), Vec<TypeError>> {
    let mut checker = Checker::default();

    // Type Checking of all the top level functions
    let mut top_level = VecDeque::new();
    let mut errors = Vec::new();
    for cst_decl in cst.top_level.iter() {
        match checker.check_decl(cst_decl) {
            Ok(decl) => top_level.push_back(decl),
            Err(err) => errors.push(err),
        }
    }

    // If there has been error while type checking toplevel functions
    // then we stop here and return type errors.
    if !errors.is_empty() {
        return Err(errors);
    }

    let (expr, t) = checker.check_expr(&cst.expr).map_err(|e| Vec::from([e]))?;

    if t.is_unknown() {
        errors.push(TypeError {
            message: format!(""),
            pos: todo!(),
        });
        return Err(errors);
    }

    Ok((AST { top_level, expr }, t))
}

/// This is the identifier type. This is how we are going to represent variable.
/// We now forget any information about the original parsed file. We are in an
/// abstract form. Even if identifier are integers they are not to be mistaken with
/// Debruijn indexes, we are not using Debruijn indexes.
///
/// The main constrains we have is, if two [Span] are equal (not in position but
/// their underlying string) then their identifier is the same.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Identifier(pub usize);

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
    pub name: Identifier,
    pub args: Vec<(Identifier, Type)>,
    pub return_type: Type,
    pub expr: Expr,
}

/// Owned Expression of the AST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    /// Block of form : `expr; expr... expr; expr`
    Block(VecDeque<Expr>),
    /// Let Bindings of form : `let var : t = expr`
    Let {
        var: Identifier,
        t: Type,
        expr: Box<Expr>,
        body: Box<Expr>,
    },
    /// While loop of form : `while ( cond ) { body }`
    While {
        cond: Box<Expr>,
        body: Box<Expr>,
    },
    /// If expression of form : `if (cond) { yes } else { no }`
    If {
        cond: Box<Expr>,
        yes: Box<Expr>,
        no: Box<Expr>,
    },
    /// Method call of form : `X.method(arg1, arg2, ...)`
    MethodCall(MethodCall),
    /// Function call of form : `name(arg1, arg2, ...)`
    Call {
        name: Identifier,
        args: VecDeque<Identifier>,
    },
    /// Indexing `x[y] ... [y]`
    Index(Index),
    /// Binary expression of form : `lhs op rhs`
    BinaryOp(BinaryOp),
    /// Unary expression of form : `op lhs`
    UnaryOp(UnaryOp),
    Constant(Constant),
    /// Empty Vector of form : `[]`
    EmptyVec,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Method call of form : `X.method(arg1, arg2, ...)`
pub struct MethodCall {
    pub index: Index,
    pub method: Method,
    pub args: VecDeque<Identifier>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
/// Indexing `x[y] ... [y]`
pub struct Index {
    pub var: Identifier,
    pub index: VecDeque<Identifier>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Binary expression of form : `lhs op rhs`
///
/// Example : x + y
pub struct BinaryOp {
    pub lhs: Identifier,
    pub op: Binary,
    pub rhs: Identifier,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Unary expression of form : `op lhs`
///
/// Example : !y, -z
pub struct UnaryOp {
    pub lhs: Identifier,
    pub op: Unary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Push,
    Pop,
    Replace,
    Len,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Binary {
    Add,
    Sub,
    Div,
    Mult,
    Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    MoreThan,
    MoreThanEqual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unary {
    Not,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Constant {
    Int(u64),
    Bool(bool),
    #[default]
    Unit,
}
impl Constant {
    pub(crate) fn get_type(&self) -> Type {
        match self {
            Constant::Int(_) => Type::u16(),
            Constant::Bool(_) => Type::bool(),
            Constant::Unit => Type::unit(),
        }
    }
}
