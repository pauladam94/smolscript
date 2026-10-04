use crate::counter_alloc::{CounterAlloc, PPBytes};
use colored::Colorize;
use compiler::{ast::cst_to_ast, ast_interpreter::interpreter::Interpreter, cst::cst_parser};
use serde_derive::{Deserialize, Serialize};
use std::{
    process::{Command, Output},
    sync::atomic::Ordering::Relaxed,
};
use test_each_file::test_each_file;

mod counter_alloc;

#[global_allocator]
static ALLOCATER: CounterAlloc = CounterAlloc::new();

test_each_file! { for ["smol", "json"] in "./examples" => full_test }

#[derive(Serialize, Deserialize)]
enum TestType {
    Pass,
    Fail,
}

#[derive(Serialize, Deserialize)]
struct TestInfo {
    parse: Option<TestType>,
    check: Option<TestType>,
    output: Option<String>,
}

fn full_test([file, json_file]: [&str; 2]) {
    use TestType::*;
    let test_info: TestInfo = serde_json::from_str(json_file).expect("");

    println!("{} :\n{}", "[ORIGINAL FILE]".purple(), file);
    // Parsing of the original file
    let file_parse = cst_parser(file);

    // If we except a failed parsed then we stop here the test
    if let Fail = test_info
        .parse
        .unwrap_or_else(|| panic!("\"parse\" field missing in this json file :\n{}", json_file))
    {
        if let Ok(cst) = file_parse {
            panic!("Parsing should fail\nSuccess with CST :\n{}", cst);
        } else {
            return;
        }
    }

    // We get the CST and continue without excepting an error.
    let cst = file_parse
        .unwrap_or_else(|err| panic!("Parsing should success but failed with errors :\n{err}"));

    // We print the CST to see what it looks like
    println!("{} : \n{}\n---", "[SUCCESS] Parsing of CST".green(), cst,);

    // We get the
    let cst_print = format!("{cst}");
    let cst_2 = cst_parser(&cst_print).unwrap_or_else(|err| {
        panic!("Parsing of the printed CST should success but failed with errors :\n{err}")
    });

    let ast = cst_to_ast(&cst);
    println!("{} : \n{}\n---", "[SUCCESS] Parsing of AST", ast);
    let ast_2 = cst_to_ast(&cst_2);

    if ast != ast_2 {
        panic!(
            "{} Printing then parsing again does not give the (structural) same AST.
>> Second CST :\n{}
>> Second AST : \n{}",
            "[ERROR]".red(),
            cst_2,
            ast_2,
        )
    }

    match test_info
        .check
        .unwrap_or_else(|| panic!("\"check\" field missing in this json file: \n{}", json_file))
    {
        Pass => (),
        Fail => return,
    }

    // Output should be interpreted to get a value
    let input_str = &test_info.output.unwrap_or_else(|| {
        panic!(
            "\"output\" field missing in this json file :\n{}",
            json_file
        )
    });
    let expected_cst_output = cst_parser(input_str)
        .unwrap_or_else(|err| panic!("Parsing should success but failed with errors :\n{err}"));

    let mut expected_interpreter = Interpreter::new(cst_to_ast(&expected_cst_output), false);
    while let Some(_) = expected_interpreter.next() {}
    let expected_value = expected_interpreter
        .get_value()
        .unwrap_or_else(|| panic!("The output failed during interpretation"));

    // File should be interpreted to get a value
    let mut interpreter = Interpreter::new(cst_to_ast(&cst), false);
    while let Some(_) = interpreter.next() {}
    let value = interpreter.get_value().unwrap_or_else(|| {
        panic!("The execution stopped but it crashed. It didn't end with a value.")
    });

    assert_eq!(
        expected_value,
        value,
        "{} Expected this value '{}' but got '{}'.",
        "[ERROR]".red(),
        expected_value,
        value
    );

    println!(
        "{} = {}",
        "[MAX ALLOCATED]".purple(),
        PPBytes(ALLOCATER.max_allocated.load(Relaxed))
    );
    // Compiling to C and comparing output
}

fn compile_to_c_and_test_output() {
    let s: Output = Command::new("gcc")
        .args(["/C", "echo hello"])
        .output()
        .expect("failed to compile file");
    let stderr: String = str::from_utf8(&s.stderr)
        .expect("Not a valid UTF8 sequence.")
        .to_owned();
}
