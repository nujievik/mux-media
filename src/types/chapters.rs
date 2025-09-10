use crate::{
    IsDefault, MediaInfo, MuxConfigArg, MuxError, ParseableArg, ToJsonArgs, ToMkvmergeArgs,
    to_json_args, types::helpers,
};
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};

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

impl ToMkvmergeArgs for Chapters {
    fn try_append_mkvmerge_args(
        &self,
        args: &mut Vec<OsString>,
        _: &mut MediaInfo,
        _: &Path,
    ) -> Result<(), MuxError> {
        if self.is_default() {
            return Ok(());
        }

        if self.no_flag {
            args.push(MuxConfigArg::NoChapters.dashed().into());
            return Ok(());
        }

        if let Some(f) = &self.file {
            args.push(MuxConfigArg::Chapters.dashed().into());
            args.push(f.into());
        }

        Ok(())
    }
}

impl ToJsonArgs for Chapters {
    fn append_json_args(&self, args: &mut Vec<String>) {
        if self.no_flag {
            args.push(to_json_args!(NoChapters));
            return;
        }

        if let Some(s) = self.file.as_ref().and_then(|f| f.to_str()) {
            args.push(to_json_args!(Chapters));
            args.push(s.to_owned());
        }
    }
}
