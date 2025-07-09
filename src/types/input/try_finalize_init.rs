use super::{Input, iters::DirIter};
use crate::{Msg, MuxError, TryFinalizeInit, types::helpers::os_str_starts_with};
use rayon::prelude::*;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

impl TryFinalizeInit for Input {
    fn try_finalize_init(&mut self) -> Result<(), MuxError> {
        self.try_upd_upmost()?;
        self.upd_dirs();
        Ok(())
    }
}

impl Input {
    #[inline(always)]
    fn upd_dirs(&mut self) {
        self.dirs = DirIter::new(&self.upmost, self.down, self.skip.as_ref()).collect();
    }

    #[inline(always)]
    fn try_upd_upmost(&mut self) -> Result<(), MuxError> {
        if self.up == 0 {
            return Ok(());
        }

        let files: Vec<PathBuf> = self
            .iter_media_in_dir(&self.dir)
            .take(self.check as usize)
            .collect();

        let stems: Vec<&OsStr> = files.iter().filter_map(|path| path.file_stem()).collect();

        if stems.is_empty() {
            return Err([(Msg::NoInputMedia, format!(": {}", self.dir.display()))]
                .as_slice()
                .into());
        }

        let parent_dirs: Vec<PathBuf> = (1..=self.up)
            .scan(self.dir.clone(), |state, _| {
                let parent = state.parent()?.to_path_buf();
                *state = parent.clone();
                Some(parent)
            })
            .collect();

        let upmost = parent_dirs
            .into_par_iter()
            .find_any(|dir| self.check_dir(dir, &stems));

        if let Some(upmost) = upmost {
            self.upmost = upmost;
            self.dir_not_upmost = true;
        }

        Ok(())
    }

    #[inline(always)]
    fn check_dir(&self, dir: &Path, stems: &[&OsStr]) -> bool {
        self.iter_media_in_dir(dir)
            .take(self.check as usize)
            .any(|path| {
                path.file_stem().map_or(false, |up_stem| {
                    stems
                        .into_iter()
                        .any(|stem| os_str_starts_with(up_stem, stem))
                })
            })
    }
}
