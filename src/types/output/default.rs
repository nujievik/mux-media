use super::Output;
use crate::types::helpers;
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};

impl Default for Output {
    fn default() -> Self {
        Self {
            dir: PathBuf::new(),
            temp_dir: PathBuf::new(),
            created_dirs: Vec::new(),
            name_begin: OsString::new(),
            name_tail: OsString::new(),
            ext: Self::default_ext(),
        }
    }
}

impl Output {
    pub(super) const DEFAULT_EXT: &'static str = "mkv";

    #[inline]
    pub(super) fn make_any_dir(dir: impl AsRef<Path>, subdir: &str) -> PathBuf {
        let dir = dir.as_ref().join(subdir);
        helpers::ensure_long_path_prefix(dir)
    }

    #[inline]
    pub(super) fn make_dir(input_dir: impl AsRef<Path>) -> PathBuf {
        let dir = Self::make_any_dir(input_dir, "muxed");
        helpers::ensure_ends_sep(dir)
    }

    #[inline]
    pub(super) fn default_ext() -> OsString {
        Self::DEFAULT_EXT.into()
    }
}
