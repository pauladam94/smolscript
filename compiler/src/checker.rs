use crate::ast::{Binary, Constant, Decl, Expr, Identifier, Index, Method, Unary};
use crate::checker::type_error::TypeError;
use crate::types::Type;
use crate::{
    ast::{BinaryOp, MethodCall, UnaryOp},
    cst::{
        CSTBinary, CSTConstant, CSTDecl, CSTExpr, CSTIndex, CSTMethod, CSTUnary, parser::span::Span,
    },
};
use alloc::format;
use alloc::{
    boxed::Box,
    collections::{btree_map::BTreeMap, vec_deque::VecDeque},
    vec::Vec,
};

pub mod type_error;

#[derive(Debug, Default)]
pub(crate) struct Checker<'src> {
    // All the function types declarations
    function_types: Vec<(Span<'src>, Identifier, Vec<Type>, Type)>,

    index: usize,
    types: Vec<(Identifier, Type)>,
    identifier_mapping: BTreeMap<&'src str, Identifier>,

    // TODO : Should be the only place where
    type_env: Vec<(&'src str, Identifier, Type)>,
}

impl Checker<'_> {
    pub fn fresh_var(&mut self) -> Identifier {
        self.index += 1;
        Identifier(self.index)
    }
}

impl<'src> Checker<'src> {
    pub(crate) fn check_decl(&mut self, cst_decl: &CSTDecl<'src>) -> Result<Decl, TypeError> {
        let CSTDecl {
            name,
            args,
            return_type,
            expr,
        } = cst_decl;

        let ast_args: Vec<(Identifier, Type)> = args
            .iter()
            .map(|(x, t)| (self.define_var(x, t), t.clone()))
            .collect();

        let fun_id = self.define_fun(name, &ast_args, return_type.clone());
        // TODO : push args in the typing environment
        // TODO : push type function
        let (expr, t) = self.check_expr(expr)?;
        // TODO : remove args from the typing environment (clear)
        if return_type.clone() != t {
            return Err(TypeError {
                message: format!(
                    "function '{name}' expected a return type '{return_type}' but got '{t}'."
                ),
                pos: name.pos,
            });
        }

        Ok(Decl {
            name: fun_id,
            args: ast_args,
            return_type,
            expr,
        })
    }

    pub(crate) fn check_block(
        &mut self,
        block: &VecDeque<CSTExpr<'src>>,
        index: usize,
    ) -> Result<(Expr, Type), TypeError> {
        let mut new_block = VecDeque::new();
        let mut final_type = Type::unit();
        for i in index..block.len() {
            let expr = &block[i];
            if let CSTExpr::Let { var, t, expr } = expr {
                let (expr, texpr) = self.check_expr(expr)?;

                let var = self.define_var(var, texpr);
                let (body, tbody) = self.check_block(block, i + 1)?;
                let tvar = self.pop_var(var)?;
                // TODO : check the inferred type of the variable
                // with the type (optionnaly) declared in the CST

                return Ok((
                    Expr::Let {
                        var: var,
                        t: texpr, // todo
                        expr: Box::new(expr),
                        body: Box::new(body),
                    },
                    tbody,
                ));
            } else {
                let (expr, texpr) = self.check_expr(expr)?;
                final_type = texpr;
                new_block.push_back(expr);
            }
        }
        Ok((Expr::Block(new_block), final_type))
    }
    pub(crate) fn check_expr(&mut self, expr: &CSTExpr<'src>) -> Result<(Expr, Type), TypeError> {
        match expr {
            CSTExpr::Block(block) => self.check_block(block, 0),
            CSTExpr::Index(cst_index) => {
                let (expr, t) = self.check_indexing(cst_index)?;
                Ok((Expr::Index(expr), t))
            }
            CSTExpr::MethodCall {
                index,
                method,
                args,
            } => {
                let fresh_vars: VecDeque<Identifier> =
                    (0..args.len()).map(|_| self.fresh_var()).collect();
                let (index, tindex) = self.check_indexing(index)?;

                let mut cbv_args = Expr::MethodCall(MethodCall {
                    index,
                    method: Method::from(method),
                    args: fresh_vars.clone(),
                });
                for (i, expr) in args.iter().enumerate() {
                    let (expr, texpr) = self.check_expr(expr)?;
                    // TODO: check that texpr is the good type for the method
                    cbv_args = Expr::Let {
                        var: fresh_vars[i],
                        t: texpr,
                        expr: Box::new(expr),
                        body: Box::new(cbv_args),
                    }
                }
                Ok((cbv_args, todo!()))
            }
            CSTExpr::FunCall { name, args } => {
                let (fun_id, args_type, return_type) = self.check_function(name)?;
                let fresh_vars: VecDeque<Identifier> =
                    (0..args.len()).map(|_| self.fresh_var()).collect();

                let mut cbv_args = Expr::Call {
                    name: fun_id,
                    args: fresh_vars.clone(),
                };

                for (i, expr) in args.iter().enumerate() {
                    let (expr, t) = self.check_expr(expr)?;
                    cbv_args = Expr::Let {
                        var: fresh_vars[i],
                        t: Type::UNIT, // todo
                        expr: Box::new(expr),
                        body: Box::new(cbv_args),
                    }
                }
                Ok((cbv_args, return_type))
            }
            CSTExpr::Let { var, t, expr } => {
                // TODO: maybe refactor that
                let (expr, texpr) = self.check_expr(expr)?;
                let t = Type::from(t.ok_or_else(|| TypeError {
                    message: format!("the type of the variable '{var}' cannot be infered, it should be indicated explicitely."),
                    pos: var.pos,
                })?);
                if texpr != t {
                    return Err(TypeError {
                        message: format!("todo 2"),
                        pos: var.pos,
                    });
                }
                let var = self.define_var(var, t);
                Ok((
                    Expr::Let {
                        var,
                        t,
                        expr: Box::new(expr),
                        body: Box::new(Expr::Constant(Constant::Unit)),
                    },
                    texpr,
                ))
            }
            CSTExpr::Assign { var, expr } => {
                let fresh = self.fresh_var();
                let (expr, texpr) = self.check_expr(expr)?;
                let (index, tindex) = self.check_indexing(var)?;
                // TODO: check type of index with assign
                Ok((
                    Expr::Let {
                        var: fresh,
                        t: Type::unit(), // todo
                        expr: Box::new(expr),
                        body: Box::new(Expr::MethodCall(MethodCall {
                            index,
                            method: Method::Replace,
                            args: VecDeque::from([fresh]),
                        })),
                    },
                    Type::unit(),
                ))
            }
            CSTExpr::While { cond, body, pos } => {
                let (cond, tcond) = self.check_expr(cond)?;
                let (body, tbody) = self.check_expr(body)?;
                if tcond != Type::bool() {
                    return Err(TypeError {
                        message: format!(
                            "while condition expected to be of type bool but got '{tcond}'."
                        ),
                        pos: *pos,
                    });
                }
                Ok((
                    Expr::While {
                        cond: Box::new(cond),
                        body: Box::new(body),
                    },
                    Type::unit(),
                ))
            }
            CSTExpr::If { cond, yes, no, pos } => {
                let (cond, tcond) = self.check_expr(cond)?;
                let (yes, tyes) = self.check_expr(yes)?;
                let (no, tno) = self.check_expr(no)?;
                if tcond != Type::bool() {
                    return Err(TypeError {
                        message: format!(
                            "if condition expected to be of type bool but got '{tcond}'."
                        ),
                        pos: *pos,
                    });
                }
                // todo check types
                Ok((
                    Expr::If {
                        cond: Box::new(cond),
                        yes: Box::new(yes),
                        no: Box::new(no),
                    },
                    tyes,
                ))
            }
            CSTExpr::Constant(constant) => {
                let cst = Constant::from(constant);
                Ok((Expr::Constant(cst), cst.get_type()))
            }
            CSTExpr::BinaryOp { lhs, op, rhs } => {
                let lhs_var = self.fresh_var();
                let rhs_var = self.fresh_var();

                let op = Binary::from(op);

                let (lhs, tlhs) = self.check_expr(lhs)?;
                let (rhs, trhs) = self.check_expr(rhs)?;

                let t = self.check_binop(tlhs, op, trhs)?;

                Ok((
                    Expr::Let {
                        var: lhs_var,
                        t: tlhs,
                        expr: Box::new(lhs),
                        body: Box::new(Expr::Let {
                            var: rhs_var,
                            t: trhs,
                            expr: Box::new(rhs),
                            body: Box::new(Expr::BinaryOp(BinaryOp {
                                lhs: lhs_var,
                                op: Binary::from(op),
                                rhs: rhs_var,
                            })),
                        }),
                    },
                    t,
                ))
            }
            CSTExpr::UnaryOp { lhs, op } => {
                let (lhs, tlhs) = self.check_expr(lhs)?;
                let lhs_var = self.fresh_var();
                Ok((
                    Expr::Let {
                        var: lhs_var,
                        t: tlhs,
                        expr: Box::new(lhs),
                        body: Box::new(Expr::UnaryOp(UnaryOp {
                            lhs: lhs_var,
                            op: Unary::from(op),
                        })),
                    },
                    self.check_unary_op(tlhs, *op)?,
                ))
            }
            CSTExpr::EmptyVec => Ok((Expr::EmptyVec, Type::unknown().array())),
        }
    }

    fn check_indexing(&mut self, cst_index: &CSTIndex<'src>) -> Result<(Index, Type), TypeError> {
        let CSTIndex { var, index } = cst_index;
        Index {
            var: self.check_span(var),
            index: index.iter().map(|span| self.check_span(span)).collect(),
        }
    }

    fn check_span(&mut self, span: &Span<'src>) -> Result<Identifier, TypeError> {
        self.identifier_mapping
            .get(span.ident)
            .map(|id| *id)
            .ok_or(TypeError {
                message: format!("identifier '{span}' not defined."),
                pos: span.pos,
            })
    }

    fn define_var(&self, var: &Span<'src>, t: &Type) -> Identifier {
        todo!()
        // match self.identifier_mapping.get(span.ident) {
        //     Some(ident) => *ident,
        //     None => {
        //         let fresh = self.fresh_var();
        //         self.identifier_mapping.insert(span.ident, fresh);
        //         fresh
        //     }
        // }
    }

    // TODO add a pos information for the return error
    fn pop_var(&self, var: Identifier) -> Result<Type, TypeError> {
        // self.types.pop_front()
        todo!()
    }

    fn check_function(
        &self,
        name: &Span<'src>,
    ) -> Result<(Identifier, Vec<Type>, Type), TypeError> {
        self.function_types
            .iter()
            .rev()
            .find(|(fn_name, _, _, _)| fn_name == name)
            .map(|(_, fn_id, args_types, return_type)| (*fn_id, args_types.clone(), *return_type))
            .ok_or_else(|| TypeError {
                message: format!(""),
                pos: name.pos,
            })
    }

    fn define_fun(
        &mut self,
        name: &Span<'_>,
        t: &Vec<(Identifier, Type)>,
        output_type: Type,
    ) -> Identifier {
        todo!()
    }

    fn check_binop(&self, tlhs: Type, op: Binary, trhs: Type) -> Result<Type, TypeError> {
        assert_eq!(tlhs, trhs);
        match (tlhs, op, trhs) {
            _ => todo!(),
        }
    }

    fn check_unary_op(&self, tlhs: Type, op: CSTUnary) -> Result<Type, TypeError> {
        todo!()
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
