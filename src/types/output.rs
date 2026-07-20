mod new;
mod to_args;

use crate::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Output configuration.
#[derive(Clone, Debug, PartialEq)]
pub struct Output {
    /// Output directory.
    dir: PathBuf,
    temp_dir: PathBuf,
    /// Length of a created directory chain up to [`Output::dir`].
    len_created_dir_chain: usize,
}

impl Output {
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    pub fn temp_dir(&self) -> &Path {
        &self.temp_dir
    }

    /// Creates non-exists directories in the directory chain up to `Output::temp_dir()`.
    pub fn create_dirs(&mut self) -> Result<()> {
        let mut dirs: Vec<&Path> = Vec::new();
        let mut dir = self.temp_dir.as_path();

        while !dir.exists() {
            dirs.push(dir);
            match dir.parent() {
                Some(parent) => dir = parent,
                None => break,
            }
        }

        for dir in dirs.iter().rev() {
            if let Err(err) = fs::create_dir(dir) {
                if !dir.exists() {
                    remove_created_dir_chain(&self.temp_dir, dirs.len());
                    return Err(err.into());
                }
            }
        }

        self.len_created_dir_chain = if dirs.len() > 1 { dirs.len() - 1 } else { 0 };
        Ok(())
    }

    /// Removes temp directory and created empty directories while last time call [`Self::create_dirs`].
    pub fn remove_created_dirs(&self) {
        let _ = fs::remove_dir_all(&self.temp_dir);
        remove_created_dir_chain(&self.dir, self.len_created_dir_chain);
    }
}

fn remove_created_dir_chain(mut dir: &Path, len_created_dir_chain: usize) {
    for _ in 0..len_created_dir_chain {
        let _ = fs::remove_dir(dir);
        match dir.parent() {
            Some(parent) => dir = parent,
            None => break,
        }
    }
}
