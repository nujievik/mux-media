use super::ConfigOutput;
use crate::{Result, ToArgs};
use core::fmt::NumBuffer;
use std::io::Write;

impl ToArgs for ConfigOutput {
    fn write_with_num_buffer<W>(&self, w: &mut W, _: &mut NumBuffer<usize>) -> Result<()>
    where
        W: Write + ?Sized,
    {
        let s = self.dir.to_str().ok_or_else(|| err!("invalid utf-8"))?;
        to_args!(w, Output)?;
        to_args!(w, s.as_bytes(), @v)?;
        Ok(())
    }
}
