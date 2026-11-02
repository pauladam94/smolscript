#![no_std]
extern crate alloc;

/// todo
pub mod utils;

/// 
pub mod types;

/// todo
pub mod cst;

// todo
pub mod ast;

/// todo
pub mod ast_interpreter;

/// This is the module that contains all of the convertion code to go
/// from a CST to the AST.
///
/// It also checks the type of the CST. Indeed the AST contains all
/// necessary type information for next step of compilation.
pub mod checker;

// todo
pub mod backend_c;

pub mod api;

// TEST SOME STUFF
pub mod compact_ast;





