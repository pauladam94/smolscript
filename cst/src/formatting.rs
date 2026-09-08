use crate::{
    CST, CSTBasicTypes, CSTBinary, CSTConstant, CSTDecl, CSTExpr, CSTIndex, CSTMethod, CSTTypes,
    CSTUnary,
};
use itertools::Itertools;
use std::fmt::{Display, Formatter, Result, Write};

struct Pad<'a, 'b> {
    f: &'a mut Formatter<'b>,
}

impl Write for Pad<'_, '_> {
    fn write_str(&mut self, s: &str) -> Result {
        for s in s.split_inclusive('\n') {
            self.f.write_str(s)?;
            if s.ends_with('\n') {
                self.f.write_str("\t")?;
            }
        }
        Ok(())
    }
}

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
            writeln!(f, "{decl}")?;
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
            CSTBinary::Mult => write!(f, "*"),
            CSTBinary::EqEq => write!(f, "=="),
            CSTBinary::Neq => write!(f, "!="),
        }
    }
}

impl Display for CSTDecl<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "fn {} ({}) -> {} {{",
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

impl Display for CSTExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use CSTExpr::*;
        match self {
            Block(block) => write!(f, "{}", block.iter().format(";\n")),
            Index(index) => write!(f, "{index}"),
            MethodCall {
                index,
                method: name,
                args,
            } => write!(f, "{index}.{name}({})", args.iter().format(", ")),
            FunCall { name: fun, args } => write!(f, "{fun}({})", args.iter().format(", ")),
            LetBindings { var, expr } => write!(f, "let {var} = {expr}"),
            Assign { var, expr } => write!(f, "{var} := {expr}"),
            While { cond, block } => {
                write!(f, "while ({cond}) {{")?;
                write!(Pad { f }, "\n{}", block.iter().format(";\n"))?;
                write!(f, "\n}}")
            }
            If { cond, yes, no } => {
                write!(f, "if ({cond}) {{")?;
                write!(Pad { f }, "\n{}", yes.iter().format(";\n"),)?;
                write!(f, "\n}} else {{")?;
                write!(Pad { f }, "\n{}", no.iter().format(";\n"))?;
                write!(f, "\n}}")
            }
            Constant(constant) => write!(f, "{constant}"),
            BinaryOp { lhs, op, rhs } => write!(f, "{lhs} {op} {rhs}"),
            UnaryOp { lhs, op } => write!(f, "{lhs} {op}"),
            EmptyVec => write!(f, "[]"),
        }
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
            CSTConstant::Bool(b) => write!(f, "{b}"),
        }
    }
}
impl Display for CSTMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            CSTMethod::Replace => write!(f, "replace"),
            CSTMethod::Push => write!(f, "push"),
            CSTMethod::Pop => write!(f, "pop"),
            CSTMethod::Len => write!(f, "len"),
        }
    }
}
impl Display for CSTBasicTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            CSTBasicTypes::U64 => write!(f, "u64"),
            CSTBasicTypes::U32 => write!(f, "u32"),
            CSTBasicTypes::U16 => write!(f, "u16"),
            CSTBasicTypes::U8 => write!(f, "u8"),
            CSTBasicTypes::Unit => write!(f, "()"),
            CSTBasicTypes::Bool => write!(f, "bool"),
        }
    }
}

impl Display for CSTTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for _ in 0..self.array_depth {
            write!(f, "[")?;
        }
        write!(f, "{}", self.basic)?;
        for _ in 0..self.array_depth {
            write!(f, "]")?;
        }
        Ok(())
    }
}
