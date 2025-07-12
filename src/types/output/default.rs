use super::Output;
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};

#[cfg(unix)]
const SEP_S: &'static str = "/";
#[cfg(unix)]
const SEP_B: &[u8] = b"/";
#[cfg(windows)]
const SEP_S: &'static str = "\\";
#[cfg(windows)]
const SEP_B: &[u8] = b"\\";

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
        Self::ensure_long_path_prefix(dir)
    }

    #[inline]
    pub(super) fn make_dir(input_dir: impl AsRef<Path>) -> PathBuf {
        let dir = Self::make_any_dir(input_dir, "muxed");
        Self::ensure_ends_sep(dir)
    }

    #[inline]
    pub(super) fn default_ext() -> OsString {
        Self::DEFAULT_EXT.into()
    }

    #[inline(always)]
    pub(super) fn ensure_long_path_prefix(path: PathBuf) -> PathBuf {
        #[cfg(unix)]
        {
            path
        }
        #[cfg(windows)]
        {
            match path.as_os_str().as_encoded_bytes().starts_with(b"\\\\?\\") {
                true => path,
                false => {
                    let mut prf_path = OsString::from("\\\\?\\");
                    prf_path.push(path.as_os_str());
                    prf_path.into()
                }
            }
        }
    }

    #[inline(always)]
    fn ensure_ends_sep(path: PathBuf) -> PathBuf {
        match path.as_os_str().as_encoded_bytes().ends_with(SEP_B) {
            true => path,
            false => {
                let mut path_sep = path.into_os_string();
                path_sep.push(SEP_S);
                path_sep.into()
            }
        }
    }
}
