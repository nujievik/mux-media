use super::Output;
use crate::{Input, MuxError, types::helpers};
use std::{
    env::current_dir,
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

impl TryFrom<&Input> for Output {
    type Error = MuxError;

    /// Attempts to construct `Self` from a subdirectory "muxed" in the input directory.
    ///
    /// Returns an error if [`current_dir`] fails.
    ///
    /// Sets `self.dir` only, other components is default.
    ///
    /// # Warning
    ///
    /// This associated function does not check whether `self.dir` exists or is writable.
    /// To ensure that the directory structure is valid and writable, call `self.try_finalize_init()`
    /// after constructing the `Self`.
    fn try_from(input: &Input) -> Result<Self, Self::Error> {
        let dir = Self::make_dir(input.get_dir());
        Self::try_from_path(dir)
    }
}

impl Output {
    /// Attempts to construct `Self` from a path pattern:
    /// `[dir][MAIN_SEPARATOR][name_begin][,][name_tail][.ext]`.
    ///
    /// Any component is optional. The method tries to infer meaningful values from the input.
    /// Returns an error if [`current_dir`] fails.
    ///
    /// # Path Parsing Rules
    ///
    /// 1. Sets `self.dir` to `current_dir().join("muxed")` if:
    ///    - The pattern is empty.
    ///    - The pattern does not exist as a directory and does not contain `MAIN_SEPARATOR`.
    ///    - `path.parent()` is `None` or `Some("")`.
    ///
    /// 2. If the pattern is an existing directory or ends with a `MAIN_SEPARATOR`,
    ///    sets `self.dir` from the full `path`.
    ///
    /// 3. Otherwise, sets `self.dir` to `path.parent()`.
    ///
    /// 4. Converts `self.dir` to an absolute path. (On Unix, allows `~` start.)
    ///
    /// 5. Ensures `self.dir` ends with a `MAIN_SEPARATOR`.
    ///
    /// 6. On Windows, prefixes `self.dir` with `\\?\` for extended-length paths.
    ///
    /// 7. Parses the `path.file_name()` (if present):
    ///    - `self.name_begin`: from start to first `,` (exclusive).
    ///    - `self.name_tail`: from first `,` (exclusive) to last `.` (exclusive).
    ///    - `self.ext`: from last `.` (exclusive) to the end, if the dot is not the first character.
    ///
    /// 8. If no name is provided, sets `self.name_begin` and `self.name_tail` to empty.
    ///
    /// 9. If no extension is provided, sets `self.ext` to `"mkv"`.
    ///
    /// 10. The first `,` and the last `.` in the `path.file_name()` are used as delimiters and
    ///     are not stored. Any remaining commas or dots are retained in `self.name_tail`.
    ///
    /// # Warning
    ///
    /// This associated function does not check whether `self.dir` exists or is writable.
    /// To ensure that the directory structure is valid and writable, call `self.try_finalize_init()`
    /// after constructing the `Self`.
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
}

impl Output {
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
            let dir = helpers::ensure_long_path_prefix(dir);
            Ok(helpers::ensure_ends_sep(dir))
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
            .ends_with(helpers::SEP_B)
        {
            return res(path.to_path_buf());
        }

        if let Some(path) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
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
