use alloc::string::String;
use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet};
use logos::Span;

pub fn annotate(input_str: &str, input: Span, title: &str) -> String {
    // Renderer::styled()
    Renderer::plain().render(&[Level::ERROR.primary_title("").element(
        Snippet::source(input_str)
            .line_start(1)
            .annotation(AnnotationKind::Primary.span(input).label(title))
            .fold(false),
    )])
}
