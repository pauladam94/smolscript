use core::fmt::{Display, Formatter, Result};
use crate::backend_c::CProgram;

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
