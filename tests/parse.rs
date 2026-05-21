use colored::Colorize;
use serde_derive::{Deserialize, Serialize};
use smolscript::{
    interpreter::interpreter::Interpreter, parser::complete::smolscript_parser,
    structural_eq::StructuralEq,
};
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
    output: Option<String>,
}

fn full_test([file, json_file]: [&str; 2]) {
    use TestType::*;
    let test_info: TestInfo = serde_json::from_str(json_file).unwrap();

    let file_parse = smolscript_parser(file);
    match test_info
        .parse
        .unwrap_or_else(|| panic!("\"parse\" field missing in this json file :\n{}", json_file))
    {
        Pass => {
            {
                // println!("Here is what the lexer gives you :");
                // let mut input = Token::lexer(file);
                // loop {
                //     match input.next() {
                //         Some(Ok(t)) => println!("-- '{t}'"),
                //         None | Some(Err(())) => break,
                //     }
                // }
            }
            let file_parse = file_parse.unwrap_or_else(|err| {
                panic!("Parsing should success but failed with errors :\n{err}")
            });
            println!(
                "{} Parsing, here is the parsed file :\n{}\n---",
                "[SUCCESS]".green(),
                file_parse
            );
            let file_parse_print = format!("{file_parse}");
            let file_parse_print_parse =
                smolscript_parser(&file_parse_print).unwrap_or_else(|err| {
                    panic!(
                        "Parsing of the printed AST should success but failed with errors :\n{err}"
                    )
                });
            if !file_parse_print_parse.structural_eq(&file_parse) {
                panic!(
                    "Printing then parsing again does not give the (structural) same AST.\n>> Second Parsing : \n{}",
                    file_parse_print_parse
                )
            }

            // Output check
            let expected_string = test_info.output.unwrap_or_else(|| {
                panic!(
                    "\"output\" field missing in this json file :\n{}",
                    json_file
                )
            });
            let expected_value = smolscript_parser(&expected_string)
                .unwrap_or_else(|err| {
                    panic!("Parsing should success but failed with errors :\n{err}")
                })
                .get_value()
                .unwrap_or_else(|| panic!("\"output\" field should parse to a value"));

            let interpreter = Interpreter::new(file_parse);
        }
        Fail => {
            if let Ok(ast) = file_parse {
                panic!("Parsing should fail\nSuccess with ast :\n{}", ast);
            }
        }
    }
}
