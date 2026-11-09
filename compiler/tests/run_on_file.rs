use crate::{counter_alloc::{CounterAlloc, PPBytes}, test_api::TestAPI};
use colored::Colorize;
use compiler::{ast_interpreter::interpreter::Interpreter, parser, type_check};
use std::{error::Error, sync::atomic::Ordering};

mod counter_alloc;
mod test_api;

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
pub fn run_one_file() -> Result<(), Box<dyn Error>> {
    // let file = include_str!("../../examples/comments_simple.smol");
    let file = include_str!("../../examples/fibo_rec.smol");
    // let file = include_str!("../../examples/simple_let.smol");
    print_allocation("BEGINS");

    let mut ast = parser(file)?;
    print_allocation("After Parsing");

    let ast_type_checked = type_check(&mut ast);
    print_allocation("After Type checking");

    let mut interpreter = Interpreter::new(ast, false, TestAPI {});
    print_allocation("After Interpreting creation");

    for _ in &mut interpreter {}
    print_allocation("After Interpreting ast_type_checked");

    let value = interpreter.get_value().unwrap();

    println!("value = {value}");

    print_allocation("END");

    Ok(())
}
