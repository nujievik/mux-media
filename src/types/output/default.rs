use super::Output;
use std::ffi::OsString;
use std::path::PathBuf;

impl Default for Output {
    fn default() -> Self {
        let dir = Self::make_dir(".");
        Self {
            temp_dir: Self::make_temp_dir(&dir),
            dir: dir,
            created_dirs: Vec::new(),
            name_begin: OsString::new(),
            name_tail: OsString::new(),
            ext: Self::default_ext(),
        }
    }
}

impl Output {
    #[inline]
    fn make_any_dir(dir: impl Into<PathBuf>, subdir: &str) -> PathBuf {
        let mut dir = dir.into();
        dir.push(subdir);
        dir
    }

    #[inline]
    pub(super) fn make_dir(input_dir: impl Into<PathBuf>) -> PathBuf {
        Self::make_any_dir(input_dir, "muxed/")
    }

    #[inline]
    pub(super) fn make_temp_dir(output_dir: impl Into<PathBuf>) -> PathBuf {
        Self::make_any_dir(output_dir, ".temp-mux-media/")
    }

    #[inline]
    pub(super) fn default_ext() -> OsString {
        OsString::from("mkv")
    }
}
