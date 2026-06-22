use ast::AST;
use colored::Colorize;
use cst::{parser::complete::cst_parser, structural_eq::StructuralEq};
use interpreter::interpreter::Interpreter;
use serde_derive::{Deserialize, Serialize};
use test_each_file::test_each_file;

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
    let test_info: TestInfo = serde_json::from_str(json_file).unwrap();

    let file_parse = cst_parser(file);
    match test_info
        .parse
        .unwrap_or_else(|| panic!("\"parse\" field missing in this json file :\n{}", json_file))
    {
        Pass => {
            // {
            // println!("Here is what the lexer gives you :");
            // let mut input = Token::lexer(file);
            // loop {
            //     match input.next() {
            //         Some(Ok(t)) => println!("-- '{t}'"),
            //         None | Some(Err(())) => break,
            //     }
            // }
            // }
            let file_parse = file_parse.unwrap_or_else(|err| {
                panic!("Parsing should success but failed with errors :\n{err}")
            });
            println!(
                "{} Parsing, here is the parsed file :\n{}\n---",
                "[SUCCESS]".green(),
                file_parse
            );
            let file_parse_print = format!("{file_parse}");
            let file_parse_print_parse = cst_parser(&file_parse_print).unwrap_or_else(|err| {
                panic!("Parsing of the printed CST should success but failed with errors :\n{err}")
            });
            if !file_parse_print_parse.structural_eq(&file_parse) {
                panic!(
                    "{}Printing then parsing again does not give the (structural) same AST.\n>> Second Parsing : \n{}",
                    "[ERROR]".red(),
                    file_parse_print_parse
                )
            }

            match test_info.check.unwrap_or_else(|| {
                panic!("\"check\" field missing in this json file: \n{}", json_file)
            }) {
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
            let expected_ast_output = cst_parser(input_str).unwrap_or_else(|err| {
                panic!("Parsing should success but failed with errors :\n{err}")
            });

            let mut expected_interpreter = Interpreter::new(AST::from(expected_ast_output));
            while let Some(_) = expected_interpreter.next() {}
            let expected_value = expected_interpreter
                .get_value()
                .unwrap_or_else(|| panic!("The output failed during interpretation"));

            // File should be interpreted to get a value
            let mut interpreter = Interpreter::new(AST::from(file_parse));
            while let Some(_) = interpreter.next() {}
            let value = interpreter.get_value().unwrap_or_else(|| {
                panic!("The execution stopped but it crashed. It didn't end with a value")
            });

            if !expected_value.eq(&value, expected_interpreter.heap(), interpreter.heap()) {
                panic!("{}Expected", "[ERROR]".red());
            }
        }
        Fail => {
            if let Ok(ast) = file_parse {
                panic!("Parsing should fail\nSuccess with ast :\n{}", ast);
            }
        }
    }
}
