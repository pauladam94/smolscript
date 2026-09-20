use core::fmt::{Formatter, Write, Result};

pub struct Pad<'a, 'b> {
    pub f: &'a mut Formatter<'b>,
}

impl Write for Pad<'_, '_> {
    fn write_str(&mut self, s: &str) -> Result {
        for s in s.split_inclusive('\n') {
            self.f.write_str(s)?;
            if s.ends_with('\n') {
                self.f.write_str("    ")?;
            }
        }
        Ok(())
    }
}
