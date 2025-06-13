use super::Output;
use crate::{Input, MuxError};
use std::env::current_dir;
use std::ffi::{OsStr, OsString};
use std::path::{MAIN_SEPARATOR, Path, PathBuf};

impl TryFrom<&Input> for Output {
    type Error = MuxError;

    fn try_from(input: &Input) -> Result<Self, Self::Error> {
        let dir = Self::make_dir(input.get_dir());
        Ok(Self::try_from_path(dir)?)
    }
}

impl Output {
    pub fn try_from_path(path: impl AsRef<Path>) -> Result<Self, MuxError> {
        let path = path.as_ref();
        let dir = Self::extract_dir(path)?;

        let name = match path.file_name() {
            Some(name) if Some(name) != dir.file_name() => name,
            _ => return Ok(Self::empty_with_dir(dir)),
        };

        let (name_begin, name_tail) = Self::split_stem(name);
        let ext = Self::extract_extension(name);

        Ok(Self {
            temp_dir: Self::make_temp_dir(&dir),
            dir,
            name_begin,
            name_tail,
            ext,
            ..Default::default()
        })
    }

    #[inline]
    fn empty_with_dir(dir: PathBuf) -> Self {
        Self {
            temp_dir: Self::make_temp_dir(&dir),
            dir,
            ext: Self::default_ext(),
            ..Default::default()
        }
    }

    #[inline]
    fn extract_dir(path: &Path) -> Result<PathBuf, MuxError> {
        let dir = if path.as_os_str().is_empty() {
            Self::make_dir(current_dir()?)
        } else if path.is_dir() {
            path.to_path_buf()
        } else if path.to_string_lossy().ends_with(MAIN_SEPARATOR) {
            path.to_path_buf()
        } else {
            match path.parent().map(|p| p.to_path_buf()) {
                Some(p) => p,
                None => Self::make_dir(current_dir()?),
            }
        };

        let dir = try_absolutize(dir)?;
        Ok(normalize(dir))
    }

    #[inline]
    fn extract_extension(file_name: &OsStr) -> OsString {
        Path::new(file_name)
            .extension()
            .map(OsString::from)
            .unwrap_or_else(|| Self::default_ext())
    }

    #[inline]
    fn split_stem(file_name: &OsStr) -> (OsString, OsString) {
        let stem = Path::new(file_name).file_stem().unwrap_or(OsStr::new(""));
        let stem_str = stem.to_string_lossy();

        match stem_str.find(',') {
            Some(pos) => {
                let (begin, tail) = stem_str.split_at(pos);
                let tail = &tail[1..];
                (OsString::from(begin), OsString::from(tail))
            }
            None => match stem_str.is_empty() {
                true => (OsString::new(), OsString::new()),
                false => (OsString::from(stem), OsString::new()),
            },
        }
    }
}

#[inline]
fn try_absolutize(path: PathBuf) -> Result<PathBuf, MuxError> {
    let path = match home::home_dir() {
        Some(home) => match path.strip_prefix("~") {
            Ok(stripped) => home.join(stripped),
            Err(_) => path,
        },
        None => path,
    };

    match path.is_absolute() {
        true => Ok(path),
        false => Ok(current_dir()?.join(path)),
    }
}

#[inline]
fn normalize(path: PathBuf) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        normalized.push(component);
    }
    normalized
}
