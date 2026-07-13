use crate::{BasicTypes, Binary, Constant, Decl, Expr, Index, Method, Types, Unary};
use cst::{
    CSTBasicTypes, CSTBinary, CSTBlock, CSTConstant, CSTDecl, CSTExpr, CSTIndex, CSTMethod,
    CSTTypes, CSTUnary,
};
use std::collections::VecDeque;

impl<'src> From<&CSTDecl<'src>> for Decl {
    fn from(cst_decl: &CSTDecl<'src>) -> Self {
        let CSTDecl {
            name,
            args,
            output_type,
            block,
        } = cst_decl;

        Decl {
            name: name.to_string(),
            args: args
                .into_iter()
                .map(|(var, t)| (var.to_string(), Types::from(t)))
                .collect(),
            output_type: Types::from(output_type),
            block: Expr::from(block),
        }
    }
}
impl From<&CSTTypes> for Types {
    fn from(cst_types: &CSTTypes) -> Self {
        let CSTTypes { depth, basic } = cst_types;

        Types {
            depth: *depth,
            basic: BasicTypes::from(basic),
        }
    }
}

impl From<&CSTBasicTypes> for BasicTypes {
    fn from(cst_basic_types: &CSTBasicTypes) -> Self {
        match cst_basic_types {
            CSTBasicTypes::U16 => BasicTypes::U16,
            CSTBasicTypes::U8 => BasicTypes::U8,
            CSTBasicTypes::Unit => BasicTypes::Unit,
            CSTBasicTypes::Bool => BasicTypes::Bool,
        }
    }
}
impl From<&CSTBlock<'_>> for Expr {
    fn from(block: &CSTBlock<'_>) -> Self {
        let CSTBlock { statements } = block;
        Expr::Block(
            statements
                .into_iter()
                .map(|expr| Expr::from(expr))
                .collect(),
        )
    }
}

impl From<&CSTExpr<'_>> for Expr {
    fn from(expr: &CSTExpr<'_>) -> Self {
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
impl From<&CSTIndex<'_>> for Index {
    fn from(cst_index: &CSTIndex<'_>) -> Self {
        let CSTIndex { var, index } = cst_index;
        Index {
            var: var.to_string(),
            index: index.into_iter().map(|span| span.to_string()).collect(),
        }
    }
}
impl From<&CSTBinary> for Binary {
    fn from(cst_binary: &CSTBinary) -> Self {
        match cst_binary {
            CSTBinary::Add => Binary::Add,
            CSTBinary::Sub => Binary::Sub,
            CSTBinary::Div => Binary::Div,
            CSTBinary::Equal => Binary::Equal,
            CSTBinary::NotEqual => Binary::NotEqual,
        }
    }
}
impl From<&CSTMethod> for Method {
    fn from(cst_method: &CSTMethod) -> Self {
        match cst_method {
            CSTMethod::Replace => Self::Replace,
            CSTMethod::Push => Self::Push,
            CSTMethod::Pop => Self::Pop,
        }
    }
}
impl From<&CSTUnary> for Unary {
    fn from(cst_unary: &CSTUnary) -> Self {
        match cst_unary {
            CSTUnary::Not => Unary::Not,
        }
    }
}
impl From<&CSTConstant> for Constant {
    fn from(cst_constant: &CSTConstant) -> Self {
        match cst_constant {
            CSTConstant::Unit => Constant::Unit,
            CSTConstant::Int(i) => Constant::Int(*i),
            CSTConstant::String(s) => todo!(),
            CSTConstant::Float(_) => todo!(),
        }
    }
}
