mod finalize;
pub(crate) mod iters;
mod to_args;

#[allow(unused_imports)]
use crate::TryFinalizeInit;
use crate::{ArcPathBuf, GlobSetPattern, RangeUsize, Result};
use enum_map::{Enum, EnumMap};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// An input configuration.
///
/// # Warning
///
/// This struct is not fully initialized after construction. You **must** call
/// [`Self::try_finalize_init`] before using some methods (e.g. [`Self::collect_fonts`]).
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Input {
    pub dir: PathBuf,
    pub range: Option<RangeUsize>,
    pub skip: Option<GlobSetPattern>,
    pub depth: u8,
    pub solo: bool,
    pub need_num: bool,
    pub dirs: EnumMap<InputFileType, Vec<ArcPathBuf>>,
}

/// A type of input file.
#[derive(Copy, Clone, Debug, Enum)]
#[non_exhaustive]
pub enum InputFileType {
    Font,
    Media,
}

impl Input {
    pub(crate) const DEPTH_DEFAULT: u8 = 16;

    pub(crate) fn try_default_dir() -> Result<PathBuf> {
        Self::try_canonicalize_and_read(".")
    }

    /// Tries canonicalize path to the directory and read its.
    ///
    /// # Errors
    ///
    /// Returns an error if the directory doesn't exist or its unreadable.
    pub(crate) fn try_canonicalize_and_read(dir: impl AsRef<Path>) -> Result<PathBuf> {
        let dir = fs::canonicalize(dir)?;
        fs::read_dir(&dir)?;
        Ok(dir)
    }
}
