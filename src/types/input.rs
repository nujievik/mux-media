mod finalize;
pub(crate) mod iters;
mod to_args;

#[allow(unused_imports)]
use crate::TryFinalizeInit;
use crate::{ArcPathBuf, FileType, GlobSetPattern, RangeUsize, Result};
use enum_map::EnumMap;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Input configuration.
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
    pub out_need_num: bool,
    pub dirs: EnumMap<FileType, Vec<ArcPathBuf>>,
}

impl Input {
    pub(crate) const DEPTH_DEFAULT: u8 = 16;

    pub(crate) fn try_default_dir() -> Result<PathBuf> {
        Self::try_canonicalize_and_read(".")
    }

    /// Updates output need number flag.
    ///
    /// When enabled, [`Self::iter_media_grouped_by_stem`] will returns media number as stem.
    pub(crate) fn upd_out_need_num(&mut self, need: bool) {
        self.out_need_num = need;
        if need {
            self.need_num = true;
        }
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
