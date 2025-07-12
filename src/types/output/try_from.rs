use super::Output;
use crate::{Input, MuxError};
use std::{
    env::current_dir,
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

#[cfg(unix)]
static MAIN_SEPARATOR_BYTES: &[u8] = b"/";

#[cfg(windows)]
static MAIN_SEPARATOR_BYTES: &[u8] = b"\\";

impl TryFrom<&Input> for Output {
    type Error = MuxError;

    fn try_from(input: &Input) -> Result<Self, Self::Error> {
        let dir = Self::make_dir(input.get_dir());
        Self::try_from_path(dir)
    }
}

impl Output {
    pub fn try_from_path(path: impl AsRef<Path>) -> Result<Self, MuxError> {
        let path = path.as_ref();

        let dir = Self::try_extract_dir(path)?;

        let name = match path.file_name() {
            Some(name) if Some(name) != dir.file_name() => name,
            _ => return Ok(Self::empty_with_dir(dir)),
        };

        let (name_begin, name_tail) = Self::split_stem(name);
        let ext = Self::extract_extension(name);

        Ok(Self {
            dir,
            temp_dir: PathBuf::new(),
            created_dirs: Vec::new(),
            name_begin,
            name_tail,
            ext,
        })
    }

    #[inline(always)]
    fn empty_with_dir(dir: PathBuf) -> Self {
        Self {
            dir,
            ..Default::default()
        }
    }

    #[inline(always)]
    fn try_extract_dir(path: &Path) -> Result<PathBuf, MuxError> {
        let res = |dir: PathBuf| -> Result<PathBuf, MuxError> {
            let dir = try_absolutize(dir)?;
            let dir = dir.components().collect();
            Ok(Self::ensure_long_path_prefix(dir))
        };

        let fallback = || -> Result<PathBuf, MuxError> {
            let dir = Self::make_dir(current_dir()?);
            res(dir)
        };

        if path.as_os_str().is_empty() {
            return fallback();
        }

        if path.is_dir() {
            return res(path.to_path_buf());
        }

        if path
            .as_os_str()
            .as_encoded_bytes()
            .ends_with(MAIN_SEPARATOR_BYTES)
        {
            return res(path.to_path_buf());
        }

        if let Some(path) = path.parent() {
            return res(path.to_path_buf());
        }

        fallback()
    }

    #[inline(always)]
    fn extract_extension(file_name: &OsStr) -> OsString {
        Path::new(file_name)
            .extension()
            .map(OsString::from)
            .unwrap_or_else(|| Self::default_ext())
    }

    #[inline(always)]
    fn split_stem(file_name: &OsStr) -> (OsString, OsString) {
        let stem = Path::new(file_name).file_stem().unwrap_or(OsStr::new(""));
        let stem_str = stem.to_string_lossy();

        match stem_str.find(',') {
            Some(pos) => {
                let (begin, tail) = stem_str.split_at(pos);
                let tail = &tail[1..];
                (OsString::from(begin), OsString::from(tail))
            }
            None if stem_str.is_empty() => (OsString::new(), OsString::new()),
            None => (OsString::from(stem), OsString::new()),
        }
    }
}

#[inline(always)]
fn try_absolutize(path: PathBuf) -> Result<PathBuf, MuxError> {
    #[cfg(unix)]
    {
        if path.starts_with("~") {
            return Ok(path);
        }
    }

    match path.is_absolute() {
        true => Ok(path),
        false => {
            let mut new = current_dir()?;
            new.push(path);
            Ok(new)
        }
    }
}
