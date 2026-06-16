use crate::{
    CSTUnary, binop::CSTBinary, block::CSTBlock, index::CSTIndex, literal::CSTConstant,
    method::CSTMethod, parser::span::Span, structural_eq::StructuralEq,
};
use std::{
    collections::VecDeque,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub enum CSTExpr<'src> {
    // x or x[y] or x[y][z] ...
    Index(CSTIndex<'src>),
    // Variable(Ident),
    MethodCall {
        index: CSTIndex<'src>,
        method: CSTMethod,
        args: VecDeque<Span<'src>>,
    },
    /// f(x, y, ...)
    /// The builtin ones are :
    /// - x.replace(b)
    /// - x.append(b)
    /// - x.pop()
    FunCall {
        name: Span<'src>,
        args: VecDeque<CSTBlock<'src>>,
    },
    /// let [var] = [block]
    LetBindings {
        var: Span<'src>,
        block: CSTBlock<'src>,
    },
    /// while ([cond]) { [block] }
    While {
        cond: Span<'src>,
        block: CSTBlock<'src>,
    },
    /// if ([cond]) { [yes] } else { [no] }
    If {
        cond: Span<'src>,
        yes: CSTBlock<'src>,
        no: CSTBlock<'src>,
    },
    /// Constants : integer, strings, booleans, unit
    Constant(CSTConstant),
    /// [lhs] [op] [rhs]
    BinaryOp {
        lhs: Span<'src>,
        op: CSTBinary,
        rhs: Span<'src>,
    },
    /// [lhs] [op] [rhs]
    UnaryOp {
        lhs: Span<'src>,
        op: CSTUnary,
    },
    /// Empty vector written : `[]`
    EmptyVec,
}

impl<'src> Display for CSTExpr<'src> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use CSTExpr::*;
        match self {
            Index(index) => write!(f, "{index}"),
            MethodCall {
                index,
                method: name,
                args,
            } => {
                write!(f, "{index}.{name}(")?;
                for (i, arg) in args.iter().enumerate() {
                    write!(f, "{arg}")?;
                    if i != args.iter().len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
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
            Constant(constant) => write!(f, "{constant}"),
            BinaryOp { op, lhs, rhs } => write!(f, "{lhs} {op} {rhs}"),
            UnaryOp { lhs, op } => write!(f, "{lhs} {op}"),
            EmptyVec => write!(f, "[]"),
        }
    }
}

impl<'src> StructuralEq for CSTExpr<'src> {
    fn structural_eq(&self, rhs: &Self) -> bool {
        use CSTExpr::*;
        match (self, rhs) {
            (Index(index), Index(index2)) => index.structural_eq(index2),

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
            (Constant(literal), Constant(literal2)) => literal.structural_eq(literal2),
            (EmptyVec, EmptyVec) => true,
            (_, _) => false,
        }
    }
}
