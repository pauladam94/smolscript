use crate::{Binary, Constant, Expr, Index, Method, Types, Unary};
use cst::{block::CSTBlock, expr::CSTExpr};
use itertools::Itertools;
use std::{
    collections::{VecDeque, vec_deque},
    fmt::{Display, Formatter},
};

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Index(index) => write!(f, "{index}"),
            Expr::MethodCall {
                index,
                method,
                args,
            } => {
                write!(f, "{index}.{method}(")?;
                let mut iter = args.iter();
                if let Some(arg) = iter.next() {
                    write!(f, "{arg}")?;
                    for arg in args.iter() {
                        write!(f, ", {arg}")?;
                    }
                }
                write!(f, ")")
            }
            Expr::FunCall { name, args } => {
                write!(f, "{name}(")?;
                let mut iter = args.iter();
                if let Some(arg) = iter.next() {
                    write!(f, "{arg}")?;
                    for arg in args {
                        write!(f, ", {arg}")?;
                    }
                }
                write!(f, ")")
            }
            Expr::LetBindings { var, t, block } => {
                write!(f, "let {var} : {t} = {}", block.iter().format(";"))
            }
            Expr::While { cond, block } => {
                write!(f, "while {cond} {{{}}}", block.iter().format(";"))
            }
            Expr::If { cond, yes, no } => write!(
                f,
                "if {cond} {{{}}} else {{{}}}",
                yes.iter().format(";"),
                no.iter().format(";")
            ),
            Expr::BinaryOp { lhs, op, rhs } => write!(f, "{lhs} {op} {rhs}"),
            Expr::UnaryOp { lhs: var, op } => write!(f, "{op} {var}"),
            Expr::Constant(constant) => write!(f, "{constant}"),
            Expr::EmptyVec => write!(f, "[]"),
            Expr::Block(block) => {
                let mut iter = block.iter();
                if let Some(expr) = iter.next() {
                    write!(f, "{expr}")?;
                    for expr in iter {
                        write!(f, ";\n{expr}")?;
                    }
                }
                Ok(())
            }
        }
    }
}

impl From<CSTBlock<'_>> for Expr {
    fn from(block: CSTBlock<'_>) -> Self {
        let CSTBlock { statements } = block;
        Expr::Block(
            statements
                .into_iter()
                .map(|expr| Expr::from(expr))
                .collect(),
        )
    }
}

impl From<CSTExpr<'_>> for Expr {
    fn from(expr: CSTExpr<'_>) -> Self {
        match expr {
            CSTExpr::Index(cstindex) => Expr::Index(Index::from(cstindex)),
            CSTExpr::MethodCall {
                index,
                method,
                args,
            } => Expr::MethodCall {
                index: Index::from(index),
                method: Method::from(method),
                args: args.into_iter().map(|span| span.to_string()).collect(),
            },
            CSTExpr::FunCall { name, args } => {
                let mut n = 0;
                let mut statements: VecDeque<Expr> = args
                    .into_iter()
                    .enumerate()
                    .map(|(i, block)| {
                        n += 1;
                        Expr::LetBindings {
                            var: format!("tmp_{i}"),
                            t: Types::UNIT,
                            block: VecDeque::from([Expr::from(block)]),
                        }
                    })
                    .collect();
                statements.push_back(Expr::FunCall {
                    name: name.to_string(),
                    args: (0..n).into_iter().map(|i| format!("tmp_{i}")).collect(),
                });
                Expr::Block(statements)
            }
            CSTExpr::LetBindings { var, block } => Expr::LetBindings {
                var: var.to_string(),
                // todo the right type
                t: Types::UNIT,
                block: VecDeque::from([Expr::from(block)]),
            },
            CSTExpr::While { cond, block } => Expr::While {
                cond: cond.to_string(),
                block: VecDeque::from([Expr::from(block)]),
            },
            CSTExpr::If { cond, yes, no } => Expr::If {
                cond: cond.to_string(),
                yes: VecDeque::from([Expr::from(yes)]),
                no: VecDeque::from([Expr::from(no)]),
            },
            CSTExpr::Constant(constant) => Expr::Constant(Constant::from(constant)),
            CSTExpr::BinaryOp { lhs, op, rhs } => Expr::BinaryOp {
                lhs: lhs.to_string(),
                op: Binary::from(op),
                rhs: rhs.to_string(),
            },
            CSTExpr::UnaryOp { lhs, op } => Expr::UnaryOp {
                lhs: lhs.to_string(),
                op: Unary::from(op),
            },
            CSTExpr::EmptyVec => Expr::EmptyVec,
        }
    }
}
