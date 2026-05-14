use crate::{
    ast::{binop::BinOpKind, block::Block, literal::Literal},
    structural_eq::StructuralEq,
};
use std::fmt::{Display, Formatter, Result};

pub enum Expr<Ident> {
    Variable(Ident),
    /// f(x, y, ...)
    /// The builtin ones are :
    /// - x.replace(b)
    /// - x.append(b)
    /// - x.pop()
    FunCall {
        name: Ident,
        args: Vec<Ident>,
    },
    /// let [var] = [block]
    LetBindings {
        var: Ident,
        block: Block<Ident>,
    },
    /// while ([cond]) { [block] }
    While {
        cond: Ident,
        block: Block<Ident>,
    },
    /// if ([cond]) { [yes] } else { [no] }
    If {
        cond: Ident,
        yes: Block<Ident>,
        no: Block<Ident>,
    },
    /// Constants : [], integer, strings ...
    Literal(Literal),
    /// [lhs] [op] [rhs]
    BinOp {
        op: BinOpKind,
        lhs: Ident,
        rhs: Ident,
    },
}

impl<Ident: Display> Display for Expr<Ident> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use Expr::*;
        match self {
            Variable(var) => {
                write!(f, "{var}")
            }
            FunCall { name: fun, args } => {
                write!(f, "{fun}(")?;
                for (i, arg) in args.iter().enumerate() {
                    write!(f, "{arg}")?;
                    if i != args.iter().len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
            LetBindings { var, block } => write!(f, "let {var} = {block}"),
            While { cond, block } => write!(f, "while {cond} {block}"),
            If { cond, yes, no } => write!(f, "if ({cond}) {yes} {no}"),
            Literal(literal) => write!(f, "{literal}"),
            BinOp { op, lhs, rhs } => write!(f, "{lhs} {op} {rhs}"),
        }
    }
}

impl<Ident: StructuralEq> StructuralEq for Expr<Ident> {
    fn structural_eq(&self, rhs: &Self) -> bool {
        use Expr::*;
        match (self, rhs) {
            (Variable(v1), Variable(v2)) => v1.structural_eq(v2),
            (
                FunCall { name: fun, args },
                FunCall {
                    name: fun2,
                    args: args2,
                },
            ) => {
                if !fun.structural_eq(fun2) {
                    return false;
                }
                if args.len() != args2.len() {
                    return false;
                }
                for (arg, arg2) in args.iter().zip(args2.iter()) {
                    if !arg.structural_eq(arg2) {
                        return false;
                    }
                }
                true
            }
            (
                While { cond, block },
                While {
                    cond: cond2,
                    block: block2,
                },
            ) => cond.structural_eq(cond2) && block.structural_eq(block2),
            (
                If { cond, yes, no },
                If {
                    cond: cond2,
                    yes: yes2,
                    no: no2,
                },
            ) => cond.structural_eq(cond2) && yes.structural_eq(yes2) && no.structural_eq(no2),
            (Literal(literal), Literal(literal2)) => literal.structural_eq(literal2),
            (_, _) => false,
        }
    }
}
