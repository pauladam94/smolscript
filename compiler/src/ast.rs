use crate::{cst::CST, from_cst::CSTConverter};
use alloc::{boxed::Box, collections::vec_deque::VecDeque};

/// Module containing all [Display] implementation for the AST
pub mod formatting;

// todo
pub mod types;

/// This function converts the CST (concrete syntax tree) to the AST (abstract
/// syntax tree). It also typechecks the CST that have partial type information
/// and procude the AST that has complete type information.
///
/// Multiple things are done during this translation :
/// - desugar the syntax of the surface language
/// - type information is builtin inside the AST
/// - typechecking is done on the CST which may fail
/// - informations from the source file (variables names, position) are removed
pub fn cst_to_ast(cst: &CST) -> AST {
    let CST { top_level, expr } = cst;
    let mut converter = CSTConverter::default();
    AST {
        top_level: top_level
            .iter()
            .map(|decl| converter.convert_decl(decl))
            .collect(),
        expr: converter.convert_expr(expr),
    }
}

/// This is the identifier type. This is how we are going to represent variable.
/// We now forget any information about the original parsed file. We are in an
/// abstract form. Even if identifier are integers they are not to be mistaken with
/// Debruijn indexes, we are not using Debruijn indexes.
///
/// The main constrains we have is, if two [Span] are equal (not in position but
/// their underlying string) then their identifier is the same.
// pub type Identifier = usize;
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
    pub args: VecDeque<(Identifier, Types)>,
    pub output_type: Types,
    pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Types {
    pub depth: u8,
    pub basic: BasicTypes,
}

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
    /// Block of form : `expr; expr... expr; expr`
    Block(VecDeque<Expr>),
    /// Let Bindings of form : `let var : t = expr`
    Let {
        var: Identifier,
        t: Types,
        expr: Box<Expr>,
    },
    /// While loop of form : `while ( cond ) { body }`
    While(While),
    /// If expression of form : `if (cond) { yes } else { no }`
    If(If),
    /// Method call of form : `X.method(arg1, arg2, ...)`
    MethodCall(MethodCall),
    /// Function call of form : `name(arg1, arg2, ...)`
    Call(Call),
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
/// While loop of form : `while ( cond ) { body }`
pub struct While {
    pub cond: Box<Expr>,
    pub body: VecDeque<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// If expression of form : `if (cond) { yes } else { no }`
pub struct If {
    pub cond: Identifier,
    pub yes: VecDeque<Expr>,
    pub no: VecDeque<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Method call of form : `X.method(arg1, arg2, ...)`
pub struct MethodCall {
    pub index: Index,
    pub method: Method,
    pub args: VecDeque<Identifier>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Function call of form : `name(arg1, arg2, ...)`
pub struct Call {
    pub name: Identifier,
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
