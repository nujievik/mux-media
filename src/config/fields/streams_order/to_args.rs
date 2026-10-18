use super::*;
use crate::{Result, ToArgs, helpers};
use core::fmt::NumBuffer;
use std::io::Write;

impl ToArgs for ConfigStreamsOrder {
    fn write_with_num_buffer<W>(&self, w: &mut W, buf: &mut NumBuffer<usize>) -> Result<()>
    where
        W: Write + ?Sized,
    {
        if self.is_default() {
            return Ok(());
        }

        to_args!(w, Order)?;

        let mut is_first = true;

        if let Some(v) = self.single_val {
            w.write((v as usize).format_into(buf).as_bytes())?;
        }

        if let Some(xs) = self.idxs.as_ref() {
            for (k, v) in xs {
                helpers::write_part_of_arg(w, &mut is_first, |w| {
                    w.write(k.format_into(buf).as_bytes())?;
                    w.write(b":")?;
                    w.write((*v as usize).format_into(buf).as_bytes())
                })?;
            }
        }

        if let Some(xs) = self.ranges.as_ref() {
            for (k, v) in xs {
                helpers::write_part_of_arg(w, &mut is_first, |w| {
                    helpers::write_range(w, k, buf)?;
                    w.write(b":")?;
                    w.write((*v as usize).format_into(buf).as_bytes())
                })?;
            }
        }

        if let Some(xs) = self.langs.as_ref() {
            for (k, v) in xs {
                helpers::write_part_of_arg(w, &mut is_first, |w| {
                    w.write(k.as_str().as_bytes())?;
                    w.write(b":")?;
                    w.write((*v as usize).format_into(buf).as_bytes())
                })?;
            }
        }

        w.write(b"\n")?;
        Ok(())
    }
}
