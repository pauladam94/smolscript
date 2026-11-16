use crate::{
    types::{TypeId, TypeNode, TypeView},
    utils::{
        arena::{Arena, ArenaId},
        span::Span,
    },
};

/// An [AST] is an abstract syntax tree.
/// It represents the language while abtracting the precise syntaxic element
/// of the language.
///
/// This is first constructed by the parse and then modified by the type
/// checker by adding
#[derive(Debug, Clone)]
pub struct AST<'src> {
    pub input: &'src str,
    pub positions: Arena<Span, ExprId>,
    pub exprs: Arena<ExprNode, ExprId>,
    pub types: Arena<TypeNode, TypeId>,
    pub root: ExprId,
}

impl<'src> PartialEq for AST<'src> {
    fn eq(&self, other: &Self) -> bool {
        self.exprs == other.exprs && self.types == other.types && self.root == other.root
    }
}

impl<'src> AST<'src> {
    pub fn new(input: &'src str) -> Self {
        Self {
            input,
            positions: Arena::new(),
            exprs: Arena::new(),
            types: Arena::new(),
            root: ExprId(0),
        }
    }
    pub fn get_span(&self, id: ExprId) -> Span {
        self.positions[id]
    }

    pub fn has_unknown_type(&self) -> bool {
        // TODO better
        false
    }
    pub(crate) fn get_slice(&self, id: ExprId) -> &'src str {
        self.input.get(self.get_span(id)).unwrap()
    }
}

pub struct ExprView<'a, 'src> {
    pub id: ExprId,
    pub ast: &'a AST<'src>,
}

impl<'src> AST<'src> {
    pub fn show<'a>(&'a self, id: ExprId) -> ExprView<'a, 'src> {
        ExprView { id, ast: self }
    }
    pub fn show_type<'a>(&'a self, id: TypeId) -> TypeView<'a, 'src> {
        TypeView { id, ast: self }
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub struct ExprId(pub u16);

impl ArenaId for ExprId {
    fn from_usize(index: usize) -> Self {
        Self(index as u16)
    }

    fn to_usize(self) -> usize {
        self.0 as usize
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ExprNode {
    // Function Declaration
    Decl {
        name: ExprId,
        args: ExprId,
        return_type: TypeId,
        expr: ExprId,
    },
    // Variable with its name
    Var,
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
    FieldAccess {
        body: ExprId,
        field: ExprId,
    },
    // x or x[y] or x[y][z] ...
    // - `body` represent the first part of the index
    // - `index` represent the last element that is gonna index the body.
    Index {
        body: ExprId,
        index: ExprId,
    },
    /// [index].[method](args1, ..., argsn)
    MethodCall {
        index: ExprId,
        method: Method,
        args: ExprId,
    },
    APICall {
        name: ExprId,
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
    /// [var] = [expr]
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
    /// Text literal between double quotes.
    /// In other programming languages this is called a string.
    ///
    /// ex: "hello world"
    Text,
    /// Constants : integer, strings, booleans, unit
    Const(Constant),
    /// [lhs] [op] [rhs]
    /// Binary expression of form : `lhs op rhs`
    ///
    /// ex: x + y
    BinaryOp {
        lhs: ExprId,
        op: Binary,
        rhs: ExprId,
    },
    /// [lhs] [op] [rhs]
    /// Unary expression of form : `op lhs`
    ///
    /// ex: !y, -z
    UnaryOp {
        lhs: ExprId,
        op: Unary,
    },
    /// Type indication of an expression.
    ///
    /// ex: x : bool
    TypeInfo {
        expr: ExprId,
        t: TypeId,
    },
    /// End of a list
    EmptyTuple,
    /// Empty vector
    ///
    /// ex: []
    EmptyVec,
    /// Unknown piece of the AST.
    /// This can be used either to indicate :
    /// - that a part of the AST has not been parsed properly
    /// - to indicate a partial type anotation
    ///
    /// ex: ?
    #[default]
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
    Int(u32),
    Float(f32),
    Bool(bool),
    Unit,
}
