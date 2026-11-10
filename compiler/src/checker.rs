use crate::ast::{AST, Binary, ExprId, ExprNode, Unary};
use crate::lexer::Pos;
use crate::types::{TypeId, TypeNode};
use crate::utils::error::CompilerError;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use logos::Span;

#[derive(Debug)]
pub(crate) struct Checker<'a, 'src> {
    type_env: Vec<(&'src str, TypeId)>,
    pub ast: &'a mut AST<'src>,
    pub errors: Vec<CompilerError>,
}

impl<'a, 'src> Checker<'a, 'src> {
    pub fn new(ast: &'a mut AST<'src>) -> Self {
        Self {
            type_env: Vec::new(),
            ast,
            errors: Vec::new(),
        }
    }
}

impl<'a, 'src> Checker<'a, 'src> {
    pub fn new_error(&mut self, message: String, span: Span) {
        self.errors.push(CompilerError { message, span })
    }
    pub fn def_new_var(&mut self, var: &'src str, t: TypeId) {
        self.type_env.push((var, t))
    }
    pub fn pop_var(&mut self) -> (&'src str, TypeId) {
        self.type_env.pop().unwrap()
    }
    pub fn get_type_var(&mut self, var: ExprId) -> Option<TypeId> {
        let ExprNode::Var(var_name) = self.ast.exprs[var] else {
            self.new_error(
                format!("Expecting a single variable here."),
                self.ast.get_span(var),
            );
            return None;
        };
        self.type_env
            .iter()
            .rev()
            .find(|(name, _)| *name == var_name)
            .map(|(_, t)| *t)
    }
    pub fn check_expr(&mut self, id: ExprId) -> Option<TypeId> {
        use crate::ast::ExprNode;
        match self.ast.exprs[id] {
            ExprNode::Decl {
                name,
                args,
                return_type,
                expr,
            } => {
                let expr_type = self.check_expr(expr)?;

                Some(self.ast.types.alloc(TypeNode::Unit))
            }
            ExprNode::Var(_) => self.get_type_var(id),
            ExprNode::Seq { fst, snd } => None,
            ExprNode::Array { head, tail } => None,
            ExprNode::Tuple { head, tail } => None,
            ExprNode::Index { body, index } => None,
            ExprNode::MethodCall {
                index,
                method,
                args,
            } => None,
            ExprNode::FunCall { name, args } => {
                let type_fun = self.get_type_var(name)?;
                None
            }
            ExprNode::Let { var, t, expr } => None,
            ExprNode::Assign { index, expr } => None,
            ExprNode::While { cond, body } => None,
            ExprNode::If { cond, yes, no } => None,
            ExprNode::Const(constant) => None,
            ExprNode::BinaryOp { lhs, op, rhs } => None,
            ExprNode::UnaryOp { lhs, op } => None,
            ExprNode::TypeInfo { expr, t } => None,
            ExprNode::EmptyTuple => None,
            ExprNode::EmptyVec => None,
            ExprNode::Unknown => None,
        }
    }
    // pub(crate) fn check_decl(&mut self, cst_decl: &CSTDecl<'src>) -> Result<Decl, TypeError> {
    //     let CSTDecl {
    //         name,
    //         args,
    //         return_type,
    //         expr,
    //     } = cst_decl;

    //     let ast_args: Vec<(Identifier, Type)> = args
    //         .iter()
    //         .map(|(x, t)| (self.define_var(x, t), t.clone()))
    //         .collect();

    //     let fun_id = self.define_fun(name, &ast_args, return_type.clone());
    //     // TODO : push args in the typing environment
    //     // TODO : push type function
    //     let (expr, t) = self.check_expr(expr)?;
    //     // TODO : remove args from the typing environment (clear)
    //     if return_type.clone() != t {
    //         return Err(TypeError {
    //             message: format!(
    //                 "function '{name}' expected a return type '{return_type}' but got '{t}'."
    //             ),
    //             pos: name.pos,
    //         });
    //     }

    //     Ok(Decl {
    //         name: fun_id,
    //         args: ast_args,
    //         return_type,
    //         expr,
    //     })
    // }

    // pub(crate) fn check_block(
    //     &mut self,
    //     block: &VecDeque<CSTExpr<'src>>,
    //     index: usize,
    // ) -> Result<(Expr, Type), TypeError> {
    //     let mut new_block = VecDeque::new();
    //     let mut final_type = Type::unit();
    //     for i in index..block.len() {
    //         let expr = &block[i];
    //         if let CSTExpr::Let { var, t, expr } = expr {
    //             let (expr, texpr) = self.check_expr(expr)?;

    //             let var = self.define_var(var, texpr);
    //             let (body, tbody) = self.check_block(block, i + 1)?;
    //             let tvar = self.pop_var(var)?;
    //             // TODO : check the inferred type of the variable
    //             // with the type (optionnaly) declared in the CST

    //             return Ok((
    //                 Expr::Let {
    //                     var: var,
    //                     t: texpr, // todo
    //                     expr: Box::new(expr),
    //                     body: Box::new(body),
    //                 },
    //                 tbody,
    //             ));
    //         } else {
    //             let (expr, texpr) = self.check_expr(expr)?;
    //             final_type = texpr;
    //             new_block.push_back(expr);
    //         }
    //     }
    //     Ok((Expr::Block(new_block), final_type))
    // }
    // pub(crate) fn check_expr(&mut self, expr: &CSTExpr<'src>) -> Result<(Expr, Type), TypeError> {
    //     match expr {
    //         CSTExpr::Block(block) => self.check_block(block, 0),
    //         CSTExpr::Index(cst_index) => {
    //             let (expr, t) = self.check_indexing(cst_index)?;
    //             Ok((Expr::Index(expr), t))
    //         }
    //         CSTExpr::MethodCall {
    //             index,
    //             method,
    //             args,
    //         } => {
    //             let fresh_vars: VecDeque<Identifier> =
    //                 (0..args.len()).map(|_| self.fresh_var()).collect();
    //             let (index, tindex) = self.check_indexing(index)?;

    //             let mut cbv_args = Expr::MethodCall(MethodCall {
    //                 index,
    //                 method: Method::from(method),
    //                 args: fresh_vars.clone(),
    //             });
    //             for (i, expr) in args.iter().enumerate() {
    //                 let (expr, texpr) = self.check_expr(expr)?;
    //                 // TODO: check that texpr is the good type for the method
    //                 cbv_args = Expr::Let {
    //                     var: fresh_vars[i],
    //                     t: texpr,
    //                     expr: Box::new(expr),
    //                     body: Box::new(cbv_args),
    //                 }
    //             }
    //             Ok((cbv_args, todo!()))
    //         }
    //         CSTExpr::FunCall { name, args } => {
    //             let (fun_id, args_type, return_type) = self.check_function(name)?;
    //             let fresh_vars: VecDeque<Identifier> =
    //                 (0..args.len()).map(|_| self.fresh_var()).collect();

    //             let mut cbv_args = Expr::Call {
    //                 name: fun_id,
    //                 args: fresh_vars.clone(),
    //             };

    //             for (i, expr) in args.iter().enumerate() {
    //                 let (expr, t) = self.check_expr(expr)?;
    //                 cbv_args = Expr::Let {
    //                     var: fresh_vars[i],
    //                     t: Type::UNIT, // todo
    //                     expr: Box::new(expr),
    //                     body: Box::new(cbv_args),
    //                 }
    //             }
    //             Ok((cbv_args, return_type))
    //         }
    //         CSTExpr::Let { var, t, expr } => {
    //             // TODO: maybe refactor that
    //             let (expr, texpr) = self.check_expr(expr)?;
    //             let t = Type::from(t.ok_or_else(|| TypeError {
    //                 message: format!("the type of the variable '{var}' cannot be infered, it should be indicated explicitely."),
    //                 pos: var.pos,
    //             })?);
    //             if texpr != t {
    //                 return Err(TypeError {
    //                     message: format!("todo 2"),
    //                     pos: var.pos,
    //                 });
    //             }
    //             let var = self.define_var(var, t);
    //             Ok((
    //                 Expr::Let {
    //                     var,
    //                     t,
    //                     expr: Box::new(expr),
    //                     body: Box::new(Expr::Constant(Constant::Unit)),
    //                 },
    //                 texpr,
    //             ))
    //         }
    //         CSTExpr::Assign { var, expr } => {
    //             let fresh = self.fresh_var();
    //             let (expr, texpr) = self.check_expr(expr)?;
    //             let (index, tindex) = self.check_indexing(var)?;
    //             // TODO: check type of index with assign
    //             Ok((
    //                 Expr::Let {
    //                     var: fresh,
    //                     t: Type::unit(), // todo
    //                     expr: Box::new(expr),
    //                     body: Box::new(Expr::MethodCall(MethodCall {
    //                         index,
    //                         method: Method::Replace,
    //                         args: VecDeque::from([fresh]),
    //                     })),
    //                 },
    //                 Type::unit(),
    //             ))
    //         }
    //         CSTExpr::While { cond, body, pos } => {
    //             let (cond, tcond) = self.check_expr(cond)?;
    //             let (body, tbody) = self.check_expr(body)?;
    //             if tcond != Type::bool() {
    //                 return Err(TypeError {
    //                     message: format!(
    //                         "while condition expected to be of type bool but got '{tcond}'."
    //                     ),
    //                     pos: *pos,
    //                 });
    //             }
    //             Ok((
    //                 Expr::While {
    //                     cond: Box::new(cond),
    //                     body: Box::new(body),
    //                 },
    //                 Type::unit(),
    //             ))
    //         }
    //         CSTExpr::If { cond, yes, no, pos } => {
    //             let (cond, tcond) = self.check_expr(cond)?;
    //             let (yes, tyes) = self.check_expr(yes)?;
    //             let (no, tno) = self.check_expr(no)?;
    //             if tcond != Type::bool() {
    //                 return Err(TypeError {
    //                     message: format!(
    //                         "if condition expected to be of type bool but got '{tcond}'."
    //                     ),
    //                     pos: *pos,
    //                 });
    //             }
    //             // todo check types
    //             Ok((
    //                 Expr::If {
    //                     cond: Box::new(cond),
    //                     yes: Box::new(yes),
    //                     no: Box::new(no),
    //                 },
    //                 tyes,
    //             ))
    //         }
    //         CSTExpr::Constant(constant) => {
    //             let cst = Constant::from(constant);
    //             Ok((Expr::Constant(cst), cst.get_type()))
    //         }
    //         CSTExpr::BinaryOp { lhs, op, rhs } => {
    //             let lhs_var = self.fresh_var();
    //             let rhs_var = self.fresh_var();

    //             let op = Binary::from(op);

    //             let (lhs, tlhs) = self.check_expr(lhs)?;
    //             let (rhs, trhs) = self.check_expr(rhs)?;

    //             let t = self.check_binop(tlhs, op, trhs)?;

    //             Ok((
    //                 Expr::Let {
    //                     var: lhs_var,
    //                     t: tlhs,
    //                     expr: Box::new(lhs),
    //                     body: Box::new(Expr::Let {
    //                         var: rhs_var,
    //                         t: trhs,
    //                         expr: Box::new(rhs),
    //                         body: Box::new(Expr::BinaryOp(BinaryOp {
    //                             lhs: lhs_var,
    //                             op: Binary::from(op),
    //                             rhs: rhs_var,
    //                         })),
    //                     }),
    //                 },
    //                 t,
    //             ))
    //         }
    //         CSTExpr::UnaryOp { lhs, op } => {
    //             let (lhs, tlhs) = self.check_expr(lhs)?;
    //             let lhs_var = self.fresh_var();
    //             Ok((
    //                 Expr::Let {
    //                     var: lhs_var,
    //                     t: tlhs,
    //                     expr: Box::new(lhs),
    //                     body: Box::new(Expr::UnaryOp(UnaryOp {
    //                         lhs: lhs_var,
    //                         op: Unary::from(op),
    //                     })),
    //                 },
    //                 self.check_unary_op(tlhs, *op)?,
    //             ))
    //         }
    //         CSTExpr::EmptyVec => Ok((Expr::EmptyVec, Type::unknown().array())),
    //     }
    // }

    fn check_binop(&self, tlhs: TypeId, op: Binary, trhs: TypeId) -> Result<TypeId, CompilerError> {
        assert_eq!(tlhs, trhs);
        match (tlhs, op, trhs) {
            _ => todo!(),
        }
    }

    fn check_unary_op(&self, tlhs: TypeId, op: Unary) -> Result<TypeId, CompilerError> {
        todo!()
    }
}
