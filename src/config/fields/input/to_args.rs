use super::{ConfigInput, InputType};
use crate::{Result, ToArgs, helpers};
use core::fmt::NumBuffer;
use std::{io::Write, path::Path};

impl ToArgs for ConfigInput {
    fn write_with_num_buffer<W>(&self, w: &mut W, buf: &mut NumBuffer<usize>) -> Result<()>
    where
        W: Write + ?Sized,
    {
        match &self.ty {
            InputType::Dir(dir) => write_input_path(w, dir)?,
            InputType::Files(files) => {
                for f in files {
                    write_input_path(w, f)?;
                }
            }
        }

        if let Some(range) = &self.range {
            to_args!(w, Range)?;
            helpers::write_range(w, range, buf)?;
            w.write(b"\n")?;
        }

        if let Some(pat) = &self.skip {
            if !pat.raw.is_empty() {
                to_args!(w, Skip)?;
                to_args!(w, pat.raw.as_bytes(), @v)?;
            }
        }

        if self.depth != Self::DEPTH_DEFAULT {
            to_args!(w, Depth)?;
            let arg = (self.depth as usize).format_into(buf);
            to_args!(w, arg.as_bytes(), @v)?;
        }

        if self.solo {
            to_args!(w, Solo)?;
        }

        Ok(())
    }
}

fn write_input_path<W>(w: &mut W, path: &Path) -> Result<()>
where
    W: Write + ?Sized,
{
    let s = path.to_str().ok_or_else(|| err!("invalid utf-8"))?;
    to_args!(w, Input)?;
    to_args!(w, s.as_bytes(), @v)?;
    Ok(())
}
