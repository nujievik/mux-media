mod default;
mod to_json_args;
mod try_finalize_init;
mod try_from;

use std::{
    ffi::{OsStr, OsString},
    fs,
    path::{Path, PathBuf},
};

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
    pub fn build_out(&self, name_middle: impl AsRef<OsStr>) -> PathBuf {
        let mut name = OsString::new();
        name.push(&self.name_begin);
        name.push(name_middle);
        name.push(&self.name_tail);
        name.push(".");
        name.push(&self.ext);

        self.dir.as_path().join(name)
    }

    pub fn need_num(&self) -> bool {
        !self.name_begin.is_empty() || !self.name_tail.is_empty()
    }

    pub fn get_temp_dir(&self) -> &Path {
        &self.temp_dir
    }

    pub fn remove_created_dirs(&self) {
        let _ = fs::remove_dir_all(&self.temp_dir); //temp_dir may not empty
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
