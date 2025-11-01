use crate::{IsDefault, MuxError, ToJsonArgs, dashed, types::helpers};
use std::path::{Path, PathBuf};

/// Settings for media chapters.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct Chapters {
    pub no_flag: bool,
    pub file: Option<PathBuf>,
}

impl Chapters {
    /// Tries construct [`Chapters`] from the given file.
    ///
    /// # Warning
    ///
    /// In current its not validates chapters inside the file.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - [`canonicalize`](std::fs::canonicalize) fails.
    ///
    /// - The path does not point to a file.
    ///
    /// - [`open`](std::fs::File::open) the file fails.
    pub fn try_from_file(file: impl AsRef<Path>) -> Result<Chapters, MuxError> {
        let file = helpers::try_canonicalize_and_open(file)?;
        Ok(Self {
            no_flag: false,
            file: Some(file),
        })
    }
}

impl ToJsonArgs for Chapters {
    fn append_json_args(&self, args: &mut Vec<String>) {
        if self.no_flag {
            args.push(dashed!(NoChapters).into());
            return;
        }

        if let Some(s) = self.file.as_ref().and_then(|f| f.to_str()) {
            args.push(dashed!(Chapters).into());
            args.push(s.to_owned());
        }
    }
}
