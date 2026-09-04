use crate::{BasicTypes, Binary, Constant, Expr, VarIndex, Method, Types, Unary};
use itertools::Itertools;
use std::fmt::{Display, Formatter, Result};

impl Display for Types {
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
impl Display for BasicTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            BasicTypes::U16 => write!(f, "u16"),
            BasicTypes::U8 => write!(f, "u8"),
            BasicTypes::Bool => write!(f, "bool"),
            BasicTypes::Unit => write!(f, "unit"),
        }
    }
}

impl Expr {
    pub fn pretty_printer<'a>(&'a self, depth: u8) -> PPExpr<'a> {
        PPExpr { expr: self, depth }
    }
}
pub struct PPExpr<'a> {
    expr: &'a Expr,
    depth: u8,
}
impl Display for PPExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.expr {
            Expr::Index(index) => write!(f, "{index}"),
            Expr::MethodCall {
                index,
                method,
                args,
            } => {
                write!(f, "{index}.{method}(")?;
                let mut iter = args.iter();
                if let Some(arg) = iter.next() {
                    write!(f, "{arg}")?;
                    for arg in args.iter() {
                        write!(f, ", {arg}")?;
                    }
                }
                write!(f, ")")
            }
            Expr::FunCall { name, args } => {
                write!(f, "{name}({})", args.iter().format(", "))
            }
            Expr::LetBindings { var, t, block } => {
                write!(
                    f,
                    "let {var} : {t} = {}",
                    block
                        .iter()
                        .map(|b| b.pretty_printer(self.depth + 1))
                        .format(";")
                )
            }
            Expr::While { cond, block } => {
                write!(
                    f,
                    "while {cond} {{{}}}",
                    block
                        .iter()
                        .map(|e| e.pretty_printer(self.depth + 1))
                        .format(";")
                )
            }
            Expr::If { cond, yes, no } => write!(
                f,
                "if {cond} {{{}}} else {{{}}}",
                yes.iter()
                    .map(|e| e.pretty_printer(self.depth + 1))
                    .format(";"),
                no.iter()
                    .map(|e| e.pretty_printer(self.depth + 1))
                    .format(";")
            ),
            Expr::BinaryOp { lhs, op, rhs } => write!(f, "{lhs} {op} {rhs}"),
            Expr::UnaryOp { lhs: var, op } => write!(f, "{op} {var}"),
            Expr::Constant(constant) => write!(f, "{constant}"),
            Expr::EmptyVec => write!(f, "[]"),
            Expr::Block(block) => {
                let mut iter = block.iter();
                if let Some(expr) = iter.next() {
                    write!(f, "{}", expr.pretty_printer(self.depth + 1))?;
                    for expr in iter {
                        write!(f, ";\n{}", expr.pretty_printer(self.depth + 1))?;
                    }
                }
                Ok(())
            }
        }
    }
}

impl Display for VarIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.var)?;
        for var in self.index.iter() {
            write!(f, "[{var}]")?;
        }
        Ok(())
    }
}

impl Display for Unary {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Unary::Not => write!(f, "not"),
        }
    }
}
impl Display for Binary {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Binary::Add => write!(f, "+"),
            Binary::Sub => write!(f, "-"),
            Binary::Div => write!(f, "/"),
            Binary::Mult => write!(f, "*"),
            Binary::Equal => write!(f, "=="),
            Binary::NotEqual => write!(f, "!="),
            Binary::LessThan => write!(f, "<"),
        }
    }
}
impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Method::Push => write!(f, "push"),
            Method::Pop => write!(f, "pop"),
            Method::Replace => write!(f, "replace"),
        }
    }
}
impl Display for Constant {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Constant::Int(i) => write!(f, "{i}"),
            Constant::Bool(b) => write!(f, "{b}"),
            Constant::Unit => write!(f, "()"),
        }
    }
}
