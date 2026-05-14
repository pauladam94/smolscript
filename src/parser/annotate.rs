use crate::parser::lexer::Token;
use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet};
use logos::Lexer;

pub fn annotate(input_str: &str, input: Lexer<'_, Token>, title: &str) -> String {
    let report = &[Level::ERROR.primary_title("Error Parsing.").element(
        Snippet::source(input_str)
            .line_start(1)
            .annotation(AnnotationKind::Primary.span(input.span()).label(title))
            .fold(false),
    )];

    let s = format!("{}", Renderer::styled().render(report));
    s
}
