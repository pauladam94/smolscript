use crate::{
    checker::type_error::TypeError,
    compact_ast::direct_parser::Parser,
    cst::{annotation::annotate, lexer::Token},
    types::{Type, TypeId},
    utils::arena::{Arena, ArenaId},
};
use alloc::{format, string::String, vec::Vec};
use logos::Logos;

/// todo
pub mod direct_parser;

/// Module containing all [Display] implementation for the AST
pub mod formatting;

pub mod type_check;

/// This function parse from an `&str` the concrete syntax tree.
/// This operation can fail and returns us with as much error as possible with
/// the [ParseError] type that can be displayed.
pub fn parser<'src>(input: &'src str) -> Result<AST<'src>, String> {
    let mut lexer = Token::lexer(input);
    let mut parser = Parser::new(&mut lexer);
    let ast = parser.parse_program();
    match ast {
        Some(o) => match lexer.next() {
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
        None => Err(annotate(input, lexer, "Parsing Error.")),
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
    pub expr: ExprId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decl<'src> {
    pub name: &'src str,
    pub args: Vec<(&'src str, TypeId)>,
    pub return_type: TypeId,
    pub expr: ExprId,
}

pub type ExprId = ArenaId;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExprNode<'src> {
    // A variable
    Var(&'src str),
    // A list of expression that can be viewed as statements
    Seq {
        fst: ExprId,
        snd: ExprId,
    },
    // Array of any expression
    Array {
        head: ExprId,
        tail: ExprId,
    },
    // Tuple of expression
    Tuple {
        head: ExprId,
        tail: ExprId,
    },
    // Pair of expression
    Pair {
        fst: ExprId,
        snd: ExprId,
    },
    // x or x[y] or x[y][z] ...
    // - `index` represent the first part of the index
    // - `var` represent with which expression you are going to index it
    Index {
        body: ExprId,
        index: ExprId,
    },
    // Variable(Ident),
    MethodCall {
        index: ExprId,
        method: Method,
        args: ExprId,
    },
    /// f(x, y, ...)
    /// The builtin ones are :
    /// - x.replace(b)
    /// - x.append(b)
    /// - x.pop()
    FunCall {
        name: ExprId,
        args: ExprId,
    },
    /// let [var] = [expr]
    Let {
        var: ExprId,
        t: TypeId,
        expr: ExprId,
    },
    /// [var] := [expr]
    Assign {
        index: ExprId,
        expr: ExprId,
    },
    /// while ([cond]) { [block] }
    While {
        cond: ExprId,
        body: ExprId,
    },
    /// if ([cond]) { [yes] } else { [no] }
    If {
        cond: ExprId,
        yes: ExprId,
        no: ExprId,
    },
    /// Constants : integer, strings, booleans, unit
    Constant(Constant),
    /// [lhs] [op] [rhs]
    /// Binary expression of form : `lhs op rhs`
    ///
    /// Example : x + y
    BinaryOp {
        lhs: ExprId,
        op: Binary,
        rhs: ExprId,
    },
    /// [lhs] [op] [rhs]
    /// Unary expression of form : `op lhs`
    ///
    /// Example : !y, -z
    UnaryOp {
        lhs: ExprId,
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Associativity {
    RightToLeft,
    LeftToRight,
}
impl Binary {
    /// Precedence of binary operators following the C standard describe
    /// [here](https://fr.cppreference.com/c/language/operator_precedence)
    pub const fn precedence(&self) -> usize {
        match self {
            Binary::Add => 3,
            Binary::Sub => 3,
            Binary::Div => 4,
            Binary::Mult => 4,
            Binary::Equal => 1,
            Binary::NotEqual => 1,
            Binary::LessThan => 2,
            Binary::LessThanEqual => 2,
            Binary::MoreThan => 2,
            Binary::MoreThanEqual => 2,
        }
    }
    /// Associativity of binary operators following the C standard describe
    /// [here](https://fr.cppreference.com/c/language/operator_precedence)
    pub const fn associativity(&self) -> Associativity {
        match self {
            Binary::Add
            | Binary::Sub
            | Binary::Div
            | Binary::Mult
            | Binary::Equal
            | Binary::NotEqual
            | Binary::LessThan
            | Binary::LessThanEqual
            | Binary::MoreThan
            | Binary::MoreThanEqual => Associativity::LeftToRight,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unary {
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Constant {
    Int(u64),
    Float(f32),
    Bool(bool),
    Unit,
}
impl Constant {
    pub(crate) fn get_type(&self) -> Type {
        todo!()
        // match self {
        //     Constant::Int(_) => Type::u16(),
        //     Constant::Bool(_) => Type::bool(),
        //     Constant::Float(_) => Type::,
        //     Constant::Unit => Type::unit(),
        // }
    }
}
