use crate::counter_alloc::{CounterAlloc, PPBytes};
use colored::Colorize;
use compiler::{ast::cst_to_ast, ast_interpreter::interpreter::Interpreter, cst::cst_parser};
use std::sync::atomic::Ordering;

mod counter_alloc;

#[global_allocator]
static ALLOCATER: CounterAlloc = CounterAlloc::new();

pub fn print_allocated() {
    println!(
        "{} = {}",
        "[ALLOCATED]".purple(),
        PPBytes(ALLOCATER.allocated.load(Ordering::Relaxed))
    );
}
pub fn print_max_allocated() {
    println!(
        "{} = {}",
        "[MAX ALLOCATED]".purple(),
        PPBytes(ALLOCATER.max_allocated.load(Ordering::Relaxed))
    );
}

#[test]
pub fn run_one_file() {
    // let file = include_str!("../../examples/fibo_imp_acc.smol");
    let file = include_str!("../../examples/simple_let.smol");
    print_allocated();
    print_max_allocated();

    let cst = cst_parser(file).unwrap();
    print_allocated();
    print_max_allocated();

    let ast = cst_to_ast(&cst);
    print_allocated();
    print_max_allocated();

    let mut interpreter = Interpreter::new(ast, false);
    print_allocated();
    print_max_allocated();

    for _ in &mut interpreter {}
    print_allocated();
    print_max_allocated();

    let value = interpreter.get_value().unwrap();
    print_allocated();
    print_max_allocated();

    println!("value = {value}");

    // print_allocated();
    print_max_allocated();
}
