mod from_arg_matches;
mod iters;
mod to_json_args;
mod try_finalize_init;

use crate::{GlobSetPattern, Range};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// Represents a collection of input paths, filters, and traversal parameters.
///
/// # Warning
///
/// This struct is not fully initialized after construction.
/// You **must** call `self.try_finalize_init` before using any methods
/// that rely on the `dirs` or `upmost` fields (e.g. [`Input::collect_fonts`],
/// [`Input::iter_media_grouped_by_stem`]).
/// Otherwise, behavior be incorrect.
///
/// Typically, final initialization includes resolving the upmost directory
/// and collecting eligible subdirectories for scanning.
#[derive(Clone)]
pub struct Input {
    dir: PathBuf,
    range: Option<Range<u64>>,
    skip: Option<GlobSetPattern>,
    up: u8,
    check: u16,
    down: u8,
    dir_not_upmost: bool,
    need_num: bool,
    out_need_num: bool,
    upmost: PathBuf,
    dirs: Vec<PathBuf>,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            dir: PathBuf::from("."),
            range: None,
            skip: None,
            up: Self::DEFAULT_UP,
            check: Self::DEFAULT_CHECK,
            down: Self::DEFAULT_DOWN,
            dir_not_upmost: false,
            need_num: false,
            out_need_num: false,
            upmost: PathBuf::from("."),
            dirs: Vec::new(),
        }
    }
}

impl Input {
    const DEFAULT_UP: u8 = 8;
    const DEFAULT_CHECK: u16 = 128;
    const DEFAULT_DOWN: u8 = 16;

    /// Returns a normalized path via [`fs::canonicalize`] and checks it's readable via [`fs::read_dir`].
    ///
    /// Fails if the directory doesn't exist or is unreadable.
    pub fn try_normalize_dir(dir: impl AsRef<Path>) -> Result<PathBuf, io::Error> {
        let dir = fs::canonicalize(dir)?;
        fs::read_dir(&dir)?;
        Ok(dir)
    }

    /// Returns a reference to the original `--input` directory.
    pub fn get_dir(&self) -> &Path {
        &self.dir
    }

    /// Returns a reference to the resolved upmost directory.
    pub fn get_upmost(&self) -> &Path {
        &self.upmost
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
