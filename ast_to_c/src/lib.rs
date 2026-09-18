use ast::AST;

pub fn ast_to_c(ast: AST) -> CProgram {
    Converter::default().convert_ast(ast)
}

pub mod formatting;
pub mod to_c;

#[derive(Debug, Clone, Default)]
struct Converter {
    program: CProgram,
}

#[derive(Default, Debug, Clone)]
pub struct CProgram {
    proc: Vec<String>,
}
