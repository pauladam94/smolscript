use crate::cst::lexer::Token;
use alloc::string::String;
use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet};
use logos::Lexer;

pub fn annotate(input_str: &str, input: Lexer<'_, Token>, title: &str) -> String {
    Renderer::styled().render(&[Level::ERROR.primary_title("Error Parsing.").element(
        Snippet::source(input_str)
            .line_start(1)
            .annotation(AnnotationKind::Primary.span(input.span()).label(title))
            .fold(false),
    )])
}
