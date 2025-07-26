use crate::{
    IsDefault, MuxError, ToJsonArgs, ToMkvmergeArgs, from_arg_matches, json_arg, mkvmerge_arg,
    mkvmerge_no_arg, to_mkvmerge_args, types::helpers,
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
    /// Attempts to construct `Self` from the given path.
    ///
    /// # Warning
    ///
    /// In current its not validates chapters in file.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// 1. [`canonicalize`](std::fs::canonicalize) fails.
    /// 2. The path does not point to a file.
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

mkvmerge_arg!(Chapters, "--chapters");
mkvmerge_no_arg!(Chapters, "--no-chapters");

impl ToMkvmergeArgs for Chapters {
    to_mkvmerge_args!(@fn);

    fn to_os_mkvmerge_args(&self, _: &mut crate::MediaInfo, _: &Path) -> Vec<OsString> {
        use crate::{IsDefault, MkvmergeArg, MkvmergeNoArg};

        if self.is_default() {
            return Vec::new();
        }

        if self.no_flag {
            return vec![Self::MKVMERGE_NO_ARG.into()];
        }

        if let Some(f) = &self.file {
            return vec![Self::MKVMERGE_ARG.into(), f.into()];
        }

        log::warn!("Unexpected None file. Return empty");
        Vec::new()
    }
}

impl ToJsonArgs for Chapters {
    fn to_json_args(&self) -> Vec<String> {
        if self.no_flag {
            return vec![json_arg!(NoChapters)];
        }

        if let Some(s) = self.file.as_ref().and_then(|f| f.to_str()) {
            return vec![json_arg!(Chapters), s.to_owned()];
        }

        Vec::new()
    }
}
