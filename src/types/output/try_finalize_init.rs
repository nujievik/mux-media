use super::{Output, remove_empty_chain_dirs};
use crate::{Result, TryFinalizeInit};
use std::{
    fs,
    path::{Path, PathBuf},
};

impl TryFinalizeInit for Output {
    /// Creates a directory chain to the [`Self::temp_dir`] inside [`Self::dir`],
    /// ensuring that both are valid and writable.
    ///
    /// # Errors
    ///
    /// Returns an error and removes all created directories if:
    ///
    /// - Directory creation fails.
    /// - [`Self::dir`] or [`Self::temp_dir`] is not writable.
    fn try_finalize_init(&mut self) -> Result<()> {
        let temp_dir = self.dir.join(".temp-mux-media");
        let created_dirs = try_create_chain_dirs(&temp_dir)?;

        try_write_in(&temp_dir, &created_dirs)?;
        try_write_in(&self.dir, &created_dirs)?;

        self.temp_dir = temp_dir;
        self.created_dirs = created_dirs;
        return Ok(());

        fn try_create_chain_dirs(downmost_dir: &Path) -> Result<Vec<PathBuf>> {
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
                    if !dir.exists() {
                        remove_empty_chain_dirs(&dirs);
                        return Err(err.into());
                    }
                }
            }

            Ok(dirs)
        }

        fn try_write_in(path: &Path, created_dirs: &Vec<PathBuf>) -> Result<()> {
            let test_file = path.join(".write_test");
            let result = fs::File::create(&test_file);
            let _ = fs::remove_file(&test_file);

            match result {
                Ok(_) => Ok(()),
                Err(e) => {
                    remove_empty_chain_dirs(&created_dirs);
                    Err(e.into())
                }
            }
        }
    }
}
