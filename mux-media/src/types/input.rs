mod from_arg_matches;
mod iters;
mod to_args;

use crate::{ArcPathBuf, FileType, GlobSetPattern, Msg, MuxError, Range, TryFinalizeInit};
use enum_map::EnumMap;
use rayon::prelude::*;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// Contains input settings, related functions and methods.
///
/// # Warning
///
/// This struct is not fully initialized after construction. You **must** call
/// [`Self::try_finalize_init`] before using some methods (e.g. [`Self::collect_fonts`]).
#[derive(Clone)]
pub struct Input {
    dir: PathBuf,
    range: Option<Range<u64>>,
    skip: Option<GlobSetPattern>,
    depth: u8,
    need_num: bool,
    out_need_num: bool,
    dirs: EnumMap<FileType, Vec<ArcPathBuf>>,
}

impl Input {
    const DEFAULT_DEPTH: u8 = 16;

    /// Returns a normalized directory.
    ///
    /// Fails if the directory doesn't exist or its unreadable.
    pub fn try_normalize_dir(dir: impl AsRef<Path>) -> Result<PathBuf, io::Error> {
        let dir = fs::canonicalize(dir)?;
        fs::read_dir(&dir)?;
        Ok(dir)
    }

    /// Returns the start directory.
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// Updates output need number flag.
    ///
    /// When enabled, [`Self::iter_media_grouped_by_stem`] will returns media number as stem.
    pub fn upd_out_need_num(&mut self, need: bool) {
        self.out_need_num = need;
        if need {
            self.need_num = true;
        }
    }

    fn try_default_dir() -> Result<PathBuf, io::Error> {
        Self::try_normalize_dir(".")
    }
}

impl TryFinalizeInit for Input {
    /// Collects subdirectories up to the initialized depth.
    ///
    /// Fails if not any media in the start directory.
    fn try_finalize_init(&mut self) -> Result<(), MuxError> {
        if let None = self.iter_media_in_dir(&self.dir).next() {
            return Err(
                [(Msg::NoInputDirMedia, format!(": {}", self.dir.display()))]
                    .as_slice()
                    .into(),
            );
        }

        let skip = match &self.skip {
            Some(skip) => Some(&skip.glob_set),
            None => None,
        };

        let mut map = FileType::map::<Vec<ArcPathBuf>>();
        let dirs: Vec<_> = iters::DirIter::new(&self.dir, self.depth as usize, skip).collect();

        let mut insert = |ft: FileType| {
            map[ft] = dirs
                .par_iter()
                .filter(|dir| self.iter_files_in_dir(ft, dir).next().is_some())
                .map(|dir| dir.clone())
                .collect();
        };

        insert(FileType::Font);
        insert(FileType::Media);

        self.dirs = map;

        Ok(())
    }
}
