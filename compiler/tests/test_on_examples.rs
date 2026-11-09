use crate::{
    counter_alloc::{CounterAlloc, PPBytes},
    test_api::TestAPI,
};
use colored::Colorize;
use compiler::{ast_interpreter::interpreter::Interpreter, parser, type_check};
use serde_derive::{Deserialize, Serialize};
use std::{
    process::{Command, Output},
    sync::atomic::Ordering::Relaxed,
};
use test_each_file::test_each_file;

mod test_api;

mod counter_alloc;

#[global_allocator]
static ALLOCATER: CounterAlloc = CounterAlloc::new();

test_each_file! { for ["smol", "json"] in "./examples" => full_test }

#[test]
fn debug_one_file() {
    full_test([
        include_str!("./../../examples/fibo_rec.smol"),
        include_str!("./../../examples/fibo_rec.json"),
    ])
}

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
    let file_parse = parser(file);

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
    let mut ast = file_parse
        .unwrap_or_else(|err| panic!("Parsing should success but failed with errors :\n{err}"));

    // We print the CST to see what it looks like
    println!(
        "{} : \n{}\n---",
        "[SUCCESS] Parsing of Initial file ".green(),
        ast
    );

    // We get the
    let ast_print = format!("{ast}");
    let mut ast_2 = parser(&ast_print).unwrap_or_else(|err| {
        panic!("[ERROR] Parsing of the printed AST should success but failed with errors :\n{err}")
    });

    let ast_type = type_check(&mut ast);
    println!("{} : \n{}\n---", "[SUCCESS] AST After Type checking :", ast);
    let ast_2_type = type_check(&mut ast_2);

    // TODO dot not check If Ids are equal -> useless
    if ast != ast_2 {
        panic!(
            "{} Printing then parsing again does not give the (structural) same AST.
>> AST :\n{}
>> AST > print > parse : \n{}
>> AST (debug) :\n{:#?}
>> AST > print > parse (debug) : \n{:#?}",
            "[ERROR]".red(),
            ast,
            ast_2,
            ast,
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

    let mut expected_cst_output = parser(input_str).unwrap_or_else(|err| {
        panic!("Parsing of expected output should success but failed with errors :\n{err}")
    });
    // TODO fail on type check
    let _ = type_check(&mut expected_cst_output);

    let mut expected_interpreter = Interpreter::new(expected_cst_output, false, TestAPI {});
    while let Some(_) = expected_interpreter.next() {}
    let expected_value = expected_interpreter
        .get_value()
        .unwrap_or_else(|| panic!("The output failed during interpretation"));

    // File should be interpreted to get a value
    let mut interpreter = Interpreter::new(ast, false, TestAPI {});
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
