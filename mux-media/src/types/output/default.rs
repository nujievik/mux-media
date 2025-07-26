use super::Output;
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};

impl Default for Output {
    /// Returns the “default value” for [`Output`].
    ///
    /// All components except extension is empty. Extension is "mkv".
    /// ```
    /// # use mux_media::Output;
    /// # use std::path::Path;
    /// #
    /// let output = Output::default();
    /// assert_eq!(Path::new(""), output.dir());
    /// assert_eq!(Path::new(""), output.temp_dir());
    /// assert_eq!("", output.name_begin());
    /// assert_eq!("", output.name_tail());
    /// assert_eq!("mkv", output.ext());
    /// assert_eq!(Path::new(".mkv"), output.build_out(""));
    /// assert_eq!(Path::new("a.mkv"), output.build_out("a"));
    /// ```
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

    #[inline(always)]
    pub(super) fn default_ext() -> OsString {
        Self::DEFAULT_EXT.into()
    }

    #[inline(always)]
    pub(super) fn make_any_dir(dir: impl AsRef<Path>, subdir: &str) -> PathBuf {
        let dir = dir.as_ref().join(subdir);
        crate::ensure_trailing_sep(dir)
    }

    #[inline(always)]
    pub(super) fn make_dir(input_dir: impl AsRef<Path>) -> PathBuf {
        Self::make_any_dir(input_dir, "muxed")
    }
}
