use crate::ast::{proc::Procedure, widget::Widget};

pub struct Ast {
    pub top_level: Vec<Widget>,
    pub proc: Vec<Procedure>,
}
