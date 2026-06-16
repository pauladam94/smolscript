use ast::Block;
use std::fmt::Display;

pub enum Continuation {
    LetCont { var: String, block: Block },
    BlockCont(Block),
    DropCont(String),
}

impl Display for Continuation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Continuation::LetCont { var, block } => write!(f, "let {var} = _ in {block}"),
            Continuation::BlockCont(block) => write!(f, "{{ _; {block}}}"),
            Continuation::DropCont(var) => write!(f, "drop({var})"),
        }
    }
}
