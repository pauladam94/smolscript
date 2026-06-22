use crate::Index;
use cst::index::CSTIndex;
use std::{collections::VecDeque, fmt::Display};

impl Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.var)?;
        for var in self.index.iter() {
            write!(f, "[{var}]")?;
        }
        Ok(())
    }
}

impl<'src> From<CSTIndex<'src>> for Index {
    fn from(cst_index: CSTIndex<'src>) -> Self {
        let CSTIndex { var, index } = cst_index;
        Index {
            var: var.to_string(),
            index: index.into_iter().map(|span| span.to_string()).collect(),
        }
    }
}

impl Index {
    pub fn var(var: String) -> Self {
        Self {
            var,
            index: VecDeque::from([]),
        }
    }
}
