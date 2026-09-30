use super::ConfigStreams;
use crate::{Result, ToArgs, helpers};
use core::fmt::NumBuffer;
use std::io::Write;

impl ToArgs for ConfigStreams {
    fn write_with_num_buffer<W>(&self, w: &mut W, buf: &mut NumBuffer<usize>) -> Result<()>
    where
        W: Write + ?Sized,
    {
        if self.no_flag {
            to_args!(w, NoStreams)?;
            return Ok(());
        }

        if self.idxs.is_none() && self.ranges.is_none() && self.langs.is_none() {
            return Ok(());
        }

        to_args!(w, Streams)?;

        if self.inverse {
            w.write(b"!")?;
        }

        let mut is_first = true;

        if let Some(xs) = &self.idxs {
            for x in xs {
                if !is_first {
                    w.write(b",")?;
                }

                w.write(x.format_into(buf).as_bytes())?;
                is_first = false;
            }
        }

        if let Some(xs) = &self.ranges {
            for x in xs {
                if !is_first {
                    w.write(b",")?;
                }
                helpers::write_range(w, x, buf)?;
                is_first = false;
            }
        }

        if let Some(xs) = &self.langs {
            for x in xs {
                if !is_first {
                    w.write(b",")?;
                }

                w.write(x.as_str().as_bytes())?;
                is_first = false;
            }
        }

        w.write(b"\n")?;

        Ok(())
    }
}
