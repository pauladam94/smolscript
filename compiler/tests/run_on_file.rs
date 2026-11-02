use crate::counter_alloc::{CounterAlloc, PPBytes};
use colored::Colorize;
use compiler::{ast::type_check, ast_interpreter::interpreter::Interpreter, cst::parser};
use std::sync::atomic::Ordering;

mod counter_alloc;

#[global_allocator]
static ALLOCATER: CounterAlloc = CounterAlloc::new();

pub fn print_allocation(s: &str) {
    println!(">>\t{s}");
    println!(
        "{} = {}",
        "[ALLOCATED]".purple(),
        PPBytes(ALLOCATER.allocated.load(Ordering::Relaxed))
    );
    println!(
        "{} = {}\n",
        "[MAX ALLOCATED]".purple(),
        PPBytes(ALLOCATER.max_allocated.load(Ordering::Relaxed))
    );
}

#[test]
pub fn run_one_file() {
    // let file = include_str!("../../examples/comments_simple.smol");
    let file = include_str!("../../examples/fibo_imp_acc.smol");
    // let file = include_str!("../../examples/simple_let.smol");
    print_allocation("BEGINS");

    let cst = parser(file).unwrap();
    print_allocation("After Parsing");

    let ast = type_check(&cst).unwrap();
    print_allocation("After Type checking");

    let mut interpreter = Interpreter::new(ast, false);
    print_allocation("After Interpreting creation");

    for _ in &mut interpreter {}
    print_allocation("After Interpreting AST");

    let value = interpreter.get_value().unwrap();

    println!("value = {value}");

    print_allocation("END");
}
