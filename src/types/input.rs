mod from_arg_matches;
mod iters;
mod try_finalize_init;

use crate::Range;
use globset::GlobSet;
use std::path::{Path, PathBuf};

pub struct Input {
    dir: PathBuf,
    range: Option<Range<u64>>,
    skip: Option<GlobSet>,
    up: u8,
    check: u16,
    down: u8,
    need_num: bool,
    out_need_num: bool,
    is_upmost_higher: bool,
    upmost: PathBuf,
    dirs: Vec<PathBuf>,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            dir: PathBuf::from("."),
            range: None,
            skip: None,
            up: Self::default_up(),
            check: Self::default_check(),
            down: Self::default_down(),
            need_num: false,
            out_need_num: false,
            is_upmost_higher: false,
            upmost: PathBuf::from("."),
            dirs: Vec::new(),
        }
    }
}

impl Input {
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

    fn default_up() -> u8 {
        8
    }

    fn default_check() -> u16 {
        128
    }

    fn default_down() -> u8 {
        16
    }
}
