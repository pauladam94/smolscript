use crate::ast::{AST, Binary, ExprId, ExprNode, Unary};
use crate::types::{TypeId, TypeNode};
use crate::utils::error::CompilerError;
use crate::utils::span::Span;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

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
        if ExprNode::Var != self.ast.exprs[var] {
            self.new_error(
                format!("Expecting a single variable here."),
                self.ast.get_span(var),
            );
            return None;
        };
        let var = self.ast.get_slice(var);
        self.type_env
            .iter()
            .rev()
            .find(|(name, _)| *name == var)
            .map(|(_, t)| *t)
    }
    pub fn check_expr(&mut self, id: ExprId) -> Option<TypeId> {
        use crate::ast::ExprNode::*;
        match self.ast.exprs[id] {
            Decl {
                name,
                mut args,
                return_type,
                expr,
                ..
            } => {
                let mut number_args = 0;
                let mut args_type = self.ast.types.alloc(TypeNode::Unit);
                while let Tuple { head, tail } = self.ast.exprs[args] {
                    number_args += 1;
                    let TypeInfo { expr, t } = self.ast.exprs[head] else {
                        // todo nice error
                        return None;
                    };
                    args_type = self.ast.types.alloc(TypeNode::Tuple {
                        head: t,
                        tail: args_type,
                    });
                    if Var != self.ast.exprs[head] {
                        // todo nice error
                        return None;
                    };
                    let var = self.ast.get_slice(head);
                    self.def_new_var(var, t);
                    args = tail;
                }
                let expr_type = self.check_expr(expr)?;
                for _ in 0..number_args {
                    self.pop_var();
                }

                self.assert_eq_type(expr_type, return_type)?;

                if Var != self.ast.exprs[name] {
                    return None;
                };
                let fn_name = self.ast.get_slice(name);
                let fn_type = self.ast.types.alloc(TypeNode::Fun {
                    args: args_type,
                    return_type,
                });
                self.def_new_var(fn_name, fn_type);

                Some(self.ast.types.alloc(TypeNode::Unit))
            }
            Var { .. } => self.get_type_var(id),
            Seq { fst, snd } => {
                let _ = self.check_expr(fst)?;
                let snd_type = self.check_expr(snd)?;

                if let Let { .. } = self.ast.exprs[fst] {
                    self.pop_var();
                }

                Some(snd_type)
            }
            // TODO
            Array { head, tail } => None,
            // TODO
            Tuple { head, tail } => None,
            // TODO
            Index { body, index } => None,
            // TODO
            MethodCall {
                index,
                method,
                args,
            } => None,
            APICall { .. } => {
                panic!("Parser should not generate APICall nodes.")
            }
            FunCall { name, args } => {
                if let Some(fun_type) = self.get_type_var(name) {
                    None
                } else {
                    self.ast.exprs[id] = APICall { name, args };
                    // TODO check if the API call is with good types
                    None
                }
            }
            Let { var, t, expr } => {
                let Let { var, t, expr } = &mut self.ast.exprs[id] else {
                    return None;
                };
                todo!()
            }
            Assign { index, expr } => None,
            While { cond, body } => None,
            If { cond, yes, no } => None,
            Text => Some(self.ast.types.alloc(TypeNode::String)),
            Const(value) => None,
            BinaryOp { lhs, op, rhs } => None,
            UnaryOp { lhs, op } => None,
            TypeInfo { expr, t } => None,
            EmptyTuple => None,
            EmptyVec => None,
            Unknown => Some(self.ast.types.alloc(TypeNode::Unknown)),
        }
    }

    fn check_binop(&self, tlhs: TypeId, op: Binary, trhs: TypeId) -> Result<TypeId, CompilerError> {
        assert_eq!(tlhs, trhs);
        match (tlhs, op, trhs) {
            _ => todo!(),
        }
    }

    fn check_unary_op(&self, tlhs: TypeId, op: Unary) -> Result<TypeId, CompilerError> {
        todo!()
    }

    fn assert_eq_type(&self, got: TypeId, expected: TypeId) -> Option<()> {
        todo!()
    }
}
