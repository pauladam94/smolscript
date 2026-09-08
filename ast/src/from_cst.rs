use crate::{BasicTypes, Binary, Constant, Decl, Expr, Method, Types, Unary, VarIndexing};
use cst::{
    CSTBasicTypes, CSTBinary, CSTConstant, CSTDecl, CSTExpr, CSTIndex, CSTMethod, CSTTypes,
    CSTUnary,
};
use std::collections::VecDeque;

impl<'src> From<&CSTDecl<'src>> for Decl {
    fn from(cst_decl: &CSTDecl<'src>) -> Self {
        let CSTDecl {
            name,
            args,
            output_type,
            expr: block,
        } = cst_decl;

        Decl {
            name: name.to_string(),
            args: args
                .iter()
                .map(|(var, t)| (var.to_string(), Types::from(t)))
                .collect(),
            output_type: Types::from(output_type),
            expr: Expr::from(block),
        }
    }
}
impl From<&CSTTypes> for Types {
    fn from(cst_types: &CSTTypes) -> Self {
        let CSTTypes {
            array_depth: depth,
            basic,
        } = cst_types;

        Types {
            depth: *depth,
            basic: BasicTypes::from(basic),
        }
    }
}

impl From<&CSTBasicTypes> for BasicTypes {
    fn from(cst_basic_types: &CSTBasicTypes) -> Self {
        match cst_basic_types {
            CSTBasicTypes::U64 => BasicTypes::U64,
            CSTBasicTypes::U32 => BasicTypes::U32,
            CSTBasicTypes::U16 => BasicTypes::U16,
            CSTBasicTypes::U8 => BasicTypes::U8,
            CSTBasicTypes::Unit => BasicTypes::Unit,
            CSTBasicTypes::Bool => BasicTypes::Bool,
        }
    }
}

impl From<&CSTExpr<'_>> for Expr {
    fn from(expr: &CSTExpr<'_>) -> Self {
        match expr {
            CSTExpr::Block(block) => Expr::Block(block.iter().map(Expr::from).collect()),
            CSTExpr::Index(cstindex) => Expr::Index(VarIndexing::from(cstindex)),
            CSTExpr::MethodCall {
                index,
                method,
                args,
            } => Expr::MethodCall {
                index: VarIndexing::from(index),
                method: Method::from(method),
                args: args.iter().map(|span| span.to_string()).collect(),
            },
            CSTExpr::FunCall { name, args } => {
                let mut n = 0;
                let mut statements: VecDeque<Expr> = args
                    .iter()
                    .enumerate()
                    .map(|(i, block)| {
                        n += 1;
                        Expr::LetBindings {
                            var: format!("tmp_{i}"),
                            t: Types::UNIT,
                            expr: Box::new(Expr::from(block)),
                        }
                    })
                    .collect();
                statements.push_back(Expr::FunCall {
                    name: name.to_string(),
                    args: (0..n).into_iter().map(|i| format!("tmp_{i}")).collect(),
                });
                Expr::Block(statements)
            }
            CSTExpr::LetBindings { var, expr } => Expr::LetBindings {
                var: var.to_string(),
                // todo the right type
                t: Types::UNIT,
                expr: Box::new(Expr::from(expr.as_ref())),
            },
            CSTExpr::Assign { var, expr } => Expr::Block(VecDeque::from([
                Expr::LetBindings {
                    var: String::from("0_tmp"),
                    t: Types::UNIT,
                    expr: Box::new(Expr::from(expr.as_ref())),
                },
                Expr::MethodCall {
                    index: VarIndexing::from(var),
                    method: Method::Replace,
                    args: VecDeque::from([String::from("0_tmp")]),
                },
            ])),
            CSTExpr::While { cond, block } => Expr::While {
                cond: Box::new(Expr::from(cond.as_ref())),
                body: block.iter().map(Expr::from).collect(),
            },
            CSTExpr::If { cond, yes, no } => Expr::Block(VecDeque::from([
                Expr::LetBindings {
                    var: String::from("0_var"),
                    t: Types::BOOL,
                    expr: Box::new(Expr::from(cond.as_ref())),
                },
                Expr::If {
                    cond: String::from("0_var"),
                    yes: yes.iter().map(Expr::from).collect(),
                    no: no.iter().map(Expr::from).collect(),
                },
            ])),
            CSTExpr::Constant(constant) => Expr::Constant(Constant::from(constant)),
            CSTExpr::BinaryOp { lhs, op, rhs } => {
                let mut block = VecDeque::new();
                let lhs_name = if let CSTExpr::Index(var_index) = lhs.as_ref()
                    && var_index.index.is_empty()
                {
                    var_index.var.clone().to_string()
                } else {
                    block.push_back(Expr::LetBindings {
                        var: String::from("0_var"),
                        t: Types::UNIT,
                        expr: Box::new(Expr::from(lhs.as_ref())),
                    });
                    String::from("0_var")
                };

                let rhs_name = if let CSTExpr::Index(var_index) = rhs.as_ref()
                    && var_index.index.is_empty()
                {
                    var_index.var.clone().to_string()
                } else {
                    block.push_back(Expr::LetBindings {
                        var: String::from("1_var"),
                        t: Types::UNIT,
                        expr: Box::new(Expr::from(rhs.as_ref())),
                    });
                    String::from("1_var")
                };
                block.push_back(Expr::BinaryOp {
                    lhs: lhs_name,
                    op: Binary::from(op),
                    rhs: rhs_name,
                });
                Expr::Block(block)
            }
            CSTExpr::UnaryOp { lhs, op } => Expr::UnaryOp {
                lhs: lhs.to_string(),
                op: Unary::from(op),
            },
            CSTExpr::EmptyVec => Expr::EmptyVec,
        }
    }
}
impl From<&CSTIndex<'_>> for VarIndexing {
    fn from(cst_index: &CSTIndex<'_>) -> Self {
        let CSTIndex { var, index } = cst_index;
        VarIndexing {
            var: var.to_string(),
            index: index.iter().map(|span| span.to_string()).collect(),
        }
    }
}
impl From<&CSTBinary> for Binary {
    fn from(cst_binary: &CSTBinary) -> Self {
        match cst_binary {
            CSTBinary::Add => Binary::Add,
            CSTBinary::Sub => Binary::Sub,
            CSTBinary::Mult => Binary::Mult,
            CSTBinary::Div => Binary::Div,
            CSTBinary::EqEq => Binary::Equal,
            CSTBinary::Neq => Binary::NotEqual,
        }
    }
}
impl From<&CSTMethod> for Method {
    fn from(cst_method: &CSTMethod) -> Self {
        match cst_method {
            CSTMethod::Replace => Self::Replace,
            CSTMethod::Push => Self::Push,
            CSTMethod::Pop => Self::Pop,
            CSTMethod::Len => Self::Len,
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
            CSTConstant::Bool(b) => Constant::Bool(*b),
        }
    }
}
