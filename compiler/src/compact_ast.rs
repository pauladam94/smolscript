use crate::{
    checker::type_error::TypeError, compact_ast::direct_parser::Parser, cst::{
        annotation::annotate,
        lexer::{Token, pos::Pos},
        parser::span::Span,
    }, types::{Type, TypeID},
};
use alloc::{format, string::String, vec::Vec};
use logos::Logos;

/// todo
pub mod direct_parser;

/// Module containing all [Display] implementation for the AST
pub mod formatting;

/// This function parse from an `&str` the concrete syntax tree.
/// This operation can fail and returns us with as much error as possible with
/// the [ParseError] type that can be displayed.
pub fn parser<'src>(input: &'src str) -> Result<AST<'src>, String> {
    let mut lexer = Token::lexer(input);
    let mut parser = Parser::new(&mut lexer);
    let ast = parser.parse_program();
    match ast {
        Ok(o) => match lexer.next() {
            Some(Ok(_)) => Err(format!(
                "Result of parsing is :\n{}\nDebug result: \n{:#?}\n{}",
                o,
                o,
                annotate(input, lexer, "There is some token left.")
            )),
            Some(Err(())) => Err(annotate(
                input,
                lexer,
                "Error in the lexer. More text but no more token.",
            )),
            None => Ok(o),
        },
        Err(_) => Err(annotate(input, lexer, "Parsing Error.")),
    }
}

/// This function converts the CST (concrete syntax tree) to the AST (abstract
/// syntax tree). It also typechecks the CST that have partial type information
/// and procude the AST that has complete type information.
///
/// Multiple things are done during this translation :
/// - desugar the syntax of the surface language
/// - type information is builtin inside the AST
/// - typechecking is done on the CST which may fail
/// - informations from the source file (variables names, positions) are removed
///
/// TODO redo comment
pub fn type_check(ast: &mut AST) -> Result<Type, Vec<TypeError>> {
    todo!()
}

/// An [Ast] is an abstract syntax tree.
/// It represents the language while abtracting the precise syntaxic element
/// of the language.
///
/// This is constructred after type checking and have types
/// information inside it that have all been resolved either by :
/// - input from the user
/// - or inferred by the type checker
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AST<'src> {
    pub top_level: Vec<Decl<'src>>,
    pub exprs: Arena<'src>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decl<'src> {
    pub name: &'src str,
    pub args: Vec<(&'src str, Type)>,
    pub return_type: Type,
    pub expr: ExprId,
}

pub type ExprID = ArenaId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExprNode<'src> {
    // A list of expression that can be viewed as statements
    Seq {
        fst: ExprID,
        snd: ExprID,
    },
    // Array of any expression
    Array {
        head: ExprID,
        tail: ExprID,
    },
    // Pair of expression
    Pair {
        fst: ExprID,
        snd: ExprID,
    },
    // x or x[y] or x[y][z] ...
    // - `index` represent the first part of the index
    // - `var` represent with which expression you are going to index it
    Index {
        body: ExprID,
        index: ExprID,
    },
    // Variable(Ident),
    MethodCall {
        index: ExprID,
        method: Method,
        args: ExprID,
    },
    /// f(x, y, ...)
    /// The builtin ones are :
    /// - x.replace(b)
    /// - x.append(b)
    /// - x.pop()
    FunCall {
        name: &'src str,
        args: ExprID,
    },
    /// let [var] = [expr]
    Let {
        var: &'src str,
        t: TypeID,
        expr: ExprID,
    },
    /// [var] := [expr]
    Assign {
        index: ExprID,
        expr: ExprID,
    },
    /// while ([cond]) { [block] }
    While {
        cond: ExprID,
        body: ExprID,
    },
    /// if ([cond]) { [yes] } else { [no] }
    If {
        cond: ExprID,
        yes: ExprID,
        no: ExprID,
    },
    /// Constants : integer, strings, booleans, unit
    Constant(Constant),
    /// [lhs] [op] [rhs]
    /// Binary expression of form : `lhs op rhs`
    ///
    /// Example : x + y
    BinaryOp {
        lhs: ExprID,
        op: Binary,
        rhs: ExprID,
    },
    /// [lhs] [op] [rhs]
    /// Unary expression of form : `op lhs`
    ///
    /// Example : !y, -z
    UnaryOp {
        lhs: ExprID,
        op: Unary,
    },
    /// Empty vector written : `[]`
    EmptyVec,
    /// Unknown piece of the AST.
    /// This can be used either to indicate :
    /// - that a part of the AST has not been parsed properly
    /// - to indicate a partial type anotation
    Unknown,
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
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
