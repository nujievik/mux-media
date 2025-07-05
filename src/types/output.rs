mod default;
mod to_json_args;
mod try_finalize_init;
mod try_from;

use std::{
    ffi::{OsStr, OsString},
    fs,
    path::{Path, PathBuf},
};

#[derive(Clone)]
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

        let mut path = self.dir.clone();
        path.push(name);
        path.set_extension(&self.ext);

        path
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
    let norm_order = match dirs.len() > 1 {
        true => dirs[1].components().count() > dirs[0].components().count(),
        false => false,
    };

    match norm_order {
        true => dirs.into_iter().rev().for_each(|dir| {
            let _ = fs::remove_dir(dir);
        }),
        false => dirs.into_iter().for_each(|dir| {
            let _ = fs::remove_dir(dir);
        }),
    }
}
