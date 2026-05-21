use crate::{
    ast::{block::Block, decl::Decl, expr::Expr},
    interpreter::value::Value,
    structural_eq::StructuralEq,
};
use std::{fmt::Display, ops::Index};

/// Complete AST of a program
pub struct Ast<Ident> {
    pub top_level: Vec<Decl<Ident>>,
    pub block: Block<Ident>,
}

impl<Ident> Display for Ast<Ident>
where
    Ident: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for decl in &self.top_level {
            write!(f, "{}", decl)?;
        }
        write!(f, "{}", self.block)
    }
}

impl<Ident: StructuralEq> StructuralEq for Ast<Ident> {
    fn structural_eq(&self, rhs: &Self) -> bool {
        for (decl, rhs) in self.top_level.iter().zip(rhs.top_level.iter()) {
            if !decl.structural_eq(rhs) {
                return false;
            }
        }
        true
    }
}

impl<Ident> Ast<Ident> {
    pub fn get_value(mut self) -> Option<Value> {
        // todo also support array get value and see how to do it ...
        if self.block.statements.len() != 1 {
            return None;
        }

        if let Expr::Constant(c) = self.block.statements.remove(0) {
            Some(Value::Constant(c))
        } else {
            None
        }
    }
}
