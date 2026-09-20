use crate::{
    AST, BasicTypes, Binary, Constant, Decl, Expr, Identifier, Method, Types, Unary, VarIndexing,
};
use itertools::Itertools;
use utils::pad::Pad;
use std::fmt::{Display, Formatter, Result, Write};

impl Display for AST {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for decl in &self.top_level {
            writeln!(f, "{decl}")?;
        }
        write!(f, "{}", self.expr)
    }
}

impl Display for Decl {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "fn {}({}) -> {} {{",
            self.name,
            self.args
                .iter()
                .format_with(", ", |(x, t), f| f(&format_args!("{x} : {t}"))),
            self.output_type
        )?;
        write!(Pad { f }, "\n{}", self.expr)?;
        write!(f, "\n}}")
    }
}

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
            BasicTypes::U64 => write!(f, "u64"),
            BasicTypes::U32 => write!(f, "u32"),
            BasicTypes::U16 => write!(f, "u16"),
            BasicTypes::U8 => write!(f, "u8"),
            BasicTypes::Bool => write!(f, "bool"),
            BasicTypes::Unit => write!(f, "unit"),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Expr::Index(index) => write!(f, "{index}"),
            Expr::MethodCall {
                index,
                method,
                args,
            } => write!(f, "{index}.{method}({})", args.iter().format(", ")),
            Expr::FunCall { name, args } => {
                write!(f, "{name}({})", args.iter().format(", "))
            }
            Expr::LetBindings { var, t, expr } => {
                write!(f, "let {var} : {t} = ")?;
                write!(Pad { f }, "{expr}")
            }
            Expr::While { cond, body: block } => {
                write!(f, "while ({cond}) {{")?;
                write!(Pad { f }, "\n{}", block.iter().format(";\n"))?;
                write!(f, "\n}}")
            }
            Expr::If { cond, yes, no } => {
                write!(f, "if {cond} {{")?;
                write!(Pad { f }, "\n{}", yes.iter().format(";\n"))?;
                write!(f, "\n}} else {{")?;
                write!(Pad { f }, "\n{}", no.iter().format(";\n"))?;
                write!(f, "\n}}")
            }
            Expr::BinaryOp { lhs, op, rhs } => write!(f, "{lhs} {op} {rhs}"),
            Expr::UnaryOp { lhs, op } => write!(f, "{op} {lhs}"),
            Expr::Constant(constant) => write!(f, "{constant}"),
            Expr::EmptyVec => write!(f, "[]"),
            Expr::Block(block) => write!(f, "{}", block.iter().format(";\n")),
        }
    }
}

impl Display for VarIndexing {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.var)?;
        for var in &self.index {
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
            Method::Len => write!(f, "len"),
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

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "x{}", self.0)
    }
}
