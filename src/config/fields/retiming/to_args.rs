use super::{ConfigRetiming, ConfigRetimingParts};
use crate::{Result, ToArgs};
use core::fmt::NumBuffer;
use std::io::Write;

impl ToArgs for ConfigRetiming {
    fn write_with_num_buffer<W>(&self, w: &mut W, buf: &mut NumBuffer<usize>) -> Result<()>
    where
        W: Write + ?Sized,
    {
        self.parts.write_with_num_buffer(w, buf)?;

        if self.no_linked {
            to_args!(w, NoLinked)?;
        }

        Ok(())
    }
}

impl ToArgs for ConfigRetimingParts {
    fn write_with_num_buffer<W>(&self, w: &mut W, _: &mut NumBuffer<usize>) -> Result<()>
    where
        W: Write + ?Sized,
    {
        let s = match self.pattern.as_ref() {
            Some(pat) if !pat.raw.is_empty() => &pat.raw,
            _ => return Ok(()),
        };

        to_args!(w, Parts)?;

        if self.inverse {
            w.write(b"!")?;
        }

        to_args!(w, s.as_bytes(), @v)?;
        Ok(())
    }
}
