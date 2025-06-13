use super::{Output, remove_empty_chain_dirs};
use crate::{MuxError, TryFinalizeInit};
use std::fs;
use std::path::{Path, PathBuf};

impl TryFinalizeInit for Output {
    fn try_finalize_init(&mut self) -> Result<(), MuxError> {
        let dirs = create_chain_dirs(&self.temp_dir)?;

        let try_write = |dir| {
            try_write_in(dir).map_err(|e| {
                remove_empty_chain_dirs(&dirs);
                e
            })
        };
        try_write(&self.temp_dir)?;
        try_write(&self.dir)?;

        self.created_dirs = dirs;
        Ok(())
    }
}

#[inline]
fn create_chain_dirs(downmost_dir: &Path) -> Result<Vec<PathBuf>, MuxError> {
    let mut dirs: Vec<PathBuf> = Vec::new();
    let mut path = PathBuf::from(downmost_dir);

    while !path.exists() {
        dirs.push(path.clone());
        if let Some(parent) = path.parent() {
            path = parent.to_path_buf();
        } else {
            break;
        }
    }

    dirs.reverse();

    for dir in &dirs {
        if let Err(err) = fs::create_dir(dir) {
            remove_empty_chain_dirs(&dirs);
            return Err(err.into());
        }
    }

    Ok(dirs)
}

#[inline]
fn try_write_in(path: &Path) -> Result<(), MuxError> {
    let test_file = path.join(".write_test");
    let result = fs::File::create(&test_file);
    let _ = fs::remove_file(&test_file);

    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}
