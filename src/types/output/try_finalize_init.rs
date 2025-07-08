use super::{Output, remove_empty_chain_dirs};
use crate::{MuxError, TryFinalizeInit};
use std::{
    fs,
    path::{Path, PathBuf},
};

impl TryFinalizeInit for Output {
    fn try_finalize_init(&mut self) -> Result<(), MuxError> {
        let temp_dir = Self::make_any_dir(&self.dir, ".temp-mux-media/");
        let created_dirs = try_create_chain_dirs(&temp_dir)?;

        let try_write = |dir| {
            try_write_in(dir).map_err(|e| {
                remove_empty_chain_dirs(&created_dirs);
                e
            })
        };

        try_write(&temp_dir)?;
        try_write(&self.dir)?;

        self.temp_dir = temp_dir;
        self.created_dirs = created_dirs;

        Ok(())
    }
}

#[inline(always)]
fn try_create_chain_dirs(downmost_dir: &Path) -> Result<Vec<PathBuf>, MuxError> {
    let mut dirs = Vec::<PathBuf>::new();
    let mut dir = downmost_dir;

    while !dir.exists() {
        dirs.push(dir.to_path_buf());

        if let Some(parent) = dir.parent() {
            dir = parent;
        } else {
            break;
        }
    }

    for dir in dirs.iter().rev() {
        if let Err(err) = fs::create_dir(dir) {
            remove_empty_chain_dirs(&dirs);
            return Err(err.into());
        }
    }

    Ok(dirs)
}

#[inline(always)]
fn try_write_in(path: &Path) -> Result<(), MuxError> {
    let test_file = path.join(".write_test");
    let result = fs::File::create(&test_file);
    let _ = fs::remove_file(&test_file);

    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}
