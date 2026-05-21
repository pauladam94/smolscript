// Definition of all the types used in the AST
pub mod ast;

// Complete Parser that builds the AST from text
pub mod parser;

// Checker of the AST (notably type checking)
pub mod check;


// Interpreter of the Language
pub mod interpreter;

pub mod structural_eq;
