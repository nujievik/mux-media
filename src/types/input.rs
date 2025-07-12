mod from_arg_matches;
mod iters;
mod to_json_args;
mod try_finalize_init;

use crate::{GlobSetPattern, Range};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

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

    pub fn try_normalize_dir(dir: impl AsRef<Path>) -> Result<PathBuf, io::Error> {
        let dir = fs::canonicalize(dir)?;
        fs::read_dir(&dir)?;
        Ok(dir)
    }

    pub fn get_dir(&self) -> &Path {
        &self.dir
    }

    pub fn get_upmost(&self) -> &Path {
        &self.upmost
    }

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
