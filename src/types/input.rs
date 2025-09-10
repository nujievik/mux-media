pub(crate) mod iters;
mod default;
mod finalize;
mod to_args;

#[allow(unused_imports)]
use crate::TryFinalizeInit;
use crate::{ArcPathBuf, FileType, GlobSetPattern, RangeU64, Result};
use enum_map::EnumMap;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Contains input settings, related functions and methods.
///
/// # Warning
///
/// This struct is not fully initialized after construction. You **must** call
/// [`Self::try_finalize_init`] before using some methods (e.g. [`Self::collect_fonts`]).
#[derive(Clone, Debug, PartialEq)]
pub struct Input {
    pub dir: PathBuf,
    pub range: Option<RangeU64>,
    pub skip: Option<GlobSetPattern>,
    pub depth: u8,
    pub solo: bool,
    pub need_num: bool,
    pub out_need_num: bool,
    pub dirs: EnumMap<FileType, Vec<ArcPathBuf>>,
}

impl Input {
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
