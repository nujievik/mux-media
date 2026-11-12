mod new;
mod to_args;

use crate::Result;
use std::{ffi::OsStr, fs, path::Path};

/// Output configuration.
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigOutput {
    // dir = temp_dir[0..dir_len]
    dir_len: usize,
    temp_dir: Box<Path>,
    /// Length of a created directory chain up to [`ConfigOutput::dir`].
    len_created_dir_chain: usize,
    pub(crate) is_from_input: bool,
}

impl ConfigOutput {
    pub fn dir(&self) -> &Path {
        let bytes = self.temp_dir.as_os_str().as_encoded_bytes();
        // SAFETY: `self.temp_dir` was constructed by pushing a single normal component
        // (".temp-mux-media") onto a `Path` of byte length `self.dir_len`. Therefore, the first
        // `self.dir_len` bytes are identical to the original valid `OsStr`.
        unsafe { Path::new(OsStr::from_encoded_bytes_unchecked(&bytes[..self.dir_len])) }
    }

    pub fn temp_dir(&self) -> &Path {
        &self.temp_dir
    }

    /// Creates non-exists directories in the directory chain up to `ConfigOutput::temp_dir()`.
    pub fn create_dirs(&mut self) -> Result<()> {
        let mut dirs: Vec<&Path> = Vec::new();
        let mut dir: &Path = &self.temp_dir;

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
        remove_created_dir_chain(self.dir(), self.len_created_dir_chain);
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
