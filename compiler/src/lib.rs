#![no_std]
extern crate alloc;

use crate::{ast::AST, from_cst::CSTConverter};
use cst::CST;



/// todo
pub mod utils;

/// todo
pub mod cst;

/// todo
pub mod ast_interpreter;

/// todo
pub mod checker;

/// This is the module that contains all of the convertion code to go
/// from a CST to the AST
pub mod from_cst;



pub mod ast;

pub mod backend_c;

