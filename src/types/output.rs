mod default;
mod to_json_args;
mod try_finalize_init;
mod try_from;

use std::{
    ffi::{OsStr, OsString},
    fs,
    path::{Path, PathBuf},
};

/// Contains output paths components, related functions and methods.
#[derive(Clone, Debug)]
pub struct Output {
    dir: PathBuf,
    temp_dir: PathBuf,
    created_dirs: Vec<PathBuf>,
    name_begin: OsString,
    name_tail: OsString,
    ext: OsString,
}

impl Output {
    /// Builds the output path for the current media.
    ///
    /// Takes a middle part of the file name and constructs the full path
    /// by prepending `self.dir` and `self.name_begin`, and appending
    /// `self.name_tail` and `self.ext`.
    pub fn build_out(&self, name_middle: impl AsRef<OsStr>) -> PathBuf {
        let mut name = OsString::from(&self.dir);
        name.push(&self.name_begin);
        name.push(name_middle);
        name.push(&self.name_tail);
        name.push(".");
        name.push(&self.ext);
        name.into()
    }

    /// Returns `true` if a media number is required to build the output name.
    ///
    /// This is the case when either `self.name_begin` or `self.name_tail` is non-empty.
    pub fn need_num(&self) -> bool {
        !self.name_begin.is_empty() || !self.name_tail.is_empty()
    }

    /// Returns a reference to the temporary directory.
    pub fn get_temp_dir(&self) -> &Path {
        &self.temp_dir
    }

    /// Removes the temporary directory and any created empty directories.
    pub fn remove_created_dirs(&self) {
        let _ = fs::remove_dir_all(&self.temp_dir);
        remove_empty_chain_dirs(&self.created_dirs);
    }
}

fn remove_empty_chain_dirs(dirs: &[PathBuf]) {
    let ascending_order =
        (dirs.len() > 1) && (dirs[1].as_os_str().len() > dirs[0].as_os_str().len());

    match ascending_order {
        true => dirs.into_iter().rev().for_each(|dir| {
            let _ = fs::remove_dir(dir);
        }),
        false => dirs.into_iter().for_each(|dir| {
            let _ = fs::remove_dir(dir);
        }),
    }
}
