use crate::ast::{block::Block, literal::Literal};

pub enum Expr<Ident> {
    /// fn f(x, y, ...) { b }
    FunDef {
        block: Block<Ident>,
        var: Ident,
        args: Vec<Block<Ident>>,
    },
    /// f(x, y, ...)
    /// The builtin ones are :
    /// - x.replace(b)
    /// - x.append(b)
    /// - x.pop()
    FunCall {
        fun: Ident,
        var: Ident,
        args: Vec<Block<Ident>>,
    },
    /// let x = e
    LetBindings { var: Ident, block: Block<Ident> },
    /// while (b) { b }
    While { cond: Block<Ident> },
    /// if (b) { b } else { b }
    If {
        cond: Block<Ident>,
        yes: Block<Ident>,
        no: Block<Ident>,
    },
    /// Constants : [], integer, strings ...
    Literal(Literal),
}
