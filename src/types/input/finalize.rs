use super::{Input, InputFileType, iters::DirIter};
use crate::{Msg, Result, TryFinalizeInit};

macro_rules! collect_dirs {
    ($self:ident, $dirs:expr, $method:ident) => {
        $dirs
            .iter()
            .filter(|dir| $self.$method(dir).next().is_some())
            .map(|dir| dir.clone())
            .collect()
    };
}

impl TryFinalizeInit for Input {
    /// Collects subdirectories up to the [`Input::depth`].
    ///
    /// # Errors
    ///
    /// Returns an error if not any media in the start directory.
    fn try_finalize_init(&mut self) -> Result<()> {
        if let None = self.iter_media_in_dir(&self.dir).next() {
            return Err(
                [(Msg::NoInputDirMedia, format!(": {}", self.dir.display()))]
                    .as_slice()
                    .into(),
            );
        }

        let skip = match &self.skip {
            Some(skip) => Some(&skip.glob_set),
            None => None,
        };

        let dirs: Vec<_> = DirIter::new(&self.dir, self.depth as usize, skip).collect();

        self.dirs[InputFileType::Font] = collect_dirs!(self, dirs, iter_fonts_in_dir);
        self.dirs[InputFileType::Media] = dirs
            .into_iter()
            .filter(|d| self.iter_media_in_dir(&d).next().is_some())
            .collect();

        Ok(())
    }
}
