// To target devices without OS, we indicate here that we cannot in this crate
// use the standard library.
#![no_std]

// However we still want to allocate things, so we add the [alloc] crate
// which is also available for embedded targets.
extern crate alloc;

use crate::{
    api::SmolAPI,
    ast::AST,
    ast_interpreter::interpreter::Interpreter,
    checker::{Checker, TypeError},
    lexer::Token,
    parser::Parser,
    types::TypeId,
    utils::annotate::annotate,
};
use alloc::{format, string::String, vec::Vec};
use logos::Logos;

/// Differents utilitaries module that are useful throughout the project.
pub mod utils;

/// All the lexer code that is based on [logos].
pub mod lexer;

/// All the parser code that builds an [AST] from the lexer.
pub mod parser;

/// Definition of the AST data structure and useful function to use it.
pub mod ast;

/// This module centralized all the implementation of the [Display] and
/// [Debug] trait that are used for (pretty) printing.
pub mod formatting;

/// All the necessary machinery for the types of the language.
pub mod types;

/// Complete interpreter of the AST.
/// This goes through step by step inside the AST evaluating things.
pub mod ast_interpreter;

/// This module contains the type checking code of the AST.
pub mod checker;

// TODO : this is very incomplete
pub mod backend_c;

/// This module contains the trait [SmolAPI] which is the API to implement
/// for any program that want to use the SmolScript language.
///
/// The first class citizen that implements this API is the OS of the SmolPhone.
/// However we could imagine implementing it on different platform.
pub mod api;

pub fn interpreter_from_str<'src, T: SmolAPI>(
    input: &'src str,
    mut api: T,
) -> Option<Interpreter<'src, T>> {
    let mut lexer = Token::lexer(input);
    let mut parser = Parser::new(&mut lexer);

    let Some(expr) = parser.parse_block() else {
        api.error(format!(""), parser.span());
        return None;
    };
    parser.ast.expr = expr;

    let mut ast = parser.ast;
    let res_type_checking = type_check(&mut ast);

    Some(Interpreter::new(ast, false, api))
}

/// This function parse from an `&str` the abstract syntax tree : [AST].
/// This operation can fail and returns us with as much error as possible.
pub fn parser<'src>(input: &'src str) -> Result<AST<'src>, String> {
    let mut lexer = Token::lexer(input);
    let mut parser = Parser::new(&mut lexer);

    let expr = parser.parse_block().ok_or_else(|| {
        format!(
            "{}\n - - -\n{:#?}",
            annotate(input, parser.span(), "Parsing Error."),
            parser.ast
        )
    })?;
    parser.ast.expr = expr;
    let current_span = parser.span();
    match parser.bump() {
        Some(_) => Err(format!(
            "Result of parsing is :\n{}\nDebug result: \n{:#?}\n{}",
            parser.ast,
            parser.ast,
            annotate(input, current_span, "There is some token left here.")
        )),
        None => Ok(parser.ast),
    }
}

/// This function type checks the [AST]. It modifies the type information of
/// the [AST] that are unknown with complete type information.
pub fn type_check(ast: &mut AST) -> Result<TypeId, Vec<TypeError>> {
    let expr = ast.expr;
    let mut checker = Checker::new(ast);
    let checker_result = checker.check_expr(expr);
    if checker.ast.has_unknown_type() || !checker.errors.is_empty() || checker_result == None {
        Err(checker.errors)
    } else {
        let typeid = checker_result.unwrap();
        Ok(typeid)
    }
}
