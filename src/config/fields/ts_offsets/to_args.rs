use super::*;
use crate::{Result, ToArgs, helpers};
use core::fmt::NumBuffer;
use std::io::{self, Write};

impl ToArgs for ConfigTsOffsets {
    fn write_with_num_buffer<W>(&self, w: &mut W, buf: &mut NumBuffer<usize>) -> Result<()>
    where
        W: Write + ?Sized,
    {
        if self.is_default() {
            return Ok(());
        }

        to_args!(w, TsOffsets)?;

        let mut is_first = true;

        if let Some(v) = self.single_val {
            write_signed_number(w, buf, v)?;
        }

        if let Some(xs) = self.idxs.as_ref() {
            for (k, v) in xs {
                helpers::write_part_of_arg(w, &mut is_first, |w| {
                    w.write(k.format_into(buf).as_bytes())?;
                    w.write(b":")?;
                    write_signed_number(w, buf, *v)
                })?;
            }
        }

        if let Some(xs) = self.ranges.as_ref() {
            for (k, v) in xs {
                helpers::write_part_of_arg(w, &mut is_first, |w| {
                    helpers::write_range(w, k, buf)?;
                    w.write(b":")?;
                    write_signed_number(w, buf, *v)
                })?;
            }
        }

        if let Some(xs) = self.langs.as_ref() {
            for (k, v) in xs {
                helpers::write_part_of_arg(w, &mut is_first, |w| {
                    w.write(k.as_str().as_bytes())?;
                    w.write(b":")?;
                    write_signed_number(w, buf, *v)
                })?;
            }
        }

        w.write(b"\n")?;
        Ok(())
    }
}

fn write_signed_number<W>(w: &mut W, buf: &mut NumBuffer<usize>, num: i32) -> io::Result<usize>
where
    W: Write + ?Sized,
{
    if num.is_negative() {
        w.write(b"-")?;
    }
    w.write((num.abs() as usize).format_into(buf).as_bytes())
}
