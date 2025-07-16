mod from_arg_matches;
mod iters;
mod to_json_args;

use crate::{MuxError, ArcPathBuf, Msg, GlobSetPattern, Range, TryFinalizeInit};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// Represents a collection of input paths, filters, and traversal parameters.
///
/// # Warning
///
/// This struct is not fully initialized after construction.
/// You **must** call [`Init.finalize_init`] before using some methods
/// (e.g. [`Input::collect_fonts`], [`Input::iter_media_grouped_by_stem`]).
/// Otherwise, behavior be incorrect.
#[derive(Clone)]
pub struct Input {
    dir: PathBuf,
    range: Option<Range<u64>>,
    skip: Option<GlobSetPattern>,
    depth: u8,
    need_num: bool,
    out_need_num: bool,
    dirs: Vec<ArcPathBuf>,
}

impl Input {
    const DEFAULT_DEPTH: u8 = 16;

    /// Returns a normalized path via [`fs::canonicalize`] and checks it's readable via [`fs::read_dir`].
    ///
    /// Fails if the directory doesn't exist or is unreadable.
    pub fn try_normalize_dir(dir: impl AsRef<Path>) -> Result<PathBuf, io::Error> {
        let dir = fs::canonicalize(dir)?;
        fs::read_dir(&dir)?;
        Ok(dir)
    }

    /// Returns a reference to the start directory.
    pub fn get_dir(&self) -> &Path {
        &self.dir
    }

    /// Sets output numbering flag, and enables input numbering if `true`.
    ///
    /// When enabled, initializes and tracks [`crate::MediaNumber`] —
    /// used for range filtering and to construct output paths via [`crate::Output::build_out`].
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
    /// Returns an error if not media files in the start directory;
    /// otherwise returns Ok.
    ///
    /// Collects subdirectories up to the initialized depth for use in `self` methods.
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
        self.dirs = iters::DirIter::new(&self.dir, self.depth as usize, skip).collect();

        Ok(())
    }
}
