use crate::{CST, CSTBlock, CSTConstant, CSTDecl, CSTExpr, CSTIndex};

pub trait StructuralEq<Rhs = Self> {
    fn structural_eq(&self, rhs: &Rhs) -> bool;
}

impl StructuralEq for CST<'_> {
    fn structural_eq(&self, rhs: &Self) -> bool {
        for (decl, rhs) in self.top_level.iter().zip(rhs.top_level.iter()) {
            if !decl.structural_eq(rhs) {
                return false;
            }
        }
        true
    }
}
impl StructuralEq for CSTBlock<'_> {
    fn structural_eq(&self, rhs: &Self) -> bool {
        if self.statements.len() != rhs.statements.len() {
            return false;
        }

        for (statement, rhs) in self.statements.iter().zip(rhs.statements.iter()) {
            if !statement.structural_eq(rhs) {
                return false;
            }
        }

        true
    }
}
impl StructuralEq for CSTDecl<'_> {
    fn structural_eq(&self, rhs: &Self) -> bool {
        if !self.block.structural_eq(&rhs.block) {
            return false;
        }
        if self.args.len() != rhs.args.len() {
            return false;
        }
        for ((arg, t), (arg2, t2)) in self.args.iter().zip(rhs.args.iter()) {
            if !arg.structural_eq(arg2) {
                return false;
            }
            if !(t == t2) {
                return false;
            }
        }

        true
    }
}

impl StructuralEq for CSTExpr<'_> {
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

impl StructuralEq for CSTIndex<'_> {
    fn structural_eq(&self, rhs: &Self) -> bool {
        if !self.var.structural_eq(&rhs.var) {
            return false;
        }
        if self.index.len() != rhs.index.len() {
            return false;
        }
        for (ident, ident2) in self.index.iter().zip(rhs.index.iter()) {
            if !ident.structural_eq(ident2) {
                return false;
            }
        }
        true
    }
}

impl StructuralEq for CSTConstant {
    fn structural_eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (CSTConstant::Int(i0), CSTConstant::Int(i1)) => i0 == i1,
            (CSTConstant::Float(f0), CSTConstant::Float(f1)) => f0 == f1,
            (CSTConstant::String(s0), CSTConstant::String(s1)) => s0 == s1,
            (CSTConstant::Unit, CSTConstant::Unit) => true,
            (_, _) => false,
        }
    }
}
