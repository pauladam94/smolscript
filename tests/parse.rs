use serde_derive::{Deserialize, Serialize};
use smolscript::parser::lexer::smolscript_parser;
use test_each_file::test_each_file;

test_each_file! { for ["smol", "json"] in "./examples" => test_parse }

#[derive(Serialize, Deserialize)]
enum TestType {
    Pass,
    Fail,
}

#[derive(Serialize, Deserialize)]
struct TestInfo {
    parse: Option<TestType>,
}

fn test_parse([smol_file, json_file]: [&str; 2]) {
    use TestType::*;
    let test_info: TestInfo = serde_json::from_str(json_file).unwrap();

    match test_info.parse {
        Some(Pass) => {
        },
        Some(Fail) => {
        },
        None => panic!(
            "Does have a \"parse\" field in this json file :\n{}",
            json_file
        ),
    }
}
