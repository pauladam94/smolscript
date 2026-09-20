use crate::{
    BasicTypes, Binary, Constant, Decl, Expr, Identifier, Method, Types, Unary, VarIndexing,
};
use cst::{
    CSTBasicTypes, CSTBinary, CSTConstant, CSTDecl, CSTExpr, CSTIndex, CSTMethod, CSTTypes,
    CSTUnary, parser::span::Span,
};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Default)]
pub(crate) struct CSTConverter<'src> {
    index: usize,
    var_table: HashMap<&'src str, Identifier>,
}
impl CSTConverter<'_> {
    pub fn fresh_var(&mut self) -> Identifier {
        self.index += 1;
        Identifier(self.index)
    }
}

impl<'src> CSTConverter<'src> {
    pub(crate) fn convert_decl(&mut self, cst_decl: &CSTDecl<'src>) -> Decl {
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
                .map(|(var, t)| (self.convert_span(var), Types::from(t)))
                .collect(),
            output_type: Types::from(output_type),
            expr: self.convert_expr(expr),
        }
    }
    pub(crate) fn convert_expr(&mut self, expr: &CSTExpr<'src>) -> Expr {
        match expr {
            CSTExpr::Block(block) => {
                Expr::Block(block.iter().map(|e| self.convert_expr(e)).collect())
            }
            CSTExpr::Index(cstindex) => Expr::Index(self.convert_var_indexing(cstindex)),
            CSTExpr::MethodCall {
                index,
                method,
                args,
            } => Expr::MethodCall {
                index: self.convert_var_indexing(index),
                method: Method::from(method),
                args: args.iter().map(|span| self.convert_span(span)).collect(),
            },
            CSTExpr::FunCall { name, args } => {
                let mut n = 0;
                let fresh_vars: VecDeque<Identifier> =
                    (0..args.len()).map(|_| self.fresh_var()).collect();
                let mut statements: VecDeque<Expr> = args
                    .iter()
                    .enumerate()
                    .map(|(i, expr)| {
                        n += 1;
                        Expr::LetBindings {
                            var: fresh_vars[i],
                            t: Types::UNIT,
                            expr: Box::new(self.convert_expr(expr)),
                        }
                    })
                    .collect();
                statements.push_back(Expr::FunCall {
                    name: self.convert_span(name),
                    args: fresh_vars,
                });
                Expr::Block(statements)
            }
            CSTExpr::LetBindings { var, expr } => Expr::LetBindings {
                var: self.convert_span(var),
                // todo the right type
                t: Types::UNIT,
                expr: Box::new(self.convert_expr(expr)),
            },
            CSTExpr::Assign { var, expr } => {
                let fresh = self.fresh_var();
                Expr::Block(VecDeque::from([
                    Expr::LetBindings {
                        var: fresh,
                        t: Types::UNIT,
                        expr: Box::new(self.convert_expr(expr)),
                    },
                    Expr::MethodCall {
                        index: self.convert_var_indexing(var),
                        method: Method::Replace,
                        args: VecDeque::from([fresh]),
                    },
                ]))
            }
            CSTExpr::While { cond, block } => Expr::While {
                cond: Box::new(self.convert_expr(cond.as_ref())),
                body: block.iter().map(|e| self.convert_expr(e)).collect(),
            },
            CSTExpr::If { cond, yes, no } => {
                let mut block = VecDeque::new();
                let cond_ident = self.ident_from_block(cond, &mut block);
                block.push_back(Expr::If {
                    cond: cond_ident,
                    yes: yes.iter().map(|e| self.convert_expr(e)).collect(),
                    no: no.iter().map(|e| self.convert_expr(e)).collect(),
                });
                Expr::Block(block)
            }
            CSTExpr::Constant(constant) => Expr::Constant(Constant::from(constant)),
            CSTExpr::BinaryOp { lhs, op, rhs } => {
                let mut block = VecDeque::new();
                let lhs_ident = self.ident_from_block(lhs, &mut block);
                let rhs_ident = self.ident_from_block(rhs, &mut block);
                block.push_back(Expr::BinaryOp {
                    lhs: lhs_ident,
                    op: Binary::from(op),
                    rhs: rhs_ident,
                });
                Expr::Block(block)
            }
            CSTExpr::UnaryOp { lhs, op } => Expr::UnaryOp {
                lhs: self.convert_span(lhs),
                op: Unary::from(op),
            },
            CSTExpr::EmptyVec => Expr::EmptyVec,
        }
    }

    /// Builds an identifier to replace a block of [Expr]
    fn ident_from_block(&mut self, lhs: &CSTExpr<'src>, block: &mut VecDeque<Expr>) -> Identifier {
        if let CSTExpr::Index(var_index) = lhs
            && var_index.index.is_empty()
        {
            self.convert_span(&var_index.var)
        } else {
            let fresh = self.fresh_var();
            block.push_back(Expr::LetBindings {
                var: fresh,
                t: Types::UNIT,
                expr: Box::new(self.convert_expr(lhs)),
            });
            fresh
        }
    }

    fn convert_var_indexing(&mut self, cst_index: &CSTIndex<'src>) -> VarIndexing {
        let CSTIndex { var, index } = cst_index;
        VarIndexing {
            var: self.convert_span(var),
            index: index.iter().map(|span| self.convert_span(span)).collect(),
        }
    }

    fn convert_span(&mut self, span: &Span<'src>) -> Identifier {
        match self.var_table.get(span.ident) {
            Some(ident) => *ident,
            None => {
                let fresh = self.fresh_var();
                self.var_table.insert(span.ident, fresh);
                fresh
            }
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
