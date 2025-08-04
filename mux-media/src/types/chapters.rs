use crate::{
    IsDefault, MediaInfo, MuxConfigArg, MuxError, ParseableArg, ToJsonArgs, ToMkvmergeArgs,
    from_arg_matches, to_json_args, types::helpers,
};
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};

/// Settings for media chapters.
#[derive(Clone, Debug, Default, PartialEq, IsDefault)]
pub struct Chapters {
    no_flag: bool,
    file: Option<PathBuf>,
}

impl Chapters {
    /// Attempts to construct [`Chapters`] from the given path.
    ///
    /// # Warning
    ///
    /// In current its not validates chapters in file.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// 1. [`canonicalize`](std::fs::canonicalize) fails.
    ///
    /// 2. The path does not point to a file.
    ///
    /// 3. [`open`](std::fs::File::open) the file fails.
    pub fn try_from_path(path: impl AsRef<Path>) -> Result<Self, MuxError> {
        let path = helpers::try_canonicalize_and_open(path)?;
        Ok(Self {
            no_flag: false,
            file: Some(path),
        })
    }

    fn no_flag(mut self, val: bool) -> Self {
        self.no_flag = val;
        self
    }
}

from_arg_matches!(@impl, Chapters, Chapters, NoChapters);

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
