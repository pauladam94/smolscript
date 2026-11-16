use alloc::string::String;
use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet};

use crate::utils::span::Span;

pub fn annotate(input_str: &str, span: Span, title: &str) -> String {
    // Renderer::styled()
    let span = core::range::legacy::Range {
        start: span.start,
        end: span.end,
    };
    Renderer::plain().render(&[Level::ERROR.primary_title("").element(
        Snippet::source(input_str)
            .line_start(1)
            .annotation(AnnotationKind::Primary.span(span).label(title))
            .fold(false),
    )])
}
