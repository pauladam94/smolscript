pub mod parser;

pub mod binop;
pub mod cst;

#[derive(Debug, Clone)]
pub enum CSTUnary {
    Not,
}
pub mod unary;


pub mod block;
pub mod decl;
pub mod expr;
pub mod index;
pub mod literal;
pub mod method;
pub mod types;

// Useful Traits
pub mod into_owned;

pub mod structural_eq;
