use super::{ConfigInput, InputFileType, InputType, iters::DirIter};
use crate::{Extension, Msg, Result, TryFinalizeInit, display};
use std::{
    fs,
    path::{Path, PathBuf},
};

impl TryFinalizeInit for ConfigInput {
    /// Collects subdirectories of input directory. Do nothing if input is files.
    ///
    /// # Errors
    ///
    /// Returns an error if not any media in the input directory.
    fn try_finalize_init(&mut self) -> Result<()> {
        let dir = match &self.ty {
            InputType::Dir(dir) => dir,
            InputType::Files(_) => return Ok(()),
        };

        if let None = self.iter_media_in_dir(dir).next() {
            return Err(err!("{}: {}", Msg::NoInputDirMedia, display(dir)));
        }

        let skip = match &self.skip {
            Some(skip) => Some(&skip.glob_set),
            None => None,
        };

        let dirs: Vec<_> = DirIter::new(dir, self.depth as usize, skip).collect();

        self.file_dirs[InputFileType::Font] = dirs
            .iter()
            .filter(|dir| self.iter_fonts_in_dir(dir).next().is_some())
            .map(|dir| dir.clone())
            .collect();

        self.file_dirs[InputFileType::Media] = dirs
            .into_iter()
            .filter(|d| self.iter_media_in_dir(&d).next().is_some())
            .collect();

        Ok(())
    }
}

impl ConfigInput {
    pub(crate) const DEPTH_DEFAULT: u8 = 16;

    pub(crate) fn try_default_dir() -> Result<PathBuf> {
        Self::try_canonicalize_and_read(".")
    }

    /// Tries canonicalize path and read its.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The path doesn't exist or its unreadable.
    /// - The path is file with unsupported extension.
    pub(crate) fn try_canonicalize_and_read(path: impl AsRef<Path>) -> Result<PathBuf> {
        let path = fs::canonicalize(path)?;
        if path.is_dir() {
            let _ = fs::read_dir(&path)?;
        } else {
            if let None = Extension::new_from_path(&path) {
                return Err(err!("file '{}' has unsupported extension", display(&path)));
            }
            let _ = fs::File::open(&path)?;
        };
        Ok(path)
    }
}
