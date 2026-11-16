use crate::api::SmolAPI;
use crate::ast::{AST, Binary, Constant, ExprId, ExprNode, ExprView, Method, Unary};
use crate::ast_interpreter::interpreter::Interpreter;
use crate::ast_interpreter::memory::Memory;
use crate::types::{TypeId, TypeView};
use crate::utils::annotate::annotate;
use crate::utils::pad::Pad;
use core::fmt::{Debug, Display, Formatter, Result, Write};
use itertools::Itertools;

impl Display for AST<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.show(self.root))
    }
}

impl Display for ExprView<'_, '_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.ast.exprs[self.id] {
            ExprNode::Decl {
                name,
                args,
                return_type,
                expr,
                ..
            } => {
                write!(
                    f,
                    "fn {}{} -> {} {{",
                    self.ast.show(name),
                    self.ast.show(args),
                    self.ast.show_type(return_type),
                )?;
                write!(Pad { f }, "\n{}", self.ast.show(expr))?;
                write!(f, "\n}}")
            }
            ExprNode::Var => write!(f, "{}", self.ast.get_slice(self.id)),
            ExprNode::Seq { .. } => {
                let mut id = self.id;
                while let ExprNode::Seq { fst, snd, .. } = self.ast.exprs[id] {
                    write!(f, "{};\n", self.ast.show(fst))?;
                    id = snd;
                }
                write!(f, "{}", self.ast.show(id))?;
                Ok(())
            }
            ExprNode::Array { .. } => {
                let mut id = self.id;
                write!(f, "[")?;
                while let ExprNode::Array { head, tail, .. } = self.ast.exprs[id] {
                    write!(f, "{}", self.ast.show(head))?;
                    if self.ast.exprs[tail] != ExprNode::EmptyVec {
                        write!(f, ", ")?;
                    }
                    id = tail;
                }
                write!(f, "]")
            }
            ExprNode::Tuple { .. } => {
                let mut id = self.id;
                write!(f, "(")?;
                while let ExprNode::Tuple { head, tail, .. } = self.ast.exprs[id] {
                    write!(f, "{}", self.ast.show(head))?;
                    if self.ast.exprs[tail] != ExprNode::EmptyTuple {
                        write!(f, ", ")?;
                    }
                    id = tail;
                }
                write!(f, ")")
            }
            ExprNode::Index { body, index, .. } => {
                write!(f, "{}", self.ast.show(body))?;
                write!(f, "[{}]", self.ast.show(index))
            }
            ExprNode::MethodCall {
                index,
                method,
                args,
                ..
            } => {
                write!(
                    f,
                    "{}.{method}({})",
                    self.ast.show(index),
                    self.ast.show(args)
                )
            }
            ExprNode::FunCall { name, args, .. } => {
                write!(f, "{}{}", self.ast.show(name), self.ast.show(args))
            }
            ExprNode::Let { var, t, expr, .. } => {
                write!(
                    f,
                    "let {} : {} = {}",
                    self.ast.show(var),
                    self.ast.show_type(t),
                    self.ast.show(expr),
                )
            }
            ExprNode::Assign { index, expr, .. } => {
                write!(f, "{} = {}", self.ast.show(index), self.ast.show(expr))
            }
            ExprNode::While { cond, body, .. } => {
                write!(f, "while ({}) {{", self.ast.show(cond))?;
                write!(Pad { f }, "\n{}", self.ast.show(body))?;
                write!(f, "\n}}")
            }
            ExprNode::If { cond, yes, no, .. } => {
                write!(f, "if ({}) {{", self.ast.show(cond))?;
                write!(Pad { f }, "\n{}", self.ast.show(yes))?;
                write!(f, "\n}} else {{")?;
                write!(Pad { f }, "\n{}", self.ast.show(no))?;
                write!(f, "\n}}")
            }
            ExprNode::Text => write!(f, "\"{}\"", self.ast.get_slice(self.id)),
            ExprNode::Const(value) => write!(f, "{value}"),
            ExprNode::BinaryOp { lhs, op, rhs, .. } => {
                write!(f, "({} {op} {})", self.ast.show(lhs), self.ast.show(rhs))
            }
            ExprNode::UnaryOp { lhs, op, .. } => {
                write!(f, "{op} {}", self.ast.show(lhs))
            }
            ExprNode::TypeInfo { expr, t, .. } => {
                write!(f, "{} : {}", self.ast.show(expr), self.ast.show_type(t))
            }
            ExprNode::EmptyVec => write!(f, "[]"),
            ExprNode::Unknown => write!(f, "?"),
            ExprNode::EmptyTuple => write!(f, "()"),
        }
    }
}

// impl Debug for ExprNode<'_> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         match self {
//             Self::Decl {
//                 name,
//                 args,
//                 return_type,
//                 expr,
//             } => f
//                 .debug_struct("Decl")
//                 .field("name", name)
//                 .field("args", args)
//                 .field("return_type", return_type)
//                 .field("expr", expr)
//                 .finish(),
//             Self::Var(arg0) => f.debug_tuple("Var").field(arg0).finish(),
//             Self::Seq { fst, snd } => f
//                 .debug_struct("Seq")
//                 .field("fst", fst)
//                 .field("snd", snd)
//                 .finish(),
//             Self::Array { head, tail } => f
//                 .debug_struct("Array")
//                 .field("head", head)
//                 .field("tail", tail)
//                 .finish(),
//             Self::Tuple { head, tail } => f
//                 .debug_struct("Tuple")
//                 .field("head", head)
//                 .field("tail", tail)
//                 .finish(),
//             Self::Index { body, index } => f
//                 .debug_struct("Index")
//                 .field("body", body)
//                 .field("index", index)
//                 .finish(),
//             Self::MethodCall {
//                 index,
//                 method,
//                 args,
//             } => f
//                 .debug_struct("MethodCall")
//                 .field("index", index)
//                 .field("method", method)
//                 .field("args", args)
//                 .finish(),
//             Self::FunCall { name, args } => f
//                 .debug_struct("FunCall")
//                 .field("name", name)
//                 .field("args", args)
//                 .finish(),
//             Self::Let { var, t, expr } => f
//                 .debug_struct("Let")
//                 .field("var", var)
//                 .field("t", t)
//                 .field("expr", expr)
//                 .finish(),
//             Self::Assign { index, expr } => f
//                 .debug_struct("Assign")
//                 .field("index", index)
//                 .field("expr", expr)
//                 .finish(),
//             Self::While { cond, body } => f
//                 .debug_struct("While")
//                 .field("cond", cond)
//                 .field("body", body)
//                 .finish(),
//             Self::If { cond, yes, no } => f
//                 .debug_struct("If")
//                 .field("cond", cond)
//                 .field("yes", yes)
//                 .field("no", no)
//                 .finish(),
//             Self::Constant(arg0) => f.debug_tuple("Constant").field(arg0).finish(),
//             Self::BinaryOp { lhs, op, rhs } => f
//                 .debug_struct("BinaryOp")
//                 .field("lhs", lhs)
//                 .field("op", op)
//                 .field("rhs", rhs)
//                 .finish(),
//             Self::UnaryOp { lhs, op } => f
//                 .debug_struct("UnaryOp")
//                 .field("lhs", lhs)
//                 .field("op", op)
//                 .finish(),
//             Self::TypeInfo { expr, t } => f
//                 .debug_struct("TypeInfo")
//                 .field("expr", expr)
//                 .field("t", t)
//                 .finish(),
//             Self::EmptyTuple => write!(f, "EmptyTuple"),
//             Self::EmptyVec => write!(f, "EmptyVec"),
//             Self::Unknown => write!(f, "Unknown"),
//         }
//     }
// }

impl Display for TypeView<'_, '_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use crate::types::TypeNode;
        match self.ast.types[self.id] {
            TypeNode::U64 => write!(f, "u64"),
            TypeNode::U32 => write!(f, "u32"),
            TypeNode::U16 => write!(f, "u16"),
            TypeNode::U8 => write!(f, "u8"),
            TypeNode::Bool => write!(f, "bool"),
            TypeNode::Unit => write!(f, "()"),
            TypeNode::Array(id) => {
                write!(f, "[{}]", self.ast.show_type(id))
            }
            TypeNode::Tuple { .. } => {
                let mut id = self.id;
                write!(f, "(")?;
                while let TypeNode::Tuple { head, tail } = self.ast.types[id] {
                    write!(f, "{}", self.ast.show_type(head))?;
                    id = tail;
                }
                write!(f, ")")
            }
            TypeNode::Fun { args, return_type } => {
                write!(
                    f,
                    "{} -> {}",
                    self.ast.show_type(args),
                    self.ast.show_type(return_type)
                )
            }
            TypeNode::Unknown => write!(f, "?"),
            TypeNode::String => write!(f, "text"),
        }
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
            Binary::LessThanEqual => write!(f, "<="),
            Binary::MoreThan => write!(f, ">"),
            Binary::MoreThanEqual => write!(f, ">="),
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
            Constant::Float(v) => write!(f, "{v}"),
            Constant::Unit => write!(f, "()"),
        }
    }
}

impl<T: SmolAPI> Display for Interpreter<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "{}{}{}", "---\n", ">> At Step ", self.step)?;
        writeln!(f, "PC : {}", self.program_counter.0)?;
        writeln!(
            f,
            "Stack Values : [{}]",
            self.values_stack.iter().format(",")
        )?;
        writeln!(
            f,
            "{}",
            annotate(
                self.ast.input,
                self.ast.get_span(self.program_counter),
                "interpreter position"
            )
        )?;
        writeln!(f, "AST (exprs) :\n{:?}", self.ast.exprs)?;
        writeln!(f, "AST (types) :\n{:?}", self.ast.types)?;
        writeln!(f, "{}\n{}", ">> Memory :", self.memory)?;
        Ok(())
    }
}

impl<'src> Display for Memory<'src> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            self.stack
                .iter()
                .format_with("\n", |(x, v), f| f(&format_args!("{x} = {v}")))
        )
    }
}

// impl Display for ExprId {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         write!(f, "[{}]", self.0)
//     }
// }
// impl Display for TypeId {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         write!(f, "[{}]", self.0)
//     }
// }
impl Debug for ExprId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[{}]", self.0)
    }
}
impl Debug for TypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[{}]", self.0)
    }
}

// impl core::fmt::Debug for AST<'_> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         f.debug_struct("AST")
//             .field("positions", &self.positions)
//             .field("exprs", &self.exprs)
//             .field("types", &self.types)
//             .field("expr", &self.expr)
//             .finish()
//     }
// }
