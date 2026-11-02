use crate::ast::{
    BasicTypes, Binary, Constant, Decl, Expr, Identifier, Index, Method, Types, Unary,
};
use crate::{
    ast::{BinaryOp, MethodCall, UnaryOp},
    cst::{
        CSTBasicTypes, CSTBinary, CSTConstant, CSTDecl, CSTExpr, CSTIndex, CSTMethod, CSTTypes,
        CSTUnary, parser::span::Span,
    },
};
use alloc::{
    boxed::Box,
    collections::{btree_map::BTreeMap, vec_deque::VecDeque},
    vec::Vec,
};

#[derive(Debug, Default)]
pub(crate) struct CSTConverter<'src> {
    index: usize,
    types: Vec<(Identifier, Types)>,
    identifier_mapping: BTreeMap<&'src str, Identifier>,
    type_errors: Vec<TypeError>
}
impl CSTConverter<'_> {
    pub fn fresh_var(&mut self) -> Identifier {
        self.index += 1;
        Identifier(self.index)
    }
}

impl<'src> CSTConverter<'src> {
    pub(crate) fn convert_decl(&mut self, cst_decl: &CSTDecl<'src>) -> Result<Decl {
        let CSTDecl {
            name,
            args,
            output_type,
            expr,
        } = cst_decl;
        Decl {
            name: self.convert_span(name),
            args: args
                .iter()
                .map(|(var, t)| (self.convert_span(var), Types::from(*t)))
                .collect(),
            output_type: Types::from(*output_type),
            expr: self.convert_expr(expr),
        }
    }

    /// todo
    pub(crate) fn convert_block(
        &mut self,
        block: &VecDeque<CSTExpr<'src>>,
        index: usize,
    ) -> VecDeque<Expr> {
        let mut new_block = VecDeque::new();
        for i in index..block.len() {
            let expr = &block[i];
            if let CSTExpr::Let { var, t, expr } = expr {
                return VecDeque::from([Expr::Let {
                    var: self.convert_span(var),
                    t: Types::UNIT, // todo
                    expr: Box::new(self.convert_expr(expr)),
                    body: Box::new(Expr::Block(self.convert_block(block, i + 1))),
                }]);
            } else {
                new_block.push_back(self.convert_expr(expr));
            }
        }
        new_block
    }
    pub(crate) fn convert_expr(&mut self, expr: &CSTExpr<'src>) -> Expr {
        match expr {
            CSTExpr::Block(block) => Expr::Block(self.convert_block(block, 0)),
            CSTExpr::Index(cstindex) => Expr::Index(self.convert_var_indexing(cstindex)),
            CSTExpr::MethodCall {
                index,
                method,
                args,
            } => {
                let fresh_vars: VecDeque<Identifier> =
                    (0..args.len()).map(|_| self.fresh_var()).collect();
                let mut cbv_args = Expr::MethodCall(MethodCall {
                    index: self.convert_var_indexing(index),
                    method: Method::from(method),
                    args: fresh_vars.clone(),
                });
                for (i, expr) in args.iter().enumerate() {
                    cbv_args = Expr::Let {
                        var: fresh_vars[i],
                        t: Types::UNIT, // todo
                        expr: Box::new(self.convert_expr(expr)),
                        body: Box::new(cbv_args),
                    }
                }
                cbv_args
            }
            CSTExpr::FunCall { name, args } => {
                let fresh_vars: VecDeque<Identifier> =
                    (0..args.len()).map(|_| self.fresh_var()).collect();

                let mut cbv_args = Expr::Call {
                    name: self.convert_span(name),
                    args: fresh_vars.clone(),
                };

                for (i, expr) in args.iter().enumerate() {
                    cbv_args = Expr::Let {
                        var: fresh_vars[i],
                        t: Types::UNIT, // todo
                        expr: Box::new(self.convert_expr(expr)),
                        body: Box::new(cbv_args),
                    }
                }
                cbv_args
            }
            CSTExpr::Let { var, t, expr } => Expr::Let {
                var: self.convert_span(var),
                // todo the right type
                t: Types::UNIT, // from(t.unwrap()),
                expr: Box::new(self.convert_expr(expr)),
                body: Box::new(Expr::Block(VecDeque::new())),
            },
            CSTExpr::Assign { var, expr } => {
                let fresh = self.fresh_var();
                Expr::Let {
                    var: fresh,
                    t: Types::UNIT, // todo
                    expr: Box::new(self.convert_expr(expr)),
                    body: Box::new(Expr::MethodCall(MethodCall {
                        index: self.convert_var_indexing(var),
                        method: Method::Replace,
                        args: VecDeque::from([fresh]),
                    })),
                }
            }
            CSTExpr::While { cond, body } => Expr::While {
                cond: Box::new(self.convert_expr(cond)),
                body: Box::new(self.convert_expr(body)),
            },
            CSTExpr::If { cond, yes, no } => Expr::If {
                cond: Box::new(self.convert_expr(cond)),
                yes: Box::new(self.convert_expr(yes)),
                no: Box::new(self.convert_expr(no)),
            },
            CSTExpr::Constant(constant) => Expr::Constant(Constant::from(constant)),
            CSTExpr::BinaryOp { lhs, op, rhs } => {
                let lhs_var = self.fresh_var();
                let rhs_var = self.fresh_var();
                Expr::Let {
                    var: lhs_var,
                    t: Types::UNIT,
                    expr: Box::new(self.convert_expr(lhs)),
                    body: Box::new(Expr::Let {
                        var: rhs_var,
                        t: Types::UNIT,
                        expr: Box::new(self.convert_expr(rhs)),
                        body: Box::new(Expr::BinaryOp(BinaryOp {
                            lhs: lhs_var,
                            op: Binary::from(op),
                            rhs: rhs_var,
                        })),
                    }),
                }
            }
            CSTExpr::UnaryOp { lhs, op } => Expr::UnaryOp(UnaryOp {
                lhs: self.convert_span(lhs),
                op: Unary::from(op),
            }),
            CSTExpr::EmptyVec => Expr::EmptyVec,
        }
    }

    fn convert_var_indexing(&mut self, cst_index: &CSTIndex<'src>) -> Index {
        let CSTIndex { var, index } = cst_index;
        Index {
            var: self.convert_span(var),
            index: index.iter().map(|span| self.convert_span(span)).collect(),
        }
    }

    fn convert_span(&mut self, span: &Span<'src>) -> Identifier {
        match self.identifier_mapping.get(span.ident) {
            Some(ident) => *ident,
            None => {
                let fresh = self.fresh_var();
                self.identifier_mapping.insert(span.ident, fresh);
                fresh
            }
        }
    }
}

impl From<CSTTypes> for Types {
    fn from(cst_types: CSTTypes) -> Self {
        let CSTTypes {
            array_depth: depth,
            basic,
        } = cst_types;

        Types {
            depth: depth,
            basic: BasicTypes::from(basic),
        }
    }
}

impl From<CSTBasicTypes> for BasicTypes {
    fn from(cst_basic_types: CSTBasicTypes) -> Self {
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

impl From<&CSTBinary> for Binary {
    fn from(cst_binary: &CSTBinary) -> Self {
        match cst_binary {
            CSTBinary::Add => Binary::Add,
            CSTBinary::Sub => Binary::Sub,
            CSTBinary::Mult => Binary::Mult,
            CSTBinary::Div => Binary::Div,
            CSTBinary::EqEq => Binary::Equal,
            CSTBinary::Neq => Binary::NotEqual,
            CSTBinary::LessThan => Binary::LessThan,
            CSTBinary::LessThanEqual => Binary::LessThanEqual,
            CSTBinary::MoreThan => Binary::MoreThan,
            CSTBinary::MoreThanEqual => Binary::MoreThanEqual,
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
