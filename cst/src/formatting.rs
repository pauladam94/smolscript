use crate::{
    CST, CSTBasicTypes, CSTBinary, CSTConstant, CSTDecl, CSTExpr, CSTIndex, CSTMethod, CSTTypes,
    CSTUnary,
};
use itertools::Itertools;
use std::fmt::{Display, Formatter, Result};

impl Display for CSTUnary {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            CSTUnary::Not => write!(f, "not"),
        }
    }
}

impl Display for CST<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for decl in &self.top_level {
            write!(f, "{}\n", decl)?;
        }
        write!(f, "{}", self.expr)
    }
}
impl Display for CSTBinary {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            CSTBinary::Add => write!(f, "+"),
            CSTBinary::Sub => write!(f, "-"),
            CSTBinary::Div => write!(f, "/"),
            CSTBinary::Equal => write!(f, "=="),
            CSTBinary::NotEqual => write!(f, "!="),
        }
    }
}

impl Display for CSTDecl<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "fn {} (", self.name)?;
        let mut iter = self.args.iter();
        if let Some((s, t)) = iter.next() {
            write!(f, "{s} : {t}")?;
            for (s, t) in iter {
                write!(f, ", {s} : {t}")?;
            }
        }
        write!(f, ") {{\n{}\n}}", self.expr)
    }
}

pub struct CSTExprPP<'a, 'src> {
    depth: u8,
    expr: &'a CSTExpr<'src>,
}
impl Display for CSTExprPP<'_, '_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use CSTExpr::*;
        match self.expr {
            Block(block) => write!(f, "{}", block.iter().format(";\n")),
            Index(index) => write!(f, "{index}"),
            MethodCall {
                index,
                method: name,
                args,
            } => write!(f, "{index}.{name}({})", args.iter().format(", ")),
            FunCall { name: fun, args } => write!(
                f,
                "{fun}({})",
                args.iter()
                    .map(|arg| arg.pretty_printer(self.depth + 1))
                    .format(", ")
            ),
            LetBindings { var, block } => write!(
                f,
                "let {var} = {}",
                block
                    .iter()
                    .map(|e| e.pretty_printer(self.depth + 1))
                    .format("")
            ),
            While { cond, block } => write!(
                f,
                "while {cond} {}",
                block
                    .iter()
                    .map(|e| e.pretty_printer(self.depth + 1))
                    .format(";\n")
            ),
            If { cond, yes, no } => write!(
                f,
                "if ({}) {{\n{}\n}} else {{\n{}\n}}",
                cond.iter()
                    .map(|e| e.pretty_printer(self.depth + 1))
                    .format(";\n"),
                yes.iter()
                    .map(|e| e.pretty_printer(self.depth + 1))
                    .format(";\n"),
                no.iter()
                    .map(|e| e.pretty_printer(self.depth + 1))
                    .format(";\n")
            ),
            Constant(constant) => write!(f, "{constant}"),
            BinaryOp { lhs, op, rhs } => write!(f, "{lhs} {op} {rhs}"),
            UnaryOp { lhs, op } => write!(f, "{lhs} {op}"),
            EmptyVec => write!(f, "[]"),
        }
    }
}
impl<'src> CSTExpr<'src> {
    pub fn pretty_printer<'a>(&'a self, depth: u8) -> CSTExprPP<'a, 'src> {
        CSTExprPP { depth, expr: self }
    }
}
impl Display for CSTExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.pretty_printer(0))
    }
}

impl Display for CSTIndex<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.var)?;
        for ident in self.index.iter() {
            write!(f, "[{ident}]")?;
        }
        Ok(())
    }
}

impl Display for CSTConstant {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            CSTConstant::Int(i) => write!(f, "{i}"),
            CSTConstant::Float(fl) => write!(f, "{fl}"),
            CSTConstant::String(s) => write!(f, "'{s}'"),
            CSTConstant::Unit => write!(f, "()"),
        }
    }
}
impl Display for CSTMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            CSTMethod::Replace => write!(f, "replace"),
            CSTMethod::Push => write!(f, "push"),
            CSTMethod::Pop => write!(f, "pop"),
        }
    }
}
impl Display for CSTBasicTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            CSTBasicTypes::U16 => write!(f, "u16"),
            CSTBasicTypes::U8 => write!(f, "u8"),
            CSTBasicTypes::Unit => write!(f, "()"),
            CSTBasicTypes::Bool => write!(f, "bool"),
        }
    }
}

impl Display for CSTTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for _ in 0..self.depth {
            write!(f, "[")?;
        }
        write!(f, "{}", self.basic)?;
        for _ in 0..self.depth {
            write!(f, "]")?;
        }
        Ok(())
    }
}
