pub(crate) mod lazy_fields;

use crate::Result;
use core::fmt::NumBuffer;
use std::{
    fs,
    io::{BufWriter, Write},
    path::Path,
};

/// Provides a delayed initialization for expensive operations.
pub trait TryFinalizeInit {
    /// Finalizes initialization.
    fn try_finalize_init(&mut self) -> Result<()>;
}

pub trait ToArgs {
    fn write_with_num_buffer<W>(&self, writer: &mut W, buf: &mut NumBuffer<usize>) -> Result<()>
    where
        W: Write + ?Sized;

    fn write<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Write + ?Sized,
    {
        let mut buf = NumBuffer::new();
        self.write_with_num_buffer(writer, &mut buf)
    }

    fn write_to_file<P>(&self, path: &P) -> Result<()>
    where
        P: AsRef<Path> + ?Sized,
    {
        let file = fs::File::create(path)?;
        let mut writer = BufWriter::new(file);

        self.write(&mut writer)?;
        writer.flush()?;

        Ok(())
    }

    fn to_args(&self) -> Result<Vec<String>> {
        let mut buf: Vec<u8> = Vec::new();
        self.write(&mut buf)?;

        let mut args: Vec<String> = Vec::new();

        for arg in buf.split(|&b| b == b'\n').filter(|s| !s.is_empty()) {
            let arg = str::from_utf8(arg)?;
            args.push(arg.into());
        }

        Ok(args)
    }
}

/// Associates a field with the marker type `F`.
pub trait Field<F> {
    type FieldType;

    /// Returns a reference to the field value.
    fn field(&self) -> &Self::FieldType;
}
