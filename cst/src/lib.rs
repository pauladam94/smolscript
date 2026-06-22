pub mod lexer;
pub mod parser;
pub mod parser_combinator;


#[derive(Debug, Clone)]
pub enum CSTUnary {
    Not,
}
pub mod unary;


pub mod binop;
pub mod cst;
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
