mod from_arg_matches;
mod iters;
mod to_json_args;
mod try_finalize_init;

use crate::{GlobSetPattern, Range};
use std::path::{Path, PathBuf};

pub struct Input {
    dir: PathBuf,
    range: Option<Range<u64>>,
    skip: Option<GlobSetPattern>,
    up: u8,
    check: u16,
    down: u8,
    need_num: bool,
    out_need_num: bool,
    dir_not_upmost: bool,
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
            need_num: false,
            out_need_num: false,
            dir_not_upmost: false,
            upmost: PathBuf::from("."),
            dirs: Vec::new(),
        }
    }
}

impl Input {
    const DEFAULT_UP: u8 = 8;
    const DEFAULT_CHECK: u16 = 128;
    const DEFAULT_DOWN: u8 = 16;

    pub fn normalize_dir(dir: impl Into<PathBuf>) -> Result<PathBuf, std::io::Error> {
        let dir = std::fs::canonicalize(dir.into())?;
        std::fs::read_dir(&dir)?;
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

    fn try_default_dir() -> Result<PathBuf, std::io::Error> {
        Self::normalize_dir(".")
    }
}
