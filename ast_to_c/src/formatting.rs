use crate::CProgram;
use std::fmt::{Display, Formatter, Result};

impl Display for CProgram {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "
        #include <stdio.h>
        #include <stdlib.h>

        "
        )?;
        Ok(())
    }
}
