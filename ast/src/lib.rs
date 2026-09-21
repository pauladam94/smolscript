use crate::from_cst::CSTConverter;
use cst::CST;
use std::collections::VecDeque;

/// This is the identifier type. This is how we are going to represent variable.
/// We now forget any information about the original parsed file. We are in an
/// abstract form. Even if identifier are integers they are not to be mistaken with
/// Debruijn indexes, we are not using Debruijn indexes.
///
/// The main constrains we have is, if two [Span] are equal (not in position but
/// their underlying string) then their identifier is the same.
// pub type Identifier = usize;
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Identifier(usize);

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

/// todo
pub mod interpreter;

/// This is the module that contains all of the convertion code to go
/// from a CST to the AST
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
    pub name: Identifier,
    pub args: VecDeque<(Identifier, Types)>,
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
    Index(VarIndexing),
    MethodCall {
        index: VarIndexing,
        method: Method,
        args: VecDeque<Identifier>,
    },
    FunCall {
        name: Identifier,
        args: VecDeque<Identifier>,
    },
    LetBindings {
        var: Identifier,
        t: Types,
        expr: Box<Expr>,
    },
    While {
        cond: Box<Expr>,
        body: VecDeque<Expr>,
    },
    If {
        cond: Identifier,
        yes: VecDeque<Expr>,
        no: VecDeque<Expr>,
    },
    BinaryOp {
        lhs: Identifier,
        op: Binary,
        rhs: Identifier,
    },
    UnaryOp {
        lhs: Identifier,
        op: Unary,
    },
    Block(VecDeque<Expr>),
    Constant(Constant),
    EmptyVec,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct VarIndexing {
    pub var: Identifier,
    pub index: VecDeque<Identifier>,
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
impl Constant {
    pub fn compute_binary(self, op: Binary, rhs: Constant) -> Constant {
        use Binary::*;
        use Constant::*;
        match (op, self, rhs) {
            (_, Unit, _)
            | (_, _, Unit)
            | (_, Int(_), Bool(_))
            | (_, Bool(_), Int(_))
            | (Add, Bool(_), _)
            | (Div, Bool(_), _)
            | (Mult, Bool(_), _)
            | (Sub, Bool(_), _)
            | (LessThan, Bool(_), Bool(_)) => {
                // dbg!(op, self, rhs);
                unreachable!("Type checking should prevent this operation.")
            }
            (Equal, l, r) => Bool(l == r),
            (NotEqual, l, r) => Bool(l != r),
            (Add, Int(l), Int(r)) => Int(l + r),
            (Sub, Int(l), Int(r)) => Int(l - r),
            (Mult, Int(l), Int(r)) => Int(l * r),
            (Div, Int(l), Int(r)) => Int(l / r),
            (LessThan, Int(l), Int(r)) => Bool(l < r),
        }
    }
}
