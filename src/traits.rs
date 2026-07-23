pub(crate) mod lazy_fields;

use crate::Result;
use std::{
    ffi::OsString,
    fs,
    io::{BufWriter, Write},
    path::Path,
};

/// Provides a delayed initialization for expensive operations.
pub trait TryFinalizeInit {
    /// Finalizes initialization.
    fn try_finalize_init(&mut self) -> Result<()>;
}

/// Converts a value to txt config arguments.
pub trait ToTxtConfig {
    /// Appends arguments to the given `args` vector.
    fn append_args(&self, args: &mut Vec<OsString>);

    /// Returns vector of arguments.
    fn to_args(&self) -> Vec<OsString> {
        let mut args = Vec::new();
        self.append_args(&mut args);
        args
    }

    /// Writes args to the given file path.
    fn write<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let args = self.to_args();

        if args.is_empty() {
            return Ok(());
        }

        let file = fs::File::create(path)?;
        let mut writer = BufWriter::new(file);

        for arg in args {
            writer.write_all(arg.as_encoded_bytes())?;
            writer.write_all(b"\n")?;
        }

        writer.flush()?;
        Ok(())
    }
}

/// Associates a field with the marker type `F`.
pub trait Field<F> {
    type FieldType;

    /// Returns a reference to the field value.
    fn field(&self) -> &Self::FieldType;
}
